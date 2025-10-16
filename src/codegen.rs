use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

/// 代码生成器
pub struct CodeGenerator;

impl CodeGenerator {
    /// 递归查找所有 .test 测试文件
    fn find_test_files(dir: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        let mut test_files = Vec::new();

        if !dir.exists() {
            return Ok(test_files);
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                test_files.extend(Self::find_test_files(&path)?);
            } else if path.extension().and_then(|s| s.to_str()) == Some("test") {
                test_files.push(path);
            }
        }

        Ok(test_files)
    }

    /// 将文件路径转换为有效的测试函数名
    fn path_to_test_name(path: &Path, test_dir: &Path) -> String {
        let relative_path = path.strip_prefix(test_dir).unwrap_or(path);
        let path_str = relative_path.to_string_lossy();

        // 移除扩展名
        let without_ext = path_str.trim_end_matches(".test");

        // 替换路径分隔符和特殊字符为下划线
        let test_name = without_ext.replace(['\\', '/', '-', '.', ' '], "_");

        format!("test_{}", test_name)
    }

    /// 生成测试代码
    pub fn generate_tests(test_dir: &str) -> Result<(), Box<dyn Error>> {
        let test_dir = Path::new(test_dir);

        if !test_dir.exists() {
            return Err(format!("测试目录不存在: {}", test_dir.display()).into());
        }

        let mut test_files = Self::find_test_files(test_dir)?;
        test_files.sort();

        if test_files.is_empty() {
            println!("⚠️ {} 目录下没有找到 .test 测试文件", test_dir.display());
            return Ok(());
        }

        println!("🔍 找到 {} 个测试文件", test_files.len());

        // 生成测试代码
        let mut code = String::from(
            r#"// 此文件由 `cargo run -- gen-tests` 自动生成
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

"#,
        );

        // 为每个测试文件生成测试函数调用
        for test_file in &test_files {
            let test_name = Self::path_to_test_name(test_file, test_dir);
            let test_path = test_file.to_string_lossy().replace('\\', "/");

            code.push_str(&format!(
                "    // {}\n    sqllogic_test!({}, \"{}\");\n\n",
                test_path, test_name, test_path
            ));

            println!("  ✓ {} -> {}", test_path, test_name);
        }

        code.push_str("}\n");

        // 写入文件
        let output_file = "src/tests.rs";
        fs::write(output_file, code)?;

        println!(
            "\n✅ 成功生成 {} ({} 个测试)",
            output_file,
            test_files.len()
        );
        println!("💡 现在运行 `cargo test` 来执行所有测试");

        Ok(())
    }
}
