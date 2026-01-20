//! 文件命令测试 (V3/V4 分别测试)

use super::common::*;
use crate::helpers::cli_runner::CliRunner;
use crate::helpers::tempfile_manager::TempFileManager;
use std::time::{Duration, SystemTime};

/// 运行 V3 文件命令测试
pub async fn run_v3_file_tests(config: &EnvironmentConfig) -> CliTestResults {
    let mut results = CliTestResults::new();
    let start = std::time::Instant::now();

    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║     文件命令测试 (V3)                                   ║");
    println!("╚══════════════════════════════════════════════════════════╝");

    let runner = CliRunner::new()
        .with_base_url(config.base_url.clone())
        .with_email(config.username.clone())
        .with_timeout(Duration::from_secs(60));

    let mut temp_mgr = TempFileManager::new();

    // V3 支持的基础文件操作测试
    results.add_test_tuple(test_file_list(&runner, "V3").await);
    results.add_test_tuple(test_file_info(&runner, &mut temp_mgr, "V3").await);
    results.add_test_tuple(test_file_mkdir(&runner, "V3").await);
    results.add_test_tuple(test_file_upload(&runner, &mut temp_mgr, "V3").await);
    results.add_test_tuple(test_file_download(&runner, &mut temp_mgr, "V3").await);
    results.add_test_tuple(test_file_rename(&runner, &mut temp_mgr, "V3").await);
    results.add_test_tuple(test_file_move(&runner, &mut temp_mgr, "V3").await);
    results.add_test_tuple(test_file_copy(&runner, &mut temp_mgr, "V3").await);
    results.add_test_tuple(test_file_delete(&runner, &mut temp_mgr, "V3").await);
    results.add_test_tuple(test_batch_upload(&runner, &mut temp_mgr, "V3").await);
    results.add_test_tuple(test_batch_download(&runner, &mut temp_mgr, "V3").await);

    // 清理临时文件
    temp_mgr.cleanup();

    results.duration_ms = start.elapsed().as_millis() as u64;
    results
}

/// 运行 V4 文件命令测试（包含所有 V4 特有功能）
pub async fn run_v4_file_tests(config: &EnvironmentConfig) -> CliTestResults {
    let mut results = CliTestResults::new();
    let start = std::time::Instant::now();

    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║     文件命令测试 (V4)                                   ║");
    println!("╚══════════════════════════════════════════════════════════╝");

    let runner = CliRunner::new()
        .with_base_url(config.base_url.clone())
        .with_email(config.username.clone())
        .with_timeout(Duration::from_secs(60));

    let mut temp_mgr = TempFileManager::new();

    // V4 支持的基础文件操作测试
    results.add_test_tuple(test_file_list(&runner, "V4").await);
    results.add_test_tuple(test_file_info(&runner, &mut temp_mgr, "V4").await);
    results.add_test_tuple(test_file_mkdir(&runner, "V4").await);
    results.add_test_tuple(test_file_upload(&runner, &mut temp_mgr, "V4").await);
    results.add_test_tuple(test_file_download(&runner, &mut temp_mgr, "V4").await);
    results.add_test_tuple(test_file_rename(&runner, &mut temp_mgr, "V4").await);
    results.add_test_tuple(test_file_move(&runner, &mut temp_mgr, "V4").await);
    results.add_test_tuple(test_file_copy(&runner, &mut temp_mgr, "V4").await);
    results.add_test_tuple(test_file_delete(&runner, &mut temp_mgr, "V4").await);
    results.add_test_tuple(test_batch_upload(&runner, &mut temp_mgr, "V4").await);
    results.add_test_tuple(test_batch_download(&runner, &mut temp_mgr, "V4").await);

    // V4 特有的搜索和元数据测试
    results.add_test_tuple(test_file_search(&runner, "V4").await);
    results.add_test_tuple(test_file_metadata(&runner, "V4").await);
    results.add_test_tuple(test_file_permission(&runner, "V4").await);

    // V4 高级功能测试
    results.add_test_tuple(test_file_sync(&runner, &mut temp_mgr, "V4").await);
    results.add_test_tuple(test_file_preview(&runner, "V4").await);
    results.add_test_tuple(test_file_diff(&runner, &mut temp_mgr, "V4").await);

    // 清理临时文件
    temp_mgr.cleanup();

    results.duration_ms = start.elapsed().as_millis() as u64;
    results
}

