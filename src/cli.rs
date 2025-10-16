use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "dm-test")]
#[command(author, version, about = "达梦数据库 ODBC 测试工具", long_about = None)]
pub struct Cli {
    /// 测试文件目录
    #[arg(short = 'd', long, default_value = "tests")]
    pub test_dir: String,

    /// JSON 结果文件输出路径
    #[arg(short = 'o', long = "json-output", default_value = "test-results.json")]
    pub json_output: String,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 生成 HTML 测试报告
    Report {
        /// 输出 HTML 报告的文件路径
        #[arg(short, long, default_value = "report.html")]
        output: String,

        /// 强制重新运行测试（即使 JSON 文件已存在）
        #[arg(short = 'f', long)]
        force: bool,
    },

    /// 生成测试代码（扫描测试目录生成 tests.rs）
    GenTests {
        /// 测试文件目录（覆盖全局选项）
        #[arg(short = 'd', long)]
        test_dir: Option<String>,
    },
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
