use crate::config::DatabaseConfig;
use crate::database::DmDatabase;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

/// 测试结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub file_path: String,
    pub success: bool,
    pub error_message: Option<String>,
}

/// 测试统计信息
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TestSummary {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub results: Vec<TestResult>,
}

impl TestSummary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_result(&mut self, result: TestResult) {
        self.total += 1;
        if result.success {
            self.passed += 1;
        } else {
            self.failed += 1;
        }
        self.results.push(result);
    }

    pub fn print_summary(&self) {
        let separator = "=".repeat(60);
        println!("\n{}", separator);
        println!("📊 测试汇总");
        println!("{}", separator);
        println!("总计: {} 个测试", self.total);
        println!("✅ 成功: {} 个", self.passed);
        println!("❌ 失败: {} 个", self.failed);

        if self.failed > 0 {
            println!("\n失败的测试:");
            for result in &self.results {
                if !result.success {
                    println!("  ❌ {}", result.file_path);
                    if let Some(err) = &result.error_message {
                        println!("     原因: {}", err);
                    }
                }
            }
        }
        println!("{}", separator);
    }

    /// 保存测试结果到 JSON 文件
    pub fn save_to_json(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        // 如果目录不存在，先创建目录
        if let Some(parent) = Path::new(file_path).parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        let json = serde_json::to_string_pretty(self)?;
        fs::write(file_path, json)?;
        println!("📄 测试结果已保存到: {}", file_path);
        Ok(())
    }
}

/// sqllogictest 运行器
pub struct SqlLogicTestRunner;

impl SqlLogicTestRunner {
    /// 运行指定的 sqllogictest 文件
    pub async fn run_file(config: &DatabaseConfig, test_file: &str) -> Result<(), Box<dyn Error>> {
        if !Path::new(test_file).exists() {
            return Err(format!("测试文件不存在: {}", test_file).into());
        }

        let mut tester = sqllogictest::Runner::new(|| async { DmDatabase::new(config.clone()) });

        tester.run_file(test_file)?;
        Ok(())
    }

    /// 递归搜索目录下的所有 .test 文件
    fn find_test_files(dir: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        let mut test_files = Vec::new();

        if !dir.exists() {
            return Ok(test_files);
        }

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // 递归搜索子目录
                test_files.extend(Self::find_test_files(&path)?);
            } else if path.extension().and_then(|s| s.to_str()) == Some("test") {
                test_files.push(path);
            }
        }

        Ok(test_files)
    }

    /// 运行测试目录下所有的 .test 文件（递归）
    pub async fn run_all_tests(
        config: &DatabaseConfig,
        test_dir: &str,
    ) -> Result<TestSummary, Box<dyn Error>> {
        let test_dir = Path::new(test_dir);

        if !test_dir.exists() {
            return Err(format!("测试目录不存在: {}", test_dir.display()).into());
        }

        let mut test_files = Self::find_test_files(test_dir)?;

        if test_files.is_empty() {
            return Err(format!("{} 目录下没有找到 .test 测试文件", test_dir.display()).into());
        }

        // 排序测试文件以保证执行顺序一致
        test_files.sort();

        println!("📁 找到 {} 个测试文件\n", test_files.len());

        let mut summary = TestSummary::new();

        for test_file in test_files {
            let relative_path = test_file.strip_prefix(test_dir).unwrap_or(&test_file);
            let path_str = relative_path.display().to_string();
            print!("🧪 {} ... ", path_str);

            match Self::run_file(config, test_file.to_str().unwrap()).await {
                Ok(_) => {
                    println!("✅ 通过");
                    summary.add_result(TestResult {
                        file_path: path_str,
                        success: true,
                        error_message: None,
                    });
                }
                Err(e) => {
                    let error_msg = e.to_string();
                    println!("❌ 失败");
                    eprintln!("   错误: {}", error_msg);
                    summary.add_result(TestResult {
                        file_path: path_str,
                        success: false,
                        error_message: Some(error_msg),
                    });
                    // 继续运行其他测试，不立即返回错误
                }
            }
        }

        Ok(summary)
    }
}
