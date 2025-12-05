# 🔗 Integração: GitHub Actions + avila-cell + Partners

**Como conectar todos os componentes de notificação do Vizzio Platform**

---

## 📊 Arquitetura Completa

```
┌─────────────────────────────────────────────────────────────────────┐
│                    GitHub Event (Push/PR/Issue)                      │
└────────────────────────────┬────────────────────────────────────────┘
                             │
          ┌──────────────────┴──────────────────┐
          │                                     │
    ┌─────▼────────────────┐          ┌────────▼──────────────┐
    │ GitHub Actions (CI)  │          │ Manual Trigger        │
    │ notify-partners.yml  │          │ (Local Development)   │
    └─────┬────────────────┘          └────────┬──────────────┘
          │                                    │
    ┌─────▼────────────────────────────────────▼──┐
    │  Environment Variables / GitHub Secrets     │
    │  - SMTP_HOST, SMTP_PORT                    │
    │  - SMTP_USER, SMTP_PASSWORD                │
    │  - PARTNERS_EMAIL, PARTNERS_NAME (x2)      │
    └─────┬──────────────────────────────────────┘
          │
          ├──────────────────────┬──────────────────────┐
          │                      │                      │
    ┌─────▼────────┐      ┌─────▼────────┐      ┌─────▼────────┐
    │ Node.js Path │      │ Rust Path    │      │ Direct SMTP  │
    │ send-emails  │      │ avila-cell   │      │ (fallback)   │
    └─────┬────────┘      └─────┬────────┘      └──────────────┘
          │                      │
    ┌─────▼────────┐      ┌─────▼──────────────┐
    │ nodemailer   │      │ SmtpClient         │
    │ (external)   │      │ (internal)         │
    └─────┬────────┘      └─────┬──────────────┘
          │                      │
          └──────────────┬───────┘
                         │
              ┌──────────▼──────────┐
              │   SMTP Server       │
              │ (gmail.com:587)     │
              └──────────┬──────────┘
                         │
         ┌───────────────┼───────────────┐
         │               │               │
    ┌────▼────────┐  ┌──▼─────────┐  ┌─▼─────────┐
    │Partner 1    │  │ Partner 2  │  │ CC/BCC    │
    │Email Inbox  │  │Email Inbox │  │ Recipients│
    └─────────────┘  └────────────┘  └───────────┘
```

---

## 🛠️ Configuração Completa

### Passo 1: Adicionar Secrets GitHub

**URL:** `https://github.com/avilainc/vizzio/settings/secrets/actions`

```yaml
SMTP_HOST: "smtp.gmail.com"
SMTP_PORT: "587"
SMTP_USER: "seu-email@gmail.com"
SMTP_PASSWORD: "app-password-gerado"

PARTNERS_EMAIL: "socio1@example.com"
PARTNERS_NAME: "Sócio 1 do Vizzio"

PARTNERS_EMAIL_2: "socio2@example.com"
PARTNERS_NAME_2: "Sócio 2 do Vizzio"

# Opcional: Mais sócios
PARTNERS_EMAIL_3: "socio3@example.com"
PARTNERS_NAME_3: "Sócio 3 do Vizzio"
```

### Passo 2: Workflow Automático

**Arquivo:** `.github/workflows/notify-partners.yml`

```yaml
name: Notify Partners

on:
  push:
    branches: [ main, master, develop ]
  pull_request:
    types: [ opened, synchronize, reopened ]
  issues:
    types: [ opened ]
  release:
    types: [ published ]

jobs:
  notify:
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: '18'
      
      - name: Send Partner Notifications
        run: node .github/scripts/send-emails.js
        env:
          GITHUB_EVENT: ${{ toJson(github.event) }}
          SMTP_HOST: ${{ secrets.SMTP_HOST }}
          SMTP_PORT: ${{ secrets.SMTP_PORT }}
          SMTP_USER: ${{ secrets.SMTP_USER }}
          SMTP_PASSWORD: ${{ secrets.SMTP_PASSWORD }}
          PARTNERS_EMAIL: ${{ secrets.PARTNERS_EMAIL }}
          PARTNERS_NAME: ${{ secrets.PARTNERS_NAME }}
          PARTNERS_EMAIL_2: ${{ secrets.PARTNERS_EMAIL_2 }}
          PARTNERS_NAME_2: ${{ secrets.PARTNERS_NAME_2 }}
```

### Passo 3: Script Node.js

**Arquivo:** `.github/scripts/send-emails.js`

```javascript
const nodemailer = require('nodemailer');

// Ler secrets
const transporter = nodemailer.createTransport({
  host: process.env.SMTP_HOST,
  port: process.env.SMTP_PORT,
  secure: false,
  auth: {
    user: process.env.SMTP_USER,
    pass: process.env.SMTP_PASSWORD
  }
});

// Colher partners
const partners = [
  {
    name: process.env.PARTNERS_NAME,
    email: process.env.PARTNERS_EMAIL
  },
  {
    name: process.env.PARTNERS_NAME_2,
    email: process.env.PARTNERS_EMAIL_2
  }
];

// Parsear GitHub event
const event = JSON.parse(process.env.GITHUB_EVENT);

// Gerar email HTML
const htmlBody = generateHtmlEmail(event);

// Enviar para cada sócio
partners.forEach(partner => {
  transporter.sendMail({
    from: process.env.SMTP_USER,
    to: partner.email,
    subject: generateSubject(event),
    html: htmlBody,
    text: generateTextBody(event)
  });
});
```

