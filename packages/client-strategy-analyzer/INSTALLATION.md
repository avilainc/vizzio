# 🚀 Guia de Instalação e Uso

## ⚙️ Pré-requisitos

- Node.js 16+ instalado
- MongoDB instalado localmente ou acesso a MongoDB cloud
- Uma chave API do OpenAI (para usar Copilot)
- Configurações de email (Gmail com app password, ou outro SMTP)

## 📥 Instalação

### 1. Backend

```bash
cd backend
npm install
```

**Configurar variáveis de ambiente:**

Copie o arquivo `.env.example` para `.env` e preencha:

```bash
cp .env.example .env
```

Edite `.env` com suas credenciais:
```env
MONGODB_URI=mongodb://localhost:27017/client-analyzer
JWT_SECRET=sua_chave_secreta_aqui
OPENAI_API_KEY=sua_chave_openai_aqui
GITHUB_TOKEN=seu_token_github
EMAIL_HOST=smtp.gmail.com
EMAIL_PORT=587
EMAIL_USER=seu_email@gmail.com
EMAIL_PASSWORD=sua_senha_app_gmail
NODE_ENV=development
PORT=3000
FRONTEND_URL=http://localhost:3001
```

**Iniciar o servidor:**

```bash
npm run dev
```

O servidor estará disponível em `http://localhost:3000`

---

### 2. Frontend

```bash
cd frontend
npm install
```

**Variáveis de ambiente já estão configuradas em `.env.local`**

**Iniciar o dashboard:**

```bash
npm run dev
```

O dashboard estará disponível em `http://localhost:3001`

---

## 🎯 Fluxo de Uso

### 1️⃣ **Registrar um Novo Caso**

1. Acesse o dashboard em `http://localhost:3001`
2. Clique em "➕ Novo Caso"
3. Preencha o formulário com:
   - Nome do cliente
   - Email do cliente
   - Descrição detalhada do caso
   - Objetivos
   - Desafios
   - Orçamento estimado
4. Clique em "Criar Caso"

### 2️⃣ **Analisar Caso com IA**

1. Vá para a aba "📋 Casos"
2. Localize o caso criado (status: `draft`)
3. Clique no botão 🔄 (Refresh/Analisar)
4. O sistema enviará o caso para análise com Copilot/OpenAI
5. Aguarde a análise (geralmente 30-60 segundos)
6. O status mudará para `analyzed`

### 3️⃣ **Enviar Proposta por Email**

1. Após a análise, o caso terá status `analyzed`
2. Clique no botão 📧 (Enviar Proposta)
3. Uma proposta profissional em HTML/CSS será gerada automaticamente
4. O email será enviado para o cliente
5. O status mudará para `proposal_sent`

---

## 📊 APIs Disponíveis

### Criar Novo Caso
```bash
POST /api/cases
Content-Type: application/json

{
  "clientName": "João Silva",
  "clientEmail": "joao@example.com",
  "clientPhone": "(11) 98765-4321",
  "clientCompany": "Empresa XYZ",
  "caseDescription": "Preciso melhorar minha presença online...",
  "caseCategory": "marketing",
  "objectives": ["Aumentar visibilidade", "Gerar leads"],
  "challenges": ["Concorrência forte", "Orçamento limitado"],
  "budget": 15000,
  "timeline": "30 dias"
}
```

### Analisar Caso
```bash
POST /api/cases/:id/analyze
```

### Enviar Proposta
```bash
POST /api/cases/:id/send-proposal
```

### Listar Casos
```bash
GET /api/cases?page=1&limit=10
```

### Obter Caso Específico
```bash
GET /api/cases/:id
```

---

## 💡 Exemplos de Casos

### Exemplo 1: Marketing Digital

**Descrição:**
"Empresa de e-commerce de moda feminina precisa aumentar o tráfego do site e melhorar a taxa de conversão em 30%. Atualmente recebem 500 visitantes/mês com taxa de conversão de 1%."

**Objetivos:**
- Aumentar tráfego em 100%
- Aumentar conversão para 1.5%
- Melhorar imagem da marca

**Desafios:**
- Orçamento limitado
- Mercado muito competitivo
- Falta de dados analíticos

---

### Exemplo 2: Vendas B2B

**Descrição:**
"Startup de SaaS para gestão de recursos humanos precisa estruturar um processo de vendas eficiente. Atualmente têm 5 clientes e querem chegar a 50 clientes em 6 meses."

**Objetivos:**
- Estruturar processo de vendas
- Criar pipeline de prospecção
- Aumentar taxa de fechamento

**Desafios:**
- Pequeno time de vendas
- Produto em desenvolvimento contínuo
- Mercado em expansão

---

## 🔧 Troubleshooting

### ❌ Erro ao conectar ao MongoDB
- Verifique se MongoDB está rodando
- Confirme a URI do MongoDB em `.env`
- Se usar MongoDB Cloud, copie a string de conexão corretamente

### ❌ Erro ao enviar email
- Ative "senhas de app" no Gmail
- Para Gmail: [Configurar senhas de app](https://myaccount.google.com/apppasswords)
- Copie a senha de app em `EMAIL_PASSWORD`

### ❌ Erro ao chamar API OpenAI
- Verifique se sua chave de API está válida
- Confirme que você tem créditos na conta OpenAI
- Verifique limite de requisições (rate limit)

### ❌ Frontend não conecta ao Backend
- Verifique se backend está rodando em `http://localhost:3000`
- Confirme CORS está ativado no backend
- Verifique `NEXT_PUBLIC_API_URL` em `.env.local`

---

## 📧 Personalizando Template de Email

O template de proposta fica em:
```
backend/src/services/ProposalGeneratorService.ts
```

Você pode customizar:
- Cores (gradientes, paleta)
- Fontes
- Layout
- Conteúdo das seções
- Logo da empresa

---

## 🚀 Deploy

### Backend (Render, Railway, Heroku)

```bash
npm install -g @vercel/ncc
ncc build src/index.ts -o dist
```

### Frontend (Vercel, Netlify)

```bash
npm run build
npm run start
```

---

## 📞 Suporte

Para dúvidas ou problemas, consulte a documentação do projeto em `README.md`
