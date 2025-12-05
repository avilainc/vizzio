# 🎯 START HERE - Guia de Início

**[🇧🇷 Português](#português) | [🇺🇸 English](#english)**

---

## Português

### O que é Vizzio?

**Vizzio** é uma **plataforma completa de automação empresarial** que consolida:

- 📧 **Gerenciamento de E-mails**
- 🤖 **Motor de Workflows**
- 💰 **Ferramentas Financeiras**
- 📱 **Automação de Marketing**
- 💼 **Pipeline de Vendas**
- ⚡ **Sistema de Atalhos**
- 🔌 **Integrações Externas**
- 🧠 **Assistente IA**

### 🚀 Começar Rápido

```bash
# 1. Clonar repositório
git clone https://github.com/avilainc/vizzio.git
cd vizzio

# 2. Instalar dependências
npm install

# 3. Subir containers
docker-compose up -d

# 4. Iniciar desenvolvimento
npm run dev

# 5. Acessar
# Dashboard: http://localhost:3001
# API: http://localhost:3000
# Redis: http://localhost:6379
# MongoDB: mongodb://localhost:27017
# RabbitMQ: http://localhost:15672
```

### 📚 Documentação

- [Arquitetura](./MONOREPO_STRUCTURE.md)
- [Workflows](./docs/WORKFLOWS.md)
- [API Reference](./docs/API.md)
- [Integrações](./docs/INTEGRATIONS.md)

### 🔑 Principais Recursos

**1. Trilhas de Automação**
- Marketing Automation
- Sales Pipeline
- Financial Automation
- HR Management
- Operations
- Customer Service

**2. E-mail Management**
- Múltiplas contas (Gmail, Outlook, SendGrid)
- Templates bilíngues
- Agendamento
- Rastreamento

**3. Atalhos**
- Teclado (Ctrl+Alt+M)
- Voz ("Começar automação")
- Mobile (gestos)
- Slash commands (/campaign)

**4. Integrações**
- Salesforce, HubSpot, Pipedrive
- Slack, Teams, WhatsApp
- Stripe, PayPal
- Google Workspace, Microsoft 365

---

## English

### What is Vizzio?

**Vizzio** is a **complete business automation platform** that consolidates:

- 📧 **Email Management**
- 🤖 **Workflow Engine**
- 💰 **Finance Tools**
- 📱 **Marketing Automation**
- 💼 **Sales Pipeline**
- ⚡ **Shortcuts System**
- 🔌 **External Integrations**
- 🧠 **AI Assistant**

### 🚀 Quick Start

```bash
# 1. Clone repository
git clone https://github.com/avilainc/vizzio.git
cd vizzio

# 2. Install dependencies
npm install

# 3. Start containers
docker-compose up -d

# 4. Start development
npm run dev

# 5. Access
# Dashboard: http://localhost:3001
# API: http://localhost:3000
# Redis: http://localhost:6379
# MongoDB: mongodb://localhost:27017
# RabbitMQ: http://localhost:15672
```

### 📚 Documentation

- [Architecture](./MONOREPO_STRUCTURE.md)
- [Workflows](./docs/WORKFLOWS.md)
- [API Reference](./docs/API.md)
- [Integrations](./docs/INTEGRATIONS.md)

### 🔑 Key Features

**1. Automation Workflows**
- Marketing Automation
- Sales Pipeline
- Financial Automation
- HR Management
- Operations
- Customer Service

**2. Email Management**
- Multiple accounts (Gmail, Outlook, SendGrid)
- Bilingual templates
- Scheduling
- Tracking

**3. Shortcuts**
- Keyboard (Ctrl+Alt+M)
- Voice ("Start automation")
- Mobile (gestures)
- Slash commands (/campaign)

**4. Integrations**
- Salesforce, HubSpot, Pipedrive
- Slack, Teams, WhatsApp
- Stripe, PayPal
- Google Workspace, Microsoft 365

---

## 📊 Tech Stack

- **Backend**: Node.js + Express + TypeScript
- **Frontend**: React 18 + Next.js
- **Mobile**: React Native
- **Database**: MongoDB + Redis
- **Message Queue**: RabbitMQ
- **AI**: OpenAI + Claude
- **Email**: Nodemailer + SendGrid

---

## 🎯 Project Structure

```
vizzio/
├── packages/
│   ├── core/                    # Shared types
│   ├── workflows/               # Workflow engine
│   ├── email-service/           # Email management
│   ├── finance-tools/           # Finance automation
│   ├── marketing-automation/    # Marketing tools
│   ├── sales-pipeline/          # Sales management
│   ├── shortcuts/               # Shortcuts system
│   ├── integrations/            # External APIs
│   ├── ai-assistant/            # AI features
│   ├── backend/                 # Main API
│   ├── frontend/                # Web dashboard
│   ├── mobile/                  # Mobile app
│   └── cli/                     # CLI tool
├── docker-compose.yml           # Docker setup
└── README.md                    # Documentation
```

---

## 🔗 Links Importantes

- **GitHub**: https://github.com/avilainc/vizzio
- **Demo**: https://vizzio-demo.com
- **Docs**: https://vizzio-docs.com
- **Community**: https://community.vizzio.com
- **Support**: support@vizzio.com

---

**Desenvolvido para automatizar todas as operações empresariais.**

Made with ❤️ by Avila Inc.
