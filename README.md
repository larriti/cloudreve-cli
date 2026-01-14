# Cloudreve CLI

Command-line interface for Cloudreve API.

## Installation

```bash
cargo install cloudreve-cli
```

## Usage

```bash
# Show help
cloudreve-cli --help

# Authenticate with your Cloudreve instance
cloudreve-cli --url https://your-cloudreve-instance.com auth

# List files in root directory
cloudreve-cli file list /

# Upload a file
cloudreve-cli file upload --file /path/to/file --path /destination

# Get user information
cloudreve-cli user info

# Create a share link
cloudreve-cli share create --uri /path/to/file --name "My Shared File"
```

## Environment Variables

You can also set the following environment variables:

- `CLOUDREVE_URL`: Cloudreve instance URL
- `CLOUDREVE_TOKEN`: Authentication token

## License

This project is licensed under the MIT License.
