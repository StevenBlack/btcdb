use serde::{Serialize, Deserialize};
use figment::{Figment, providers::{Env, Format, Toml, Serialized}};

/// The fields required to address the local SQL data store.

#[derive(Debug, Deserialize, Serialize)]
pub struct SQLConfig {
    pub(crate) host: String,
    pub(crate) dbname: String,
    pub(crate) schema: String,
    pub(crate) username: String,
    pub(crate) password: String,
}

impl Default for SQLConfig {
    fn default() -> Self {
        SQLConfig {
            host: "localhost".to_string(),
            dbname: "bitcoin".to_string(),
            schema: "public".to_string(),
            username: "rpc".to_string(),
            password: "YOURPASSWORD".to_string(),
        }
    }
}

pub fn get_sqlconfig() -> Figment {
    Figment::from(Serialized::defaults(SQLConfig::default()))
    .merge(Toml::file("App.toml"))
    .merge(Env::prefixed("APP_"))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RPCConfig {
    pub(crate) url: String,
    pub(crate) username: String,
    pub(crate) password: String,
}

impl Default for RPCConfig {
    fn default() -> Self {
        RPCConfig {
            url: "http://localhost:8332".to_string(), 
            username: "YOURUSERNAME".to_string(), 
            password: "YOURPASSWORD".to_string(),
        }
    }
}

pub fn get_rpcconfig() -> Figment {
    Figment::from(Serialized::defaults(RPCConfig::default()))
    .merge(Toml::file("App.toml"))
    .merge(Env::prefixed("APP_"))
}