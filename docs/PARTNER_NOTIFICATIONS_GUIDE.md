# 📧 Vizzio Partner Notifications Guide

**Sistema de Notificação Integrado para Sócios do Vizzio Platform**

Guia completo sobre como o Vizzio Platform envia notificações personalizadas em HTML+CSS para sócios/stakeholders sobre eventos do GitHub (Push, Pull Requests, Issues, Releases).

---

## 📋 Índice

1. [Visão Geral](#visão-geral)
2. [Arquitetura](#arquitetura)
3. [Componentes](#componentes)
4. [Configuração](#configuração)
5. [Uso](#uso)
6. [Exemplos](#exemplos)
7. [Troubleshooting](#troubleshooting)

---

## 🎯 Visão Geral

O Vizzio Platform oferece **dois sistemas integrados** de notificação para sócios sobre atualizações do repositório:

| Sistema | Tipo | Linguagem | Vantagem | Quando Usar |
|---------|------|-----------|----------|------------|
| **GitHub Actions** | Automático | Node.js | Sem configuração, webhook nativo | Produção |
| **avila-cell Native** | Programático | Rust | Controle total, integrado | Desenvolvimento |

### 🔄 Fluxo de Notificação

```
GitHub Event (Push/PR/Issue)
         ↓
[GitHub Actions Webhook] OR [avila-cell Client]
         ↓
SMTP Email Transmission
         ↓
Partner/Stakeholder Inbox
         ↓
Personalized HTML Email
```

---

## 🏗️ Arquitetura

### Sistema 1: GitHub Actions (Automático)

```
┌─────────────────────────────────────────────────────┐
│  GitHub Repository Event                             │
│  (Push, PR opened, Issue created, Release published) │
└──────────────────┬──────────────────────────────────┘
                   ↓
┌─────────────────────────────────────────────────────┐
│  .github/workflows/notify-partners.yml               │
│  - Triggered on: push, pull_request, issues, releases│
│  - Parses GitHub context                            │
│  - Calls send-emails.js script                      │
└──────────────────┬──────────────────────────────────┘
                   ↓
┌─────────────────────────────────────────────────────┐
│  .github/scripts/send-emails.js (Node.js)           │
│  - Reads partners configuration                      │
│  - Generates HTML email                              │
│  - Uses nodemailer + SMTP                            │
└──────────────────┬──────────────────────────────────┘
                   ↓
┌─────────────────────────────────────────────────────┐
│  SMTP Server (Gmail, SendGrid, etc)                 │
└──────────────────┬──────────────────────────────────┘
                   ↓
         📧 Partner Email Inbox
```

**Arquivos:**
- `.github/workflows/notify-partners.yml` - GitHub Actions workflow
- `.github/scripts/send-emails.js` - Email generation script

**Dependências:**
- Node.js (built-in to GitHub Actions)
- nodemailer

---

### Sistema 2: avila-cell Native (Programático)

```
┌─────────────────────────────────────────────────────┐
│  Rust Application / Example                          │
│  (cargo run --example partner_notifications)        │
└──────────────────┬──────────────────────────────────┘
                   ↓
┌─────────────────────────────────────────────────────┐
│  NotificationClient (avila-cell)                    │
│  - Connects to SMTP server                          │
│  - Authenticates with credentials                   │
│  - Generates HTML emails                            │
└──────────────────┬──────────────────────────────────┘
                   ↓
┌─────────────────────────────────────────────────────┐
│  SmtpClient (avila-cell-core)                       │
│  - EHLO, AUTH, DATA commands                        │
│  - TLS/STARTTLS support                             │
│  - Multipart MIME support                           │
└──────────────────┬──────────────────────────────────┘
                   ↓
┌─────────────────────────────────────────────────────┐
│  SMTP Server (Gmail, SendGrid, etc)                 │
└──────────────────┬──────────────────────────────────┘
                   ↓
         📧 Partner Email Inbox
```

**Arquivos:**
- `packages/avila/avila-cell/src/notification.rs` - NotificationClient implementation
- `packages/avila/avila-cell/examples/partner_notifications.rs` - Usage example

**Dependências:**
- Tokio (async runtime)
- Serde (serialization)
- Chrono (date/time)

---

## 🔧 Componentes

### 1. NotificationClient (Rust)

**Localização:** `packages/avila/avila-cell/src/notification.rs`

```rust
pub struct NotificationClient {
    smtp_client: SmtpClient,
    from_address: EmailAddress,
}

impl NotificationClient {
    pub async fn new(
        smtp_host: &str,
        smtp_port: u16,
        from_email: &str,
        _from_name: &str,
        username: &str,
        password: &str,
    ) -> Result<Self>

    pub async fn send_github_notification(
        &mut self,
        event: &GitHubEventNotification,
        recipient: &Partner,
    ) -> Result<()>

    pub async fn close(&mut self) -> Result<()>
}
```

### 2. GitHubEventNotification (Rust)

```rust
pub struct GitHubEventNotification {
    pub event_type: GitHubEventType,
    pub repository: String,
    pub actor: String,
    pub timestamp: String,
    pub details: HashMap<String, String>,
    pub html_url: String,
}

pub enum GitHubEventType {
    Push,
    PullRequest,
    Issue,
    Release,
    Workflow,
}
```

### 3. Partner Struct (Rust)

```rust
pub struct Partner {
    pub name: String,
    pub email: String,
}
```

### 4. GitHub Actions Workflow (YAML)

**Localização:** `.github/workflows/notify-partners.yml`

Triggers:
- `push` - Repository push
- `pull_request` - PR opened/synchronize/reopened
- `issues` - Issue created/opened
- `release` - Release published

---

## ⚙️ Configuração

### Opção 1: GitHub Actions (Recomendado para Produção)

#### Passo 1: Adicionar Secrets no GitHub

1. Ir para: **Settings → Secrets and variables → Actions**
2. Adicionar os seguintes secrets:

| Secret | Valor | Exemplo |
|--------|-------|---------|
| `SMTP_HOST` | SMTP server hostname | `smtp.gmail.com` |
| `SMTP_PORT` | SMTP port | `587` |
| `SMTP_USER` | SMTP username/email | `seu-email@gmail.com` |
| `SMTP_PASSWORD` | SMTP password/app-password | `sua-app-password` |
| `PARTNER_1_EMAIL` | Partner 1 email | `nicolas@avila.inc` |
| `PARTNER_1_NAME` | Partner 1 name | `Nicolas` |
| `PARTNER_2_EMAIL` | Partner 2 email | `marcelosavazzi1@gmail.com` |
| `PARTNER_2_NAME` | Partner 2 name | `Marcelo Savazzi` |
| `PARTNER_3_EMAIL` | Partner 3 email | `rafaelochiussi@hotmail.com` |
| `PARTNER_3_NAME` | Partner 3 name | `Rafael Ochiussi` |

#### Passo 2: Gmail (Exemplo)

Se usar Gmail:

1. Habilitar 2FA em sua conta Google
2. Gerar "App Password": https://myaccount.google.com/apppasswords
3. Usar o app password como `SMTP_PASSWORD`

#### Passo 3: Validar Workflow

```bash
# Fazer um push para triggar o workflow
git add .
git commit -m "Test: GitHub Actions notification workflow"
git push origin main

# Verificar execução em: GitHub → Actions
```

---

### Opção 2: avila-cell Native (Desenvolvimento/Custom)

#### Passo 1: Instalar Dependências

```bash
cd packages/avila/avila-cell
cargo build --examples
```

#### Passo 2: Configurar Variáveis de Ambiente

**PowerShell:**
```powershell
$env:SMTP_HOST = "smtp.gmail.com"
$env:SMTP_PORT = "587"
$env:SMTP_USER = "seu-email@gmail.com"
$env:SMTP_PASSWORD = "sua-app-password"
```

**Bash/Linux:**
```bash
export SMTP_HOST="smtp.gmail.com"
export SMTP_PORT="587"
export SMTP_USER="seu-email@gmail.com"
export SMTP_PASSWORD="sua-app-password"
```

#### Passo 3: Executar Exemplo

```bash
cargo run --example partner_notifications
```

**Output esperado:**
```
=== Vizzio Platform - Partner Notifications ===

📧 Configuração:
   SMTP Host: smtp.gmail.com
   SMTP Port: 587
   From: seu-email@gmail.com

🔗 Conectando ao servidor SMTP...
✅ Conectado com sucesso!

👥 Sócios a serem notificados:
   - Nicolas (nicolas@avila.inc)
   - Marcelo Savazzi (marcelosavazzi1@gmail.com)
   - Rafael Ochiussi (rafaelochiussi@hotmail.com)

📤 Exemplo 1: Notificação de PUSH
   📧 Enviando para Nicolas...
   ✅ Email enviado com sucesso!
   📧 Enviando para Sócio 2...
   ✅ Email enviado com sucesso!

[... mais eventos ...]

=== Demonstração Concluída ===
📧 8 notificações foram enviadas
👥 Parceiros notificados: 2
📊 Eventos demonstrados: 4 (Push, PR, Issue, Release)
```

---

## 💻 Uso

### Via GitHub Actions (Automático)

**Não requer ação do desenvolvedor!** Simplesmente faça push de código:

```bash
git add .
git commit -m "Feature: Nova funcionalidade"
git push origin main
```

Parceiros receberão automaticamente:
- **Tema:** 🔔 Vizzio Platform - 📤 PUSH - avilainc/vizzio
- **Corpo:** Detalhes do commit (branch, mensagem, arquivos alterados)
- **Email:** HTML com design responsivo

---

### Via avila-cell (Programático)

```rust
use avila_cell::notification::{
    NotificationClient,
    GitHubEventNotification,
    GitHubEventType,
    Partner
};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Criar client
    let mut client = NotificationClient::new(
        "smtp.gmail.com",
        587,
        "seu-email@gmail.com",
        "Vizzio Platform",
        "seu-email@gmail.com",
        "sua-app-password",
    ).await?;

    // 2. Criar evento
    let mut details = HashMap::new();
    details.insert("branch".to_string(), "main".to_string());
    details.insert("message".to_string(), "Feature: Nova API".to_string());

    let event = GitHubEventNotification {
        event_type: GitHubEventType::Push,
        repository: "avilainc/vizzio".to_string(),
        actor: "developer".to_string(),
        timestamp: chrono::Local::now()
            .format("%d/%m/%Y %H:%M:%S")
            .to_string(),
        details,
        html_url: "https://github.com/avilainc/vizzio/commit/abc123".to_string(),
    };

    // 3. Enviar para sócios
    let partners = vec![
        Partner {
            name: "Nicolas".to_string(),
            email: "nicolas@avila.inc".to_string(),
        },
        Partner {
            name: "Marcelo Savazzi".to_string(),
            email: "marcelosavazzi1@gmail.com".to_string(),
        },
        Partner {
            name: "Rafael Ochiussi".to_string(),
            email: "rafaelochiussi@hotmail.com".to_string(),
        },
    ];

    for partner in &partners {
        client.send_github_notification(&event, partner).await?;
    }

    // 4. Fechar conexão
    client.close().await?;

    Ok(())
}
```

---

## 📧 Exemplos

### Exemplo 1: Notificação de PUSH

**Assunto:** 🔔 Vizzio Platform - 📤 PUSH - avilainc/vizzio

**Email (HTML):**
```
┌────────────────────────────────────────┐
│         📤 PUSH to Repository          │
│        avilainc/vizzio                 │
├────────────────────────────────────────┤
│                                        │
│  Branch: master                        │
│  Message: Add notification module      │
│  Files Changed: 5                      │
│  Insertions: +245                      │
│                                        │
│  Ator: developer-name                  │
│  Time: 15/01/2025 14:30:45             │
│                                        │
│  [View on GitHub]                      │
└────────────────────────────────────────┘
```

---

### Exemplo 2: Notificação de Pull Request

**Assunto:** 🔔 Vizzio Platform - 🔀 PULL REQUEST - avilainc/vizzio

**Email (HTML):**
```
┌────────────────────────────────────────┐
│       🔀 PULL REQUEST #42              │
│    Feature: Implement OAuth2           │
├────────────────────────────────────────┤
│                                        │
│  Status: 🆕 OPENED                    │
│  From: feature/oauth2                  │
│  To: master                            │
│                                        │
│  Author: feature-developer             │
│  Time: 15/01/2025 14:35:20             │
│                                        │
│  [Review on GitHub]                    │
└────────────────────────────────────────┘
```

---

### Exemplo 3: Notificação de Issue

**Assunto:** 🔔 Vizzio Platform - ⚠️ ISSUE - avilainc/vizzio

**Email (HTML):**
```
┌────────────────────────────────────────┐
│       ⚠️  ISSUE #101                   │
│    Bug: Serialization error            │
├────────────────────────────────────────┤
│                                        │
│  Priority: Alta                        │
│  Labels: bug, critical                 │
│                                        │
│  Reporter: bug-reporter                │
│  Time: 15/01/2025 15:00:00             │
│                                        │
│  [View on GitHub]                      │
└────────────────────────────────────────┘
```

---

### Exemplo 4: Notificação de Release

**Assunto:** 🔔 Vizzio Platform - 🎉 RELEASE - avilainc/vizzio

**Email (HTML):**
```
┌────────────────────────────────────────┐
│       🎉 RELEASE v0.2.0                │
│     Minor Release Published            │
├────────────────────────────────────────┤
│                                        │
│  Version: v0.2.0                       │
│  Type: Minor Release                   │
│  New Features: 5                       │
│  Bug Fixes: 12                         │
│                                        │
│  Release Manager: release-manager      │
│  Time: 15/01/2025 16:00:00             │
│                                        │
│  [View Release]                        │
└────────────────────────────────────────┘
```

---

## 🔍 Troubleshooting

### Problema 1: GitHub Actions Workflow não executa

**Causas comuns:**
- Secrets não configurados
- Workflow arquivo em branch errado

**Solução:**
```bash
# Verificar arquivo YAML
cat .github/workflows/notify-partners.yml

# Revalidar secrets em: Settings → Secrets
# Fazer novo push para triggar
git add .github/workflows/notify-partners.yml
git commit -m "Fix: Revalidate workflow"
git push origin main
```

---

### Problema 2: Erro "Invalid sender"

**Causa:** Email `from` não configurado corretamente

**Solução:**
```bash
# Gmail: Use email da conta
$env:SMTP_USER = "seu-email@gmail.com"  # ✅ Correto

# Não use nomes fictícios
$env:SMTP_USER = "noreply@vizzio.dev"   # ❌ Incorreto (se não existir)
```

---

### Problema 3: Conexão SMTP recusada

**Causa:** Credenciais erradas ou servidor bloqueado

**Solução:**

**Gmail:**
```bash
# Use App Password, NÃO senha regular
# Gere em: https://myaccount.google.com/apppasswords
```

**Verificar conectividade:**
```powershell
# Testar conexão SMTP
Test-NetConnection -ComputerName smtp.gmail.com -Port 587
```

---

### Problema 4: Email não recebido

**Possíveis causas:**
1. Email em spam
2. Servidor SMTP não autentica
3. Partner email inválido

**Verificação:**
```bash
# Ver logs do GitHub Actions
# GitHub → Actions → Workflow run → Email step logs

# Verificar email em spam folder
# Procurar por: "Vizzio Platform"

# Validar email de partner
echo "Check email format: partner@example.com"
```

---

### Problema 5: avila-cell não compila

**Solução:**

```bash
# Verificar versão Rust
rustc --version  # Deve ser >= 1.56

# Limpar cache e reconstruir
cd packages/avila/avila-cell
cargo clean
cargo build --examples

# Se persistir, verificar dependências
cargo tree
```

---

## 📚 Arquivos Relacionados

```
Vizzio/
├── .github/
│   ├── workflows/
│   │   └── notify-partners.yml          ← GitHub Actions workflow
│   └── scripts/
│       └── send-emails.js                ← Email generation (Node.js)
├── packages/avila/avila-cell/
│   ├── src/
│   │   ├── notification.rs               ← NotificationClient (Rust)
│   │   ├── smtp.rs                       ← SMTP protocol
│   │   ├── message.rs                    ← Email structure
│   │   └── lib.rs                        ← Main exports
│   ├── examples/
│   │   └── partner_notifications.rs      ← Usage example
│   └── Cargo.toml                        ← Dependencies
└── PARTNER_NOTIFICATIONS_GUIDE.md         ← Este arquivo
```

---

## 🚀 Próximos Passos

1. ✅ Configurar secrets no GitHub
2. ✅ Fazer push e validar workflow
3. ✅ Testar notificação com Pull Request
4. ✅ Adicionar mais sócios conforme necessário
5. ✅ Customizar templates HTML se desejado

---

## 📞 Suporte

Para perguntas ou issues relacionadas:

1. Verificar `.github/workflows/notify-partners.yml`
2. Consultar logs em: GitHub → Actions
3. Revisar `NOTIFICATION_SETUP.md` para configuração detalhada
4. Testar avila-cell com: `cargo run --example partner_notifications`

---

**Versão:** 1.0.0
**Última atualização:** Janeiro 2025
**Status:** ✅ Produção
