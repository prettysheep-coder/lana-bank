use chrono::{DateTime, Utc};

use crate::primitives::*;

use super::{Loan, LoanCollaterizationState, LoanEvent};

pub struct IncrementalPayment {
    pub cents: UsdCents,
    pub recorded_at: DateTime<Utc>,
    pub tx_id: LedgerTxId,
}

pub enum LoanHistory {
    Payment(IncrementalPayment),
    Interest(InterestAccrued),
    Collateral(CollateralUpdated),
    Origination(LoanOrigination),
    Collateralization(CollateralizationUpdated),
}

pub struct InterestAccrued {
    pub cents: UsdCents,
    pub recorded_at: DateTime<Utc>,
    pub tx_id: LedgerTxId,
}

pub struct CollateralUpdated {
    pub satoshis: Satoshis,
    pub recorded_at: DateTime<Utc>,
    pub action: CollateralAction,
    pub tx_id: LedgerTxId,
}

pub struct LoanOrigination {
    pub cents: UsdCents,
    pub recorded_at: DateTime<Utc>,
    pub tx_id: LedgerTxId,
}

pub struct CollateralizationUpdated {
    pub state: LoanCollaterizationState,
    pub collateral: Satoshis,
    pub outstanding_interest: UsdCents,
    pub outstanding_principal: UsdCents,
    pub recorded_at: DateTime<Utc>,
    pub price: PriceOfOneBTC,
}

pub struct LoanProjection<'a> {
    loan: &'a Loan,
}

impl<'a> LoanProjection<'a> {
    pub(super) fn new(loan: &'a Loan) -> Self {
        Self { loan }
    }

    pub fn history(&self) -> Vec<LoanHistory> {
        let mut history = vec![];

        for event in self.loan.events.iter().rev() {
            match event {
                LoanEvent::CollateralUpdated {
                    abs_diff,
                    action,
                    recorded_at,
                    tx_id,
                    ..
                } => match action {
                    CollateralAction::Add => {
                        history.push(LoanHistory::Collateral(CollateralUpdated {
                            satoshis: *abs_diff,
                            action: *action,
                            recorded_at: *recorded_at,
                            tx_id: *tx_id,
                        }));
                    }
                    CollateralAction::Remove => {
                        history.push(LoanHistory::Collateral(CollateralUpdated {
                            satoshis: *abs_diff,
                            action: *action,
                            recorded_at: *recorded_at,
                            tx_id: *tx_id,
                        }));
                    }
                },

                LoanEvent::InterestIncurred {
                    amount,
                    recorded_at,
                    tx_id,
                    ..
                } => {
                    history.push(LoanHistory::Interest(InterestAccrued {
                        cents: *amount,
                        recorded_at: *recorded_at,
                        tx_id: *tx_id,
                    }));
                }

                LoanEvent::PaymentRecorded {
                    principal_amount,
                    interest_amount,
                    recorded_at: transaction_recorded_at,
                    tx_id,
                    ..
                } => {
                    history.push(LoanHistory::Payment(IncrementalPayment {
                        cents: *principal_amount + *interest_amount,
                        recorded_at: *transaction_recorded_at,
                        tx_id: *tx_id,
                    }));
                }

                LoanEvent::Approved {
                    tx_id, recorded_at, ..
                } => {
                    history.push(LoanHistory::Origination(LoanOrigination {
                        cents: self.loan.initial_principal(),
                        recorded_at: *recorded_at,
                        tx_id: *tx_id,
                    }));
                }

                LoanEvent::CollateralizationChanged {
                    state,
                    collateral,
                    outstanding,
                    price,
                    recorded_at,
                    ..
                } => {
                    history.push(LoanHistory::Collateralization(CollateralizationUpdated {
                        state: *state,
                        collateral: *collateral,
                        outstanding_interest: outstanding.interest,
                        outstanding_principal: outstanding.principal,
                        price: *price,
                        recorded_at: *recorded_at,
                    }));
                }
                _ => {}
            }
        }

        history
    }
}
