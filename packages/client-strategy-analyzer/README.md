# 🎯 Client Strategy Analyzer

[🇧🇷 Português](#) | [🇺🇸 English](./README.en.md)

Sistema inteligente de análise de casos de clientes com geração automática de propostas usando IA (Copilot/OpenAI).

> **Recebe → Analisa → Propõe → Envia por Email**

---

## 🚀 Funcionalidades

- ✅ **Formulário Web** para registrar casos de clientes
- ✅ **Análise com IA** (ChatGPT/Copilot) de estratégias personalizadas
- ✅ **Geração Automática** de propostas profissionais em HTML/CSS
- ✅ **Envio por Email** para clientes (SMTP/Gmail)
- ✅ **Dashboard Admin** com lista de casos e status
- ✅ **Banco de Dados** (MongoDB) para histórico
- ✅ **API REST** completa e documentada
- ✅ **Interface Responsiva** (Desktop/Mobile)

---

## 📋 Fluxo de Trabalho

```
┌──────────────┐
│  1. NOVO     │  Cliente registra seu caso via formulário
│  CASO        │  (nome, email, descrição, objetivos, etc)
└──────────────┘
       ↓
┌──────────────┐
│  2. ANÁLISE  │  IA analisa o caso e gera:
│  COM IA      │  • Estratégia detalhada
└──────────────┘  • Recomendações práticas
       ↓          • Timeline de implementação
┌──────────────┐  • Orçamento estimado
│  3. PROPOSTA │  • Riscos identificados
│  GERADA      │
└──────────────┘
       ↓
┌──────────────┐
│  4. ENVIO    │  Proposta profissional (HTML/CSS)
│  EMAIL       │  enviada automaticamente para cliente
└──────────────┘
```

---

## 🎯 Casos de Uso

### 📱 Marketing Digital
- E-commerce quer aumentar vendas online
- Agência precisa estruturar estratégia
- Startup busca crescimento rápido

### 💼 Vendas B2B
- Empresas precisam de CRM
- Estruturação de pipeline
- Processos de prospecção

### 🏢 Operacional
- Reengenharia de processos
- Implementação de sistemas
- Consultoria de otimização

### 💰 Financeiro
- Planejamento financeiro
- Gestão de custos
- Previsões e cenários

---

## 🛠️ Tech Stack

| Camada | Tecnologia |
|--------|-----------|
| **Frontend** | React 18 + Next.js 13 + TailwindCSS |
| **Backend** | Node.js + Express + TypeScript |
| **Database** | MongoDB + Mongoose |
| **IA** | OpenAI API (GPT-4) |
| **Email** | Nodemailer + SMTP |
| **Auth** | JWT (pronto para usar) |

---

## 📁 Estrutura do Projeto

```
client-strategy-analyzer/
│
├── backend/                    # Node.js + TypeScript
│   ├── src/
│   │   ├── models/            # Schemas MongoDB
│   │   ├── services/          # Lógica de negócio
│   │   ├── routes/            # Endpoints API
│   │   └── index.ts           # Servidor principal
│   ├── dist/                  # Build compilado
│   └── .env.example           # Template de env
│
├── frontend/                   # React + Next.js
│   ├── pages/                 # Páginas da aplicação
│   ├── components/            # Componentes React
│   ├── styles/                # CSS global
│   └── .env.local             # Configuração
│
├── templates/                 # Templates HTML de propostas
│
├── QUICKSTART.md              # Início rápido (5 min)
├── INSTALLATION.md            # Guia de instalação
├── ARCHITECTURE.md            # Arquitetura do sistema
├── API.md                     # Documentação das APIs
└── README.md                  # Este arquivo
```

---

## ⚡ Quick Start (5 minutos)

### 1. Clone e Configure
```bash
# Instalar dependências backend
cd backend
npm install
cp .env.example .env

# Editar .env com suas credenciais:
# - MONGODB_URI
# - OPENAI_API_KEY
# - EMAIL_USER / EMAIL_PASSWORD
```

### 2. Instalar Frontend
```bash
cd frontend
npm install
```

### 3. Iniciar Servidores
```bash
# Terminal 1 - Backend
cd backend
npm run dev

# Terminal 2 - Frontend
cd frontend
npm run dev
```

### 4. Acessar
- **Dashboard**: http://localhost:3001
- **API**: http://localhost:3000
- **Health Check**: http://localhost:3000/health

---

## 📡 APIs Principais

```bash
# Criar novo caso
POST /api/cases

# Listar casos
GET /api/cases?page=1&limit=10

# Obter caso específico
GET /api/cases/:id

# Analisar com IA
POST /api/cases/:id/analyze

# Enviar proposta por email
POST /api/cases/:id/send-proposal
```

Documentação completa em: [`API.md`](./API.md)

---

## 🔐 Configuração

### MongoDB
```env
MONGODB_URI=mongodb://localhost:27017/client-analyzer
# ou MongoDB Atlas: mongodb+srv://user:pass@cluster.mongodb.net/db
```

### OpenAI
```env
OPENAI_API_KEY=sk-...
# Gere em: https://platform.openai.com/api-keys
```

### Email (Gmail)
```env
EMAIL_HOST=smtp.gmail.com
EMAIL_PORT=587
EMAIL_USER=seu_email@gmail.com
EMAIL_PASSWORD=sua_senha_app
# Gere senha de app em: https://myaccount.google.com/apppasswords
```

Mais detalhes em: [`INSTALLATION.md`](./INSTALLATION.md)

---

## 🎨 Screenshots

### Dashboard Admin
```
┌─────────────────────────────────────────┐
│  📊 Client Strategy Analyzer            │
│                                         │
│  [📋 Casos] [➕ Novo Caso]             │
│                                         │
│  Cliente        | Empresa   | Status   │
│  ────────────────────────────────────  │
│  João Silva     | Tech Inc  | analyzed │
│  Maria Costa    | Inovatech | draft    │
│  Pedro Lima     | StartupXY | sent     │
└─────────────────────────────────────────┘
```

### Formulário Novo Caso
```
Novo Caso
├─ Nome: [____________]
├─ Email: [____________]
├─ Empresa: [____________]
├─ Descrição: [________________]
├─ Categoria: [Marketing ▼]
├─ Objetivos: [________________]
├─ Orçamento: [R$ ________]
└─ [Criar Caso]
```

### Proposta HTML (Email)
```html
┌─────────────────────────────────────┐
│ 🎯 PROPOSTA DE ESTRATÉGIA           │
│                                     │
│ Cliente: João Silva                 │
│ Data: 05/12/2024                   │
│                                     │
│ 📋 Estratégia Recomendada           │
│ Lorem ipsum dolor sit amet...       │
│                                     │
│ 💡 Recomendações:                   │
│ ✓ Recomendação 1                    │
│ ✓ Recomendação 2                    │
│                                     │
│ 📅 Timeline: 30 dias                │
│ 💰 Orçamento: R$ 25.000             │
│                                     │
│ [Agendar Reunião]                   │
└─────────────────────────────────────┘
```

---

## 📚 Documentação

| Arquivo | Descrição |
|---------|-----------|
| [`QUICKSTART.md`](./QUICKSTART.md) | Início rápido (5 min) |
| [`INSTALLATION.md`](./INSTALLATION.md) | Instalação detalhada |
| [`ARCHITECTURE.md`](./ARCHITECTURE.md) | Arquitetura do sistema |
| [`API.md`](./API.md) | Documentação das APIs |

---

## 🧪 Exemplo Completo

### Passo 1: Criar Caso
```bash
curl -X POST http://localhost:3000/api/cases \
  -H "Content-Type: application/json" \
  -d '{
    "clientName": "João Silva",
    "clientEmail": "joao@empresa.com",
    "clientCompany": "Tech Solutions",
    "caseDescription": "Empresa precisa melhorar conversão de vendas",
    "caseCategory": "vendas",
    "objectives": ["Aumentar conversão", "Estruturar pipeline"],
    "challenges": ["Equipe pequena", "Sem dados"],
    "budget": 20000,
    "timeline": "60 dias"
  }'
```

Retorna: `{ "caseId": "ABC123" }`

### Passo 2: Analisar com IA
```bash
curl -X POST http://localhost:3000/api/cases/ABC123/analyze
```

IA retorna estratégia, recomendações, timeline e orçamento!

### Passo 3: Enviar Proposta
```bash
curl -X POST http://localhost:3000/api/cases/ABC123/send-proposal
```

Email profissional é enviado para `joao@empresa.com`!

---

## 🎁 Bônus: Customizações

### Mudar Cores da Proposta
```
backend/src/services/ProposalGeneratorService.ts
Edite: #667eea (roxo), #764ba2 (rosa)
```

### Adicionar Logo
```html
<img src="seu_logo.png" alt="Logo" class="logo">
```

### Mudar Template de Email
Você controla 100% do HTML/CSS da proposta!

---

## 🚀 Deploy

### Backend (Railway, Render, Heroku)
```bash
npm run build
npm start
```

### Frontend (Vercel, Netlify)
```bash
npm run build
npm start
```

---

## 🛠️ Desenvolvimento

### Adicionar Nova Feature
1. Crie branch: `git checkout -b feature/minha-feature`
2. Faça mudanças
3. Teste localmente
4. Commit: `git commit -am 'feat: descrição'`
5. Push: `git push origin feature/minha-feature`

### Possíveis Melhorias
- [ ] Autenticação de usuários
- [ ] Dashboard com analytics
- [ ] Geração de PDF
- [ ] Integração com CRM
- [ ] WhatsApp integration
- [ ] Assinatura digital
- [ ] Editor visual de templates
- [ ] Integração Zapier
- [ ] Mobile app (React Native)
- [ ] Suporte multi-idioma

---

## 🐛 Troubleshooting

### MongoDB não conecta
```
→ Verifique se MongoDB está rodando
→ Confirme MONGODB_URI em .env
```

### Erro ao analisar com IA
```
→ Verifique OPENAI_API_KEY
→ Confirme que tem créditos
→ Aguarde rate limit reset
```

### Email não é enviado
```
→ Use senha de app (não senha de conta)
→ Ative 2FA no Gmail
→ Teste SMTP: smtp-connection-test.com
```

Veja [`INSTALLATION.md`](./INSTALLATION.md) para mais troubleshooting.

---

## 📞 Suporte & Comunidade

- 📧 Email: suporte@example.com
- 💬 Discord: [link]
- 🐦 Twitter: [@seu_user]
- 📖 Docs: Veja arquivos `.md` na raiz

---

## 📄 Licença

MIT - Use livremente em projetos comerciais e pessoais

---

## 🙏 Agradecimentos

Desenvolvido com ❤️ para análise inteligente de casos de clientes.

**Última atualização:** Dezembro 2024
