//! SMTP Email Example
//!
//! This example demonstrates how to send emails through any SMTP server.
//!
//! # Environment Variables
//!
//! Uses the following environment variables (configured in your environment):
//! - SMTP_HOST: SMTP server host (default: smtp.gmail.com)
//! - SMTP_PORT: SMTP server port (default: 587)
//! - SMTP_USER: Email username/address
//! - SMTP_PASS: Email password or app password

use avila_cell::{
    smtp::SmtpClient,
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
    let smtp_host = env::var("SMTP_HOST")
        .unwrap_or_else(|_| "smtp.gmail.com".to_string());
    let smtp_port: u16 = env::var("SMTP_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(587);
    let smtp_user = env::var("SMTP_USER")
        .expect("SMTP_USER environment variable not set");
    let smtp_pass = env::var("SMTP_PASS")
        .expect("SMTP_PASS environment variable not set");

    println!("🔌 Connecting to SMTP server {}:{}...", smtp_host, smtp_port);

    // Connect to SMTP server
    let mut client = SmtpClient::connect(&smtp_host, smtp_port).await?;

    println!("✅ Connected to {}:{}", smtp_host, smtp_port);

    // Send EHLO to discover server capabilities
    println!("🤝 Sending EHLO...");
    client.ehlo("avila.inc").await?;
    println!("✅ EHLO successful");

    // Authenticate using PLAIN mechanism
    println!("🔐 Authenticating...");
    client.auth_plain(&smtp_user, &smtp_pass).await?;
    println!("✅ Authentication successful");

    // Create a test email
    let email = Email {
        from: smtp_user.clone(),
        to: vec![smtp_user.clone()],
        cc: vec![],
        bcc: vec![],
        subject: "Test Email from Avila Cell".to_string(),
        body: "Hello from Avila Cell! 🚀\n\nThis is a test email sent using the avila-cell SMTP client.".to_string(),
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

    println!("\n🎉 SMTP test completed successfully!");
    println!("Check your inbox at: {}", smtp_user);

    Ok(())
}