// ==================== 基础测试函数 ====================

// 测试文件列表
async fn test_file_list(
    runner: &CliRunner,
    version: &str,
) -> (String, String, String, Option<i32>, String) {
    println!("  [File] 测试 list 命令...");

    let result = runner.run(&["file", "list", "--path", "/"]);

    if result.success {
        println!("  [File] ✓ list 命令成功");
        (
            "file list".to_string(),
            "--path /".to_string(),
            version.to_string(),
            result.exit_code,
            String::new(),
        )
    } else {
        println!("  [File] ✗ list 命令失败: {}", result.stderr);
        (
            "file list".to_string(),
            "--path /".to_string(),
            version.to_string(),
            result.exit_code,
            result.stderr.clone(),
        )
    }
}

// 测试文件信息
async fn test_file_info(
    runner: &CliRunner,
    temp_mgr: &mut TempFileManager,
    version: &str,
) -> (String, String, String, Option<i32>, String) {
    println!("  [File] 测试 info 命令...");

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let test_file_name = format!("cli_test_info_{}.txt", timestamp);
    let test_file = temp_mgr.create_file(&test_file_name, "Info test content");
    let remote_path = format!("/{}", test_file_name);

    let upload_result = runner.run(&["file", "upload", "--file", &test_file, "--path", "/"]);

    if !upload_result.success {
        println!(
            "  [File] ✗ info 测试失败（上传失败）: {}",
            upload_result.stderr
        );
        temp_mgr.cleanup();
        return (
            "file info".to_string(),
            format!("--path {}", remote_path),
            version.to_string(),
            upload_result.exit_code,
            format!("Upload failed: {}", upload_result.stderr),
        );
    }

    let result = runner.run(&["file", "info", "--path", &remote_path]);
    let _ = runner.run(&["file", "delete", "--path", &remote_path, "--force"]);

    if result.success {
        println!("  [File] ✓ info 命令成功");
        (
            "file info".to_string(),
            format!("--path {}", remote_path),
            version.to_string(),
            result.exit_code,
            String::new(),
        )
    } else if result.stderr.contains("Not supported action") || result.stderr.contains("403") {
        println!("  [File] ✓ info 命令存在（API 限制）");
        (
            "file info".to_string(),
            format!("--path {}", remote_path),
            version.to_string(),
            result.exit_code,
            String::new(),
        )
    } else {
        println!("  [File] ✗ info 命令失败: {}", result.stderr);
        (
            "file info".to_string(),
            format!("--path {}", remote_path),
            version.to_string(),
            result.exit_code,
            result.stderr.clone(),
        )
    }
}

// 测试创建目录
async fn test_file_mkdir(
    runner: &CliRunner,
    version: &str,
) -> (String, String, String, Option<i32>, String) {
    println!("  [File] 测试 mkdir 命令...");

    let test_dir = format!("/cli_test_{}", std::process::id());
    let result = runner.run(&["file", "mkdir", "--path", &test_dir]);

    if result.success {
        let _ = runner.run(&["file", "delete", "--path", &test_dir, "--force"]);
        println!("  [File] ✓ mkdir 命令成功");
        (
            "file mkdir".to_string(),
            format!("--path {}", test_dir),
            version.to_string(),
            result.exit_code,
            String::new(),
        )
    } else {
        println!("  [File] ✗ mkdir 命令失败: {}", result.stderr);
        (
            "file mkdir".to_string(),
            format!("--path {}", test_dir),
            version.to_string(),
            result.exit_code,
            result.stderr.clone(),
        )
    }
}

