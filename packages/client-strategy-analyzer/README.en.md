# 🎯 Client Strategy Analyzer

**English** | [Português](./README.md)

Intelligent client case analysis system with automatic proposal generation using AI (Copilot/OpenAI).

> **Receives → Analyzes → Proposes → Sends via Email**

---

## 🚀 Features

- ✅ **Web Form** to register client cases
- ✅ **AI Analysis** (ChatGPT/Copilot) with personalized strategies
- ✅ **Automatic Generation** of professional proposals in HTML/CSS
- ✅ **Email Delivery** to clients (SMTP/Gmail)
- ✅ **Admin Dashboard** with case list and status
- ✅ **Database** (MongoDB) for case history
- ✅ **Complete REST API** with documentation
- ✅ **Responsive Interface** (Desktop/Mobile)

---

## 📋 Workflow

```
┌──────────────┐
│  1. NEW      │  Client registers their case via form
│  CASE        │  (name, email, description, objectives, etc)
└──────────────┘
       ↓
┌──────────────┐
│  2. AI       │  AI analyzes the case and generates:
│  ANALYSIS    │  • Detailed strategy
└──────────────┘  • Practical recommendations
       ↓          • Implementation timeline
┌──────────────┐  • Estimated budget
│  3. PROPOSAL │  • Identified risks
│  GENERATED   │
└──────────────┘
       ↓
┌──────────────┐
│  4. EMAIL    │  Professional proposal (HTML/CSS)
│  DELIVERY    │  sent automatically to client
└──────────────┘
```

---

## 🎯 Use Cases

### 📱 Digital Marketing
- E-commerce wants to increase online sales
- Agency needs to structure strategy
- Startup seeking rapid growth

### 💼 B2B Sales
- Companies need CRM implementation
- Pipeline structuring
- Prospecting processes

### 🏢 Operations
- Process reengineering
- System implementation
- Optimization consulting

### 💰 Finance
- Financial planning
- Cost management
- Forecasting and scenarios

---

## 🛠️ Tech Stack

| Layer | Technology |
|-------|-----------|
| **Frontend** | React 18 + Next.js 13 + TailwindCSS |
| **Backend** | Node.js + Express + TypeScript |
| **Database** | MongoDB + Mongoose |
| **AI** | OpenAI API (GPT-4) |
| **Email** | Nodemailer + SMTP |
| **Auth** | JWT (ready to use) |

---

## 📁 Project Structure

```
client-strategy-analyzer/
│
├── backend/                    # Node.js + TypeScript
│   ├── src/
│   │   ├── models/            # MongoDB Schemas
│   │   ├── services/          # Business logic
│   │   ├── routes/            # API endpoints
│   │   └── index.ts           # Main server
│   ├── dist/                  # Compiled build
│   └── .env.example           # Environment template
│
├── frontend/                   # React + Next.js
│   ├── pages/                 # Application pages
│   ├── components/            # React components
│   ├── styles/                # Global CSS
│   └── .env.local             # Configuration
│
├── i18n/                      # Internationalization
│   ├── pt-BR.json            # Portuguese
│   └── en-US.json            # English
│
├── QUICKSTART.md              # Quick start (5 min)
├── INSTALLATION.md            # Installation guide
├── ARCHITECTURE.md            # System architecture
├── API.md                     # API documentation
├── README.md                  # Portuguese docs
└── README.en.md               # English docs (this file)
```

---

## ⚡ Quick Start (5 minutes)

### 1. Clone and Configure
```bash
# Install backend dependencies
cd backend
npm install
cp .env.example .env

# Edit .env with your credentials:
# - MONGODB_URI
# - OPENAI_API_KEY
# - EMAIL_USER / EMAIL_PASSWORD
```

### 2. Install Frontend
```bash
cd frontend
npm install
```

### 3. Start Servers
```bash
# Terminal 1 - Backend
cd backend
npm run dev

# Terminal 2 - Frontend
cd frontend
npm run dev
```

### 4. Access
- **Dashboard**: http://localhost:3001
- **API**: http://localhost:3000
- **Health Check**: http://localhost:3000/health

---

## 📡 Main APIs

```bash
# Create new case
POST /api/cases

# List cases
GET /api/cases?page=1&limit=10

# Get specific case
GET /api/cases/:id

# Analyze with AI
POST /api/cases/:id/analyze

# Send proposal via email
POST /api/cases/:id/send-proposal
```

Complete documentation in: [`API.md`](./API.md)

---

## 🔐 Configuration

### MongoDB
```env
MONGODB_URI=mongodb://localhost:27017/client-analyzer
# or MongoDB Atlas: mongodb+srv://user:pass@cluster.mongodb.net/db
```

### OpenAI
```env
OPENAI_API_KEY=sk-...
# Generate at: https://platform.openai.com/api-keys
```

