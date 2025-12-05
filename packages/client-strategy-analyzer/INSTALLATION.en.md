# 🚀 Installation & Usage Guide

**English** | [Português](./INSTALLATION.md)

## ⚙️ Prerequisites

- Node.js 16+ installed
- MongoDB installed locally or access to MongoDB cloud
- OpenAI API key (to use Copilot)
- Email configuration (Gmail with app password, or another SMTP)

## 📥 Installation

### 1. Backend

```bash
cd backend
npm install
```

**Configure environment variables:**

Copy `.env.example` to `.env` and fill in:

```bash
cp .env.example .env
```

Edit `.env` with your credentials:
```env
MONGODB_URI=mongodb://localhost:27017/client-analyzer
JWT_SECRET=your_secret_key_here
OPENAI_API_KEY=your_openai_key_here
GITHUB_TOKEN=your_github_token
EMAIL_HOST=smtp.gmail.com
EMAIL_PORT=587
EMAIL_USER=your_email@gmail.com
EMAIL_PASSWORD=your_gmail_app_password
NODE_ENV=development
PORT=3000
FRONTEND_URL=http://localhost:3001
```

**Start the server:**

```bash
npm run dev
```

The server will be available at `http://localhost:3000`

---

### 2. Frontend

```bash
cd frontend
npm install
```

**Environment variables are already configured in `.env.local`**

**Start the dashboard:**

```bash
npm run dev
```

The dashboard will be available at `http://localhost:3001`

---

## 🎯 Usage Flow

### 1️⃣ **Register a New Case**

1. Access the dashboard at `http://localhost:3001`
2. Click on "➕ New Case"
3. Fill the form with:
   - Client name
   - Client email
   - Detailed case description
   - Objectives
   - Challenges
   - Estimated budget
4. Click "Create Case"

### 2️⃣ **Analyze Case with AI**

1. Go to "📋 Cases" tab
2. Find the created case (status: `draft`)
3. Click the 🔄 button (Refresh/Analyze)
4. The system will send the case for analysis with Copilot/OpenAI
5. Wait for analysis (usually 30-60 seconds)
6. Status will change to `analyzed`

### 3️⃣ **Send Proposal via Email**

1. After analysis, the case will have status `analyzed`
2. Click the 📧 button (Send Proposal)
3. A professional HTML/CSS proposal will be automatically generated
4. The email will be sent to the client
5. Status will change to `proposal_sent`

---

## 📡 Available APIs

### Create New Case
```bash
POST /api/cases
Content-Type: application/json

{
  "clientName": "John Smith",
  "clientEmail": "john@example.com",
  "clientPhone": "(11) 98765-4321",
  "clientCompany": "Company XYZ",
  "caseDescription": "Need to improve my online presence...",
  "caseCategory": "marketing",
  "objectives": ["Increase visibility", "Generate leads"],
  "challenges": ["Strong competition", "Limited budget"],
  "budget": 15000,
  "timeline": "30 days"
}
```

### Analyze Case
```bash
POST /api/cases/:id/analyze
```

### Send Proposal
```bash
POST /api/cases/:id/send-proposal
```

### List Cases
```bash
GET /api/cases?page=1&limit=10
```

### Get Specific Case
```bash
GET /api/cases/:id
```

---

## 💡 Case Examples

### Example 1: Digital Marketing

**Description:**
"Women's fashion e-commerce company needs to increase website traffic and improve conversion rate by 30%. Currently receives 500 visitors/month with 1% conversion rate."

**Objectives:**
- Increase traffic by 100%
- Increase conversion to 1.5%
- Improve brand image

**Challenges:**
- Limited budget
- Very competitive market
- Lack of analytics data

---

### Example 2: B2B Sales

**Description:**
"HR management SaaS startup needs to structure an efficient sales process. Currently has 5 clients and wants to reach 50 clients in 6 months."

**Objectives:**
- Structure sales process
- Create prospecting pipeline
- Increase closing rate

**Challenges:**
- Small sales team
- Product in continuous development
- Expanding market

---

## 🔧 Troubleshooting

### ❌ Error connecting to MongoDB
- Check if MongoDB is running
- Confirm the MongoDB URI in `.env`
- If using MongoDB Cloud, copy the connection string correctly

### ❌ Error sending email
- Enable "app passwords" in Gmail
- For Gmail: [Configure app passwords](https://myaccount.google.com/apppasswords)
- Copy the app password to `EMAIL_PASSWORD`

### ❌ Error calling OpenAI API
- Verify your API key is valid
- Confirm you have credits in your OpenAI account
- Check requests limit (rate limit)

### ❌ Frontend doesn't connect to Backend
- Check if backend is running at `http://localhost:3000`
- Verify CORS is enabled in backend
- Check `NEXT_PUBLIC_API_URL` in `.env.local`

---

## 📧 Customizing Email Template

The proposal template is located at:
```
backend/src/services/ProposalGeneratorService.ts
```

You can customize:
- Colors (gradients, palette)
- Fonts
- Layout
- Section content
- Company logo

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

## 📞 Support

For questions or problems, consult the project documentation in `README.en.md`
