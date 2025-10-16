use dm_test::{
    Cli, CodeGenerator, DatabaseConfig, HtmlReportGenerator, SqlLogicTestRunner, TestSummary,
    cli::Commands,
};
use std::error::Error;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse_args();

    match cli.command {
        Some(Commands::Report { output, force }) => {
            generate_report(&cli.test_dir, &cli.json_output, &output, force).await
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
    force: bool,
) -> Result<(), Box<dyn Error>> {
    println!("🚀 生成 HTML 测试报告");
    println!("================================================\n");

    // 检查 JSON 文件是否存在，或者强制重新运行
    if force || !Path::new(json_output).exists() {
        if force {
            println!("🔄 强制重新运行测试...\n");
        } else {
            println!("⚠️  测试结果文件 {} 不存在", json_output);
            println!("📝 正在运行测试生成结果...\n");
        }

        // 先运行测试生成 JSON
        run_tests(test_dir, json_output).await?;

        println!("\n");
    } else {
        println!("✓ 使用现有的测试结果: {}\n", json_output);
    }

    // 生成 HTML 报告
    HtmlReportGenerator::generate(test_dir, json_output, html_output)?;

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