### Email (Gmail)
```env
EMAIL_HOST=smtp.gmail.com
EMAIL_PORT=587
EMAIL_USER=your_email@gmail.com
EMAIL_PASSWORD=your_app_password
# Generate app password at: https://myaccount.google.com/apppasswords
```

See [`INSTALLATION.md`](./INSTALLATION.md) for details.

---

## 🎨 Screenshots

### Admin Dashboard
```
┌─────────────────────────────────────────┐
│  📊 Client Strategy Analyzer            │
│                                         │
│  [📋 Cases] [➕ New Case]              │
│                                         │
│  Client      | Company   | Status      │
│  ────────────────────────────────────  │
│  John Smith  | Tech Inc  | analyzed    │
│  Mary Costa  | Inovatech | draft       │
│  Peter Lima  | StartupXY | sent        │
└─────────────────────────────────────────┘
```

### New Case Form
```
New Case
├─ Name: [____________]
├─ Email: [____________]
├─ Company: [____________]
├─ Description: [________________]
├─ Category: [Marketing ▼]
├─ Objectives: [________________]
├─ Budget: [$ ________]
└─ [Create Case]
```

### Proposal HTML (Email)
```html
┌─────────────────────────────────────┐
│ 🎯 STRATEGY PROPOSAL                │
│                                     │
│ Client: John Smith                  │
│ Date: 12/05/2024                   │
│                                     │
│ 📋 Recommended Strategy             │
│ Lorem ipsum dolor sit amet...       │
│                                     │
│ 💡 Recommendations:                 │
│ ✓ Recommendation 1                  │
│ ✓ Recommendation 2                  │
│                                     │
│ 📅 Timeline: 30 days                │
│ 💰 Budget: $ 25,000                 │
│                                     │
│ [Schedule Meeting]                  │
└─────────────────────────────────────┘
```

---

## 📚 Documentation

| File | Description |
|------|-------------|
| [`QUICKSTART.md`](./QUICKSTART.md) | Quick start (5 min) |
| [`INSTALLATION.md`](./INSTALLATION.md) | Detailed installation |
| [`ARCHITECTURE.md`](./ARCHITECTURE.md) | System architecture |
| [`API.md`](./API.md) | API documentation |

---

## 🧪 Complete Example

### Step 1: Create Case
```bash
curl -X POST http://localhost:3000/api/cases \
  -H "Content-Type: application/json" \
  -d '{
    "clientName": "John Smith",
    "clientEmail": "john@company.com",
    "clientCompany": "Tech Solutions",
    "caseDescription": "Company needs to improve sales conversion",
    "caseCategory": "sales",
    "objectives": ["Increase conversion", "Structure pipeline"],
    "challenges": ["Small team", "No data"],
    "budget": 20000,
    "timeline": "60 days"
  }'
```

Returns: `{ "caseId": "ABC123" }`

### Step 2: Analyze with AI
```bash
curl -X POST http://localhost:3000/api/cases/ABC123/analyze
```

AI returns strategy, recommendations, timeline and budget!

### Step 3: Send Proposal
```bash
curl -X POST http://localhost:3000/api/cases/ABC123/send-proposal
```

Professional email is sent to `john@company.com`!

---

## 🎁 Bonus: Customizations

### Change Proposal Colors
```
backend/src/services/ProposalGeneratorService.ts
Edit: #667eea (purple), #764ba2 (pink)
```

### Add Logo
```html
<img src="your_logo.png" alt="Logo" class="logo">
```

### Change Email Template
You control 100% of the HTML/CSS of the proposal!

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

## 🛠️ Development

### Add New Feature
1. Create branch: `git checkout -b feature/my-feature`
2. Make changes
3. Test locally
4. Commit: `git commit -am 'feat: description'`
5. Push: `git push origin feature/my-feature`

### Possible Improvements
- [ ] User authentication
- [ ] Analytics dashboard
- [ ] PDF generation
- [ ] CRM integration
- [ ] WhatsApp integration
- [ ] Digital signature
- [ ] Visual template editor
- [ ] Zapier integration
- [ ] Mobile app (React Native)
- [ ] Multi-language support (done!)

---

## 🐛 Troubleshooting

### MongoDB not connecting
```
→ Check if MongoDB is running
→ Confirm MONGODB_URI in .env
```

### Error analyzing with AI
```
→ Verify OPENAI_API_KEY
→ Confirm you have credits
→ Wait for rate limit reset
```

### Email not sending
```
→ Use app password (not account password)
→ Enable 2FA on Gmail
→ Test SMTP: smtp-connection-test.com
```

See [`INSTALLATION.md`](./INSTALLATION.md) for more troubleshooting.

---

## 📞 Support & Community

- 📧 Email: support@example.com
- 💬 Discord: [link]
- 🐦 Twitter: [@your_user]
- 📖 Docs: See `.md` files in root

---

## 📄 License

MIT - Use freely in commercial and personal projects

---

## 🙏 Acknowledgments

Developed with ❤️ for intelligent client case analysis.

**Last updated:** December 2024
