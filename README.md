# AWS Connect TOTP Tool

A command-line TOTP (Time-based One-Time Password) tool designed for AWS MFA authentication. This tool securely stores TOTP secrets in your system's keychain and generates tokens for AWS CLI MFA processes.

## Installation

### From Homebrew

`brew install carlos-chen-tb/tap/awsconnect` OR

```bash
brew tap carlos-chen-tb/tap
brew install awsconnect
```

## Usage Examples

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
