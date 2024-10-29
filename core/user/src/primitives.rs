use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt::Display, str::FromStr};

pub use audit::AuditInfo;
pub use shared_primitives::{AllOrOne, UserId};

#[derive(Clone, Eq, Hash, PartialEq, Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct Role(Cow<'static, str>);
impl Role {
    pub const fn new(job_type: &'static str) -> Self {
        Role(Cow::Borrowed(job_type))
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Debug)]
pub struct SuperuserInit {
    pub email: String,
    pub role: Role,
}

#[derive(Clone, Copy, Debug, PartialEq, strum::EnumDiscriminants)]
#[strum_discriminants(derive(strum::Display, strum::EnumString))]
#[strum_discriminants(strum(serialize_all = "kebab-case"))]
pub enum UserModuleAction {
    User(UserEntityAction),
}

#[derive(PartialEq, Clone, Copy, Debug, strum::Display, strum::EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum UserEntityAction {
    Read,
    Create,
    List,
    Update,
    AssignRole,
    RevokeRole,
}

impl Display for UserModuleAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:", UserModuleActionDiscriminants::from(self))?;
        use UserModuleAction::*;
        match self {
            User(action) => action.fmt(f),
        }
    }
}

impl FromStr for UserModuleAction {
    type Err = strum::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (entity, action) = s.split_once(':').expect("missing colon");
        use UserModuleActionDiscriminants::*;
        let res = match entity.parse()? {
            User => UserModuleAction::from(action.parse::<UserEntityAction>()?),
        };
        Ok(res)
    }
}

impl From<UserEntityAction> for UserModuleAction {
    fn from(action: UserEntityAction) -> Self {
        UserModuleAction::User(action)
    }
}

pub type UserAllOrOne = AllOrOne<UserId>;

#[derive(Clone, Copy, Debug, PartialEq, strum::EnumDiscriminants)]
#[strum_discriminants(derive(strum::Display, strum::EnumString))]
#[strum_discriminants(strum(serialize_all = "kebab-case"))]
pub enum UserObject {
    User(UserAllOrOne),
}

impl Display for UserObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let discriminant = UserObjectDiscriminants::from(self);
        use UserObject::*;
        match self {
            User(obj_ref) => write!(f, "{}/{}", discriminant, obj_ref),
        }
    }
}

impl FromStr for UserObject {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (entity, id) = s.split_once('/').expect("missing slash");
        use UserObjectDiscriminants::*;
        let res = match entity.parse().expect("invalid entity") {
            User => {
                let obj_ref = id.parse().map_err(|_| "could not parse UserObject")?;
                UserObject::User(obj_ref)
            }
        };
        Ok(res)
    }
}
