# 📡 API Documentation

**English** | [Português](./API.md)

## Base URL
```
http://localhost:3000/api
```

---

## 📝 POST /cases

**Create a new case**

### Request
```bash
POST /api/cases
Content-Type: application/json

{
  "clientName": "John Smith",
  "clientEmail": "john@example.com",
  "clientPhone": "(11) 98765-4321",
  "clientCompany": "Tech Solutions Ltda",
  "caseDescription": "Company needs to restructure its sales process. Currently has 10 salespeople, but conversion is low (0.5%). We want to implement CRM and better processes.",
  "caseCategory": "sales",
  "objectives": ["Increase conversion by 200%", "Structure pipeline", "Implement CRM"],
  "challenges": ["Team resistance to change", "Limited budget", "Lack of data"],
  "budget": 25000,
  "timeline": "60 days"
}
```

### Response (201 Created)
```json
{
  "success": true,
  "message": "Case created successfully",
  "caseId": "507f1f77bcf86cd799439011"
}
```

---

## 🔍 GET /cases

**List all cases with pagination**

### Request
```bash
GET /api/cases?page=1&limit=10
```

### Query Parameters
| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `page` | number | 1 | Page number |
| `limit` | number | 10 | Items per page |

### Response (200 OK)
```json
{
  "success": true,
  "data": [
    {
      "_id": "507f1f77bcf86cd799439011",
      "clientName": "John Smith",
      "clientEmail": "john@example.com",
      "clientCompany": "Tech Solutions",
      "caseCategory": "sales",
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

**Get details of a specific case**

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
    "clientName": "John Smith",
    "clientEmail": "john@example.com",
    "clientPhone": "(11) 98765-4321",
    "clientCompany": "Tech Solutions Ltda",
    "caseDescription": "Company needs to restructure its sales process...",
    "caseCategory": "sales",
    "objectives": ["Increase conversion by 200%", "Structure pipeline"],
    "challenges": ["Team resistance", "Limited budget"],
    "budget": 25000,
    "timeline": "60 days",
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
  "message": "Case not found"
}
```

---

## 🤖 POST /cases/:id/analyze

**Analyze case with AI (Copilot/OpenAI)**

### Request
```bash
POST /api/cases/507f1f77bcf86cd799439011/analyze
Content-Type: application/json
```

Body can be empty `{}`.

### Response (200 OK)
```json
{
  "success": true,
  "message": "Case analyzed successfully",
  "analysis": {
    "strategy": "Implement integrated CRM with sales automation. Structure process in 3 phases: (1) Diagnosis and training (2 weeks), (2) CRM implementation (3 weeks), (3) Optimization and reporting (2 weeks). Focus on process documentation and change management.",
    "recommendations": [
      "Implement CRM like Salesforce or Pipedrive",
      "Structure sales funnel in 5 stages",
      "Create playbooks for each stage",
      "Implement conversion KPIs by stage",
      "Train team with 1-on-1 consulting",
      "Build data-driven culture"
    ],
    "timeline": "Phase 1: Diagnosis (2 weeks) → Phase 2: Implementation (3 weeks) → Phase 3: Optimization (2 weeks)",
    "estimatedBudget": 22000,
    "risks": [
      "Team resistance to change",
      "Quality of initial data",
      "CRM learning curve",
      "Lack of discipline in records"
    ]
  }
}
```

### Errors

**Case not found (404)**
```json
{
  "success": false,
  "message": "Case not found"
}
```

**Error in OpenAI API (500)**
```json
{
  "success": false,
  "message": "Error analyzing case",
  "error": "..."
}
```

---

## 📧 POST /cases/:id/send-proposal

**Generate and send proposal via email**

### Request
```bash
POST /api/cases/507f1f77bcf86cd799439011/send-proposal
Content-Type: application/json
```

Body can be empty `{}`.

### Response (200 OK)
```json
{
  "success": true,
  "message": "Proposal sent successfully"
}
```