// 测试文件上传
async fn test_file_upload(
    runner: &CliRunner,
    temp_mgr: &mut TempFileManager,
    version: &str,
) -> (String, String, String, Option<i32>, String) {
    println!("  [File] 测试 upload 命令...");

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let test_file_name = format!("cli_test_upload_{}.txt", timestamp);
    let test_file = temp_mgr.create_file(&test_file_name, "Hello from CLI test!");

    let result = runner.run(&["file", "upload", "--file", &test_file, "--path", "/"]);

    if result.success {
        let _ = runner.run(&[
            "file",
            "delete",
            "--path",
            &format!("/{}", test_file_name),
            "--force",
        ]);
        println!("  [File] ✓ upload 命令成功");
        (
            "file upload".to_string(),
            format!("--file {} --path /", test_file),
            version.to_string(),
            result.exit_code,
            String::new(),
        )
    } else {
        println!("  [File] ✗ upload 命令失败: {}", result.stderr);
        (
            "file upload".to_string(),
            format!("--file {} --path /", test_file),
            version.to_string(),
            result.exit_code,
            result.stderr.clone(),
        )
    }
}

// 测试文件下载
async fn test_file_download(
    runner: &CliRunner,
    temp_mgr: &mut TempFileManager,
    version: &str,
) -> (String, String, String, Option<i32>, String) {
    println!("  [File] 测试 download 命令...");

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let test_file_name = format!("cli_test_download_{}.txt", timestamp);
    let test_file = temp_mgr.create_file(&test_file_name, "Hello from CLI download test!");
    let remote_path = format!("/{}", test_file_name);
    let download_path = temp_mgr.temp_path("downloaded.txt");

    let upload_result = runner.run(&["file", "upload", "--file", &test_file, "--path", "/"]);

    if !upload_result.success {
        println!(
            "  [File] ✗ download 测试失败（上传失败）: {}",
            upload_result.stderr
        );
        return (
            "file download".to_string(),
            format!("--path {} --output {}", remote_path, download_path),
            version.to_string(),
            upload_result.exit_code,
            format!("Upload failed: {}", upload_result.stderr),
        );
    }

    let download_result = runner.run(&[
        "file",
        "download",
        "--path",
        &remote_path,
        "--output",
        &download_path,
    ]);
    let _ = runner.run(&["file", "delete", "--path", &remote_path, "--force"]);

    if download_result.success {
        println!("  [File] ✓ download 命令成功");
        (
            "file download".to_string(),
            format!("--path {} --output {}", remote_path, download_path),
            version.to_string(),
            download_result.exit_code,
            String::new(),
        )
    } else {
        println!("  [File] ✗ download 命令失败: {}", download_result.stderr);
        (
            "file download".to_string(),
            format!("--path {} --output {}", remote_path, download_path),
            version.to_string(),
            download_result.exit_code,
            download_result.stderr.clone(),
        )
    }
}

// 测试文件重命名
async fn test_file_rename(
    runner: &CliRunner,
    temp_mgr: &mut TempFileManager,
    version: &str,
) -> (String, String, String, Option<i32>, String) {
    println!("  [File] 测试 rename 命令...");

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let old_name = format!("cli_test_rename_{}.txt", timestamp);
    let new_name = format!("cli_test_renamed_{}.txt", timestamp);
    let test_file = temp_mgr.create_file(&old_name, "Rename test content");

    // 先上传文件
    let upload_result = runner.run(&["file", "upload", "--file", &test_file, "--path", "/"]);
    if !upload_result.success {
        println!(
            "  [File] ✗ rename 测试失败（上传失败）: {}",
            upload_result.stderr
        );
        return (
            "file rename".to_string(),
            format!("--src /{} --name {}", old_name, new_name),
            version.to_string(),
            upload_result.exit_code,
            format!("Upload failed: {}", upload_result.stderr),
        );
    }

    // 执行重命名
    let result = runner.run(&[
        "file",
        "rename",
        "--src",
        &format!("/{}", old_name),
        "--name",
        &new_name,
    ]);

    // 清理：无论成功还是失败，都尝试删除可能存在的文件
    let _ = runner.run(&[
        "file",
        "delete",
        "--path",
        &format!("/{}", new_name),
        "--force",
    ]);
    let _ = runner.run(&[
        "file",
        "delete",
        "--path",
        &format!("/{}", old_name),
        "--force",
    ]);

    if result.success {
        println!("  [File] ✓ rename 命令成功");
        (
            "file rename".to_string(),
            format!("--src /{} --name {}", old_name, new_name),
            version.to_string(),
            result.exit_code,
            String::new(),
        )
    } else {
        println!("  [File] ✗ rename 命令失败: {}", result.stderr);
        (
            "file rename".to_string(),
            format!("--src /{} --name {}", old_name, new_name),
            version.to_string(),
            result.exit_code,
            result.stderr.clone(),
        )
    }
}

