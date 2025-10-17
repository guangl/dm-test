use crate::config::DatabaseConfig;
use crate::error::DatabaseError;
use encoding_rs::GBK;
use odbc_api::{
    Connection, ConnectionOptions, Cursor, Environment,
    buffers::{TextColumn, TextRowSet},
};
use sqllogictest::{AsyncDB, DBOutput};

pub struct DmDatabase {
    environment: Environment,
    config: DatabaseConfig,
}

impl DmDatabase {
    /// 创建新的数据库实例
    pub fn new(config: DatabaseConfig) -> Result<Self, DatabaseError> {
        config.validate()?;

        let environment = Environment::new().map_err(|e| {
            DatabaseError::Connection(format!("Failed to create environment: {}", e))
        })?;

        Ok(Self {
            environment,
            config,
        })
    }

    /// 获取数据库连接
    pub fn get_connection(&self) -> Result<Connection<'_>, DatabaseError> {
        self.environment
            .connect_with_connection_string(
                &self.config.connection_string,
                ConnectionOptions::default(),
            )
            .map_err(|e| DatabaseError::Connection(format!("Failed to connect: {}", e)))
    }

    /// 执行查询并返回结果
    pub fn execute_query(&self, sql: &str) -> Result<Vec<Vec<String>>, DatabaseError> {
        let connection = self.get_connection()?;

        if let Some(cursor) = connection
            .execute(sql, (), None)
            .map_err(|e| DatabaseError::Query(format!("Failed to execute query: {}", e)))?
        {
            self.fetch_results(cursor)
        } else {
            Ok(Vec::new())
        }
    }

    /// 执行语句（非查询）
    pub fn execute_statement(&self, sql: &str) -> Result<u64, DatabaseError> {
        let connection = self.get_connection()?;

        match connection.execute(sql, (), None) {
            Ok(_) => Ok(0),
            Err(e) => Err(DatabaseError::Query(format!(
                "Failed to execute statement: {}",
                e
            ))),
        }
    }

    /// 从游标获取结果
    fn fetch_results(&self, cursor: impl Cursor) -> Result<Vec<Vec<String>>, DatabaseError> {
        // 使用更大的缓冲区来支持中文字符 (每个字符最多4个字节)
        let mut columns = Vec::new();
        for i in 1..=10u16 {
            // 增加缓冲区大小：4096 字节每个单元格，支持约 1000 个中文字符
            columns.push((i, TextColumn::<u8>::new(100, 4096)));
        }

        let mut row_set = TextRowSet::new(columns);
        let mut cursor = cursor
            .bind_buffer(&mut row_set)
            .map_err(|e| DatabaseError::Query(format!("Failed to bind buffer: {}", e)))?;

        let mut rows = Vec::new();
        while let Some(batch) = cursor
            .fetch()
            .map_err(|e| DatabaseError::Query(format!("Failed to fetch: {}", e)))?
        {
            for row_index in 0..batch.num_rows() {
                let mut row = Vec::new();
                let mut found_data = false;

                for col_index in 0..10 {
                    // 先获取原始字节
                    let bytes_opt = batch.at(col_index, row_index);

                    match bytes_opt {
                        Some(bytes) if !bytes.is_empty() => {
                            // 尝试 UTF-8 解码
                            let text = match std::str::from_utf8(bytes) {
                                Ok(s) => s.to_string(),
                                Err(_) => {
                                    // UTF-8 失败，尝试 GBK 解码
                                    let (decoded, _, _) = GBK.decode(bytes);
                                    decoded.to_string()
                                }
                            };
                            row.push(text);
                            found_data = true;
                        }
                        Some(_) | None => {
                            // 空字节或 NULL
                            // 尝试用 at_as_str 检查是否真的是 NULL
                            match batch.at_as_str(col_index, row_index) {
                                Ok(Some(s)) => {
                                    row.push(s.to_string());
                                    found_data = true;
                                }
                                Ok(None) => {
                                    row.push("NULL".to_string());
                                }
                                Err(_) => {
                                    break; // 没有更多列
                                }
                            }
                        }
                    }
                }

                if found_data {
                    // 移除尾部的 NULL 值
                    while row.last() == Some(&"NULL".to_string()) && row.len() > 1 {
                        row.pop();
                    }
                    rows.push(row);
                }
            }
        }

        Ok(rows)
    }
}

#[async_trait::async_trait]
impl AsyncDB for DmDatabase {
    type Error = DatabaseError;
    type ColumnType = sqllogictest::DefaultColumnType;

    async fn run(&mut self, sql: &str) -> Result<DBOutput<Self::ColumnType>, Self::Error> {
        let sql = sql.trim();

        if sql.is_empty() {
            return Ok(DBOutput::StatementComplete(0));
        }

        // 检查是否是查询语句
        if sql.to_uppercase().starts_with("SELECT") {
            let rows = self.execute_query(sql)?;

            if rows.is_empty() {
                return Ok(DBOutput::Rows {
                    types: vec![],
                    rows: vec![],
                });
            }

            // 推断列类型
            let types: Vec<sqllogictest::DefaultColumnType> = rows[0]
                .iter()
                .map(|value| {
                    if value == "NULL" || value.parse::<i64>().is_ok() {
                        sqllogictest::DefaultColumnType::Integer
                    } else {
                        sqllogictest::DefaultColumnType::Text
                    }
                })
                .collect();

            Ok(DBOutput::Rows { types, rows })
        } else {
            let affected_rows = self.execute_statement(sql)?;
            Ok(DBOutput::StatementComplete(affected_rows))
        }
    }

    fn engine_name(&self) -> &str {
        "DM Database"
    }

    async fn shutdown(&mut self) {}
}