**Actions performed:**
1. Generates professional HTML/CSS proposal
2. Configures email with Nodemailer
3. Sends to `clientEmail`
4. Saves proposal in DB
5. Updates status to `proposal_sent`

### Errors

**Case not found (404)**
```json
{
  "success": false,
  "message": "Case not found"
}
```

**Case not analyzed (400)**
```json
{
  "success": false,
  "message": "Case has not been analyzed yet"
}
```

**Error sending email (500)**
```json
{
  "success": false,
  "message": "Error sending proposal",
  "error": "..."
}
```

---

## 🔄 Case Status

A case goes through these states:

```
draft (created)
  ↓
analyzing (analyzing with AI)
  ↓
analyzed (analysis ready)
  ↓
proposal_sent (proposal sent)
  ↓
completed (finalized)
```

---

## 📊 Complete Workflow Example

### 1. Create Case
```bash
curl -X POST http://localhost:3000/api/cases \
  -H "Content-Type: application/json" \
  -d '{
    "clientName": "Maria Smith",
    "clientEmail": "maria@company.com",
    "caseDescription": "Increase online sales",
    "caseCategory": "marketing",
    "objectives": ["Increase traffic"],
    "challenges": ["Competition"]
  }'
```

Response:
```json
{ "success": true, "caseId": "ABC123" }
```

### 2. Analyze Case
```bash
curl -X POST http://localhost:3000/api/cases/ABC123/analyze \
  -H "Content-Type: application/json"
```

Response:
```json
{
  "success": true,
  "analysis": { ... }
}
```

### 3. Send Proposal
```bash
curl -X POST http://localhost:3000/api/cases/ABC123/send-proposal \
  -H "Content-Type: application/json"
```

Response:
```json
{ "success": true, "message": "Proposal sent successfully" }
```

Email is sent to `maria@company.com` with professional proposal!

---

## 🔐 Authentication (Future)

**Will be implemented with JWT:**

```bash
Authorization: Bearer jwt_token_here
```

---

## 📝 Example Proposal HTML Sent

The proposal includes:

```
📋 Client Information
🎯 Recommended Strategy
💡 Main Recommendations
📅 Implementation Timeline
💰 Estimated Budget
⚠️ Identified Risks
🚀 Next Steps
```

Fully customizable at:
```
backend/src/services/ProposalGeneratorService.ts
```

---

## 🧪 Test APIs with Postman/Insomnia

Import collection:

```
POST http://localhost:3000/api/cases
GET http://localhost:3000/api/cases
GET http://localhost:3000/api/cases/:id
POST http://localhost:3000/api/cases/:id/analyze
POST http://localhost:3000/api/cases/:id/send-proposal
```

---

## ⚡ Rate Limiting

Currently not implemented. Add in production:

```typescript
import rateLimit from 'express-rate-limit';

const limiter = rateLimit({
  windowMs: 15 * 60 * 1000, // 15 minutes
  max: 100 // 100 requests limit
});

app.use('/api/', limiter);
```

---

## 📈 Response Times

| Endpoint | Average Time |
|----------|------------|
| POST /cases | 150ms |
| GET /cases | 200ms |
| POST /cases/:id/analyze | 45-60s |
| POST /cases/:id/send-proposal | 8-12s |

---

## 🐛 Troubleshooting

### "MongoDB connection failed"
```
→ Check MONGODB_URI in .env
→ Is MongoDB running?
```

### "Invalid OpenAI API Key"
```
→ Check OPENAI_API_KEY in .env
→ Key should be without spaces
```

### "Email delivery failed"
```
→ Use app password (not account password)
→ Check EMAIL_USER and EMAIL_PASSWORD
→ SMTP may be blocked (check firewall)
```

---

## 📞 Support

See:
- `QUICKSTART.md` - Quick start
- `INSTALLATION.md` - Detailed installation
- `ARCHITECTURE.md` - System architecture