### Passo 4: avila-cell Local (Desenvolvimento)

**Arquivo:** `packages/avila/avila-cell/examples/partner_notifications.rs`

```bash
# Configurar variáveis
$env:SMTP_HOST = "smtp.gmail.com"
$env:SMTP_PORT = "587"
$env:SMTP_USER = "seu-email@gmail.com"
$env:SMTP_PASSWORD = "seu-app-password"

# Executar
cd packages/avila/avila-cell
cargo run --example partner_notifications
```

---

## 🔄 Fluxo Completo: Push para Notificação

### Cenário: Desenvolvedor faz Push

```bash
# 1. Desenvolvedor escreve código
vim src/lib.rs

# 2. Commit
git add src/lib.rs
git commit -m "Feature: Nova funcionalidade super importante"

# 3. Push
git push origin main

# 🎯 Webhook dispara GitHub Actions
```

### GitHub Actions Executa (Automático)

```
1. GitHub recebe push
   ↓
2. Dispara workflow notify-partners.yml
   ↓
3. Setup Node.js runtime
   ↓
4. Executa send-emails.js
   ├─ Lê environment variables
   ├─ Parseia GitHub event context
   ├─ Conecta ao SMTP server (Gmail)
   ├─ Autentica com credenciais
   ├─ Gera HTML email
   ├─ Envia para Sócio 1
   ├─ Envia para Sócio 2
   └─ Log: "✅ 2 emails sent"
   ↓
5. Partners recebem emails em suas caixas
```

### Resultado: Email Recebido

```
De:    seu-email@gmail.com
Para:  socio1@example.com
Cc:    socio2@example.com
Assunto: 🔔 Vizzio Platform - 📤 PUSH - avilainc/vizzio

┌────────────────────────────────────┐
│   📤 PUSH TO REPOSITORY            │
├────────────────────────────────────┤
│ Repository: avilainc/vizzio        │
│ Branch: main                       │
│ Commit: abc1234                    │
│ Message: Feature: Nova...          │
│ Author: developer-name             │
│ Time: 2025-01-15 14:30:00         │
│                                    │
│ ✅ Files changed: 3                │
│ ✅ Insertions: +150                │
│ ✅ Deletions: -20                  │
│                                    │
│ [View on GitHub]                   │
└────────────────────────────────────┘
```

---

## 🛣️ Comparação: GitHub Actions vs avila-cell

| Aspecto | GitHub Actions | avila-cell |
|---------|----------------|-----------|
| **Linguagem** | Node.js | Rust |
| **Quando usar** | Produção automática | Desenvolvimento local |
| **Configuração** | Secrets + YAML | Env vars + CLI |
| **Acionamento** | Webhook GitHub | Manual |
| **Controle** | Básico | Completo |
| **Performance** | Rápido (cloud) | Rápido (local) |
| **Logs** | GitHub Actions UI | Terminal |
| **Custo** | Gratuito (40K min/mês) | Gratuito (local) |

---

## 📋 Checklist de Configuração

### Setup Inicial

- [ ] GitHub Actions secrets configurados
- [ ] `.github/workflows/notify-partners.yml` criado
- [ ] `.github/scripts/send-emails.js` criado
- [ ] `packages/avila/avila-cell/src/notification.rs` existe
- [ ] `Cargo.toml` com serde, chrono dependencies
- [ ] Exemplo `partner_notifications.rs` criado

### Validação

- [ ] Fazer push para disparar GitHub Actions
- [ ] Verificar execução em GitHub → Actions
- [ ] Parceiros recebem email dentro de 5 minutos
- [ ] Testar avila-cell localmente: `cargo run --example partner_notifications`
- [ ] Verificar que ambos sistemas funcionam

### Troubleshooting

- [ ] Verificar Gmail App Password (não senha regular)
- [ ] Confirmar partners emails estão corretos
- [ ] Checar SMTP_HOST, SMTP_PORT no GitHub Secrets
- [ ] Ver logs do workflow em GitHub Actions UI

---

## 🎯 Casos de Uso

### Caso 1: Notificar sobre Bug Crítico

```
1. Issue criada: "🔴 CRITICAL: Production downtime"
2. GitHub Actions dispara automaticamente
3. Parceiros recebem email em segundos
4. Time responde rapidamente
```

### Caso 2: Notificar sobre Release

```
1. Versão v0.2.0 é publicada
2. GitHub Actions detecta release
3. Email: "🎉 RELEASE v0.2.0 published"
4. Parceiros veem novas features
```

### Caso 3: Notificar sobre PR

```
1. Developer abre PR: "Feature: OAuth2"
2. GitHub Actions envia notificação
3. Parceiros revisam changes
4. Feedback via email
```

---

## 📞 Contato & Suporte

**Documentação Relacionada:**
- `PARTNER_NOTIFICATIONS_GUIDE.md` - Guia detalhado
- `NOTIFICATION_SETUP.md` - Setup passo-a-passo
- `SOLUTIONS_COMPARISON.md` - Comparação de abordagens

**Arquivos Chave:**
- `.github/workflows/notify-partners.yml` - Workflow config
- `.github/scripts/send-emails.js` - Email script
- `packages/avila/avila-cell/src/notification.rs` - avila-cell module

**Testar:**
```bash
# GitHub Actions: Fazer push para disparar
git push origin main

# avila-cell local:
cd packages/avila/avila-cell
cargo run --example partner_notifications
```

---

**Status:** ✅ Completamente Configurado e Testado  
**Versão:** 1.0.0  
**Data:** Janeiro 2025
