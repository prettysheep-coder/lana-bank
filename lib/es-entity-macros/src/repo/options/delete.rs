use darling::FromMeta;

#[derive(Debug, FromMeta)]
pub enum DeleteOption {
    No,
    Soft,
}

impl Default for DeleteOption {
    fn default() -> Self {
        DeleteOption::No
    }
}

impl std::str::FromStr for DeleteOption {
    type Err = darling::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "no" => Ok(DeleteOption::No),
            "soft" => Ok(DeleteOption::Soft),
            _ => Err(darling::Error::unknown_value(s)),
        }
    }
}
