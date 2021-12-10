use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct LocalConfig {
    pub mysql_username: String,
    pub mysql_password: String,
    pub mysql_database: String,
    pub mysql_url: Option<String>,
    pub mysql_port: Option<u16>,
    pub bind_address: String,
    pub bind_port: u16,
    pub socket_dir: Option<String>,
    pub sql_name: Option<String>,
    pub max_req: u16,
    pub jwt_key: String
}
