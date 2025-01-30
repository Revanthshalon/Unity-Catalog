use serde::Deserialize;

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct ADLSConfig {
    #[serde(alias = "storageAccountName")]
    storage_account_name: Option<String>,
    #[serde(alias = "tenantId")]
    tenant_id: Option<String>,
    #[serde(alias = "clientId")]
    client_id: Option<String>,
    #[serde(alias = "clientSecret")]
    client_secret: Option<String>,
}
