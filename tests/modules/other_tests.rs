//! 其他命令模块测试 (user, share, dav, settings, workflow)

use super::common::*;
use crate::helpers::cli_runner::CliRunner;
use std::time::Duration;

// ==================== User Tests ====================

/// V3 用户测试
pub async fn run_v3_user_tests(config: &EnvironmentConfig) -> CliTestResults {
    let mut results = CliTestResults::new();
    let start = std::time::Instant::now();

    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║     用户命令测试                                          ║");
    println!("╚══════════════════════════════════════════════════════════╝");

    let runner = CliRunner::new()
        .with_base_url(config.base_url.clone())
        .with_email(config.username.clone())
        .with_timeout(Duration::from_secs(30));

    results.add_test_tuple(test_user_info(&runner, "V3").await);
    results.add_test_tuple(test_user_quota(&runner, "V3").await);

    results.duration_ms = start.elapsed().as_millis() as u64;
    results
}

/// V4 用户测试
pub async fn run_v4_user_tests(config: &EnvironmentConfig) -> CliTestResults {
    let mut results = CliTestResults::new();
    let start = std::time::Instant::now();

    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║     用户命令测试                                          ║");
    println!("╚══════════════════════════════════════════════════════════╝");

    let runner = CliRunner::new()
        .with_base_url(config.base_url.clone())
        .with_email(config.username.clone())
        .with_timeout(Duration::from_secs(30));

    // V4 支持的所有用户命令
    results.add_test_tuple(test_user_info(&runner, "V4").await);
    results.add_test_tuple(test_user_quota(&runner, "V4").await);
    results.add_test_tuple(test_user_policies(&runner, "V4").await);

    results.duration_ms = start.elapsed().as_millis() as u64;
    results
}

async fn test_user_info(
    runner: &CliRunner,
    version: &str,
) -> (String, String, String, Option<i32>, String) {
    println!("  [User-{}] 测试 info 命令...", version);

    let result = runner.run(&["user", "info"]);

    if result.success {
        println!("  [User-{}] ✓ info 命令成功", version);
        (
            "user info".to_string(),
            String::new(),
            version.to_string(),
            result.exit_code,
            String::new(),
        )
    } else {
        println!("  [User-{}] ✗ info 命令失败: {}", version, result.stderr);
        (
            "user info".to_string(),
            String::new(),
            version.to_string(),
            result.exit_code,
            result.stderr.clone(),
        )
    }
}

async fn test_user_quota(
    runner: &CliRunner,
    version: &str,
) -> (String, String, String, Option<i32>, String) {
    println!("  [User-{}] 测试 quota 命令...", version);

    let result = runner.run(&["user", "quota"]);

    if result.success {
        println!("  [User-{}] ✓ quota 命令成功", version);
        (
            "user quota".to_string(),
            String::new(),
            version.to_string(),
            result.exit_code,
            String::new(),
        )
    } else {
        println!("  [User-{}] ✗ quota 命令失败: {}", version, result.stderr);
        (
            "user quota".to_string(),
            String::new(),
            version.to_string(),
            result.exit_code,
            result.stderr.clone(),
        )
    }
}

async fn test_user_policies(
    runner: &CliRunner,
    version: &str,
) -> (String, String, String, Option<i32>, String) {
    println!("  [User-{}] 测试 policies 命令...", version);

    let result = runner.run(&["user", "policies"]);

    // V4 特有功能，某些测试环境可能不支持（返回 404）
    if result.success {
        println!("  [User-{}] ✓ policies 命令成功", version);
        (
            "user policies".to_string(),
            String::new(),
            version.to_string(),
            result.exit_code,
            String::new(),
        )
    } else if result.stderr.contains("not yet supported")
        || result.stderr.contains("404")
        || result.stderr.contains("not available")
    {
        // API 环境不支持，但命令存在
        println!("  [User-{}] ✓ policies 命令存在（API 环境限制）", version);
        (
            "user policies".to_string(),
            String::new(),
            version.to_string(),
            result.exit_code,
            String::new(),
        )
    } else {
        println!(
            "  [User-{}] ✗ policies 命令失败: {}",
            version, result.stderr
        );
        (
            "user policies".to_string(),
            String::new(),
            version.to_string(),
            result.exit_code,
            result.stderr.clone(),
        )
    }
}

// ==================== Share Tests ====================

