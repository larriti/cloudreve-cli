use cloudreve_api::Result;
use cloudreve_api::api::v4::ApiV4Client;
use cloudreve_api::api::v4::models::TaskType;
use log::info;
use serde_json::Value;

pub async fn handle_list(client: &ApiV4Client, category: String, per_page: String) -> Result<()> {
    info!("Listing workflow tasks (category: {})...", category);

    let page_size = per_page.parse().unwrap_or(25);
    let response = client.list_workflow_tasks(page_size, &category).await?;

    if response.tasks.is_empty() {
        info!("No tasks found");
        return Ok(());
    }

    info!("");
    info!(
        "╔══════════════════════════════════════════════════════════════════════════════════════╗"
    );
    info!("║ 📋 Workflow Tasks - Total: {:<68}║", response.tasks.len());
    info!(
        "╠══════════════════════════════════════════════════════════════════════════════════════╣"
    );

    for task in &response.tasks {
        let status_icon = match task.status {
            cloudreve_api::api::v4::models::TaskStatus::Queued => "⏳",
            cloudreve_api::api::v4::models::TaskStatus::Processing => "🔄",
            cloudreve_api::api::v4::models::TaskStatus::Suspending => "⏸️",
            cloudreve_api::api::v4::models::TaskStatus::Error => "❌",
            cloudreve_api::api::v4::models::TaskStatus::Canceled => "🚫",
            cloudreve_api::api::v4::models::TaskStatus::Completed => "✅",
        };

        let type_icon = match task.r#type {
            TaskType::RemoteDownload => "📥",
            TaskType::CreateArchive => "📦",
            TaskType::ExtractArchive => "📂",
            TaskType::Import => "📥",
            TaskType::Relocate => "🔄",
            TaskType::MediaMeta => "🏷️",
            _ => "📋",
        };

        let type_str = format!("{:?}", task.r#type);
        let type_str = type_str.to_lowercase().replace('_', " ");

        // 格式化时间
        let created = task
            .created_at
            .replace('T', " ")
            .replace("+00:00", " UTC")
            .trim()
            .to_string();
        let created = created.chars().take(19).collect::<String>();

        info!(
            "║ {} {} {} | {} | {:<20} ║",
            status_icon, type_icon, task.id, type_str, created
        );

        // 从 props 中提取详细信息
        if let Some(summary) = &task.summary
            && let Some(props) = summary.props.as_object()
        {
            // 根据任务类型显示不同信息
            match task.r#type {
                TaskType::RemoteDownload | TaskType::Import => {
                    // 下载/导入任务：显示 URL、目标路径、文件大小
                    let mut found_source = false;
                    for (key, label) in [
                        ("url", "📎 URL"),
                        ("url_str", "📎 URL"),
                        ("src", "📎 Source"),
                        ("src_str", "📎 Source"),
                        ("source", "📎 Source"),
                    ] {
                        if let Some(value) = props.get(key).and_then(|v| v.as_str())
                            && !value.is_empty()
                        {
                            // URL 完整显示，自动换行
                            info!("║   ├─ {}:", label);
                            for chunk in value.as_bytes().chunks(95) {
                                let chunk_str = std::str::from_utf8(chunk).unwrap_or("");
                                info!("║   │   {} ║", chunk_str);
                            }
                            info!("║   │");
                            found_source = true;
                            break;
                        }
                    }

                    if let Some(dst) = props
                        .get("dst")
                        .or(props.get("dst_str"))
                        .or(props.get("path"))
                        .and_then(|v| v.as_str())
                    {
                        info!("║   ├─ 💾 Save to: {} ║", truncate(dst, 95));
                        info!("║   │");
                    }
                    if let Some(name) = props.get("name").and_then(|v| v.as_str()) {
                        info!("║   ├─ 📄 Filename: {} ║", truncate(name, 93));
                        info!("║   │");
                    }
                    if let Some(size) = props
                        .get("size")
                        .or(props.get("total"))
                        .and_then(|v| v.as_i64().or_else(|| v.as_f64().map(|f| f as i64)))
                    {
                        info!("║   └─ 📊 Size: {} ║", format_size(size));
                    } else if !found_source {
                        info!("║   └");
                    }

                    // 如果没有找到源信息，显示所有可用的字段名来帮助调试
                    if !found_source && !props.is_empty() {
                        info!("║   │");
                        info!("║   ├─ 📋 Available fields:");
                        for (key, value) in props.iter().take(6) {
                            let value_preview = if value.is_string() {
                                let s = value.as_str().unwrap_or("");
                                if s.is_empty() {
                                    "\"\"".to_string()
                                } else {
                                    format!("\"{}\"", truncate(s, 35))
                                }
                            } else if value.is_number() {
                                format!("{}", value)
                            } else if value.is_array() {
                                let arr = value.as_array().unwrap();
                                if arr.is_empty() {
                                    "[]".to_string()
                                } else {
                                    format!("[{} items]", arr.len())
                                }
                            } else if value.is_object() {
                                let obj = value.as_object().unwrap();
                                format!(
                                    "{{{}}}",
                                    obj.keys().take(3).cloned().collect::<Vec<_>>().join(",")
                                )
                            } else if value.is_null() {
                                "null".to_string()
                            } else {
                                format!("{}", value)
                            };
                            info!(
                                "║   │    {}: {}",
                                truncate(key, 20),
                                truncate(&value_preview, 47)
                            );
                        }
                        if props.len() > 6 {
                            info!("║   └─ ... and {} more fields", props.len() - 6);
                        } else {
                            info!("║   └");
                        }
                    }
                }
                TaskType::CreateArchive => {
                    // 归档任务：显示源文件、目标路径
                    if let Some(src) = props.get("src").and_then(|v| v.as_array()) {
                        let files: Vec<&str> = src.iter().filter_map(|v| v.as_str()).collect();
                        if !files.is_empty() {
                            info!("║   ├─ 📁 Files:");
                            for (i, file) in files.iter().enumerate().take(3) {
                                if i == 2 && files.len() > 3 {
                                    info!("║   │      ... and {} more", files.len() - 2);
                                    break;
                                }
                                info!("║   │      {} ║", truncate(file, 84));
                            }
                            info!("║   │");
                        }
                    }
                    if let Some(dst) = props.get("dst").and_then(|v| v.as_str()) {
                        info!("║   └─ 📦 Output: {} ║", truncate(dst, 88));
                    }
                }
                TaskType::ExtractArchive => {
                    // 解压任务：显示归档文件、目标路径
                    if let Some(src) = props.get("src").and_then(|v| v.as_array())
                        && let Some(archive) = src.first().and_then(|v| v.as_str())
                    {
                        info!("║   ├─ 📦 Archive: {} ║", truncate(archive, 87));
                        info!("║   │");
                    }
                    if let Some(dst) = props.get("dst").and_then(|v| v.as_str()) {
                        info!("║   └─ 📂 Extract to: {} ║", truncate(dst, 85));
                    }
                }
                TaskType::Relocate => {
                    // 迁移任务：显示源文件、目标策略
                    if let Some(src) = props.get("src").and_then(|v| v.as_array()) {
                        let files: Vec<&str> = src.iter().filter_map(|v| v.as_str()).collect();
                        if !files.is_empty() {
                            info!("║   ├─ 📁 Files:");
                            for (i, file) in files.iter().enumerate().take(3) {
                                if i == 2 && files.len() > 3 {
                                    info!("║   │      ... and {} more", files.len() - 2);
                                    break;
                                }
                                info!("║   │      {} ║", truncate(file, 84));
                            }
                            info!("║   │");
                        }
                    }
                    if let Some(policy) = props
                        .get("dst_policy_id")
                        .or(props.get("dst_policy"))
                        .and_then(|v| v.as_str())
                    {
                        info!("║   └─ 🔄 Target policy: {} ║", truncate(policy, 83));
                    }
                }
                _ => {
                    // 其他任务类型：显示所有可用信息
                    if let Some(phase) = &summary.phase {
                        info!("║   ├─ 📍 Phase: {} ║", truncate(phase, 88));
                        info!("║   │");
                    }
                    // 显示 JSON 中的前几个字段
                    for (key, value) in props.iter().take(3) {
                        if key != "phase" {
                            let value_str = format_value(value);
                            info!(
                                "║   ├─ {}: {} ║",
                                truncate(key, 15),
                                truncate(&value_str, 75)
                            );
                        }
                    }
                    if props.len() > 3 {
                        info!("║   └─ ... and {} more fields", props.len() - 3);
                    } else {
                        info!("║   └");
                    }
                }
            }

            // 显示阶段信息（如果有）
            if !matches!(
                task.r#type,
                TaskType::RemoteDownload
                    | TaskType::CreateArchive
                    | TaskType::ExtractArchive
                    | TaskType::Import
                    | TaskType::Relocate
            ) && let Some(phase) = &summary.phase
            {
                info!("║   │");
                info!("║   └─ 📍 Phase: {} ║", truncate(phase, 85));
            }
        }

        // 显示错误信息
        if let Some(error) = &task.error {
            info!("║   │");
            info!("║   ❌ Error: {} ║", truncate(error, 88));
        }

        // 显示持续时间
        if let Some(duration) = task.duration {
            info!("║   ⏱️ Duration: {:.1}s ║", duration as f64 / 1000.0);
        }

        info!(
            "╠══════════════════════════════════════════════════════════════════════════════════════╣"
        );
    }

    info!(
        "║ Total: {} task(s) {} ║",
        response.tasks.len(),
        " ".repeat(77)
    );
    info!(
        "╚══════════════════════════════════════════════════════════════════════════════════════╝"
    );
    info!("");

    Ok(())
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

fn format_size(size: i64) -> String {
    const KB: i64 = 1024;
    const MB: i64 = KB * 1024;
    const GB: i64 = MB * 1024;
    const TB: i64 = GB * 1024;

    if size >= TB {
        format!("{:.2} TB", size as f64 / TB as f64)
    } else if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
}

fn format_value(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
        Value::Array(a) => format!("[{} items]", a.len()),
        Value::Object(o) => format!("{{{} fields}}", o.len()),
    }
}
