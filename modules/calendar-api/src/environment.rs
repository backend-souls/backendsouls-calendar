#[derive(Clone, Copy)]
pub enum Environment {
    QA,
    LOCAL,
    STAGING,
    PRODUCTION,
    DEVELOPMENT,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::QA => "qa",
            Environment::LOCAL => "local",
            Environment::STAGING => "staging",
            Environment::PRODUCTION => "production",
            Environment::DEVELOPMENT => "development",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "qa" => Ok(Self::QA),
            "local" => Ok(Self::LOCAL),
            "staging" => Ok(Self::STAGING),
            "production" => Ok(Self::PRODUCTION),
            "development" => Ok(Self::DEVELOPMENT),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}
