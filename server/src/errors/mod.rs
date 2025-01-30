use config::ConfigError;

#[allow(unused)]
pub type UnityCatalogResult<T> = Result<T, UnityCatalogError>;

#[derive(Debug)]
pub enum UnityCatalogError {
    Config(ConfigError),
    Url(url::ParseError),
}

impl std::fmt::Display for UnityCatalogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnityCatalogError::Config(e) => write!(f, "Configration Error: {}", e),
            UnityCatalogError::Url(e) => write!(f, "Url Parse Error: {}", e),
        }
    }
}

impl std::error::Error for UnityCatalogError {}

impl From<ConfigError> for UnityCatalogError {
    fn from(value: ConfigError) -> Self {
        UnityCatalogError::Config(value)
    }
}

impl From<url::ParseError> for UnityCatalogError {
    fn from(value: url::ParseError) -> Self {
        UnityCatalogError::Url(value)
    }
}
