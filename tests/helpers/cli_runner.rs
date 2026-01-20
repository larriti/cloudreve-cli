//! CLI 进程执行和输出验证工具

use assert_cmd::Command;
use std::process::Command as StdCommand;
use std::time::Duration;

// cargo_bin! 宏位置
#[allow(deprecated)]
use assert_cmd::cargo::cargo_bin;

/// CLI 测试运行器
pub struct CliRunner {
    base_url: Option<String>,
    test_email: Option<String>,
    timeout: Duration,
}

impl CliRunner {
    pub fn new() -> Self {
        Self {
            base_url: None,
            test_email: None,
            timeout: Duration::from_secs(30),
        }
    }

    pub fn with_base_url(mut self, url: String) -> Self {
        self.base_url = Some(url);
        self
    }

    pub fn with_email(mut self, email: String) -> Self {
        self.test_email = Some(email);
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// 执行 CLI 命令并返回结果
    pub fn run(&self, args: &[&str]) -> CliResult {
        let mut std_cmd = StdCommand::new(cargo_bin!("cloudreve-cli"));

        // 添加全局参数
        if let Some(url) = &self.base_url {
            std_cmd.arg("--url").arg(url);
        }
        if let Some(email) = &self.test_email {
            std_cmd.arg("--email").arg(email);
        }

        // 添加命令参数
        std_cmd.args(args);

        // 转换为 assert_cmd::Command 以获得超时等功能
        let mut cmd = Command::from(std_cmd);

        // 设置超时
        cmd.timeout(self.timeout);

        // 执行并捕获输出
        let output = cmd.output();

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();

                CliResult {
                    success: output.status.success(),
                    exit_code: output.status.code(),
                    stdout,
                    stderr,
                }
            }
            Err(e) => CliResult {
                success: false,
                exit_code: None,
                stdout: String::new(),
                stderr: format!("Command execution failed: {}", e),
            },
        }
    }

    /// 执行命令并验证成功
    #[allow(dead_code)]
    pub fn run_success(&self, args: &[&str]) -> CliResult {
        let result = self.run(args);
        if !result.success {
            panic!(
                "Command failed unexpectedly: {}\nstderr: {}",
                args.join(" "),
                result.stderr
            );
        }
        result
    }

    /// 执行命令并提供标准输入
    pub fn run_with_input(&self, args: &[&str], input: &str) -> CliResult {
        let mut std_cmd = StdCommand::new(cargo_bin!("cloudreve-cli"));

        if let Some(url) = &self.base_url {
            std_cmd.arg("--url").arg(url);
        }
        if let Some(email) = &self.test_email {
            std_cmd.arg("--email").arg(email);
        }

        std_cmd.args(args);

        let mut cmd = Command::from(std_cmd);
        cmd.write_stdin(input);
        cmd.timeout(self.timeout);

        let output = cmd.output().expect("Failed to execute command");

        CliResult {
            success: output.status.success(),
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        }
    }
}

impl Default for CliRunner {
    fn default() -> Self {
        Self::new()
    }
}

/// CLI 命令执行结果
#[derive(Debug)]
pub struct CliResult {
    pub success: bool,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

impl CliResult {
    /// 验证 stdout 包含指定文本
    #[allow(dead_code)]
    pub fn assert_stdout_contains(&self, text: &str) -> &Self {
        assert!(
            self.stdout.contains(text),
            "Expected stdout to contain '{}', but got:\n{}",
            text,
            self.stdout
        );
        self
    }

    /// 验证 stderr 包含指定文本
    #[allow(dead_code)]
    pub fn assert_stderr_contains(&self, text: &str) -> &Self {
        assert!(
            self.stderr.contains(text),
            "Expected stderr to contain '{}', but got:\n{}",
            text,
            self.stderr
        );
        self
    }

    /// 打印完整输出（用于调试）
    #[allow(dead_code)]
    pub fn debug_print(&self) -> &Self {
        println!("=== Command Output ===");
        println!("Exit Code: {:?}", self.exit_code);
        println!("Success: {}", self.success);
        println!("STDOUT:\n{}", self.stdout);
        if !self.stderr.is_empty() {
            println!("STDERR:\n{}", self.stderr);
        }
        println!("========================");
        self
    }
}
