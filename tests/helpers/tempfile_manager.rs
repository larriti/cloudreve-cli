//! 临时文件和目录管理

use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::time::SystemTime;

pub struct TempFileManager {
    base_dir: PathBuf,
    files: Vec<PathBuf>,
    dirs: Vec<PathBuf>,
}

impl TempFileManager {
    pub fn new() -> Self {
        let base_dir = std::env::temp_dir().join(format!(
            "cloudreve-cli-test-{}",
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        ));

        fs::create_dir_all(&base_dir).expect("Failed to create temp directory");

        Self {
            base_dir,
            files: Vec::new(),
            dirs: Vec::new(),
        }
    }

    /// 创建临时文件
    pub fn create_file(&mut self, name: &str, content: &str) -> String {
        let file_path = self.base_dir.join(name);

        let mut file = File::create(&file_path)
            .unwrap_or_else(|_| panic!("Failed to create temp file: {:?}", file_path));

        file.write_all(content.as_bytes())
            .expect("Failed to write temp file");

        self.files.push(file_path.clone());
        file_path.to_string_lossy().to_string()
    }

    /// 创建临时目录
    pub fn temp_dir(&mut self, name: &str) -> String {
        let dir_path = self.base_dir.join(name);
        fs::create_dir_all(&dir_path)
            .unwrap_or_else(|_| panic!("Failed to create temp dir: {:?}", dir_path));

        self.dirs.push(dir_path.clone());
        dir_path.to_string_lossy().to_string()
    }

    /// 在指定子目录中创建临时文件
    pub fn create_file_in_dir(&mut self, dir_name: &str, file_name: &str, content: &str) -> String {
        let dir_path = self.base_dir.join(dir_name);
        fs::create_dir_all(&dir_path)
            .unwrap_or_else(|_| panic!("Failed to create temp dir: {:?}", dir_path));

        let file_path = dir_path.join(file_name);

        let mut file = File::create(&file_path)
            .unwrap_or_else(|_| panic!("Failed to create temp file: {:?}", file_path));

        file.write_all(content.as_bytes())
            .expect("Failed to write temp file");

        self.files.push(file_path.clone());
        file_path.to_string_lossy().to_string()
    }

    /// 获取临时路径
    pub fn temp_path(&self, name: &str) -> String {
        self.base_dir.join(name).to_string_lossy().to_string()
    }

    /// 清理所有临时文件和目录
    pub fn cleanup(&mut self) {
        // 删除文件
        for file in &self.files {
            let _ = fs::remove_file(file);
        }

        // 删除目录
        for dir in &self.dirs {
            let _ = fs::remove_dir_all(dir);
        }

        // 删除基础目录
        let _ = fs::remove_dir_all(&self.base_dir);

        // 清空列表，防止 Drop 时重复删除
        self.files.clear();
        self.dirs.clear();
    }
}

impl Drop for TempFileManager {
    fn drop(&mut self) {
        // 确保 Drop 时清理
        for file in &self.files {
            let _ = fs::remove_file(file);
        }
        for dir in &self.dirs {
            let _ = fs::remove_dir_all(dir);
        }
        let _ = fs::remove_dir_all(&self.base_dir);
    }
}
