# Client Strategy Analyzer - Guia Rápido

## 🎯 O que é?

Sistema inteligente que:
1. ✅ Recebe casos de clientes (via formulário web)
2. ✅ Analisa com AI/Copilot (estratégia personalizada)
3. ✅ Gera propostas profissionais (HTML/CSS)
4. ✅ Envia por email (automaticamente)

## 🚀 Start Rápido (5 minutos)

### 1. Configure Backend
```bash
cd backend
npm install
cp .env.example .env
# Edite .env com suas credenciais (MongoDB, OpenAI, Email)
npm run dev
```

### 2. Configure Frontend
```bash
cd frontend
npm install
npm run dev
```

### 3. Acesse
- Dashboard: http://localhost:3001
- API: http://localhost:3000

---

## 📋 Fluxo Simples

```
1. Novo Caso (Formulário)
   ↓
2. Analisar (Com IA/Copilot)
   ↓
3. Enviar Proposta (Por Email)
   ↓
4. Cliente Recebe Proposta (HTML/CSS Profissional)
```

---

## 📦 Arquivos Principais

```
backend/
├── src/models/Case.ts           → Estrutura de dados
├── src/services/
│   ├── CopilotAnalysisService   → Análise com IA
│   ├── EmailService             → Envio de emails
│   └── ProposalGeneratorService → Templates HTML
└── src/routes/cases.ts          → API endpoints

frontend/
├── pages/index.tsx              → Dashboard principal
├── components/
│   ├── CaseForm.tsx             → Formulário novo caso
│   └── CasesList.tsx            → Lista de casos
└── styles/globals.css           → Estilos
```

---

## 🔧 Configurações Necessárias

### MongoDB
- Local: `mongodb://localhost:27017/client-analyzer`
- Cloud: Use string de conexão do MongoDB Atlas

### OpenAI
- Gere chave em: https://platform.openai.com/api-keys
- Insira em `OPENAI_API_KEY`

### Email (Gmail)
1. Ative 2FA na conta Google
2. Gere senha de app: https://myaccount.google.com/apppasswords
3. Use senha de app em `EMAIL_PASSWORD`

---

## 📧 Template de Proposta

O template HTML/CSS fica em:
```
backend/src/services/ProposalGeneratorService.ts
```

Customizar:
- Cores (gradientes roxo/rosa)
- Fontes
- Logo
- Seções

---

## 🎨 Interface

### Dashboard Admin
- Listar casos
- Ver status (draft → analyzing → analyzed → proposal_sent)
- Botões de ação (analisar, enviar)
- Filtros por status/categoria

### Novo Caso
- Nome cliente
- Email cliente
- Descrição caso
- Objetivos
- Desafios
- Orçamento
- Timeline

---

## 💡 Exemplos de Casos

### Marketing Digital
```
Cliente: Loja de moda online
Problema: Baixa conversão (1%)
Objetivo: Aumentar para 3%
Desafio: Orçamento limitado

AI irá gerar:
✓ Estratégia (SEO, social media, email marketing)
✓ Recomendações (ferramentas, canais)
✓ Timeline (30-90 dias)
✓ Orçamento estimado
✓ Riscos identificados
```

### Vendas B2B
```
Cliente: SaaS de RH
Problema: Processos de vendas ineficientes
Objetivo: Estruturar pipeline
Desafio: Pequeno time

AI irá gerar:
✓ Estratégia de vendas
✓ Processos e ferramentas
✓ Timeline de implementação
✓ Investimento necessário
✓ KPIs para medir sucesso
```

---

## 🔌 APIs Disponíveis

```
POST   /api/cases                    # Criar caso
GET    /api/cases                    # Listar casos
GET    /api/cases/:id                # Obter caso
POST   /api/cases/:id/analyze        # Analisar com IA
POST   /api/cases/:id/send-proposal  # Enviar proposta
```

---

## 📊 Status de Caso

| Status | Significado |
|--------|------------|
| `draft` | Caso criado, aguardando análise |
| `analyzing` | IA analisando... |
| `analyzed` | Análise pronta, pronto para enviar |
| `proposal_sent` | Proposta enviada ao cliente |
| `completed` | Caso finalizado |

---

## ⚠️ Troubleshooting

### Erro: "Cannot connect to MongoDB"
```
→ Verifique se MongoDB está rodando
→ Confirme MONGODB_URI em .env
```

### Erro: "Invalid API Key"
```
→ Gere nova chave em https://platform.openai.com/api-keys
→ Adicione em OPENAI_API_KEY
```

### Erro: "Email delivery failed"
```
→ Use senha de app (não senha da conta)
→ Ative "Less secure apps" (se necessário)
→ Teste SMTP em: https://www.smtp-connection-test.com/
```

---

## 🎓 Próximos Passos

1. **Autenticação**: Adicionar login de usuários
2. **Analytics**: Dashboard com métricas
3. **Templates**: Editor visual de propostas
4. **Integrações**: CRM, WhatsApp, Zapier
5. **Mobile**: App React Native
6. **Documentos**: Gerar PDF também

---

## 📞 Suporte

Documentação completa em:
- `INSTALLATION.md` → Instalação detalhada
- `ARCHITECTURE.md` → Arquitetura do sistema
- `API.md` → Documentação das APIs

---

## 📄 Licença

MIT License - Use livremente

---

**Criado para análise inteligente de casos de clientes com propostas automáticas.**
