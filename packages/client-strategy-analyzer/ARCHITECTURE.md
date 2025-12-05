# 🏗️ Arquitetura do Sistema

## 📊 Fluxo de Dados

```
┌─────────────────────────────────────────────────────────────────┐
│                         CLIENTE (Frontend)                       │
│                    http://localhost:3001                         │
│                                                                   │
│  ┌────────────────┐  ┌────────────────┐  ┌────────────────┐    │
│  │  Dashboard     │  │  Novo Caso     │  │  Listar Casos  │    │
│  │    (React)     │  │   (Formulário) │  │    (Tabela)    │    │
│  └────────────────┘  └────────────────┘  └────────────────┘    │
└─────────────────────────────────────────────────────────────────┘
                            │
                            │ HTTP/REST API
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                    BACKEND (Node.js)                            │
│                  http://localhost:3000                          │
│                                                                   │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                   Express.js Server                      │  │
│  │                                                           │  │
│  │  Routes:                                                  │  │
│  │  ├─ POST   /api/cases          (Criar caso)             │  │
│  │  ├─ GET    /api/cases          (Listar casos)           │  │
│  │  ├─ GET    /api/cases/:id      (Obter caso)             │  │
│  │  ├─ POST   /api/cases/:id/analyze   (Analisar IA)      │  │
│  │  └─ POST   /api/cases/:id/send-proposal  (Enviar)      │  │
│  └──────────────────────────────────────────────────────────┘  │
│                            │                                     │
│        ┌───────────────────┼───────────────────┐                │
│        │                   │                   │                │
│        ▼                   ▼                   ▼                │
│  ┌──────────────┐  ┌──────────────┐   ┌──────────────────┐   │
│  │  MongoDB     │  │  Copilot     │   │  Email Service   │   │
│  │  (Mongoose)  │  │  (OpenAI)    │   │  (Nodemailer)    │   │
│  │              │  │              │   │                  │   │
│  │ - Cases      │  │ Análise de   │   │ Envio de propostas │ │
│  │ - Users      │  │ estratégia   │   │ em HTML/CSS      │   │
│  │ - Proposals  │  │ e proposta   │   │                  │   │
│  └──────────────┘  └──────────────┘   └──────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                  SERVIÇOS EXTERNOS                              │
│                                                                   │
│  ┌──────────────┐  ┌──────────────┐   ┌──────────────────┐   │
│  │  MongoDB     │  │  OpenAI API  │   │  SMTP (Gmail)    │   │
│  │  (Database)  │  │  (LLM/IA)    │   │  (Email)         │   │
│  └──────────────┘  └──────────────┘   └──────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

---

## 📁 Estrutura de Pastas

```
client-strategy-analyzer/
│
├── backend/                          # API Node.js
│   ├── src/
│   │   ├── index.ts                 # Entrada principal
│   │   ├── models/
│   │   │   ├── Case.ts              # Modelo de casos
│   │   │   └── User.ts              # Modelo de usuários
│   │   ├── services/
│   │   │   ├── CopilotAnalysisService.ts    # Análise com IA
│   │   │   ├── EmailService.ts               # Envio de emails
│   │   │   └── ProposalGeneratorService.ts  # Geração de propostas
│   │   └── routes/
│   │       └── cases.ts              # Rotas de casos
│   ├── dist/                        # Build compilado
│   ├── package.json
│   ├── tsconfig.json
│   └── .env.example
│
├── frontend/                         # Dashboard React
│   ├── pages/
│   │   ├── index.tsx               # Home/Dashboard
│   │   ├── _app.tsx                # App wrapper
│   │   └── _document.tsx           # Document wrapper
│   ├── components/
│   │   ├── CaseForm.tsx            # Formulário de novo caso
│   │   └── CasesList.tsx           # Tabela de casos
│   ├── styles/
│   │   └── globals.css             # Estilos globais
│   ├── public/                     # Assets estáticos
│   ├── package.json
│   ├── tsconfig.json
│   ├── next.config.js
│   ├── tailwind.config.js
│   └── .env.local
│
├── templates/                       # Templates de propostas (HTML)
│   └── proposal-template.html
│
├── docs/
│   ├── ARCHITECTURE.md             # Este arquivo
│   └── API.md                      # Documentação das APIs
│
├── README.md                       # Documentação geral
├── INSTALLATION.md                 # Guia de instalação
└── .gitignore
```

---

## 🔄 Fluxo de Processo

### 1️⃣ Criação de Caso

```
Usuário preenche formulário
    ↓
Validação de dados
    ↓
POST /api/cases
    ↓
Salvar em MongoDB (status: "draft")
    ↓
