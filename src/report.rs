use crate::sqllogic::TestSummary;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::Path;

/// 测试用例详情
#[derive(Debug, Clone)]
pub struct TestCase {
    pub file_path: String,
    pub success: bool,
    pub error_message: Option<String>,
    pub name: String,
    pub description: String,
    pub category: String,
    pub sql_statements: Vec<String>,
}

impl TestCase {
    /// 从测试文件解析测试用例详情
    pub fn from_file(
        file_path: &str,
        success: bool,
        error_message: Option<String>,
        test_dir: &str,
    ) -> Self {
        let full_path = Path::new(test_dir).join(file_path);

        let mut test_case = TestCase {
            file_path: file_path.to_string(),
            success,
            error_message,
            name: String::new(),
            description: String::new(),
            category: String::new(),
            sql_statements: Vec::new(),
        };

        if let Ok(content) = fs::read_to_string(&full_path) {
            test_case.parse_test_content(&content);
        }

        test_case
    }

    /// 解析测试文件内容
    fn parse_test_content(&mut self, content: &str) {
        let lines: Vec<&str> = content.lines().collect();

        // 提取元数据
        for line in &lines {
            let trimmed = line.trim();

            if let Some(name) = trimmed.strip_prefix("# Test Case Name:") {
                self.name = name.trim().to_string();
            } else if let Some(desc) = trimmed.strip_prefix("# Description:") {
                self.description = desc.trim().to_string();
            } else if let Some(cat) = trimmed.strip_prefix("# Category:") {
                self.category = cat.trim().to_string();
            }
        }

        // 提取 SQL 语句
        let mut current_sql = Vec::new();
        let mut in_sql = false;

        for line in lines {
            let trimmed = line.trim();

            // 跳过注释和空行
            if trimmed.starts_with('#') || trimmed.is_empty() {
                continue;
            }

            // 检测 statement 或 query 开始
            if trimmed.starts_with("statement") || trimmed.starts_with("query") {
                if !current_sql.is_empty() {
                    let sql = current_sql.join(" ").trim().to_string();
                    if !sql.is_empty() && !sql.starts_with("----") {
                        self.sql_statements.push(sql);
                    }
                    current_sql.clear();
                }
                in_sql = true;
                continue;
            }

            // 遇到结果分隔符，结束当前 SQL
            if trimmed == "----" {
                if !current_sql.is_empty() {
                    let sql = current_sql.join(" ").trim().to_string();
                    if !sql.is_empty() {
                        self.sql_statements.push(sql);
                    }
                    current_sql.clear();
                }
                in_sql = false;
                continue;
            }

            // 收集 SQL 语句
            if in_sql {
                current_sql.push(trimmed);
            }
        }

        // 处理最后一个 SQL
        if !current_sql.is_empty() {
            let sql = current_sql.join(" ").trim().to_string();
            if !sql.is_empty() {
                self.sql_statements.push(sql);
            }
        }
    }

    /// 获取一级和二级分类
    pub fn get_category_parts(&self) -> (String, Option<String>) {
        if self.category.contains('/') {
            let parts: Vec<&str> = self.category.split('/').collect();
            if parts.len() >= 2 {
                return (parts[0].to_string(), Some(parts[1].to_string()));
            }
            return (parts[0].to_string(), None);
        }
        (self.category.clone(), None)
    }
}

/// HTML 报告生成器
pub struct HtmlReportGenerator;

impl HtmlReportGenerator {
    /// 生成 HTML 报告
    pub fn generate(
        test_dir: &str,
        json_output: &str,
        html_output: &str,
    ) -> Result<(), Box<dyn Error>> {
        println!("正在加载测试结果...");
        let summary = Self::load_test_results(json_output)?;

        println!("正在解析测试文件...");
        let test_cases = Self::parse_all_tests(&summary, test_dir);

        println!("正在生成 HTML 报告...");
        Self::generate_html(&test_cases, &summary, html_output)?;

        println!("\n✅ 完成!");
        println!("总计: {} 个测试", summary.total);
        println!("通过: {} 个", summary.passed);
        println!("失败: {} 个", summary.failed);
        println!(
            "通过率: {:.1}%",
            (summary.passed as f64 / summary.total as f64) * 100.0
        );
        println!("\n报告已保存至: {}", html_output);

        Ok(())
    }

    /// 加载测试结果 JSON 文件
    fn load_test_results(json_path: &str) -> Result<TestSummary, Box<dyn Error>> {
        let content = fs::read_to_string(json_path)?;
        let summary: TestSummary = serde_json::from_str(&content)?;
        Ok(summary)
    }

    /// 解析所有测试用例
    fn parse_all_tests(summary: &TestSummary, test_dir: &str) -> Vec<TestCase> {
        summary
            .results
            .iter()
            .map(|result| {
                TestCase::from_file(
                    &result.file_path,
                    result.success,
                    result.error_message.clone(),
                    test_dir,
                )
            })
            .collect()
    }

    /// 收集分类信息
    fn collect_categories(test_cases: &[TestCase]) -> HashMap<String, HashSet<String>> {
        let mut categories: HashMap<String, HashSet<String>> = HashMap::new();

        for tc in test_cases {
            let (cat1, cat2) = tc.get_category_parts();
            let cat2_set = categories.entry(cat1.clone()).or_default();
            if let Some(cat2_val) = cat2 {
                cat2_set.insert(cat2_val);
            }
        }

        categories
    }

