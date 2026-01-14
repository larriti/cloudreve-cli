# cloudreve-cli

[![Crates.io](https://img.shields.io/crates/v/cloudreve-cli)](https://crates.io/crates/cloudreve-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A powerful command-line interface for [Cloudreve](https://github.com/cloudreve/Cloudreve), providing easy access to all major Cloudreve API features from your terminal.

## Features

### Core Features
- 📁 **File Operations**: Upload, download, list, move, copy, rename, delete files
- 🔍 **File Search**: Search files by name, type, size, or extension with recursive support
- 📦 **Batch Operations**: Upload/download multiple files or entire directories at once
- 🔗 **Share Management**: Create, list, update, and delete share links
- 👤 **User Management**: View profile, storage quota, and manage settings

### Advanced Features
- 🔄 **File Sync**: Synchronize local and remote files with dry-run preview
- 📄 **File Preview**: Preview text and JSON files directly in the terminal
- 📊 **File Comparison**: Compare local and remote files for differences
- 📈 **Progress Bars**: Visual progress indicators for uploads and downloads

### User Experience
- ⚙️ **Configuration File**: Set default values in `~/.config/cloudreve-cli/config.toml`
- 🐚 **Shell Completion**: Generate completion scripts for bash, zsh, fish, and PowerShell
- 🎨 **Clean Output**: Minimal log output by default with `--log-prefix` option for debugging
- 📏 **Human-Readable Sizes**: Automatic size formatting (B/KB/MB/GB/TB)

## Installation

### Install from Crates.io

```bash
cargo install cloudreve-cli
```

### Build from Source

```bash
git clone https://github.com/larriti/cloudreve-cli
cd cloudreve-cli
cargo build --release
```

The binary will be available at `target/release/cloudreve-cli`.

## Quick Start

### 1. Authentication

First, authenticate with your Cloudreve instance:

```bash
cloudreve-cli --url https://your-cloudreve-instance.com auth
```

You'll be prompted for your email and password. The token is cached for future use.

### 2. Basic File Operations

```bash
# List files in root directory
cloudreve-cli file list --path /

# Upload a file
cloudreve-cli file upload --file ./photo.jpg --path /photos

# Download a file
cloudreve-cli file download --uri /photos/photo.jpg --output ./

# Get file information
cloudreve-cli file info --uri /photos/photo.jpg
```

### 3. Batch Operations

```bash
# Batch upload multiple files
cloudreve-cli file batch-upload --paths file1.txt file2.jpg --dest /docs

# Batch upload with recursive directory support
cloudreve-cli file batch-upload --paths ./my-folder --dest /backup --recursive

# Batch download multiple files
cloudreve-cli file batch-download --uris /file1 /file2 --output ./downloads
```

### 4. File Search

```bash
# Search by name
cloudreve-cli file search --name "report" --recursive

# Search PDF files larger than 1MB
cloudreve-cli file search --extension pdf --min-size 1048576 --recursive

# Search only folders
cloudreve-cli file search --type folder --name "backup"
```

### 5. File Synchronization

```bash
# Sync local files to remote (preview only)
cloudreve-cli file sync --local ./docs --remote /docs --direction up --dry-run

# Sync remote files to local
cloudreve-cli file sync --local ./docs --remote /docs --direction down
```

### 6. File Preview

```bash
# Preview text file
cloudreve-cli file preview --uri /notes.txt --type text

# Preview JSON with formatting
cloudreve-cli file preview --uri /config.json --type json
```

### 7. File Comparison

```bash
# Compare local and remote files
cloudreve-cli file diff --local ./file.txt --remote /file.txt
```

## Configuration File

Create a configuration file at `~/.config/cloudreve-cli/config.toml`:

```toml
# ~/.config/cloudreve-cli/config.toml
default_url = "https://your-cloudreve-instance.com"
default_email = "user@example.com"
default_policy = "1"
default_upload_path = "/"
default_download_dir = "./downloads"
log_level = "info"
```

With this configuration, you can omit the `--url` parameter:

```bash
cloudreve-cli file list --path /
```

## Shell Completion

Generate completion scripts for your shell:

### Zsh

```bash
cloudreve-cli completions --shell zsh > ~/.zsh/completion/_cloudreve-cli
```

Add to your `~/.zshrc`:
```bash
fpath=(~/.zsh/completion $fpath)
autoload -U compinit && compinit
```

### Bash

```bash
cloudreve-cli completions --shell bash > ~/.local/share/bash-completion/completions/cloudreve-cli
```

### Fish

```bash
cloudreve-cli completions --shell fish > ~/.config/fish/completions/cloudreve-cli.fish
```

## Command Reference

### Global Options

| Option | Description |
|--------|-------------|
| `--url` | Cloudreve instance URL |
| `--email` | Login email |
| `--token` | Authentication token |
| `--log-level` | Log level (trace, debug, info, warn, error) |
| `--log-prefix` | Show full log prefix with timestamp and level |

### File Commands

| Command | Description |
|---------|-------------|
| `file list` | List files in a directory |
| `file info` | Get file information |
| `file upload` | Upload a file |
| `file download` | Download a file |
| `file delete` | Delete files |
| `file rename` | Rename a file |
| `file move` | Move files |
| `file copy` | Copy files |
| `file mkdir` | Create a directory |
| `file batch-upload` | Upload multiple files/directories |
| `file batch-download` | Download multiple files |
| `file search` | Search for files |
| `file sync` | Synchronize files |
| `file preview` | Preview file content |
| `file diff` | Compare local and remote files |

### User Commands

| Command | Description |
|---------|-------------|
| `user info` | Get user information |
| `user quota` | View storage quota |
| `user policies` | List storage policies |

### Share Commands

| Command | Description |
|---------|-------------|
| `share list` | List my share links |
| `share create` | Create a share link |
| `share update` | Update a share link |
| `share delete` | Delete a share link |

### Settings Commands

| Command | Description |
|---------|-------------|
| `settings get` | Get settings |
| `settings set` | Set configuration value |

## Examples

### Upload with Progress Bar

```bash
cloudreve-cli file upload --file large-file.zip --path /uploads
# [00:00:15] [======================>] 50/100 chunks (0:00:15)
```

### Batch Upload Directory

```bash
cloudreve-cli file batch-upload --paths ./project --dest /backups --recursive
# Starting batch upload of 1 items
# ✓ Uploaded: ./project/src/main.rs
# ✓ Uploaded: ./project/src/utils.rs
# Batch upload summary:
#   Uploaded: 15 files
#   Failed: 0 files
#   Total size: 1.25 MB
```

### Search and Download

```bash
# Find all PDF files
cloudreve-cli file search --extension pdf --recursive

# Download the found files
cloudreve-cli file batch-download --uris /doc1.pdf /doc2.pdf --output ./pdfs
```

## Troubleshooting

### Clean Output vs Debug Output

By default, the CLI shows clean output without log prefixes. For debugging, use `--log-prefix`:

```bash
# Clean output (default)
cloudreve-cli file list --path /
# Listing files in path: /

# With full log prefix
cloudreve-cli file list --path / --log-prefix
# [2025-01-14T12:34:56Z INFO] Listing files in path: /
```

### Token Caching

Tokens are cached in `~/.cache/cloudreve-cli/tokens.json`. To re-authenticate:

```bash
cloudreve-cli auth
```

### Connection Issues

If you encounter connection issues:

1. Verify the URL is correct (include `https://`)
2. Check your network connection
3. Use `--log-level debug` for detailed logs

```bash
cloudreve-cli --log-level debug --url https://instance.com file list --path /
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Support

If you encounter any issues or have questions, please [file an issue](https://github.com/larriti/cloudreve-cli/issues).

## Acknowledgments

- Built with [Clap](https://github.com/clap-rs/clap) for CLI parsing
- Uses [cloudreve-api](https://github.com/larriti/cloudreve-api) library
- Part of the [Cloudreve](https://github.com/cloudreve/Cloudreve) ecosystem