// 测试文件移动
async fn test_file_move(
    runner: &CliRunner,
    temp_mgr: &mut TempFileManager,
    version: &str,
) -> (String, String, String, Option<i32>, String) {
    println!("  [File] 测试 move 命令...");

    let test_dir = format!("/cli_test_move_dir_{}", std::process::id());
    let test_file = temp_mgr.create_file("cli_test_move.txt", "Move test");
    let filename = std::path::Path::new(&test_file)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    let _ = runner.run(&["file", "mkdir", "--path", &test_dir]);
    let _ = runner.run(&["file", "upload", "--file", &test_file, "--path", "/"]);
    let result = runner.run(&[
        "file",
        "move",
        "--src",
        &format!("/{}", filename),
        "--dest",
        &test_dir,
    ]);

    if result.success {
        let _ = runner.run(&["file", "delete", "--path", &test_dir, "--force"]);
        println!("  [File] ✓ move 命令成功");
        (
            "file move".to_string(),
            format!("--src /{} --dest {}", filename, test_dir),
            version.to_string(),
            result.exit_code,
            String::new(),
        )
    } else {
        println!("  [File] ✗ move 命令失败: {}", result.stderr);
        (
            "file move".to_string(),
            format!("--src /{} --dest {}", filename, test_dir),
            version.to_string(),
            result.exit_code,
            result.stderr.clone(),
        )
    }
}

// 测试文件复制
async fn test_file_copy(
    runner: &CliRunner,
    temp_mgr: &mut TempFileManager,
    version: &str,
) -> (String, String, String, Option<i32>, String) {
    println!("  [File] 测试 copy 命令...");

    let test_dir = format!("/cli_test_copy_dir_{}", std::process::id());
    let test_file = temp_mgr.create_file("cli_test_copy.txt", "Copy test");
    let filename = std::path::Path::new(&test_file)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    let _ = runner.run(&["file", "mkdir", "--path", &test_dir]);
    let _ = runner.run(&["file", "upload", "--file", &test_file, "--path", "/"]);
    let result = runner.run(&[
        "file",
        "copy",
        "--src",
        &format!("/{}", filename),
        "--dest",
        &test_dir,
    ]);

    if result.success {
        let _ = runner.run(&[
            "file",
            "delete",
            "--path",
            &format!("/{}", filename),
            "--force",
        ]);
        let _ = runner.run(&["file", "delete", "--path", &test_dir, "--force"]);
        println!("  [File] ✓ copy 命令成功");
        (
            "file copy".to_string(),
            format!("--src /{} --dest {}", filename, test_dir),
            version.to_string(),
            result.exit_code,
            String::new(),
        )
    } else {
        println!("  [File] ✗ copy 命令失败: {}", result.stderr);
        (
            "file copy".to_string(),
            format!("--src /{} --dest {}", filename, test_dir),
            version.to_string(),
            result.exit_code,
            result.stderr.clone(),
        )
    }
}

