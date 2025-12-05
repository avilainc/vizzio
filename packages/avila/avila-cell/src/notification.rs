//! Notification module for Vizzio Platform
//! 
//! Specialized email notifications for GitHub events using avila-cell SMTP capabilities

use crate::{message::Email, smtp::SmtpClient, EmailAddress, Result};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Represents a GitHub event type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GitHubEventType {
    Push,
    PullRequest,
    Issue,
    Release,
    Workflow,
}

impl std::fmt::Display for GitHubEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitHubEventType::Push => write!(f, "📤 PUSH"),
            GitHubEventType::PullRequest => write!(f, "🔀 PULL REQUEST"),
            GitHubEventType::Issue => write!(f, "⚠️ ISSUE"),
            GitHubEventType::Release => write!(f, "🎉 RELEASE"),
            GitHubEventType::Workflow => write!(f, "⚙️ WORKFLOW"),
        }
    }
}

/// GitHub event notification details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubEventNotification {
    pub event_type: GitHubEventType,
    pub repository: String,
    pub actor: String,
    pub timestamp: String,
    pub details: HashMap<String, String>,
    pub html_url: String,
}

/// Partner/Stakeholder information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Partner {
    pub name: String,
    pub email: String,
}

/// SMTP Notification client with GitHub event support
pub struct NotificationClient {
    smtp_client: SmtpClient,
    from_address: EmailAddress,
}

impl NotificationClient {
    /// Create new notification client
    pub async fn new(
        smtp_host: &str,
        smtp_port: u16,
        from_email: &str,
        _from_name: &str,
        username: &str,
        password: &str,
    ) -> Result<Self> {
        let mut client = SmtpClient::connect(smtp_host, smtp_port).await?;
        
        // EHLO handshake
        client.ehlo("vizzio.dev").await?;
        
        // Authenticate
        client.auth_plain(username, password).await?;
        
        let from_address = EmailAddress::new(from_email)?;
        
        Ok(Self {
            smtp_client: client,
            from_address,
        })
    }

    /// Send notification email for GitHub event
    pub async fn send_github_notification(
        &mut self,
        event: &GitHubEventNotification,
        recipient: &Partner,
    ) -> Result<()> {
        let to_address = EmailAddress::new(&recipient.email)?;
        
        let subject = format!(
            "🔔 Vizzio Platform - {} - {}",
            event.event_type,
            event.repository
        );

        let html_body = self.generate_html_body(&event, &recipient);
        let text_body = self.generate_text_body(&event, &recipient);

        let mut email = Email::new(
            self.from_address.clone(),
            vec![to_address],
            subject,
            text_body,
        );

        // Set HTML body
        email.html_body = Some(html_body);

        // Add headers
        email.add_header("X-GitHub-Event".to_string(), format!("{:?}", event.event_type));
        email.add_header("X-Vizzio-Notification".to_string(), "true".to_string());
        email.add_header("X-Mailer".to_string(), "Avila Cell Notification".to_string());

        // Send via SMTP
        self.smtp_client.send_email(&email).await?;

        Ok(())
    }

