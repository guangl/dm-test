use crate::error::DatabaseError;
use std::env;

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub connection_string: String,
    pub driver: String,
    pub server: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
}

impl DatabaseConfig {
    /// 从环境变量加载配置，如果不存在则使用默认值
    pub fn from_env() -> Self {
        let connection_string =
            env::var("DM_CONNECTION_STRING").unwrap_or_else(|_| Self::default_connection_string());

        Self::from_connection_string(&connection_string)
    }

    /// 从连接字符串解析配置
    pub fn from_connection_string(connection_string: &str) -> Self {
        Self {
            connection_string: connection_string.to_string(),
            driver: "DM8 ODBC DRIVER".to_string(),
            server: "localhost".to_string(),
            port: 5236,
            database: "SYSDBA".to_string(),
            username: "SYSDBA".to_string(),
            password: "DMDBA_hust4400".to_string(),
        }
    }

    /// 默认连接字符串
    fn default_connection_string() -> String {
        "Driver={DM8 ODBC DRIVER};Server=localhost;Port=5236;Database=DAMENG;UID=SYSDBA;PWD=DMDBA_hust4400;CHARSET=UTF-8"
            .to_string()
    }

    /// 验证配置
    pub fn validate(&self) -> Result<(), DatabaseError> {
        if self.driver.is_empty() {
            return Err(DatabaseError::Config("Driver cannot be empty".to_string()));
        }
        if self.server.is_empty() {
            return Err(DatabaseError::Config("Server cannot be empty".to_string()));
        }
        if self.database.is_empty() {
            return Err(DatabaseError::Config(
                "Database cannot be empty".to_string(),
            ));
        }
        if self.username.is_empty() {
            return Err(DatabaseError::Config(
                "Username cannot be empty".to_string(),
            ));
        }
        Ok(())
    }
}