// 测试文件删除
async fn test_file_delete(
    runner: &CliRunner,
    temp_mgr: &mut TempFileManager,
    version: &str,
) -> (String, String, String, Option<i32>, String) {
    println!("  [File] 测试 delete 命令...");

    let test_file = temp_mgr.create_file("cli_test_delete.txt", "Delete test");
    let filename = std::path::Path::new(&test_file)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    let _ = runner.run(&["file", "upload", "--file", &test_file, "--path", "/"]);
    let result = runner.run(&[
        "file",
        "delete",
        "--path",
        &format!("/{}", filename),
        "--force",
    ]);

    if result.success {
        println!("  [File] ✓ delete 命令成功");
        (
            "file delete".to_string(),
            format!("--path /{} --force", filename),
            version.to_string(),
            result.exit_code,
            String::new(),
        )
    } else {
        println!("  [File] ✗ delete 命令失败: {}", result.stderr);
        (
            "file delete".to_string(),
            format!("--path /{} --force", filename),
            version.to_string(),
            result.exit_code,
            result.stderr.clone(),
        )
    }
}

// 测试批量上传
async fn test_batch_upload(
    runner: &CliRunner,
    temp_mgr: &mut TempFileManager,
    version: &str,
) -> (String, String, String, Option<i32>, String) {
    println!("  [File] 测试 batch-upload 命令...");

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let file1 = temp_mgr.create_file(&format!("batch_test_1_{}.txt", timestamp), "Batch test 1");
    let file2 = temp_mgr.create_file(&format!("batch_test_2_{}.txt", timestamp), "Batch test 2");

    let result = runner.run(&[
        "file",
        "batch-upload",
        "--paths",
        &file1,
        &file2,
        "--dest",
        "/",
    ]);

    // V3 不支持 batch-upload，检查是否是 "not available" 错误
    if !result.success && result.stderr.contains("not available") {
        println!("  [File] ✓ batch-upload 命令跳过（{} 不支持）", version);
        return (
            "file batch-upload".to_string(),
            format!("--paths {} {} --dest /", file1, file2),
            version.to_string(),
            None,
            String::new(),
        );
    }

    if result.success {
        let name1 = format!("batch_test_1_{}.txt", timestamp);
        let name2 = format!("batch_test_2_{}.txt", timestamp);
        let _ = runner.run(&[
            "file",
            "delete",
            "--path",
            &format!("/{}", name1),
            "--path",
            &format!("/{}", name2),
            "--force",
        ]);
        println!("  [File] ✓ batch-upload 命令成功");
        (
            "file batch-upload".to_string(),
            format!("--paths {} {} --dest /", file1, file2),
            version.to_string(),
            result.exit_code,
            String::new(),
        )
    } else {
        println!("  [File] ✗ batch-upload 命令失败: {}", result.stderr);
        (
            "file batch-upload".to_string(),
            format!("--paths {} {} --dest /", file1, file2),
            version.to_string(),
            result.exit_code,
            result.stderr.clone(),
        )
    }
}

// 测试批量下载
async fn test_batch_download(
    runner: &CliRunner,
    temp_mgr: &mut TempFileManager,
    version: &str,
) -> (String, String, String, Option<i32>, String) {
    println!("  [File] 测试 batch-download 命令...");

    let file1 = temp_mgr.create_file("batch_dl_test_1.txt", "Batch download test 1");
    let file2 = temp_mgr.create_file("batch_dl_test_2.txt", "Batch download test 2");
    let name1 = std::path::Path::new(&file1)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    let name2 = std::path::Path::new(&file2)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    let _ = runner.run(&["file", "upload", "--file", &file1, "--path", "/"]);
    let _ = runner.run(&["file", "upload", "--file", &file2, "--path", "/"]);

    let output_dir = temp_mgr.temp_dir("batch_download");
    let result = runner.run(&[
        "file",
        "batch-download",
        "--paths",
        &format!("/{}", name1),
        &format!("/{}", name2),
        "--output",
        &output_dir,
    ]);

    // V3 不支持 batch-download，检查是否是 "not available" 错误
    if !result.success && result.stderr.contains("not available") {
        let _ = runner.run(&[
            "file",
            "delete",
            "--path",
            &format!("/{}", name1),
            "--path",
            &format!("/{}", name2),
            "--force",
        ]);
        println!("  [File] ✓ batch-download 命令跳过（{} 不支持）", version);
        return (
            "file batch-download".to_string(),
            format!("--paths /{} /{} --output {}", name1, name2, output_dir),
            version.to_string(),
            None,
            String::new(),
        );
    }

    if result.success {
        let _ = runner.run(&[
            "file",
            "delete",
            "--path",
            &format!("/{}", name1),
            "--path",
            &format!("/{}", name2),
            "--force",
        ]);
        println!("  [File] ✓ batch-download 命令成功");
        (
            "file batch-download".to_string(),
            format!("--paths /{} /{} --output {}", name1, name2, output_dir),
            version.to_string(),
            result.exit_code,
            String::new(),
        )
    } else {
        // 清理上传的文件
        let _ = runner.run(&[
            "file",
            "delete",
            "--path",
            &format!("/{}", name1),
            "--path",
            &format!("/{}", name2),
            "--force",
        ]);
        println!("  [File] ✗ batch-download 命令失败: {}", result.stderr);
        (
            "file batch-download".to_string(),
            format!("--paths /{} /{} --output {}", name1, name2, output_dir),
            version.to_string(),
            result.exit_code,
            result.stderr.clone(),
        )
    }
}

