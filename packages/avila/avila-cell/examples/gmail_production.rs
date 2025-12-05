//! Gmail SMTP Production Example
//! 
//! This example demonstrates how to send emails through Gmail's SMTP server
//! with STARTTLS authentication.
//! 
//! # Setup
//! 
//! 1. Enable 2-Factor Authentication in your Gmail account
//! 2. Generate an App Password: https://myaccount.google.com/apppasswords
//! 3. Set environment variables:
//!    - GMAIL_USER: your email address (e.g., user@gmail.com)
//!    - GMAIL_APP_PASSWORD: the app password generated
//! 
//! # Usage
//! 
//! ```bash
//! export GMAIL_USER="your-email@gmail.com"
//! export GMAIL_APP_PASSWORD="your-app-password"
//! cargo run --example gmail_production
//! ```

use avila_cell::{
    smtp::{SmtpClient, SmtpSecurity},
    message::Email,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for debugging
    tracing_subscriber::fmt()
        .with_env_filter("avila_cell=debug")
        .init();

    // Get credentials from environment
    let gmail_user = env::var("GMAIL_USER")
        .expect("GMAIL_USER environment variable not set");
    let gmail_password = env::var("GMAIL_APP_PASSWORD")
        .expect("GMAIL_APP_PASSWORD environment variable not set");

    println!("🔌 Connecting to Gmail SMTP server...");
    
    // Connect to Gmail SMTP with STARTTLS (port 587)
    let mut client = SmtpClient::connect_with_security(
        "smtp.gmail.com",
        587,
        SmtpSecurity::StartTls
    ).await?;

    println!("✅ Connected to smtp.gmail.com:587");

    // Send EHLO to discover server capabilities
    println!("🤝 Sending EHLO...");
    client.ehlo("avila.inc").await?;
    println!("✅ EHLO successful");

    // Upgrade to TLS using STARTTLS
    println!("🔒 Upgrading to TLS with STARTTLS...");
    client.starttls().await?;
    println!("✅ TLS connection established");

    // Re-send EHLO after STARTTLS (required by protocol)
    println!("🤝 Re-sending EHLO after TLS...");
    client.ehlo("avila.inc").await?;
    println!("✅ EHLO successful");

    // Authenticate using PLAIN mechanism
    println!("🔐 Authenticating with AUTH PLAIN...");
    client.auth_plain(&gmail_user, &gmail_password).await?;
    println!("✅ Authentication successful");

    // Create a test email
    let email = Email {
        from: gmail_user.clone(),
        to: vec![gmail_user.clone()], // Send to yourself for testing
        cc: vec![],
        bcc: vec![],
        subject: "Test Email from Avila Cell".to_string(),
        body: "Hello from Avila Cell! 🚀\n\nThis is a test email sent using the avila-cell SMTP client with TLS support.".to_string(),
        html_body: None,
        attachments: vec![],
    };

    // Send the email
    println!("📧 Sending test email...");
    client.send_email(&email).await?;
    println!("✅ Email sent successfully!");

    // Gracefully close the connection
    println!("👋 Closing connection...");
    client.quit().await?;
    println!("✅ Connection closed");

    println!("\n🎉 Gmail SMTP test completed successfully!");
    println!("Check your inbox at: {}", gmail_user);

    Ok(())
}
