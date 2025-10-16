// 此文件由 `cargo run -- gen-tests` 自动生成
// 请勿手动编辑

#[cfg(test)]
mod sqllogic_tests {
    use crate::{DatabaseConfig, SqlLogicTestRunner};
    use std::path::Path;

    // 辅助宏：为每个测试文件生成独立的测试函数
    macro_rules! sqllogic_test {
        ($name:ident, $path:expr) => {
            #[tokio::test]
            async fn $name() -> Result<(), Box<dyn std::error::Error>> {
                let config = DatabaseConfig::from_env();
                let test_file = $path;

                if !Path::new(test_file).exists() {
                    println!("⚠️ 跳过不存在的测试文件: {}", test_file);
                    return Ok(());
                }

                println!("🧪 运行测试: {}", test_file);
                SqlLogicTestRunner::run_file(&config, test_file).await?;
                println!("✅ {} 测试通过", test_file);
                Ok(())
            }
        };
    }

    // tests/advanced/joins.test
    sqllogic_test!(test_advanced_joins, "tests/advanced/joins.test");

    // tests/advanced/nested/subquery.test
    sqllogic_test!(test_advanced_nested_subquery, "tests/advanced/nested/subquery.test");

    // tests/basic/simple.test
    sqllogic_test!(test_basic_simple, "tests/basic/simple.test");

    // tests/complex_test.test
    sqllogic_test!(test_complex_test, "tests/complex_test.test");

}