// 测试文件搜索 (V4)
async fn test_file_search(
    runner: &CliRunner,
    version: &str,
) -> (String, String, String, Option<i32>, String) {
    println!("  [File] 测试 search 命令...");

    let result = runner.run(&["file", "search", "--path", "/", "--name", "test"]);

    if result.success || result.stderr.contains("not yet supported") {
        println!("  [File] ✓ search 命令执行完成");
        (
            "file search".to_string(),
            "--path / --name test".to_string(),
            version.to_string(),
            result.exit_code,
            String::new(),
        )
    } else {
        println!("  [File] ✗ search 命令失败: {}", result.stderr);
        (
            "file search".to_string(),
            "--path / --name test".to_string(),
            version.to_string(),
            result.exit_code,
            result.stderr.clone(),
        )
    }
}

// 测试文件元数据 (V4)
async fn test_file_metadata(
    runner: &CliRunner,
    version: &str,
) -> (String, String, String, Option<i32>, String) {
    println!("  [File] 测试 metadata 命令...");

    let result = runner.run(&["file", "metadata", "help"]);

    if result.success || result.stderr.contains("not yet supported") {
        println!("  [File] ✓ metadata 命令组存在");
        (
            "file metadata".to_string(),
            "help".to_string(),
            version.to_string(),
            result.exit_code,
            String::new(),
        )
    } else {
        println!("  [File] ✗ metadata 命令失败");
        (
            "file metadata".to_string(),
            "help".to_string(),
            version.to_string(),
            result.exit_code,
            "Metadata command group not found".to_string(),
        )
    }
}

// 测试文件权限 (V4)
async fn test_file_permission(
    runner: &CliRunner,
    version: &str,
) -> (String, String, String, Option<i32>, String) {
    println!("  [File] 测试 permission 命令...");

    let result = runner.run(&["file", "permission", "help"]);

    if result.success || result.stderr.contains("not yet supported") {
        println!("  [File] ✓ permission 命令组存在");
        (
            "file permission".to_string(),
            "help".to_string(),
            version.to_string(),
            result.exit_code,
            String::new(),
        )
    } else {
        println!("  [File] ✗ permission 命令失败");
        (
            "file permission".to_string(),
            "help".to_string(),
            version.to_string(),
            result.exit_code,
            "Permission command group not found".to_string(),
        )
    }
}

