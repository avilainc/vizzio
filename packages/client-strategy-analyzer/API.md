# 📡 Documentação de APIs

## Base URL
```
http://localhost:3000/api
```

---

## 📝 POST /cases

**Criar um novo caso**

### Request
```bash
POST /api/cases
Content-Type: application/json

{
  "clientName": "João Silva",
  "clientEmail": "joao@example.com",
  "clientPhone": "(11) 98765-4321",
  "clientCompany": "Tech Solutions Ltda",
  "caseDescription": "Empresa precisa reestruturar seu processo de vendas. Atualmente tem 10 vendedores, mas conversão está baixa (0.5%). Queremos implementar CRM e processos melhores.",
  "caseCategory": "vendas",
  "objectives": ["Aumentar conversão em 200%", "Estruturar pipeline", "Implementar CRM"],
  "challenges": ["Equipe resistente a mudanças", "Orçamento limitado", "Falta de dados"],
  "budget": 25000,
  "timeline": "60 dias"
}
```

### Response (201 Created)
```json
{
  "success": true,
  "message": "Caso criado com sucesso",
  "caseId": "507f1f77bcf86cd799439011"
}
```

---

## 🔍 GET /cases

**Listar todos os casos com paginação**

### Request
```bash
GET /api/cases?page=1&limit=10
```

### Query Parameters
| Parâmetro | Tipo | Padrão | Descrição |
|-----------|------|--------|-----------|
| `page` | number | 1 | Número da página |
| `limit` | number | 10 | Itens por página |

### Response (200 OK)
```json
{
  "success": true,
  "data": [
    {
      "_id": "507f1f77bcf86cd799439011",
      "clientName": "João Silva",
      "clientEmail": "joao@example.com",
      "clientCompany": "Tech Solutions",
      "caseCategory": "vendas",
      "status": "draft",
      "createdAt": "2024-12-05T10:30:00Z",
      "updatedAt": "2024-12-05T10:30:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 10,
    "total": 42,
    "pages": 5
  }
}
```

---

## 📋 GET /cases/:id

**Obter detalhes de um caso específico**

### Request
```bash
GET /api/cases/507f1f77bcf86cd799439011
```

### Response (200 OK)
```json
{
  "success": true,
  "data": {
    "_id": "507f1f77bcf86cd799439011",
    "clientName": "João Silva",
    "clientEmail": "joao@example.com",
    "clientPhone": "(11) 98765-4321",
    "clientCompany": "Tech Solutions Ltda",
    "caseDescription": "Empresa precisa reestruturar seu processo de vendas...",
    "caseCategory": "vendas",
    "objectives": ["Aumentar conversão em 200%", "Estruturar pipeline"],
    "challenges": ["Equipe resistente", "Orçamento limitado"],
    "budget": 25000,
    "timeline": "60 dias",
    "status": "draft",
    "analysis": null,
    "proposal": null,
    "createdAt": "2024-12-05T10:30:00Z",
    "updatedAt": "2024-12-05T10:30:00Z"
  }
}
```

### Response (404 Not Found)
```json
{
  "success": false,
  "message": "Caso não encontrado"
}
```

---

## 🤖 POST /cases/:id/analyze

**Analisar caso com IA (Copilot/OpenAI)**

### Request
```bash
POST /api/cases/507f1f77bcf86cd799439011/analyze
Content-Type: application/json
```

O corpo pode estar vazio `{}` ou conter override de parâmetros.

### Response (200 OK)
```json
{
  "success": true,
  "message": "Caso analisado com sucesso",
  "analysis": {
    "strategy": "Implementar CRM integrado com automação de vendas. Estruturar processo em 3 fases: (1) Diagnóstico e treinamento (2 semanas), (2) Implementação do CRM (3 semanas), (3) Otimização e relatórios (2 semanas). Focar em documentação de processos e resistência à mudança.",
    "recommendations": [
      "Implementar CRM como Salesforce ou Pipedrive",
      "Estruturar funil de vendas em 5 estágios",
      "Criar roteiros para cada estágio",
      "Implementar KPIs de conversão por estágio",
      "Treinar equipe com consultoria 1-on-1",
      "Criar cultura de dados e análise"
    ],
    "timeline": "Fase 1: Diagnóstico (2 semanas) → Fase 2: Implementação (3 semanas) → Fase 3: Otimização (2 semanas)",
    "estimatedBudget": 22000,
    "risks": [
      "Resistência da equipe à mudança",
      "Qualidade dos dados iniciais",
      "Curva de aprendizado do CRM",
      "Falta de disciplina nos registros"
    ]
  }
}
```

### Erros

**Case não encontrado (404)**
```json
{
  "success": false,
  "message": "Caso não encontrado"
}
```

