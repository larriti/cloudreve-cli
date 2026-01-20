//! 共享测试工具和配置

use serde::Deserialize;
use std::fs;
use std::path::Path;

/// CLI 测试配置（复用 API 测试配置）
#[derive(Debug, Clone, Deserialize)]
pub struct CliTestConfig {
    #[allow(dead_code)]
    pub general: GeneralConfig,
    pub environments: EnvironmentsConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GeneralConfig {
    #[serde(default = "default_timeout")]
    #[allow(dead_code)]
    pub timeout: u64,
    #[serde(default = "default_verbose")]
    #[allow(dead_code)]
    pub verbose: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvironmentsConfig {
    pub v3: Option<EnvironmentConfig>,
    pub v4: Option<EnvironmentConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnvironmentConfig {
    pub base_url: String,
    pub username: String,
    pub password: String,
    #[serde(default)]
    #[allow(dead_code)]
    pub otp_secret: Option<String>,
}

fn default_timeout() -> u64 {
    300
}

fn default_verbose() -> bool {
    true
}

impl CliTestConfig {
    /// 从文件加载配置（复用 API 测试的配置文件）
    pub fn load() -> Result<Self, String> {
        // 尝试多个配置路径
        let paths = [
            "cloudreve-api/tests/config/test_config.toml",
            "../cloudreve-api/tests/config/test_config.toml",
            "tests/config/test_config.toml",
        ];

        for config_path in &paths {
            if Path::new(config_path).exists() {
                let content = fs::read_to_string(config_path)
                    .map_err(|e| format!("无法读取配置文件: {}", e))?;

                let config: toml::Value =
                    toml::from_str(&content).map_err(|e| format!("解析配置文件失败: {}", e))?;

                // 提取我们需要的字段
                return Ok(Self {
                    general: GeneralConfig {
                        timeout: config
                            .get("general")
                            .and_then(|g| g.get("timeout"))
                            .and_then(|t| t.as_integer())
                            .unwrap_or(300) as u64,
                        verbose: config
                            .get("general")
                            .and_then(|g| g.get("verbose"))
                            .and_then(|v| v.as_bool())
                            .unwrap_or(true),
                    },
                    environments: EnvironmentsConfig {
                        v3: config
                            .get("environments")
                            .and_then(|e| e.get("v3"))
                            .and_then(parse_environment),
                        v4: config
                            .get("environments")
                            .and_then(|e| e.get("v4"))
                            .and_then(parse_environment),
                    },
                });
            }
        }

        Err("配置文件未找到".to_string())
    }

    pub fn v3_enabled(&self) -> bool {
        self.environments.v3.is_some()
    }

    pub fn v4_enabled(&self) -> bool {
        self.environments.v4.is_some()
    }

    pub fn v3_config(&self) -> Option<&EnvironmentConfig> {
        self.environments.v3.as_ref()
    }

    pub fn v4_config(&self) -> Option<&EnvironmentConfig> {
        self.environments.v4.as_ref()
    }
}

fn parse_environment(value: &toml::Value) -> Option<EnvironmentConfig> {
    Some(EnvironmentConfig {
        base_url: value.get("base_url")?.as_str()?.to_string(),
        username: value.get("username")?.as_str()?.to_string(),
        password: value.get("password")?.as_str()?.to_string(),
        otp_secret: value
            .get("otp_secret")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string()),
    })
}

/// CLI 测试结果
#[derive(Debug, Default)]
pub struct CliTestResults {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub failures: Vec<CliTestFailure>,
    pub duration_ms: u64,
}

#[derive(Debug)]
pub struct CliTestFailure {
    pub command: String,
    pub args: String,
    pub version: String,
    pub exit_code: Option<i32>,
    pub error: String,
}

impl CliTestResults {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_success(&mut self) {
        self.total += 1;
        self.passed += 1;
    }

    pub fn add_failure(
        &mut self,
        command: String,
        args: String,
        version: String,
        exit_code: Option<i32>,
        error: String,
    ) {
        self.total += 1;
        self.failed += 1;
        self.failures.push(CliTestFailure {
            command,
            args,
            version,
            exit_code,
            error,
        });
    }

    pub fn add_skip(&mut self) {
        self.total += 1;
        self.skipped += 1;
    }

    pub fn merge(&mut self, other: CliTestResults) {
        self.total += other.total;
        self.passed += other.passed;
        self.failed += other.failed;
        self.skipped += other.skipped;
        self.failures.extend(other.failures);
        self.duration_ms += other.duration_ms;
    }

    pub fn is_success(&self) -> bool {
        self.failed == 0
    }

    pub fn print_summary(&self) {
        println!("\n╔══════════════════════════════════════════════════════════╗");
        println!("║              CLI 测试结果汇总                            ║");
        println!("╚══════════════════════════════════════════════════════════╝");
        println!(
            "总计: {} | 通过: {} | 失败: {} | 跳过: {}",
            self.total, self.passed, self.failed, self.skipped
        );
        println!("耗时: {}ms\n", self.duration_ms);

        if !self.failures.is_empty() {
            println!("失败的命令:");
            for failure in &self.failures {
                let exit_info = failure
                    .exit_code
                    .map(|c| format!("(exit: {})", c))
                    .unwrap_or_default();
                println!(
                    "  [{}] {} {} {}: {}",
                    failure.version, failure.command, failure.args, exit_info, failure.error
                );
            }
            println!();
        }
    }
}

/// 辅助函数：将测试结果元组转换为添加到结果的操作
pub trait TestResultExt {
    fn add_test_tuple(&mut self, result: (String, String, String, Option<i32>, String));
}

impl TestResultExt for CliTestResults {
    fn add_test_tuple(&mut self, result: (String, String, String, Option<i32>, String)) {
        let (command, args, version, exit_code, error) = result;
        // exit_code 为 None 表示跳过的测试（如 V3 不支持的功能）
        if exit_code.is_none() {
            self.add_skip();
        } else if error.is_empty() {
            self.add_success();
        } else if error.starts_with("Skipped:") || error.starts_with("跳过") {
            self.add_skip();
        } else {
            self.add_failure(command, args, version, exit_code, error);
        }
    }
}
