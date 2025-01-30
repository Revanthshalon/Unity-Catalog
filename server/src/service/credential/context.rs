#![allow(unused)]

use std::{collections::HashSet, str::FromStr};

use crate::errors::UnityCatalogResult;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Privilege {
    Select,
    Update,
}

#[derive(Debug)]
pub struct CredentailContext {
    pub storage_scheme: String,
    pub storage_base: String,
    pub privileges: HashSet<Privilege>,
    pub locations: Vec<String>,
}

impl CredentailContext {
    pub fn new(location_uri: &str, privileges: HashSet<Privilege>) -> UnityCatalogResult<Self> {
        use url::Url;

        let parsed_uri = Url::from_str(location_uri)?;
        let storage_scheme = parsed_uri.scheme().to_string();
        let storage_base = format!("{}://{}", storage_scheme, parsed_uri.host_str().unwrap());
        let locations = vec![location_uri.to_string()];

        Ok(CredentailContext {
            storage_scheme,
            storage_base,
            privileges,
            locations,
        })
    }

    pub fn get_storage_schema(&self) -> &str {
        &self.storage_scheme
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn credential_context_works() {
        let mut privileges = HashSet::new();
        privileges.insert(Privilege::Update);
        privileges.insert(Privilege::Select);

        let context = CredentailContext::new("https://example.com/data", privileges);

        assert!(context.is_ok());

        let context = context.unwrap();

        assert_eq!(context.get_storage_schema(), "https");
    }
}
