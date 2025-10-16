pub mod cli;
pub mod codegen;
pub mod config;
pub mod database;
pub mod error;
pub mod report;
pub mod sqllogic;

#[cfg(test)]
mod tests;

// 重新导出主要类型，方便使用
pub use cli::Cli;
pub use codegen::CodeGenerator;
pub use config::DatabaseConfig;
pub use database::DmDatabase;
pub use error::DatabaseError;
pub use report::HtmlReportGenerator;
pub use sqllogic::{SqlLogicTestRunner, TestResult, TestSummary};
