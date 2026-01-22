use cloudreve_api::{CloudreveAPI, Result};
use glob::Pattern as GlobPattern;

/// 展开通配符模式，返回匹配的文件列表（本地文件）
pub fn expand_glob_patterns(patterns: &[String]) -> Vec<String> {
    let mut result = Vec::new();

    for pattern in patterns {
        if pattern.contains('*') || pattern.contains('?') {
            // 通配符模式
            if let Ok(matches) = glob::glob(pattern) {
                for entry in matches.filter_map(|e| e.ok()).filter(|p| p.is_file()) {
                    result.push(entry.to_string_lossy().to_string());
                }
            }
        } else {
            // 普通路径
            result.push(pattern.clone());
        }
    }

    result.sort();
    result.dedup();
    result
}

/// 测试路径是否包含通配符
pub fn has_glob_pattern(path: &str) -> bool {
    path.contains('*') || path.contains('?')
}

/// 解析远程通配符模式
///
/// 支持格式：
/// - `*.txt` -> 目录: "/", 模式: "*.txt"
/// - `/path/*.txt` -> 目录: "/path/", 模式: "*.txt"
/// - `/path/foo*.txt` -> 目录: "/path/", 模式: "foo*.txt"
fn parse_remote_pattern(pattern: &str) -> Result<(String, GlobPattern)> {
    let pattern = pattern.trim_start_matches("cloudreve://");

    // 查找最后一个 '/'
    let last_slash = pattern.rfind('/');

    let (dir, file_pattern) = match last_slash {
        Some(pos) => {
            let dir = if pos == 0 {
                "/".to_string()
            } else {
                format!("{}/", &pattern[..pos])
            };
            let file_pattern = &pattern[pos + 1..];
            (dir, file_pattern)
        }
        None => ("/".to_string(), pattern),
    };

    // 编译 glob 模式
    let glob_pattern = GlobPattern::new(file_pattern).map_err(|e| {
        cloudreve_api::Error::InvalidResponse(format!("Invalid glob pattern: {}", e))
    })?;

    Ok((dir, glob_pattern))
}

/// 扩展远程通配符模式
///
/// # 参数
/// - `api`: Cloudreve API 客户端
/// - `patterns`: 要扩展的模式列表
/// - `include_folders`: 是否包含文件夹（默认为 false，只返回文件）
///
/// # 示例
/// ```
/// // 只匹配文件
/// let patterns = vec!["*.txt".to_string()];
/// let files = expand_remote_patterns(api, &patterns, false).await?;
///
/// // 匹配所有内容（文件和文件夹）- 用于 /* 模式
/// let patterns = vec!["/*".to_string()];
/// let all = expand_remote_patterns(api, &patterns, true).await?;
/// ```
pub async fn expand_remote_patterns(
    api: &CloudreveAPI,
    patterns: &[String],
    include_folders: bool,
) -> Result<Vec<String>> {
    let mut result = Vec::new();

    for pattern in patterns {
        if has_glob_pattern(pattern) {
            // 解析模式
            let (dir, glob_pattern) = parse_remote_pattern(pattern)?;

            // 列出目录下的所有文件
            let file_list = api.list_files_all(&dir, None).await?;

            // 过滤匹配的文件
            for file in file_list.items() {
                // 根据 include_folders 决定是否包含文件夹
                if !include_folders && file.is_folder {
                    continue;
                }

                if glob_pattern.matches(&file.name) {
                    // 构建完整路径：确保目录和文件名之间有 '/'
                    let full_path = if dir == "/" {
                        format!("/{}", file.name)
                    } else {
                        format!("{}/{}", dir.trim_end_matches('/'), file.name)
                    };
                    result.push(full_path);
                }
            }
        } else {
            // 普通路径，直接添加
            result.push(pattern.clone());
        }
    }

    result.sort();
    result.dedup();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_remote_pattern() {
        // 测试简单模式
        let (dir, pattern) = parse_remote_pattern("*.txt").unwrap();
        assert_eq!(dir, "/");
        assert!(pattern.matches("file.txt"));
        assert!(!pattern.matches("file.pdf"));

        // 测试带路径的模式
        let (dir, pattern) = parse_remote_pattern("/docs/*.txt").unwrap();
        assert_eq!(dir, "/docs/");
        assert!(pattern.matches("file.txt"));
        assert!(!pattern.matches("file.pdf"));

        // 测试带前缀的模式
        let (dir, pattern) = parse_remote_pattern("/path/foo*.txt").unwrap();
        assert_eq!(dir, "/path/");
        assert!(pattern.matches("foo123.txt"));
        assert!(pattern.matches("foobar.txt"));
        assert!(!pattern.matches("bar123.txt"));

        // 测试 /* 模式（清空文件夹）
        let (dir, pattern) = parse_remote_pattern("/*").unwrap();
        assert_eq!(dir, "/");
        assert!(pattern.matches("*")); // * 匹配所有内容
    }
}