**Erro na API OpenAI (500)**
```json
{
  "success": false,
  "message": "Erro ao analisar caso",
  "error": "..."
}
```

---

## 📧 POST /cases/:id/send-proposal

**Gerar e enviar proposta por email**

### Request
```bash
POST /api/cases/507f1f77bcf86cd799439011/send-proposal
Content-Type: application/json
```

O corpo pode estar vazio `{}`.

### Response (200 OK)
```json
{
  "success": true,
  "message": "Proposta enviada com sucesso"
}
```

**Ações realizadas:**
1. Gera HTML/CSS profissional da proposta
2. Configura email com Nodemailer
3. Envia para `clientEmail`
4. Salva proposta em BD
5. Atualiza status para `proposal_sent`

### Erros

**Caso não encontrado (404)**
```json
{
  "success": false,
  "message": "Caso não encontrado"
}
```

**Caso não foi analisado (400)**
```json
{
  "success": false,
  "message": "Caso não foi analisado ainda"
}
```

**Erro ao enviar email (500)**
```json
{
  "success": false,
  "message": "Erro ao enviar proposta",
  "error": "..."
}
```

---

## 🔄 Status de Caso

Um caso passa por estes estados:

```
draft (criado)
  ↓
analyzing (analisando com IA)
  ↓
analyzed (análise pronta)
  ↓
proposal_sent (proposta enviada)
  ↓
completed (finalizado)
```

---

## 📊 Exemplo Completo de Fluxo

### 1. Criar Caso
```bash
curl -X POST http://localhost:3000/api/cases \
  -H "Content-Type: application/json" \
  -d '{
    "clientName": "Maria Silva",
    "clientEmail": "maria@empresa.com",
    "caseDescription": "Aumentar vendas online",
    "caseCategory": "marketing",
    "objectives": ["Aumentar tráfego"],
    "challenges": ["Concorrência"]
  }'
```

Resposta:
```json
{ "success": true, "caseId": "ABC123" }
```

### 2. Analisar Caso
```bash
curl -X POST http://localhost:3000/api/cases/ABC123/analyze \
  -H "Content-Type: application/json"
```

Resposta:
```json
{
  "success": true,
  "analysis": { ... }
}
```

### 3. Enviar Proposta
```bash
curl -X POST http://localhost:3000/api/cases/ABC123/send-proposal \
  -H "Content-Type: application/json"
```

Resposta:
```json
{ "success": true, "message": "Proposta enviada com sucesso" }
```

Email é enviado para `maria@empresa.com` com proposta profissional!

---

## 🔐 Autenticação (Futuro)

**Será implementado com JWT:**

```bash
Authorization: Bearer token_jwt_aqui
```

---

## 📝 Exemplo de Proposta HTML Enviada

A proposta inclui:

```
📋 Informações do Cliente
🎯 Estratégia Recomendada
💡 Recomendações Principais
📅 Timeline de Implementação
💰 Orçamento Estimado
⚠️ Riscos Identificados
🚀 Próximos Passos
```

Totalmente customizável em:
```
backend/src/services/ProposalGeneratorService.ts
```

---

## 🧪 Testar APIs com Postman/Insomnia

Importe coleção:

```
POST http://localhost:3000/api/cases
GET http://localhost:3000/api/cases
GET http://localhost:3000/api/cases/:id
POST http://localhost:3000/api/cases/:id/analyze
POST http://localhost:3000/api/cases/:id/send-proposal
```

---

## ⚡ Rate Limiting

Atualmente não implementado. Adicione em produção:

```typescript
import rateLimit from 'express-rate-limit';

const limiter = rateLimit({
  windowMs: 15 * 60 * 1000, // 15 minutos
  max: 100 // limite de 100 requisições
});

app.use('/api/', limiter);
```

---

## 📈 Response Times

| Endpoint | Tempo Médio |
|----------|------------|
| POST /cases | 150ms |
| GET /cases | 200ms |
| POST /cases/:id/analyze | 45-60s |
| POST /cases/:id/send-proposal | 8-12s |

---

## 🐛 Troubleshooting

### "MongoDB connection failed"
```
→ Verifique MONGODB_URI em .env
→ MongoDB está rodando?
```

### "Invalid OpenAI API Key"
```
→ Verifique OPENAI_API_KEY em .env
→ Key deve estar sem espaços
```

### "Email delivery failed"
```
→ Use senha de app (não senha da conta)
→ Verifique EMAIL_USER e EMAIL_PASSWORD
→ SMTP pode estar bloqueado (verifique firewall)
```

---

## 📞 Suporte

Veja:
- `QUICKSTART.md` - Início rápido
- `INSTALLATION.md` - Instalação detalhada
- `ARCHITECTURE.md` - Arquitetura do sistema
