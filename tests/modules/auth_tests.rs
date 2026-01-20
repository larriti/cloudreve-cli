//! 认证命令测试

use super::common::*;
use crate::helpers::cli_runner::CliRunner;
use std::time::Duration;

/// V3 认证测试
pub async fn run_v3_auth_tests(config: &EnvironmentConfig) -> CliTestResults {
    let mut results = CliTestResults::new();
    let start = std::time::Instant::now();

    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║     认证命令测试                                          ║");
    println!("╚══════════════════════════════════════════════════════════╝");

    results.add_test_tuple(test_auth(config, "V3").await);

    results.duration_ms = start.elapsed().as_millis() as u64;
    results
}

/// V4 认证测试
pub async fn run_v4_auth_tests(config: &EnvironmentConfig) -> CliTestResults {
    let mut results = CliTestResults::new();
    let start = std::time::Instant::now();

    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║     认证命令测试                                          ║");
    println!("╚══════════════════════════════════════════════════════════╝");

    results.add_test_tuple(test_auth(config, "V4").await);

    results.duration_ms = start.elapsed().as_millis() as u64;
    results
}

async fn test_auth(
    config: &EnvironmentConfig,
    version: &str,
) -> (String, String, String, Option<i32>, String) {
    println!("  [{}] 测试认证命令...", version);

    let runner = CliRunner::new()
        .with_base_url(config.base_url.clone())
        .with_email(config.username.clone())
        .with_timeout(Duration::from_secs(30));

    // 测试密码认证
    let result = runner.run_with_input(
        &["auth", "--password", &config.password],
        "", // 密码通过参数传递，不需要 stdin
    );

    if result.success {
        println!("  [{}] ✓ 认证成功", version);
        (
            "auth".to_string(),
            "--password ***".to_string(),
            version.to_string(),
            result.exit_code,
            String::new(),
        )
    } else {
        println!("  [{}] ✗ 认证失败: {}", version, result.stderr);
        (
            "auth".to_string(),
            "--password ***".to_string(),
            version.to_string(),
            result.exit_code,
            result.stderr.clone(),
        )
    }
}