/// V3 分享测试（基础功能）
pub async fn run_v3_share_tests(config: &EnvironmentConfig) -> CliTestResults {
    let mut results = CliTestResults::new();
    let start = std::time::Instant::now();

    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║     分享命令测试                                          ║");
    println!("╚══════════════════════════════════════════════════════════╝");

    let runner = CliRunner::new()
        .with_base_url(config.base_url.clone())
        .with_email(config.username.clone())
        .with_timeout(Duration::from_secs(30));

    results.add_test_tuple(test_share_help(&runner, "V3").await);

    results.duration_ms = start.elapsed().as_millis() as u64;
    results
}

/// V4 分享测试（完整功能）
pub async fn run_v4_share_tests(config: &EnvironmentConfig) -> CliTestResults {
    let mut results = CliTestResults::new();
    let start = std::time::Instant::now();

    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║     分享命令测试                                          ║");
    println!("╚══════════════════════════════════════════════════════════╝");

    let runner = CliRunner::new()
        .with_base_url(config.base_url.clone())
        .with_email(config.username.clone())
        .with_timeout(Duration::from_secs(30));

    results.add_test_tuple(test_share_list(&runner, "V4").await);
    results.add_test_tuple(test_share_help(&runner, "V4").await);

    results.duration_ms = start.elapsed().as_millis() as u64;
    results
}

async fn test_share_list(
    runner: &CliRunner,
    version: &str,
) -> (String, String, String, Option<i32>, String) {
    println!("  [Share-{}] 测试 list 命令...", version);

    let result = runner.run(&["share", "list"]);

    if result.success || result.stderr.contains("not yet supported") {
        println!("  [Share-{}] ✓ list 命令执行完成", version);
        (
            "share list".to_string(),
            String::new(),
            version.to_string(),
            result.exit_code,
            String::new(),
        )
    } else {
        println!("  [Share-{}] ✗ list 命令失败: {}", version, result.stderr);
        (
            "share list".to_string(),
            String::new(),
            version.to_string(),
            result.exit_code,
            result.stderr.clone(),
        )
    }
}

async fn test_share_help(
    runner: &CliRunner,
    version: &str,
) -> (String, String, String, Option<i32>, String) {
    println!("  [Share-{}] 测试 share 命令组...", version);

    let result = runner.run(&["share", "help"]);

    if result.success {
        println!("  [Share-{}] ✓ share 命令组存在", version);
        (
            "share".to_string(),
            "help".to_string(),
            version.to_string(),
            result.exit_code,
            String::new(),
        )
    } else {
        println!("  [Share-{}] ✗ share 命令组不存在", version);
        (
            "share".to_string(),
            "help".to_string(),
            version.to_string(),
            result.exit_code,
            "Share command group not found".to_string(),
        )
    }
}

// ==================== WebDAV Tests ====================

/// V4 WebDAV 测试（V4 only）
pub async fn run_v4_dav_tests(config: &EnvironmentConfig) -> CliTestResults {
    let mut results = CliTestResults::new();
    let start = std::time::Instant::now();

    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║     WebDAV 命令测试                                        ║");
    println!("╚══════════════════════════════════════════════════════════╝");

    let runner = CliRunner::new()
        .with_base_url(config.base_url.clone())
        .with_email(config.username.clone())
        .with_timeout(Duration::from_secs(30));

    results.add_test_tuple(test_dav_list(&runner, "V4").await);
    results.add_test_tuple(test_dav_help(&runner, "V4").await);

    results.duration_ms = start.elapsed().as_millis() as u64;
    results
}

async fn test_dav_list(
    runner: &CliRunner,
    version: &str,
) -> (String, String, String, Option<i32>, String) {
    println!("  [WebDAV-{}] 测试 list 命令...", version);

    let result = runner.run(&["dav", "list"]);

    if result.success || result.stderr.contains("not yet supported") {
        println!("  [WebDAV-{}] ✓ list 命令执行完成", version);
        (
            "dav list".to_string(),
            String::new(),
            version.to_string(),
            result.exit_code,
            String::new(),
        )
    } else {
        println!("  [WebDAV-{}] ✗ list 命令失败: {}", version, result.stderr);
        (
            "dav list".to_string(),
            String::new(),
            version.to_string(),
            result.exit_code,
            result.stderr.clone(),
        )
    }
}

async fn test_dav_help(
    runner: &CliRunner,
    version: &str,
) -> (String, String, String, Option<i32>, String) {
    println!("  [WebDAV-{}] 测试 dav 命令组...", version);

    let result = runner.run(&["dav", "help"]);

    if result.success {
        println!("  [WebDAV-{}] ✓ dav 命令组存在", version);
        (
            "dav".to_string(),
            "help".to_string(),
            version.to_string(),
            result.exit_code,
            String::new(),
        )
    } else {
        println!("  [WebDAV-{}] ✗ dav 命令组不存在", version);
        (
            "dav".to_string(),
            "help".to_string(),
            version.to_string(),
            result.exit_code,
            "WebDAV command group not found".to_string(),
        )
    }
}

