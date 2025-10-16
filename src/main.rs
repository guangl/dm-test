use dm_test::{Cli, CodeGenerator, DatabaseConfig, SqlLogicTestRunner, TestSummary, cli::Commands};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse_args();

    match cli.command {
        Some(Commands::Report { output }) => {
            generate_report(&cli.test_dir, &cli.json_output, &output).await
        }
        Some(Commands::GenTests { test_dir }) => {
            let dir = test_dir.as_deref().unwrap_or(&cli.test_dir);
            CodeGenerator::generate_tests(dir)
        }
        None => run_sql_tests(&cli.test_dir, &cli.json_output).await,
    }
}

/// 生成 HTML 测试报告
async fn generate_report(
    test_dir: &str,
    json_output: &str,
    html_output: &str,
) -> Result<(), Box<dyn Error>> {
    println!("🚀 生成 HTML 测试报告");
    println!("================================================\n");

    let summary = run_tests(test_dir, json_output).await?;

    // 生成 HTML 报告（待实现）
    println!("\n📄 HTML 报告: {}", html_output);
    println!("⚠️ HTML 报告生成功能待实现...");

    if summary.failed > 0 {
        return Err(format!("有 {} 个测试失败", summary.failed).into());
    }

    Ok(())
}

/// 运行 SQL Logic Test
async fn run_sql_tests(test_dir: &str, json_output: &str) -> Result<(), Box<dyn Error>> {
    println!("🚀 达梦数据库 SQL Logic Test");
    println!("================================================\n");

    let summary = run_tests(test_dir, json_output).await?;

    if summary.failed > 0 {
        return Err(format!("有 {} 个测试失败", summary.failed).into());
    }

    Ok(())
}

/// 运行测试并保存结果
async fn run_tests(test_dir: &str, json_output: &str) -> Result<TestSummary, Box<dyn Error>> {
    let config = DatabaseConfig::from_env();
    let summary = SqlLogicTestRunner::run_all_tests(&config, test_dir).await?;

    summary.print_summary();
    summary.save_to_json(json_output)?;

    Ok(summary)
}