Retornar ID do caso
```

### 2️⃣ Análise com Copilot/IA

```
Usuário clica em "Analisar"
    ↓
POST /api/cases/:id/analyze
    ↓
Buscar caso no MongoDB
    ↓
Atualizar status para "analyzing"
    ↓
Enviar dados para OpenAI API
    ↓
Receber análise (estratégia, recomendações, timeline, orçamento)
    ↓
Salvar análise em MongoDB (status: "analyzed")
    ↓
Retornar análise para frontend
```

### 3️⃣ Geração e Envio de Proposta

```
Usuário clica em "Enviar Proposta"
    ↓
POST /api/cases/:id/send-proposal
    ↓
Buscar caso e análise
    ↓
Gerar HTML/CSS da proposta (ProposalGeneratorService)
    ↓
Configurar email com Nodemailer
    ↓
Enviar proposta para cliente (SMTP)
    ↓
Salvar proposta em MongoDB
    ↓
Atualizar status para "proposal_sent"
    ↓
Retornar confirmação para frontend
```

---

## 🔐 Segurança

- **JWT**: Autenticação de usuários (pronto para implementar)
- **HTTPS**: Use SSL/TLS em produção
- **Validação**: Joi para validar inputs
- **Rate Limiting**: Implementar em produção
- **CORS**: Configurado para o frontend

---

## 💾 Banco de Dados (MongoDB)

### Coleção: Cases

```json
{
  "_id": "ObjectId",
  "clientName": "String",
  "clientEmail": "String",
  "clientPhone": "String",
  "clientCompany": "String",
  "caseDescription": "String",
  "caseCategory": "String",
  "objectives": ["String"],
  "challenges": ["String"],
  "budget": "Number",
  "timeline": "String",
  "analysis": {
    "strategy": "String",
    "recommendations": ["String"],
    "timeline": "String",
    "estimatedBudget": "Number",
    "risks": ["String"]
  },
  "proposal": {
    "htmlContent": "String",
    "sent": "Boolean",
    "sentAt": "Date"
  },
  "status": "draft|analyzing|analyzed|proposal_sent|completed",
  "createdAt": "Date",
  "updatedAt": "Date"
}
```

---

## 🔌 Integrações

### OpenAI API

```typescript
// Análise inteligente de casos
const analysis = await analysisService.analyzeCase({
  caseDescription: "...",
  objectives: ["..."],
  challenges: ["..."]
});

// Retorna:
{
  strategy: "String",
  recommendations: ["String"],
  timeline: "String",
  estimatedBudget: Number,
  risks: ["String"]
}
```

### Email (SMTP)

```typescript
// Envio de propostas
await emailService.sendProposal({
  to: "cliente@example.com",
  subject: "Sua Proposta",
  html: "<html>...</html>",
  clientName: "João"
});
```

---

## 🚀 Escalabilidade

### Melhorias Futuras

- [ ] **Cache com Redis**: Cachear análises similares
- [ ] **Fila de Jobs**: Bull para processar análises assincronamente
- [ ] **Autenticação**: Implementar login de usuários
- [ ] **Painel Admin**: Dashboard com analytics
- [ ] **Webhooks**: Notificações em tempo real
- [ ] **Múltiplos LLMs**: Suportar Claude, Llama, etc.
- [ ] **Upload de Arquivos**: Anexos em propostas
- [ ] **Templates Customizáveis**: UI builder para propostas
- [ ] **Integração CRM**: Salesforce, HubSpot, etc.
- [ ] **Assinatura Digital**: E-assinatura em propostas

---

## 📊 Performance

- **API Response**: ~200ms
- **Análise IA**: ~30-60s (depende de OpenAI)
- **Envio Email**: ~5-10s
- **Database Queries**: ~50-100ms

---

## 🛠️ Tech Stack

| Layer | Tecnologia |
|-------|-----------|
| **Frontend** | React, Next.js, TailwindCSS |
| **Backend** | Node.js, Express, TypeScript |
| **Database** | MongoDB, Mongoose |
| **IA** | OpenAI API (GPT-4) |
| **Email** | Nodemailer, SMTP |
| **Auth** | JWT (pronto para usar) |
| **Logging** | Console (melhorar com Winston) |

---

## 📝 Notas de Desenvolvimento

1. **Variáveis de Ambiente**: Nunca commitar `.env` com dados sensíveis
2. **Error Handling**: Melhorar com try-catch estruturado
3. **Logging**: Implementar Winston para logs estruturados
4. **Testing**: Adicionar testes com Jest
5. **CI/CD**: Configurar GitHub Actions para deploys automáticos
