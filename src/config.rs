use figment::{
    Figment, 
    Provider, 
    Error, 
    Metadata, 
    Profile,
    providers::{Format, Toml},
    value::{Map, Dict}
};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
extern crate dirs;

/// Public functions

/// Returns a config file Path 
pub fn get_config_file() -> PathBuf {
    let config_dir = dirs::config_local_dir().unwrap();
    let config_file = config_dir.join("config.toml");
    config_file
}

/// Returns the complete configuration Figment
pub fn get_config() -> Config {
    Config::default()   
}

/// Returns the sql configuration Figment
pub fn get_sqlconfig() -> SQLConfig {
    get_config().sql.extract().unwrap()
}

/// Returns the Bitcoin RCP server configuration Figment
pub fn get_rpcconfig() -> Figment {
    get_config().rpc

}

/// structs
/// 
#[derive(Debug)]
pub struct Config {
    pub sql: Figment,
    pub rpc: Figment,
}

// merge(SQLConfig::default()).

impl Default for Config {
    fn default() -> Self {
        Config {
            sql: Figment::from(SQLConfig::default()).merge(get_sqlconfig()),
            rpc: Figment::from(RPCConfig::default()).merge(get_rpcconfig()),
        }
    }
}

/// Configuration items for the local SQL data store.
#[derive(Debug, Deserialize, Serialize)]
pub struct SQLConfig {
    pub host: String,
    pub dbname: String,
    pub schema: String,
    pub username: String,
    pub password: String,
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

impl Provider for SQLConfig {
    fn metadata(&self) -> Metadata {
        Metadata::named("btcdb SQL Config")
    }

    fn data(&self) -> Result<Map<Profile, Dict>, Error>  {
        figment::providers::Serialized::defaults(SQLConfig::default()).data()
    }

    fn profile(&self) -> Option<Profile> {
        // Optionally, a profile that's selected by default.
        Some("sql".into())
    }
}

/// Configuration items for Bitcoin RPC.
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

impl Provider for RPCConfig {
    fn metadata(&self) -> Metadata {
        Metadata::named("btcdb RPC Config")
    }

    fn data(&self) -> Result<Map<Profile, Dict>, Error>  {
        figment::providers::Serialized::defaults(RPCConfig::default()).data()
    }

    fn profile(&self) -> Option<Profile> {
        // Optionally, a profile that's selected by default.
        Some("rpc".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        // let config = get_config();
        // println!("{:?}", config);
        // let sqlconfig = get_sqlconfig();
        // println!("{:?}", sqlconfig);
        let rpcconfig = get_rpcconfig();
        println!("{:?}", rpcconfig);
    }
}
