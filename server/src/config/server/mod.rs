#![allow(dead_code)]

mod adls;
mod gcs;
mod s3;

use std::collections::HashMap;

use adls::ADLSConfig;
use gcs::GCSConfig;
use s3::S3Config;
use serde::Deserialize;

use crate::service::credential::aws::{S3StorageConfig, S3StorageConfigBuilder};

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    environment: String,
    authorization: ServerAuthorization,
    #[serde(alias = "authorization-url")]
    authorization_url: Option<String>,
    #[serde(alias = "token-url")]
    token_url: Option<String>,
    #[serde(alias = "client-id")]
    client_id: Option<String>,
    #[serde(alias = "client-secret")]
    client_secret: Option<String>,
    #[serde(alias = "redirect-port")]
    redirect_port: Option<u16>,
    #[serde(alias = "cookie-timeout")]
    cookie_timeout: Option<String>,

    /// Cloud storage or file based allowed.
    /// If no root specified, the current working directory of the server is used.
    ///
    /// * storage-root = s3://my-s3-bucket/root
    #[serde(alias = "storage-root")]
    storage_root: Option<String>,

    /// s3 Configuration
    s3: Option<Vec<S3Config>>,

    /// gcs Configuration
    gcs: Option<Vec<GCSConfig>>,

    /// adls Configuration
    adls: Option<Vec<ADLSConfig>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServerAuthorization {
    Enable,
    Disable,
}

impl ServerConfig {
    pub fn get_s3_configurations(&self) -> HashMap<&str, S3StorageConfig> {
        let mut s3_bucket_config_map = HashMap::new();
        if let Some(s3_configs) = &self.s3 {
            for s3_config in s3_configs {
                let bucket_path = s3_config.bucket_path.as_ref().unwrap();

                let s3_storage_config = S3StorageConfigBuilder::new()
                    .with_bucket_path(s3_config.bucket_path.as_ref().unwrap())
                    .with_region(s3_config.region.as_ref().unwrap())
                    .with_aws_role_arn(s3_config.aws_role_arn.as_ref().unwrap())
                    .with_access_key(s3_config.access_key.as_ref().unwrap())
                    .with_secret_key(s3_config.secret_key.as_ref().unwrap())
                    .with_session_token(s3_config.session_token.as_ref().unwrap())
                    .build();

                s3_bucket_config_map.insert(bucket_path.as_str(), s3_storage_config);
            }
        }
        s3_bucket_config_map
    }
}
