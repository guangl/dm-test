# 达梦数据库 ODBC 连接测试

这个项目演示如何使用 Rust 通过 ODBC 连接达梦数据库，并使用 sqllogictest 进行测试。

## 环境要求

1. 安装达梦数据库 8
2. 安装达梦 ODBC 驱动程序
3. 配置 ODBC 数据源

## 安装步骤

### 1. 安装达梦 ODBC 驱动

从达梦官网下载并安装 ODBC 驱动程序。

### 2. 配置环境变量

```bash
# Windows
set DM_CONNECTION_STRING="Driver={DM8 ODBC DRIVER};Server=localhost;Port=5236;Database=DAMENG;UID=SYSDBA;PWD=DMDBA_hust4400"

# Linux
export DM_CONNECTION_STRING="Driver={DM8 ODBC DRIVER};Server=localhost;Port=5236;Database=DAMENG;UID=SYSDBA;PWD=DMDBA_hust4400"
```

### 3. 运行项目

```bash
cargo run
```

## 连接字符串参数说明

- `Driver`: ODBC 驱动名称
- `Server`: 数据库服务器地址
- `Port`: 数据库端口（默认 5236）
- `Database`: 数据库名称
- `UID`: 用户名
- `PWD`: 密码

## 功能特性

- ✅ ODBC 连接达梦数据库
- ✅ 执行 SQL 查询和语句
- ✅ 错误处理和类型转换
- ✅ SQL Logic Test 集成
- ✅ 异步支持
- ✅ 模块化架构
- ✅ 自动测试代码生成
- ✅ 递归目录扫描
- ✅ 可配置测试目录
- ✅ 详细的测试结果记录
- ✅ 自动保存 JSON 格式测试结果

## 使用方法

### 基本用法

```bash
# 运行所有测试（使用默认 tests 目录）
# 测试结果会自动保存到 test-results.json
cargo run

# 指定测试目录
cargo run -- --test-dir tests/basic

# 指定 JSON 输出文件路径
cargo run -- --json-output results/my-test.json

# 同时指定测试目录和输出文件
cargo run -- --test-dir tests/advanced --json-output advanced-results.json

# 查看帮助
cargo run -- --help
```

### 生成 HTML 报告

```bash
# 生成 HTML 报告（同时会生成 JSON 结果文件）
cargo run -- report

# 指定 HTML 报告输出路径
cargo run -- report --output my-report.html

# 指定 JSON 输出路径和 HTML 报告路径
cargo run -- --json-output custom.json report --output custom.html
```

### 测试结果

程序会显示详细的测试汇总信息：
- 总测试数量
- 成功的测试数量
- 失败的测试数量
- 每个失败测试的详细错误信息

**所有测试运行后，结果会自动保存到 JSON 文件**（默认: `test-results.json`），格式如下：

```json
{
  "total": 5,
  "passed": 4,
  "failed": 1,
  "results": [
    {
      "file_path": "basic\\simple.test",
      "success": true,
      "error_message": null
    },
    {
      "file_path": "fail_test.test",
      "success": false,
      "error_message": "query result mismatch: ..."
    }
  ]
}
```

### 生成测试代码

```bash
# 扫描默认 tests 目录并生成测试代码
cargo run -- gen-tests

# 扫描指定目录并生成测试代码
cargo run -- gen-tests --test-dir tests/basic

# 生成后运行测试
cargo test
```

### 测试目录结构

测试文件使用 `.test` 扩展名，支持多层级目录结构：

```
tests/
  ├── basic/
  │   └── simple.test
  ├── advanced/
  │   ├── joins.test
  │   └── nested/
  │       └── subquery.test
  └── complex_test.test
```

### 命令选项

**全局选项：**
- `-d, --test-dir <TEST_DIR>` - 指定测试文件目录（默认: tests）
- `-o, --json-output <JSON_OUTPUT>` - 指定 JSON 结果文件输出路径（默认: test-results.json）

**子命令：**
- `gen-tests` - 生成测试代码
  - `-d, --test-dir <TEST_DIR>` - 覆盖全局测试目录设置
- `report` - 生成 HTML 测试报告（同时生成 JSON 结果）
  - `-o, --output <OUTPUT>` - 指定 HTML 报告输出文件（默认: report.html）

**自动生成的文件：**
- JSON 结果文件 - 每次运行测试后自动生成（可通过 `--json-output` 指定路径）
- HTML 报告 - 使用 `report` 命令时生成（待实现）

## 测试内容## 测试内容

项目包含以下测试：

1. **连接测试**: 验证数据库连接是否成功
2. **基本查询测试**: 查询系统版本信息
3. **DDL 测试**: 创建和删除表
4. **DML 测试**: 插入、查询数据
5. **聚合查询测试**: COUNT、AVG 等函数
6. **SQL Logic Test**: 自动化测试套件

## 故障排除

### 常见问题

1. **连接失败**
   - 检查达梦数据库服务是否启动
   - 验证连接字符串参数
   - 确认 ODBC 驱动已正确安装

2. **驱动程序未找到**
   - 确保已安装达梦 ODBC 驱动
   - 检查驱动名称是否正确
   - 在 Windows 上检查 ODBC 数据源管理器

3. **权限问题**
   - 确认用户名和密码正确
   - 检查用户是否有相应的数据库权限

## 扩展功能

可以进一步扩展的功能：

- 连接池管理
- 事务处理
- 批量操作
- 更复杂的数据类型支持
- 性能监控和日志记录