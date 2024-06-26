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
use core::fmt::Display;
extern crate dirs;

/// Public functions

/// Returns a config file Path 
pub fn get_config_file() -> PathBuf {
    let config_dir = dirs::config_local_dir().unwrap();
    let config_file = config_dir.join("btcdb").join("config.toml");
    config_file
}

/// Returns the complete configuration Figment
pub fn get_config() -> Config {
    Config::default()   
}

/// Returns the sql configuration
pub fn get_sqlconfig() -> SQLConfig {
    SQLConfig::figment().extract().unwrap()
}

/// Returns the Bitcoin RCP server configuration Figment
pub fn get_rpcconfig() -> RPCConfig {
    RPCConfig::figment().extract().unwrap()
}

/// structs
/// 
#[derive(Debug)]
pub struct Config {
    pub sql: SQLConfig,
    pub rpc: RPCConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            sql: get_sqlconfig(),
            rpc: get_rpcconfig(),
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

impl SQLConfig {
    pub fn figment() -> Figment {
        let defaultfig = Figment::from(SQLConfig::default());
        let filefig = Figment::from(Toml::file(get_config_file()))
                .focus("sql");
        defaultfig.merge(filefig)
    }
}

impl Provider for SQLConfig {
    fn metadata(&self) -> Metadata {
        Metadata::named("btcdb SQL Config")
    }

    fn data(&self) -> Result<Map<Profile, Dict>, Error>  {
        figment::providers::Serialized::defaults(SQLConfig::default())
        .data()
    }

    fn profile(&self) -> Option<Profile> {
        // Optionally, a profile that's selected by default.
        Some("sql".into())
    }
}

impl Display for SQLConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        println!("SQL Configuration:");
        write!(f, "  host: {}\n  dbname: {}\n  schema: {}\n  username: {}\n  password: {}", self.host, self.dbname, self.schema, self.username, self.password)
    }
}

/// Configuration items for Bitcoin RPC.
#[derive(Debug, Deserialize, Serialize)]
pub struct RPCConfig {
    pub url: String,
    pub username: String,
    pub password: String,
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

impl RPCConfig {
    pub fn figment() -> Figment {
        let defaultfig = Figment::from(RPCConfig::default());  
        let filefig = Figment::from(Toml::file(get_config_file()))
            .focus("rpc");
        defaultfig.merge(filefig)
    }
}

impl Provider for RPCConfig {
    fn metadata(&self) -> Metadata {
        Metadata::named("btcdb RPC Config")
    }

    fn data(&self) -> Result<Map<Profile, Dict>, Error>  {
        figment::providers::Serialized::defaults(RPCConfig::default())
        .data()
    }

    fn profile(&self) -> Option<Profile> {
        // Optionally, a profile that's selected by default.
        Some("rpc".into())
    }
}

impl Display for RPCConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        println!("RPC Configuration:");
        write!(f, "  url: {}\n  username: {}\n  password: {}", self.url, self.username, self.password)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let config = get_config();
        println!("{:?}", config);
        assert_eq!(config.sql.host, "localhost");
        assert_eq!(config.rpc.url, "http://localhost:8332");
    }

    #[test]
    fn test_config_file() {
        let config_file = get_config_file();
        println!("{:?}", config_file);
        assert!(config_file.ends_with("config.toml"));
        assert!(config_file.exists());
        assert!(config_file.is_file());
    }

    #[test]
    fn test_sqlconfig_default() {
        let sql_config = SQLConfig::default();
        assert_eq!(sql_config.host, "localhost");
        assert_eq!(sql_config.dbname, "bitcoin");
        assert_eq!(sql_config.schema, "public");
        assert_eq!(sql_config.username, "rpc");
        assert_eq!(sql_config.password, "YOURPASSWORD");
    }

    #[test]
    fn test_rpcconfig_default() {
        let rpc_config = RPCConfig::default();
        assert_eq!(rpc_config.url, "http://localhost:8332");
        assert_eq!(rpc_config.username, "YOURUSERNAME");
        assert_eq!(rpc_config.password, "YOURPASSWORD");
    }

    #[test]
    fn test_sqlconfig_figment() {
        let sql_config: SQLConfig = SQLConfig::figment().extract().unwrap();
        assert_eq!(sql_config.host, "localhost");
        assert_eq!(sql_config.dbname, "bitcoin");
        assert_eq!(sql_config.schema, "public");
        assert_eq!(sql_config.username, "rpc");
        assert_eq!(sql_config.password, "YOURPASSWORD");
    }
}
