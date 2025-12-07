# 📊 VIZZIO Build Monitor

Monitor em **tempo real** de builds, testes e deployments com **dashboard web** + **emails automáticos**.

## 🎯 Funcionalidades

✅ **Dashboard Web** - Interface em tempo real com WebSocket
✅ **Email HTML** - Relatórios automáticos em HTML/CSS
✅ **API REST** - Endpoints para integração com CI/CD
✅ **Banco de Dados** - Histórico completo de builds
✅ **Filtros & Busca** - Procure builds por status, branch, workflow
✅ **Notificações** - Logs em tempo real durante execução

---

## 🚀 Setup Rápido

### 1. Instalar Dependências

```bash
cd packages/build-monitor
npm install
```

### 2. Configurar Variáveis de Ambiente

```bash
cp .env.example .env
```

Edite `.env` com suas credenciais:
- **MongoDB URI** - Banco de dados
- **SMTP** - Servidor de email
- **EMAIL_RECIPIENTS** - Quem recebe notificações

### 3. Iniciar Servidor

```bash
npm run dev
```

O servidor estará em: `http://localhost:3000`

### 4. Acessar Dashboard

```
http://localhost:3000/dashboard
```

---

## 📧 Configurar Email

### Gmail (Recomendado)

1. Ativar **2FA** em sua conta Google
2. Gerar **App Password** em: https://myaccount.google.com/apppasswords
3. Adicionar ao `.env`:

```env
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_SECURE=false
SMTP_USER=seu-email@gmail.com
SMTP_PASS=sua-app-password
EMAIL_RECIPIENTS=nicolas@avila.inc
```

---

## 🔗 Integrar com GitHub Actions

### Adicionar ao Workflow

```yaml
- name: 📡 Notify Build Start
  run: |
    curl -X POST "${{ secrets.MONITOR_URL }}/api/builds/start" \
      -H "Content-Type: application/json" \
      -d '{
        "buildId": "${{ github.run_id }}-${{ github.run_number }}",
        "workflow": "${{ github.workflow }}",
        "branch": "${{ github.ref_name }}",
        "commit": "${{ github.sha }}",
        "author": "${{ github.actor }}",
        "message": "Build iniciado"
      }' || true
```

### Adicionar Secret no GitHub

Settings → Secrets → `MONITOR_URL`

```
https://seu-dominio.com/api/builds
```

---

## 📊 API Endpoints

### Start Build

```bash
POST /api/builds/start
Content-Type: application/json

{
  "buildId": "123-456",
  "workflow": "CI",
  "branch": "main",
  "commit": "abc123",
  "author": "nicolas",
  "message": "Build iniciado"
}
```

### Add Log

```bash
POST /api/builds/{buildId}/log
Content-Type: application/json

{
  "level": "info",
  "message": "Tests passed",
  "step": "test-rust"
}
```

### Complete Build

```bash
POST /api/builds/{buildId}/complete
Content-Type: application/json

{
  "status": "success",
  "duration": 125,
  "details": {
    "testsRun": 50,
    "testsPassed": 50,
    "testsFailed": 0,
    "coverage": 85
  }
}
```

### Get Stats

```bash
GET /api/stats

{
  "total": 150,
  "successful": 145,
  "failed": 5,
  "successRate": "96.67%",
  "avgDuration": 125
}
```

---

## 📧 Email Template

Os emails são enviados automaticamente com:

- ✅ Status do build (sucesso/falha/cancelado)
- 📊 Estatísticas de testes
- 🔍 Lista de erros/warnings
- 📝 Últimos logs
- 🔗 Link para dashboard completo

---

## 🔄 WebSocket Events

### Client → Server

```javascript
// Solicitar builds recentes
socket.emit('request-recent-builds');
```

### Server → Client

```javascript
// Builds recentes carregados
socket.on('recent-builds', (builds) => { });

// Build iniciado
socket.on('build-started', (build) => { });

// Log adicionado
socket.on('build-log', ({ buildId, log }) => { });

// Build completado
socket.on('build-completed', (build) => { });
```

---

## 🐳 Deploy com Docker

```bash
# Build
docker build -t vizzio-monitor .

# Run
docker run -p 3000:3000 \
  -e MONGODB_URI=mongodb://mongo:27017 \
  -e SMTP_USER=seu-email \
  -e SMTP_PASS=sua-senha \
  vizzio-monitor
```

---

## 📱 Exemplos de Email

### Build Bem-sucedido ✅

```
SUCESSO ✅
CI/CD Pipeline • main

Workflow: CI/CD Pipeline
Branch: main
Commit: abc12345
Autor: nicolas
Duração: 2m 15s
Data: 06/12/2025 14:30

Testes: 50
Passaram: 50 ✅
Falharam: 0 ❌
Coverage: 85%

[Últimos Logs]
[14:28:00] [test-rust] Running tests...
[14:29:30] [test-rust] All tests passed ✅
```

### Build com Falha ❌

```
FALHA ❌
CI/CD Pipeline • feature/new-feature

Workflow: CI/CD Pipeline
Branch: feature/new-feature
Commit: def67890
Autor: carlos
Duração: 1m 45s
Data: 06/12/2025 14:25

Testes: 50
Passaram: 48 ✅
Falharam: 2 ❌
Coverage: 82%

[Últimos Logs com Erros]
[14:23:15] [test-rust] Testing...
[14:24:30] [test-rust] ❌ Test failed: auth_test
[14:24:45] [test-rust] ❌ Test failed: db_test
```

---

## 🔧 Configurações Avançadas

### Notificações Slack

```env
SLACK_WEBHOOK_URL=https://hooks.slack.com/services/YOUR/WEBHOOK
```

### Notificações Discord

```env
DISCORD_WEBHOOK_URL=https://discord.com/api/webhooks/YOUR/WEBHOOK
```

### Autenticação API

```env
API_KEY=seu-chave-secreta
```

---

## 📖 Documentação Completa

Ver: `/docs/BUILD_MONITOR.md`

---

## 🤝 Suporte

Email: nicolas@avila.inc
Issues: https://github.com/avilainc/vizzio/issues

---

**VIZZIO Build Monitor** • Monitoramento automático de builds 🚀