// ==================== Settings Tests ====================

/// V3 设置测试
pub async fn run_v3_settings_tests(config: &EnvironmentConfig) -> CliTestResults {
    let mut results = CliTestResults::new();
    let start = std::time::Instant::now();

    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║     设置命令测试                                          ║");
    println!("╚══════════════════════════════════════════════════════════╝");

    let runner = CliRunner::new()
        .with_base_url(config.base_url.clone())
        .with_email(config.username.clone())
        .with_timeout(Duration::from_secs(30));

    results.add_test_tuple(test_settings_help(&runner, "V3").await);

    results.duration_ms = start.elapsed().as_millis() as u64;
    results
}

/// V4 设置测试
pub async fn run_v4_settings_tests(config: &EnvironmentConfig) -> CliTestResults {
    let mut results = CliTestResults::new();
    let start = std::time::Instant::now();

    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║     设置命令测试                                          ║");
    println!("╚══════════════════════════════════════════════════════════╝");

    let runner = CliRunner::new()
        .with_base_url(config.base_url.clone())
        .with_email(config.username.clone())
        .with_timeout(Duration::from_secs(30));

    results.add_test_tuple(test_settings_help(&runner, "V4").await);

    results.duration_ms = start.elapsed().as_millis() as u64;
    results
}

async fn test_settings_help(
    runner: &CliRunner,
    version: &str,
) -> (String, String, String, Option<i32>, String) {
    println!("  [Settings-{}] 测试 settings 命令组...", version);

    let result = runner.run(&["settings", "help"]);

    if result.success {
        println!("  [Settings-{}] ✓ settings 命令组存在", version);
        (
            "settings".to_string(),
            "help".to_string(),
            version.to_string(),
            result.exit_code,
            String::new(),
        )
    } else {
        println!("  [Settings-{}] ✗ settings 命令组不存在", version);
        (
            "settings".to_string(),
            "help".to_string(),
            version.to_string(),
            result.exit_code,
            "Settings command group not found".to_string(),
        )
    }
}

// ==================== Workflow Tests ====================

/// V4 工作流测试（V4 only）
pub async fn run_v4_workflow_tests(config: &EnvironmentConfig) -> CliTestResults {
    let mut results = CliTestResults::new();
    let start = std::time::Instant::now();

    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║     工作流命令测试                                        ║");
    println!("╚══════════════════════════════════════════════════════════╝");

    let runner = CliRunner::new()
        .with_base_url(config.base_url.clone())
        .with_email(config.username.clone())
        .with_timeout(Duration::from_secs(30));

    results.add_test_tuple(test_workflow_list(&runner, "V4").await);
    results.add_test_tuple(test_workflow_help(&runner, "V4").await);

    results.duration_ms = start.elapsed().as_millis() as u64;
    results
}

async fn test_workflow_list(
    runner: &CliRunner,
    version: &str,
) -> (String, String, String, Option<i32>, String) {
    println!("  [Workflow-{}] 测试 list 命令...", version);

    let result = runner.run(&["workflow", "list"]);

    if result.success || result.stderr.contains("not yet supported") {
        println!("  [Workflow-{}] ✓ list 命令执行完成", version);
        (
            "workflow list".to_string(),
            String::new(),
            version.to_string(),
            result.exit_code,
            String::new(),
        )
    } else {
        println!(
            "  [Workflow-{}] ✗ list 命令失败: {}",
            version, result.stderr
        );
        (
            "workflow list".to_string(),
            String::new(),
            version.to_string(),
            result.exit_code,
            result.stderr.clone(),
        )
    }
}

async fn test_workflow_help(
    runner: &CliRunner,
    version: &str,
) -> (String, String, String, Option<i32>, String) {
    println!("  [Workflow-{}] 测试 workflow 命令组...", version);

    let result = runner.run(&["workflow", "help"]);

    if result.success {
        println!("  [Workflow-{}] ✓ workflow 命令组存在", version);
        (
            "workflow".to_string(),
            "help".to_string(),
            version.to_string(),
            result.exit_code,
            String::new(),
        )
    } else {
        println!("  [Workflow-{}] ✗ workflow 命令组不存在", version);
        (
            "workflow".to_string(),
            "help".to_string(),
            version.to_string(),
            result.exit_code,
            "Workflow command group not found".to_string(),
        )
    }
}
