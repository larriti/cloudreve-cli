/// Utility functions for CLI display formatting

/// Format bytes into human-readable size (KB, MB, GB, TB)
pub fn format_bytes(bytes: i64) -> String {
    const TB: i64 = 1024 * 1024 * 1024 * 1024;
    const GB: i64 = 1024 * 1024 * 1024;
    const MB: i64 = 1024 * 1024;
    const KB: i64 = 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
