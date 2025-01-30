use serde::Deserialize;

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct GCSConfig {
    #[serde(alias = "bucketPath")]
    bucket_path: Option<String>,
    #[serde(alias = "jsonKeyFilePath")]
    json_key_file_path: Option<String>,
}
