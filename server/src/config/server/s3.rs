use serde::Deserialize;

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct S3Config {
    #[serde(alias = "bucketPath")]
    pub bucket_path: Option<String>,
    pub region: Option<String>,
    #[serde(alias = "awsRoleArn")]
    pub aws_role_arn: Option<String>,
    #[serde(alias = "accessKey")]
    pub access_key: Option<String>,
    #[serde(alias = "secretKey")]
    pub secret_key: Option<String>,
    #[serde(alias = "sessionToken")]
    pub session_token: Option<String>,
}
