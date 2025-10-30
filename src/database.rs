use crate::column::DamengColumnType;
use crate::error::DatabaseError;

use dameng::Connection;
use sqllogictest::{DB, DBOutput};

pub struct DmDatabase {
    connection: Option<Connection>,
}

impl DmDatabase {
    pub fn new() -> Self {
        Self { connection: None }
    }

    pub fn connect(
        &mut self,
        username: &str,
        password: &str,
        host: &str,
    ) -> Result<(), DatabaseError> {
        match Connection::connect(username, password, host) {
            Ok(c) => {
                self.connection = Some(c);
                Ok(())
            }
            Err(e) => Err(DatabaseError::Connection(format!("{}", e))),
        }
    }

    pub fn close(&mut self) -> Result<(), DatabaseError> {
        if let Some(mut conn) = self.connection.take() {
            conn.close()
                .map_err(|e| DatabaseError::Connection(format!("{}", e)))
        } else {
            Ok(())
        }
    }
}

impl DB for DmDatabase {
    type Error = DatabaseError;
    type ColumnType = DamengColumnType;

    fn run(&mut self, sql: &str) -> Result<DBOutput<Self::ColumnType>, Self::Error> {
        if self.connection.is_none() {
            return Err(DatabaseError::Connection("not connected".to_string()));
        }

        let conn = self.connection.as_mut().unwrap();

        let mut statement = conn
            .statement(sql)
            .build()
            .map_err(|e| DatabaseError::Query(format!("build statement error: {}", e)))?;

        if statement.is_query() {
            let mut rows: Vec<Vec<String>> = Vec::new();
            let mut types: Vec<Self::ColumnType> = Vec::new();

            let mut resultset = statement
                .query(&[])
                .map_err(|e| DatabaseError::Query(format!("query error: {}", e)))?;

            for column in resultset.column_info() {
                let col_type = match format!("{:?}", column.dameng_type()).as_str() {
                    "INT" | "INTEGER" => DamengColumnType::Integer,
                    "FLOAT" | "DOUBLE" | "REAL" => DamengColumnType::FloatingPoint,
                    "VARCHAR" | "CHAR" | "TEXT" => DamengColumnType::Text,
                    _ => DamengColumnType::Any, // fallback
                };
                types.push(col_type);
            }

            while let Some(row_result) = resultset.next() {
                let row = row_result.unwrap();
                let mut row_data: Vec<String> = Vec::new();

                for value in row.sql_values() {
                    match value.get::<Option<String>>() {
                        Ok(column_data) => {
                            if column_data.is_none() {
                                row_data.push("<NULL>".to_string());
                            } else {
                                if column_data == Some("".to_string()) {
                                    row_data.push("<EMPTY>".to_string());
                                } else {
                                    row_data.push(column_data.unwrap());
                                }
                            }
                        }
                        Err(e) => panic!("Error is {}", e),
                    };
                }

                rows.push(row_data);
            }

            Ok(DBOutput::Rows { types, rows })
        } else {
            match statement.execute(&[]) {
                Ok(_) => Ok(DBOutput::StatementComplete(0)),
                Err(e) => Err(DatabaseError::Query(format!("{}", e))),
            }
        }
    }

    fn engine_name(&self) -> &str {
        "DM Database"
    }

    fn shutdown(&mut self) {
        let _ = self.close();
    }
}