    /// Generate HTML email body for GitHub event
    fn generate_html_body(&self, event: &GitHubEventNotification, recipient: &Partner) -> String {
        format!(
            r#"<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Notificação Vizzio</title>
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}
        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            padding: 20px;
            line-height: 1.6;
        }}
        .container {{
            max-width: 600px;
            margin: 0 auto;
            background: white;
            border-radius: 12px;
            overflow: hidden;
            box-shadow: 0 10px 40px rgba(0,0,0,0.2);
        }}
        .header {{
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 30px 20px;
            text-align: center;
        }}
        .header h1 {{
            font-size: 28px;
            margin-bottom: 10px;
        }}
        .content {{
            padding: 30px 20px;
        }}
        .badge {{
            display: inline-block;
            padding: 8px 16px;
            border-radius: 20px;
            font-size: 12px;
            font-weight: bold;
            margin-bottom: 20px;
            background: #f3e5f5;
            color: #7b1fa2;
        }}
        .info-block {{
            background: #f5f5f5;
            border-left: 4px solid #667eea;
            padding: 15px;
            margin: 15px 0;
            border-radius: 4px;
        }}
        .info-block label {{
            font-weight: bold;
            color: #333;
            display: block;
            margin-bottom: 5px;
        }}
        .info-block .value {{
            color: #666;
            word-break: break-all;
        }}
        .btn {{
            display: inline-block;
            padding: 12px 24px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white !important;
            text-decoration: none;
            border-radius: 6px;
            margin-top: 20px;
            font-weight: bold;
        }}
        .footer {{
            background: #f9f9f9;
            border-top: 1px solid #eee;
            padding: 20px;
            text-align: center;
            font-size: 12px;
            color: #999;
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>📊 Vizzio Platform</h1>
            <p>Notificação de Atualização do Repositório</p>
        </div>
        
        <div class="content">
            <p>Olá <strong>{}</strong>,</p>
            <p style="margin-top: 15px;">Uma nova atualização foi realizada no repositório Vizzio Platform!</p>
            
            <span class="badge">{}</span>
            
            <div class="info-block">
                <label>📦 Repositório:</label>
                <span class="value">{}</span>
            </div>
            
            <div class="info-block">
                <label>👤 Autor:</label>
                <span class="value">{}</span>
            </div>
            
            <div class="info-block">
                <label>⏰ Data:</label>
                <span class="value">{}</span>
            </div>
            
            {}
            
            <a href="{}" class="btn">Ver Detalhes no GitHub →</a>
        </div>
        
        <div class="footer">
            <p>© 2025 Vizzio Platform - Todos os direitos reservados</p>
            <p>Você está recebendo este email como sócio do projeto.</p>
        </div>
    </div>
</body>
</html>"#,
            recipient.name,
            event.event_type,
            event.repository,
            event.actor,
            event.timestamp,
            self.generate_event_details(&event.event_type, &event.details),
            event.html_url,
        )
    }

    /// Generate plain text email body for GitHub event
    fn generate_text_body(&self, event: &GitHubEventNotification, recipient: &Partner) -> String {
        format!(
            r#"Olá {},

Uma nova atualização foi realizada no repositório Vizzio Platform!

Tipo de Evento: {}
Repositório: {}
Autor: {}
Data: {}

Mais informações: {}

---
© 2025 Vizzio Platform
Você está recebendo este email como sócio do projeto."#,
            recipient.name, event.event_type, event.repository, event.actor, event.timestamp, event.html_url
        )
    }

    /// Generate event-specific details HTML
    fn generate_event_details(&self, event_type: &GitHubEventType, details: &HashMap<String, String>) -> String {
        let mut html = String::new();
        
        match event_type {
            GitHubEventType::Push => {
                if let Some(branch) = details.get("branch") {
                    html.push_str(&format!(
                        r#"<div class="info-block">
                            <label>🌿 Branch:</label>
                            <span class="value">{}</span>
                        </div>"#,
                        branch
                    ));
                }
                if let Some(message) = details.get("message") {
                    html.push_str(&format!(
                        r#"<div class="info-block">
                            <label>📝 Mensagem:</label>
                            <span class="value">{}</span>
                        </div>"#,
                        message
                    ));
                }
            },
            GitHubEventType::PullRequest => {
                if let Some(title) = details.get("title") {
                    html.push_str(&format!(
                        r#"<div class="info-block">
                            <label>PR: {}</label>
                            <span class="value"></span>
                        </div>"#,
                        title
                    ));
                }
                if let Some(action) = details.get("action") {
                    html.push_str(&format!(
                        r#"<div class="info-block">
                            <label>Status:</label>
                            <span class="value">{}</span>
                        </div>"#,
                        action
                    ));
                }
            },
            GitHubEventType::Issue => {
                if let Some(title) = details.get("title") {
                    html.push_str(&format!(
                        r#"<div class="info-block">
                            <label>Issue: {}</label>
                            <span class="value"></span>
                        </div>"#,
                        title
                    ));
                }
            },
            _ => {}
        }
        
        html
    }

    /// Close SMTP connection
    pub async fn close(mut self) -> Result<()> {
        self.smtp_client.quit().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_type_display() {
        assert_eq!(GitHubEventType::Push.to_string(), "📤 PUSH");
        assert_eq!(GitHubEventType::PullRequest.to_string(), "🔀 PULL REQUEST");
        assert_eq!(GitHubEventType::Issue.to_string(), "⚠️ ISSUE");
    }

    #[test]
    fn test_partner_creation() {
        let partner = Partner {
            name: "Sócio 1".to_string(),
            email: "socio@example.com".to_string(),
        };
        assert_eq!(partner.name, "Sócio 1");
        assert_eq!(partner.email, "socio@example.com");
    }
}
