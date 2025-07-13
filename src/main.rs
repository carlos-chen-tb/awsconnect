use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use keyring::Entry;
use totp_rs::{Algorithm, Secret, TOTP};

#[derive(Parser)]
#[command(name = "awsconnect")]
#[command(about = "A TOTP tool for AWS MFA authentication")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Store a TOTP secret in the keystore
    Store {
        /// Name/identifier for the TOTP secret
        #[arg(short, long)]
        name: String,
        /// TOTP secret key (base32 encoded)
        #[arg(short, long)]
        secret: String,
    },
    /// Generate and display current TOTP token
    Generate {
        /// Name/identifier for the TOTP secret
        #[arg(short, long)]
        name: String,
    },
    /// List stored TOTP names
    List,
    /// Remove a stored TOTP secret
    Remove {
        /// Name/identifier for the TOTP secret to remove
        #[arg(short, long)]
        name: String,
    },
}

const SERVICE_NAME: &str = "awsconnect";

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Store { name, secret } => store_secret(&name, &secret).await?,
        Commands::Generate { name } => generate_token(&name).await?,
        Commands::List => list_secrets().await?,
        Commands::Remove { name } => remove_secret(&name).await?,
    }

    Ok(())
}

async fn store_secret(name: &str, secret: &str) -> Result<()> {
    // Validate the secret by trying to create a TOTP instance
    let _totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        Secret::Encoded(secret.to_string())
            .to_bytes()
            .context("Invalid secret format")?,
    )
    .context("Failed to create TOTP with provided secret")?;

    // Store the TOTP secret in the keyring
    let entry = Entry::new(SERVICE_NAME, name).context("Failed to create keyring entry")?;

    entry
        .set_password(secret)
        .context("Failed to store secret in keyring")?;

    println!("Successfully stored TOTP secret for '{name}'");
    Ok(())
}

async fn generate_token(name: &str) -> Result<()> {
    let entry = Entry::new(SERVICE_NAME, name).context("Failed to create keyring entry")?;

    let secret = entry
        .get_password()
        .context("Failed to retrieve secret from keyring")?;

    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        Secret::Encoded(secret)
            .to_bytes()
            .context("Invalid stored secret format")?,
    )
    .context("Failed to create TOTP instance")?;

    let token = totp
        .generate_current()
        .context("Failed to generate TOTP token")?;

    // Output just the token for use with AWS CLI
    println!("{token}");

    Ok(())
}

async fn list_secrets() -> Result<()> {
    // Note: keyring doesn't provide a direct way to list all entries
    // This is a limitation of the underlying keystore systems
    println!("Listing stored secrets is not supported by the keystore backend.");
    println!("Use 'generate --name <name>' to test if a secret exists.");

    Ok(())
}

async fn remove_secret(name: &str) -> Result<()> {
    let entry = Entry::new(SERVICE_NAME, name).context("Failed to create keyring entry")?;

    entry
        .delete_credential()
        .context("Failed to remove secret from keystore")?;

    println!("Successfully removed TOTP secret for '{name}'");
    Ok(())
}
