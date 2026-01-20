//! Cloudreve CLI 端到端集成测试
//!
//! 分别测试 V3 和 V4 的所有 CLI 功能

mod helpers;
mod modules;

use modules::auth_tests;
use modules::common::*;
use modules::file_tests;
use modules::other_tests;

#[tokio::test]
async fn run_cli_integration_tests() {
    // 检查是否启用集成测试（通过环境变量或配置文件存在性）
    let integration_enabled = std::env::var("INTEGRATION_TEST_ENABLED")
        .map(|v| v == "1" || v.to_lowercase() == "true")
        .unwrap_or(false);

    // 尝试加载配置，如果失败且未启用集成测试则跳过
    let config = match CliTestConfig::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            if !integration_enabled {
                println!(
                    "CLI 集成测试已跳过（未配置 INTEGRATION_TEST_ENABLED=1 且配置文件不存在）"
                );
                println!("要运行集成测试，请:");
                println!("  1. 设置环境变量: INTEGRATION_TEST_ENABLED=1");
                println!("  2. 或创建配置文件: cloudreve-api/tests/config/test_config.toml");
                println!("\n提示: {}", e);
                return;
            }
            println!("\n错误: {}", e);
            println!("\n请按以下步骤配置测试环境:");
            println!(
                "1. 复制配置文件: cp cloudreve-api/tests/config/test_config.example.toml cloudreve-api/tests/config/test_config.toml"
            );
            println!("2. 编辑 cloudreve-api/tests/config/test_config.toml，填入你的测试环境信息");
            println!("3. 重新运行测试\n");
            panic!("配置文件未找到或无效");
        }
    };

    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║     Cloudreve CLI 端到端测试套件                         ║");
    println!("╚══════════════════════════════════════════════════════════╝");

    println!("配置加载成功:");
    if config.v3_enabled() {
        println!("  - V3: 已启用 ({})", config.v3_config().unwrap().base_url);
    } else {
        println!("  - V3: 未配置");
    }
    if config.v4_enabled() {
        println!("  - V4: 已启用 ({})", config.v4_config().unwrap().base_url);
    } else {
        println!("  - V4: 未配置");
    }

    let mut all_results = CliTestResults::new();

    // ========== V3 测试 ==========
    if let Some(v3_config) = config.v3_config() {
        println!("\n╔══════════════════════════════════════════════════════════╗");
        println!("║     V3 API 测试                                          ║");
        println!("╚══════════════════════════════════════════════════════════╝");

        // 认证测试
        let auth_results = auth_tests::run_v3_auth_tests(v3_config).await;
        all_results.merge(auth_results);

        // 文件命令测试（V3 支持的功能）
        let file_results = file_tests::run_v3_file_tests(v3_config).await;
        all_results.merge(file_results);

        // 用户命令测试
        let user_results = other_tests::run_v3_user_tests(v3_config).await;
        all_results.merge(user_results);

        // 分享命令测试
        let share_results = other_tests::run_v3_share_tests(v3_config).await;
        all_results.merge(share_results);

        // 设置命令测试
        let settings_results = other_tests::run_v3_settings_tests(v3_config).await;
        all_results.merge(settings_results);
    }

    // ========== V4 测试 ==========
    if let Some(v4_config) = config.v4_config() {
        println!("\n╔══════════════════════════════════════════════════════════╗");
        println!("║     V4 API 测试                                          ║");
        println!("╚══════════════════════════════════════════════════════════╝");

        // 认证测试
        let auth_results = auth_tests::run_v4_auth_tests(v4_config).await;
        all_results.merge(auth_results);

        // 文件命令测试（V4 支持的所有功能，包括 sync/diff/preview）
        let file_results = file_tests::run_v4_file_tests(v4_config).await;
        all_results.merge(file_results);

        // 用户命令测试
        let user_results = other_tests::run_v4_user_tests(v4_config).await;
        all_results.merge(user_results);

        // 分享命令测试
        let share_results = other_tests::run_v4_share_tests(v4_config).await;
        all_results.merge(share_results);

        // WebDAV 命令测试（V4 only）
        let dav_results = other_tests::run_v4_dav_tests(v4_config).await;
        all_results.merge(dav_results);

        // 设置命令测试
        let settings_results = other_tests::run_v4_settings_tests(v4_config).await;
        all_results.merge(settings_results);

        // 工作流命令测试（V4 only）
        let workflow_results = other_tests::run_v4_workflow_tests(v4_config).await;
        all_results.merge(workflow_results);
    }

    // 打印汇总
    all_results.print_summary();

    // 如果有测试失败，打印详细信息
    if !all_results.is_success() {
        println!("\n╔══════════════════════════════════════════════════════════╗");
        println!("║     测试发现的问题汇总                                  ║");
        println!("╚══════════════════════════════════════════════════════════╝");
        println!("上述测试失败表明可能存在以下问题：");
        println!("1. CLI 命令参数解析错误");
        println!("2. 命令输出格式不符合预期");
        println!("3. 与 API 交互失败");
        println!("4. 文件操作失败");
        println!("\n建议：");
        println!("1. 检查失败命令的参数格式");
        println!("2. 验证 API 测试是否通过（确保后端正常）");
        println!("3. 查看 CLI 日志输出了解详细信息");
        println!("4. 使用 --log-prefix 和 --log-level debug 运行 CLI 进行调试");
    }
}
