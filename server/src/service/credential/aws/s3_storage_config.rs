#[derive(Debug)]
pub struct S3StorageConfig {
    bucket_path: String,
    region: String,
    aws_role_arn: String,
    access_key: String,
    secret_key: String,
    session_token: String,
}

impl S3StorageConfig {
    pub fn get_bucket_path(&self) -> &str {
        &self.bucket_path
    }

    pub fn get_region(&self) -> &str {
        &self.region
    }

    pub fn get_aws_role_arn(&self) -> &str {
        &self.aws_role_arn
    }

    pub fn get_access_key(&self) -> &str {
        &self.access_key
    }

    pub fn get_secret_key(&self) -> &str {
        &self.secret_key
    }

    pub fn get_session_token(&self) -> &str {
        &self.session_token
    }
}

pub struct S3StorageConfigBuilder {
    bucket_path: Option<String>,
    region: Option<String>,
    aws_role_arn: Option<String>,
    access_key: Option<String>,
    secret_key: Option<String>,
    session_token: Option<String>,
}

impl S3StorageConfigBuilder {
    pub fn new() -> Self {
        Self {
            bucket_path: None,
            region: None,
            aws_role_arn: None,
            access_key: None,
            secret_key: None,
            session_token: None,
        }
    }

    pub fn with_bucket_path(mut self, bucket_path: &str) -> Self {
        self.bucket_path = Some(bucket_path.to_string());
        self
    }

    pub fn with_region(mut self, region: &str) -> Self {
        self.region = Some(region.to_string());
        self
    }

    pub fn with_aws_role_arn(mut self, aws_role_arn: &str) -> Self {
        self.aws_role_arn = Some(aws_role_arn.to_string());
        self
    }

    pub fn with_access_key(mut self, access_key: &str) -> Self {
        self.access_key = Some(access_key.to_string());
        self
    }

    pub fn with_secret_key(mut self, secret_key: &str) -> Self {
        self.secret_key = Some(secret_key.to_string());
        self
    }

    pub fn with_session_token(mut self, session_token: &str) -> Self {
        self.session_token = Some(session_token.to_string());
        self
    }

    pub fn build(self) -> S3StorageConfig {
        S3StorageConfig {
            bucket_path: self.bucket_path.unwrap(),
            region: self.region.unwrap(),
            aws_role_arn: self.aws_role_arn.unwrap(),
            access_key: self.access_key.unwrap(),
            secret_key: self.secret_key.unwrap(),
            session_token: self.session_token.unwrap(),
        }
    }
}