    /// 生成 HTML 报告
    fn generate_html(
        test_cases: &[TestCase],
        summary: &TestSummary,
        output_path: &str,
    ) -> Result<(), Box<dyn Error>> {
        let categories = Self::collect_categories(test_cases);

        // 将分类转换为 JSON
        let mut categories_json = HashMap::new();
        for (cat1, cat2_set) in &categories {
            let mut cat2_vec: Vec<String> = cat2_set.iter().cloned().collect();
            cat2_vec.sort();
            categories_json.insert(cat1.clone(), cat2_vec);
        }
        let categories_json_str = serde_json::to_string(&categories_json)?;

        // 生成测试用例的 HTML
        let mut test_cases_html = String::new();
        for tc in test_cases {
            let status_class = if tc.success { "passed" } else { "failed" };
            let status_text = if tc.success { "通过" } else { "失败" };
            let (cat1, cat2) = tc.get_category_parts();
            let category_display = if let Some(ref c2) = cat2 {
                format!("{} / {}", cat1, c2)
            } else {
                cat1.clone()
            };

            let test_name = if tc.name.is_empty() {
                tc.file_path.clone()
            } else {
                tc.name.clone()
            };

            test_cases_html.push_str(&format!(
                r#"            <div class="test-case {}" data-name="{}" data-description="{}" data-category1="{}" data-category2="{}" data-status="{}">
                <div class="test-header" onclick="toggleTest(this)">
                    <div class="test-title">
                        <div class="test-name">{}</div>
                        <div class="test-meta">
                            <span class="meta-item">
                                <span class="category-badge">{}</span>
                            </span>
                            <span class="meta-item">📄 {}</span>
                        </div>
                    </div>
                    <div style="display: flex; align-items: center; gap: 15px;">
                        <span class="status-badge {}">{}</span>
                        <span class="expand-icon">▼</span>
                    </div>
                </div>
                <div class="test-body">
"#,
                status_class,
                Self::html_escape(&test_name),
                Self::html_escape(&tc.description),
                Self::html_escape(&cat1),
                Self::html_escape(&cat2.unwrap_or_default()),
                status_class,
                Self::html_escape(&test_name),
                Self::html_escape(&category_display),
                Self::html_escape(&tc.file_path),
                status_class,
                status_text
            ));

            // 添加描述
            if !tc.description.is_empty() {
                test_cases_html.push_str(&format!(
                    r#"                    <div class="test-description">
                        <strong>描述：</strong>{}
                    </div>
"#,
                    Self::html_escape(&tc.description)
                ));
            }

            // 添加 SQL 语句
            if !tc.sql_statements.is_empty() {
                // 将所有 SQL 语句合并，用单个换行分隔（不留空行）
                let all_sql = tc.sql_statements.join("\n");
                let sql_escaped = Self::html_escape(&all_sql);

                test_cases_html.push_str(&format!(
                    r#"                    <div class="sql-statements">
                        <div class="sql-header">
                            <h4>📝 SQL 语句</h4>
                            <button class="copy-btn" onclick="copySql(this)">
                                <span>📋</span>
                                <span>复制</span>
                            </button>
                        </div>
                        <div class="sql-container">{}</div>
                    </div>
"#,
                    sql_escaped
                ));
            }

            // 添加错误信息
            if !tc.success
                && let Some(ref error_msg) = tc.error_message
            {
                test_cases_html.push_str(&format!(
                    r#"                    <div class="error-message">
                        <h4>❌ 错误信息</h4>
                        <pre>{}</pre>
                    </div>
"#,
                    Self::html_escape(error_msg)
                ));
            }

            test_cases_html.push_str(
                r#"                </div>
            </div>
"#,
            );
        }

        // 生成完整的 HTML
        let html_content = include_str!("report_template.html")
            .replace("{{TOTAL}}", &summary.total.to_string())
            .replace("{{ TOTAL }}", &summary.total.to_string())
            .replace("{{PASSED}}", &summary.passed.to_string())
            .replace("{{ PASSED }}", &summary.passed.to_string())
            .replace("{{FAILED}}", &summary.failed.to_string())
            .replace("{{ FAILED }}", &summary.failed.to_string())
            .replace(
                "{{PASS_RATE}}",
                &format!(
                    "{:.1}",
                    (summary.passed as f64 / summary.total as f64) * 100.0
                ),
            )
            .replace(
                "{{ PASS_RATE }}",
                &format!(
                    "{:.1}",
                    (summary.passed as f64 / summary.total as f64) * 100.0
                ),
            )
            .replace("{{TEST_CASES}}", &test_cases_html)
            .replace("{{ TEST_CASES }}", &test_cases_html)
            .replace("{{CATEGORIES_JSON}}", &categories_json_str)
            .replace("{{ CATEGORIES_JSON }}", &categories_json_str);

        // 写入文件
        fs::write(output_path, html_content)?;

        Ok(())
    }

    /// HTML 转义
    fn html_escape(text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;")
    }
}
