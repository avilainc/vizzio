// Example: Vizzio Partner Notifications using avila-cell
//
// This example demonstrates how to use avila-cell to send notifications
// to project partners about GitHub events (Push, PR, Issue, etc.)
//
// Environment variables required:
// - SMTP_HOST: SMTP server hostname (default: smtp.gmail.com)
// - SMTP_PORT: SMTP server port (default: 587)
// - SMTP_USER: SMTP username/email
// - SMTP_PASSWORD: SMTP password or app-specific password
//
// Example:
// ```bash
// $env:SMTP_USER="your-email@gmail.com"
// $env:SMTP_PASSWORD="your-app-password"
// cargo run --example partner_notifications
// ```

use avila_cell::{
    notification::{NotificationClient, GitHubEventNotification, GitHubEventType, Partner},
    EmailAddress,
};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Vizzio Platform - Partner Notifications ===\n");

    // 🔐 Configuration (in production, use environment variables)
    let smtp_host = std::env::var("SMTP_HOST").unwrap_or_else(|_| "smtp.gmail.com".to_string());
    let smtp_port: u16 = std::env::var("SMTP_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(587);
    let from_email = std::env::var("SMTP_USER").unwrap_or_else(|_| "noreply@vizzio.dev".to_string());
    let smtp_user = std::env::var("SMTP_USER").unwrap_or_else(|_| from_email.clone());
    let smtp_password = std::env::var("SMTP_PASSWORD").unwrap_or_else(|_| "your-app-password".to_string());

    println!("📧 Configuração:");
    println!("   SMTP Host: {}", smtp_host);
    println!("   SMTP Port: {}", smtp_port);
    println!("   From: {}\n", from_email);

    // ✅ Create notification client
    println!("🔗 Conectando ao servidor SMTP...");
    let mut notification_client = NotificationClient::new(
        &smtp_host,
        smtp_port,
        &from_email,
        "Vizzio Platform",
        &smtp_user,
        &smtp_password,
    )
    .await?;

    println!("✅ Conectado com sucesso!\n");

    // 👥 Define partners/stakeholders
    let partners = vec![
        Partner {
            name: "Sócio 1".to_string(),
            email: "socio1@example.com".to_string(),
        },
        Partner {
            name: "Sócio 2".to_string(),
            email: "socio2@example.com".to_string(),
        },
    ];

    println!("👥 Sócios a serem notificados:");
    for partner in &partners {
        println!("   - {} ({})", partner.name, partner.email);
    }
    println!();

    // 📤 Example 1: Push Notification
    println!("📤 Exemplo 1: Notificação de PUSH");
    let mut push_details = HashMap::new();
    push_details.insert("branch".to_string(), "master".to_string());
    push_details.insert("message".to_string(), "Add: Novo módulo de notificações".to_string());
    push_details.insert("files_changed".to_string(), "5".to_string());
    push_details.insert("insertions".to_string(), "245".to_string());

    let push_event = GitHubEventNotification {
        event_type: GitHubEventType::Push,
        repository: "avilainc/vizzio".to_string(),
        actor: "developer-name".to_string(),
        timestamp: chrono::Local::now().format("%d/%m/%Y %H:%M:%S").to_string(),
        details: push_details,
        html_url: "https://github.com/avilainc/vizzio/commit/abc123".to_string(),
    };

    for partner in &partners {
        println!("   📧 Enviando para {}...", partner.name);
        match notification_client.send_github_notification(&push_event, partner).await {
            Ok(_) => println!("   ✅ Email enviado com sucesso!"),
            Err(e) => println!("   ❌ Erro ao enviar: {}", e),
        }
    }
    println!();

    // 🔀 Example 2: Pull Request Notification
    println!("🔀 Exemplo 2: Notificação de PULL REQUEST");
    let mut pr_details = HashMap::new();
    pr_details.insert("title".to_string(), "Feature: Implementar autenticação OAuth2".to_string());
    pr_details.insert("number".to_string(), "#42".to_string());
    pr_details.insert("action".to_string(), "🆕 ABERTO".to_string());
    pr_details.insert("from".to_string(), "feature/oauth2".to_string());
    pr_details.insert("to".to_string(), "master".to_string());

    let pr_event = GitHubEventNotification {
        event_type: GitHubEventType::PullRequest,
        repository: "avilainc/vizzio".to_string(),
        actor: "feature-developer".to_string(),
        timestamp: chrono::Local::now().format("%d/%m/%Y %H:%M:%S").to_string(),
        details: pr_details,
        html_url: "https://github.com/avilainc/vizzio/pull/42".to_string(),
    };

    for partner in &partners {
        println!("   📧 Enviando para {}...", partner.name);
        match notification_client.send_github_notification(&pr_event, partner).await {
            Ok(_) => println!("   ✅ Email enviado com sucesso!"),
            Err(e) => println!("   ❌ Erro ao enviar: {}", e),
        }
    }
    println!();

    // ⚠️ Example 3: Issue Notification
    println!("⚠️ Exemplo 3: Notificação de ISSUE");
    let mut issue_details = HashMap::new();
    issue_details.insert("title".to_string(), "Bug: Erro na serialização de dados".to_string());
    issue_details.insert("number".to_string(), "#101".to_string());
    issue_details.insert("priority".to_string(), "Alta".to_string());
    issue_details.insert("labels".to_string(), "bug, critical".to_string());

    let issue_event = GitHubEventNotification {
        event_type: GitHubEventType::Issue,
        repository: "avilainc/vizzio".to_string(),
        actor: "bug-reporter".to_string(),
        timestamp: chrono::Local::now().format("%d/%m/%Y %H:%M:%S").to_string(),
        details: issue_details,
        html_url: "https://github.com/avilainc/vizzio/issues/101".to_string(),
    };

    for partner in &partners {
        println!("   📧 Enviando para {}...", partner.name);
        match notification_client.send_github_notification(&issue_event, partner).await {
            Ok(_) => println!("   ✅ Email enviado com sucesso!"),
            Err(e) => println!("   ❌ Erro ao enviar: {}", e),
        }
    }
    println!();

    // 🎉 Example 4: Release Notification
    println!("🎉 Exemplo 4: Notificação de RELEASE");
    let mut release_details = HashMap::new();
    release_details.insert("version".to_string(), "v0.2.0".to_string());
    release_details.insert("type".to_string(), "Minor Release".to_string());
    release_details.insert("features".to_string(), "5 novas features".to_string());
    release_details.insert("bugfixes".to_string(), "12 bugs corrigidos".to_string());

    let release_event = GitHubEventNotification {
        event_type: GitHubEventType::Release,
        repository: "avilainc/vizzio".to_string(),
        actor: "release-manager".to_string(),
        timestamp: chrono::Local::now().format("%d/%m/%Y %H:%M:%S").to_string(),
        details: release_details,
        html_url: "https://github.com/avilainc/vizzio/releases/tag/v0.2.0".to_string(),
    };

    for partner in &partners {
        println!("   📧 Enviando para {}...", partner.name);
        match notification_client.send_github_notification(&release_event, partner).await {
            Ok(_) => println!("   ✅ Email enviado com sucesso!"),
            Err(e) => println!("   ❌ Erro ao enviar: {}", e),
        }
    }
    println!();

    // 🔌 Close connection
    println!("🔌 Fechando conexão SMTP...");
    notification_client.close().await?;
    println!("✅ Desconectado com sucesso!\n");

    println!("=== Demonstração Concluída ===");
    println!("📧 {} notificações foram enviadas", partners.len() * 4);
    println!("👥 Parceiros notificados: {}", partners.len());
    println!("📊 Eventos demonstrados: 4 (Push, PR, Issue, Release)\n");

    Ok(())
}
