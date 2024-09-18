mod config;
pub mod error;
mod job;
mod repo;
mod sumsub_auth;
pub mod sumsub_public;

use job::{SumsubExportConfig, SumsubExportInitializer};
use serde::{Deserialize, Serialize};

use crate::{
    customer::Customers,
    data_export::Export,
    job::Jobs,
    primitives::{CustomerId, JobId},
};

pub use config::*;
use error::ApplicantError;
use sumsub_auth::*;

use repo::ApplicantRepo;

#[derive(Clone)]
pub struct Applicants {
    pool: sqlx::PgPool,
    sumsub_client: SumsubClient,
    users: Customers,
    repo: ApplicantRepo,
    jobs: Jobs,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ReviewAnswer {
    Green,
    Red,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SumsubKycLevel {
    BasicKycLevel,
    AdvancedKycLevel,
}

impl std::fmt::Display for SumsubKycLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SumsubKycLevel::BasicKycLevel => write!(f, "basic-kyc-level"),
            SumsubKycLevel::AdvancedKycLevel => write!(f, "advanced-kyc-level"),
        }
    }
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(tag = "type")]
pub enum SumsubCallbackPayload {
    #[serde(rename = "applicantCreated")]
    #[serde(rename_all = "camelCase")]
    ApplicantCreated {
        applicant_id: String,
        inspection_id: String,
        correlation_id: String,
        level_name: SumsubKycLevel,
        external_user_id: CustomerId,
        review_status: String,
        created_at_ms: String,
        client_id: Option<String>,
    },
    #[serde(rename = "applicantPending")]
    #[serde(rename_all = "camelCase")]
    ApplicantPending {
        applicant_id: String,
        inspection_id: String,
        applicant_type: Option<String>,
        correlation_id: String,
        level_name: SumsubKycLevel,
        external_user_id: CustomerId,
        review_status: String,
        created_at_ms: String,
        client_id: Option<String>,
    },
    #[serde(rename = "applicantReviewed")]
    #[serde(rename_all = "camelCase")]
    ApplicantReviewed {
        applicant_id: String,
        inspection_id: String,
        correlation_id: String,
        external_user_id: CustomerId,
        level_name: SumsubKycLevel,
        review_result: ReviewResult,
        review_status: String,
        created_at_ms: String,
    },
    #[serde(rename = "applicantOnHold")]
    #[serde(rename_all = "camelCase")]
    ApplicantOnHold {
        applicant_id: String,
        inspection_id: String,
        applicant_type: Option<String>,
        correlation_id: String,
        level_name: SumsubKycLevel,
        external_user_id: CustomerId,
        review_result: ReviewResult,
        review_status: String,
        created_at_ms: String,
        client_id: Option<String>,
    },
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewResult {
    pub review_answer: ReviewAnswer,
    pub moderation_comment: Option<String>,
    pub client_comment: Option<String>,
    pub reject_labels: Option<Vec<String>>,
    pub review_reject_type: Option<String>,
}

impl Applicants {
    pub fn new(
        pool: &sqlx::PgPool,
        config: &SumsubConfig,
        users: &Customers,
        jobs: &Jobs,
        export: &Export,
    ) -> Self {
        let sumsub_client = SumsubClient::new(config);
        jobs.add_initializer(SumsubExportInitializer::new(
            export.clone(),
            sumsub_client.clone(),
        ));

        Self {
            repo: ApplicantRepo::new(pool, export),
            pool: pool.clone(),
            sumsub_client,
            users: users.clone(),
            jobs: jobs.clone(),
        }
    }

    pub async fn handle_callback(&self, payload: serde_json::Value) -> Result<(), ApplicantError> {
        let customer_id: CustomerId = payload["externalUserId"]
            .as_str()
            .ok_or_else(|| ApplicantError::UnhandledCallbackType(payload.to_string()))?
            .parse()?;

        let _ = &self
            .repo
            .persist_webhook(customer_id, payload.clone())
            .await?;

        self.process_payload(payload).await
    }

    async fn process_payload(&self, payload: serde_json::Value) -> Result<(), ApplicantError> {
        match serde_json::from_value(payload)? {
            SumsubCallbackPayload::ApplicantCreated {
                external_user_id,
                applicant_id,
                ..
            } => {
                self.users.start_kyc(external_user_id, applicant_id).await?;
            }
            SumsubCallbackPayload::ApplicantReviewed {
                external_user_id,
                review_result:
                    ReviewResult {
                        review_answer: ReviewAnswer::Red,
                        ..
                    },
                applicant_id,
                ..
            } => {
                self.users
                    .deactivate(external_user_id, applicant_id)
                    .await?;
            }
            SumsubCallbackPayload::ApplicantReviewed {
                external_user_id,
                review_result:
                    ReviewResult {
                        review_answer: ReviewAnswer::Green,
                        ..
                    },
                applicant_id,
                level_name: SumsubKycLevel::BasicKycLevel,
                ..
            } => {
                self.users
                    .approve_basic(external_user_id, applicant_id)
                    .await?;

                // db_tx should also be in approve basic?
                let mut db_tx = self.pool.begin().await?;

                self.jobs
                    .create_and_spawn_job::<SumsubExportInitializer, _>(
                        &mut db_tx,
                        JobId::new(),
                        // does job_name need to be unique?
                        // this won't work if that is a requirement
                        // ie: if multiple exports are needed for a same customer
                        format!("sumsub-export:{}", external_user_id),
                        SumsubExportConfig {
                            customer_id: external_user_id,
                        },
                    )
                    .await?;
            }
            SumsubCallbackPayload::ApplicantReviewed {
                review_result:
                    ReviewResult {
                        review_answer: ReviewAnswer::Green,
                        ..
                    },
                level_name: SumsubKycLevel::AdvancedKycLevel,
                ..
            } => {
                return Err(ApplicantError::UnhandledCallbackType(
                    "Advanced KYC level is not supported".to_string(),
                ));
            }
            SumsubCallbackPayload::ApplicantOnHold {
                external_user_id, ..
            }
            | SumsubCallbackPayload::ApplicantPending {
                external_user_id, ..
            } => {
                return Err(ApplicantError::UnhandledCallbackType(format!(
                    "callback event not processed for {external_user_id}"
                )));
            }
        }
        Ok(())
    }

    pub async fn create_access_token(
        &self,
        user_id: CustomerId,
    ) -> Result<AccessTokenResponse, ApplicantError> {
        let client = reqwest::Client::new();

        let level_name = SumsubKycLevel::BasicKycLevel;

        self.sumsub_client
            .create_access_token(&client, user_id, &level_name.to_string())
            .await
    }

    pub async fn create_permalink(
        &self,
        user_id: CustomerId,
    ) -> Result<PermalinkResponse, ApplicantError> {
        let client = reqwest::Client::new();

        let level_name = SumsubKycLevel::BasicKycLevel;

        self.sumsub_client
            .create_permalink(&client, user_id, &level_name.to_string())
            .await
    }
}
