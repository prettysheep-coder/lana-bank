use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthorizationConfig {
    #[serde(default = "default_seed_permissions")]
    pub seed_roles: bool,
    #[serde(default = "default_super_user_email")]
    pub super_user_email: String,
}

impl Default for AuthorizationConfig {
    fn default() -> Self {
        Self {
            seed_roles: default_seed_permissions(),
            super_user_email: default_super_user_email(),
        }
    }
}

fn default_seed_permissions() -> bool {
    true
}

fn default_super_user_email() -> String {
    "super.user@example.com".to_string()
}
