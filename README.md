# AWS Connect TOTP Tool

A command-line TOTP (Time-based One-Time Password) tool designed for AWS MFA authentication. This tool securely stores TOTP secrets in your system's keychain and generates tokens for AWS CLI MFA processes.

## Features

- **Secure Storage**: Uses your system's keychain (macOS Keychain, Windows Credential Manager, Linux Secret Service) to store TOTP secrets
- **AWS Integration**: Designed to work with AWS CLI `mfa_process` configuration
- **Simple CLI**: Easy-to-use command-line interface
- **Cross-platform**: Works on macOS, Windows, and Linux

## Platform Support

The tool automatically uses the appropriate secure storage backend for your platform:

- **macOS**: Uses the macOS Keychain (Security Framework)
- **Windows**: Uses Windows Credential Manager (Windows API)
- **Linux**: Uses the Secret Service API (GNOME Keyring, KDE KWallet, etc.)

The platform-specific features are automatically selected at compile time, so you only get the dependencies you need for your target platform.

## Installation

### From Source

```bash
# Clone the repository
git clone <your-repo-url>
cd awsconnect

# Build the project
cargo build --release

# The binary will be available at ./target/release/awsconnect
```

### Cross-compilation

To build for different platforms:

```bash
# For Linux (from macOS/Windows)
cargo build --release --target x86_64-unknown-linux-gnu

# For Windows (from macOS/Linux) 
cargo build --release --target x86_64-pc-windows-gnu

# For macOS (from Linux/Windows)
cargo build --release --target x86_64-apple-darwin
```

Note: Cross-compilation may require additional setup and toolchains for the target platform.

### Copy to PATH

```bash
# Copy the binary to a directory in your PATH
sudo cp ./target/release/awsconnect /usr/local/bin/
# or
cp ./target/release/awsconnect ~/.local/bin/
```

## Usage

### Store a TOTP Secret

Store a TOTP secret for your AWS MFA device:

```bash
awsconnect store --name "aws-mfa-device" --secret "YOUR_BASE32_TOTP_SECRET"
```

The secret should be the base32-encoded string you get when setting up your MFA device.

### Generate TOTP Token

Generate a current TOTP token:

```bash
awsconnect generate --name "aws-mfa-device"
```

This will output just the 6-digit token, making it perfect for scripting.

### List Stored Secrets

```bash
awsconnect list
```

Note: Due to keychain limitations, this doesn't list actual secret names but provides guidance on testing if a secret exists.

### Remove a Secret

Remove a stored TOTP secret:

```bash
awsconnect remove --name "aws-mfa-device"
```

## AWS CLI Integration

To use this tool with AWS CLI for MFA, add the following to your AWS CLI configuration:

### 1. Update `~/.aws/config`

```ini
[profile your-profile-name]
region = us-east-1
mfa_process = awsconnect generate --name aws-mfa-device
```

### 2. Update `~/.aws/credentials`

```ini
[your-profile-name]
aws_access_key_id = YOUR_ACCESS_KEY_ID
aws_secret_access_key = YOUR_SECRET_ACCESS_KEY
aws_mfa_device = arn:aws:iam::ACCOUNT_ID:mfa/USERNAME
```

### 3. Use AWS CLI with MFA

```bash
aws s3 ls --profile your-profile-name
```

The AWS CLI will automatically call `awsconnect generate --name aws-mfa-device` to get the TOTP token and authenticate.

## Getting Your TOTP Secret

When setting up MFA on AWS:

1. Go to AWS IAM → Users → [Your User] → Security credentials
2. Click "Assign MFA device"
3. Choose "Virtual MFA device"
4. When the QR code appears, click "Show secret key"
5. Copy the secret key (base32 string) and use it with the `store` command

## Security

- TOTP secrets are stored securely in your system's keychain
- Secrets are never written to disk in plain text
- The tool validates TOTP secrets before storing them

## Examples

```bash
# Store your AWS MFA secret
awsconnect store --name "aws-work" --secret "JBSWY3DPEHPK3PXP"

# Generate a token
awsconnect generate --name "aws-work"
# Output: 123456

# Use in a script
TOKEN=$(awsconnect generate --name "aws-work")
echo "Current token: $TOKEN"

# Remove the secret when no longer needed
awsconnect remove --name "aws-work"
```

## Troubleshooting

### "Failed to retrieve secret from keyring"

This error occurs when:
- The secret name doesn't exist
- The keyring is locked
- Permission issues with the keychain

### "Invalid secret format"

This error occurs when:
- The provided secret is not valid base32
- The secret is malformed

### Keychain Access on macOS

You may be prompted to allow access to the keychain. Click "Allow" or "Always Allow" to grant permission.

## Dependencies

- `keyring`: For secure storage in system keychain
- `totp-rs`: For TOTP token generation
- `clap`: For command-line interface
- `anyhow`: For error handling
- `tokio`: For async runtime

## License

[Your License Here]
