use serde::Deserialize;

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    #[serde(alias = "database-type")]
    database_type: DatabaseType,
    username: String,
    password: String,
    host: String,
    port: u16,
    #[serde(alias = "database-name")]
    database_name: String,
    #[serde(alias = "max-idle-connections")]
    max_idle_connections: u32,
    #[serde(alias = "max-open-connections")]
    max_open_connections: u32,
    #[serde(alias = "connection-timeout")]
    connection_timeout: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DatabaseType {
    Postgres,
    MySql,
}