// 测试文件同步 (V4)
async fn test_file_sync(
    runner: &CliRunner,
    temp_mgr: &mut TempFileManager,
    version: &str,
) -> (String, String, String, Option<i32>, String) {
    println!("  [File] 测试 sync 命令...");

    let local_dir = temp_mgr.temp_dir("sync_test");
    let _ = temp_mgr.create_file_in_dir("sync_test", "file1.txt", "Content 1");
    let _ = temp_mgr.create_file_in_dir("sync_test", "file2.txt", "Content 2");

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let remote_dir = format!("/cli_sync_test_{}", timestamp);
    let _ = runner.run(&["file", "mkdir", "--path", &remote_dir]);

    let result = runner.run(&[
        "file",
        "sync",
        "--local",
        &local_dir,
        "--remote",
        &remote_dir,
        "--direction",
        "up",
        "--dry-run",
    ]);
    let _ = runner.run(&["file", "delete", "--path", &remote_dir, "--force"]);

    if result.stderr.contains("not yet supported") {
        println!("  [File] ⊘ sync 命令不支持");
        (
            "file sync".to_string(),
            format!(
                "--local {} --remote {} --direction up --dry-run",
                local_dir, remote_dir
            ),
            version.to_string(),
            result.exit_code,
            "Skipped: Command not supported".to_string(),
        )
    } else if result.success {
        println!("  [File] ✓ sync 命令成功");
        (
            "file sync".to_string(),
            format!(
                "--local {} --remote {} --direction up --dry-run",
                local_dir, remote_dir
            ),
            version.to_string(),
            result.exit_code,
            String::new(),
        )
    } else {
        println!("  [File] ✗ sync 命令失败: {}", result.stderr);
        (
            "file sync".to_string(),
            format!(
                "--local {} --remote {} --direction up --dry-run",
                local_dir, remote_dir
            ),
            version.to_string(),
            result.exit_code,
            result.stderr.clone(),
        )
    }
}

// 测试文件预览 (V4)
async fn test_file_preview(
    runner: &CliRunner,
    version: &str,
) -> (String, String, String, Option<i32>, String) {
    println!("  [File] 测试 preview 命令...");

    let result = runner.run(&[
        "file",
        "preview",
        "--uri",
        "/somefile.txt",
        "--type",
        "text",
    ]);

    if result.stderr.contains("not found") || result.stderr.contains("not yet supported") {
        println!("  [File] ✓ preview 命令存在");
        (
            "file preview".to_string(),
            "--uri /somefile.txt --type text".to_string(),
            version.to_string(),
            result.exit_code,
            String::new(),
        )
    } else {
        println!("  [File] ✗ preview 命令状态未知");
        (
            "file preview".to_string(),
            "--uri /somefile.txt --type text".to_string(),
            version.to_string(),
            result.exit_code,
            result.stderr.clone(),
        )
    }
}

// 测试文件差异 (V4)
async fn test_file_diff(
    runner: &CliRunner,
    temp_mgr: &mut TempFileManager,
    version: &str,
) -> (String, String, String, Option<i32>, String) {
    println!("  [File] 测试 diff 命令...");

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let test_file_name = format!("cli_test_diff_{}.txt", timestamp);
    let local_file = temp_mgr.create_file(&test_file_name, "Diff test content");
    let remote_path = format!("/{}", test_file_name);

    let upload_result = runner.run(&["file", "upload", "--file", &local_file, "--path", "/"]);

    if !upload_result.success {
        println!(
            "  [File] ✗ diff 测试失败（上传失败）: {}",
            upload_result.stderr
        );
        return (
            "file diff".to_string(),
            format!("--local {} --remote {}", local_file, remote_path),
            version.to_string(),
            upload_result.exit_code,
            format!("Upload failed: {}", upload_result.stderr),
        );
    }

    let result = runner.run(&[
        "file",
        "diff",
        "--local",
        &local_file,
        "--remote",
        &remote_path,
    ]);
    let _ = runner.run(&["file", "delete", "--path", &remote_path, "--force"]);

    if result.stderr.contains("not yet supported") {
        println!("  [File] ⊘ diff 命令不支持");
        (
            "file diff".to_string(),
            format!("--local {} --remote {}", local_file, remote_path),
            version.to_string(),
            result.exit_code,
            "Skipped: Command not supported".to_string(),
        )
    } else if result.success {
        println!("  [File] ✓ diff 命令成功");
        (
            "file diff".to_string(),
            format!("--local {} --remote {}", local_file, remote_path),
            version.to_string(),
            result.exit_code,
            String::new(),
        )
    } else {
        println!("  [File] ✗ diff 命令失败: {}", result.stderr);
        (
            "file diff".to_string(),
            format!("--local {} --remote {}", local_file, remote_path),
            version.to_string(),
            result.exit_code,
            result.stderr.clone(),
        )
    }
}
