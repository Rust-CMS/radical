use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct LocalConfig {
    pub mysql_username: String,
    pub mysql_password: String,
    pub mysql_database: String,
    pub mysql_url: String,
    pub mysql_port: u16,
    pub bind_address: String,
    pub bind_port: u16,
    pub max_req: u16
}