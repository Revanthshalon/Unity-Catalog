#![allow(unused)]

use std::collections::HashMap;

use crate::config::ServerConfig;
use crate::errors::UnityCatalogResult;
use crate::service::credential::context::CredentailContext;

use super::S3StorageConfig;

pub struct AwsCredentialVendor<'a> {
    s3_configurations: HashMap<&'a str, S3StorageConfig>,
}

impl<'a> AwsCredentialVendor<'a> {
    pub fn new(server_config: &'a ServerConfig) -> UnityCatalogResult<Self> {
        Ok(Self {
            s3_configurations: server_config.get_s3_configurations(),
        })
    }

    pub fn vend_aws_credentials(&self, context: CredentailContext) -> () {
        let s3_config = self.s3_configurations.get(context.storage_base.as_str());
        if let Some(config) = s3_config {}
    }
}
