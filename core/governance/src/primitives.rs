use std::{fmt::Display, str::FromStr};

pub use shared_primitives::{AllOrOne, CommitteeId, PolicyId, UserId};

#[derive(Clone, Copy, Debug, PartialEq, strum::EnumDiscriminants)]
#[strum_discriminants(derive(strum::Display, strum::EnumString))]
#[strum_discriminants(strum(serialize_all = "kebab-case"))]
pub enum GovernanceAction {
    Committee(CommitteeAction),
    Policy(PolicyAction),
}

impl Display for GovernanceAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:", GovernanceActionDiscriminants::from(self))?;
        use GovernanceAction::*;
        match self {
            Committee(action) => action.fmt(f),
            Policy(action) => action.fmt(f),
        }
    }
}

impl FromStr for GovernanceAction {
    type Err = strum::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (entity, action) = s.split_once(':').expect("missing colon");
        use GovernanceActionDiscriminants::*;
        let res = match entity.parse()? {
            Committee => GovernanceAction::from(action.parse::<CommitteeAction>()?),
            Policy => GovernanceAction::from(action.parse::<PolicyAction>()?),
        };
        Ok(res)
    }
}

#[derive(PartialEq, Clone, Copy, Debug, strum::Display, strum::EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum CommitteeAction {
    Create,
}

#[derive(PartialEq, Clone, Copy, Debug, strum::Display, strum::EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum PolicyAction {
    Create,
}

pub type CommitteeAllOrOne = AllOrOne<CommitteeId>;
pub type PolicyAllOrOne = AllOrOne<PolicyId>;

#[derive(Clone, Copy, Debug, PartialEq, strum::EnumDiscriminants)]
#[strum_discriminants(derive(strum::Display, strum::EnumString))]
#[strum_discriminants(strum(serialize_all = "kebab-case"))]
pub enum GovernanceObject {
    Committee(CommitteeAllOrOne),
    Policy(PolicyAllOrOne),
}

impl Display for GovernanceObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let discriminant = GovernanceObjectDiscriminants::from(self);
        use GovernanceObject::*;
        match self {
            Committee(obj_ref) => write!(f, "{}/{}", discriminant, obj_ref),
            Policy(obj_ref) => write!(f, "{}/{}", discriminant, obj_ref),
        }
    }
}

impl FromStr for GovernanceObject {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (entity, id) = s.split_once('/').expect("missing slash");
        use GovernanceObjectDiscriminants::*;
        let res = match entity.parse().expect("invalid entity") {
            Committee => {
                let obj_ref = id.parse().map_err(|_| "could not parse GovernanceObject")?;
                GovernanceObject::Committee(obj_ref)
            }
            Policy => {
                let obj_ref = id.parse().map_err(|_| "could not parse GovernanceObject")?;
                GovernanceObject::Policy(obj_ref)
            }
        };
        Ok(res)
    }
}

impl From<CommitteeAction> for GovernanceAction {
    fn from(action: CommitteeAction) -> Self {
        GovernanceAction::Committee(action)
    }
}

impl From<PolicyAction> for GovernanceAction {
    fn from(action: PolicyAction) -> Self {
        GovernanceAction::Policy(action)
    }
}

pub(crate) fn g_action(a: impl Into<GovernanceAction>) -> GovernanceAction {
    a.into()
}
