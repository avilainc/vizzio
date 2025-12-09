# 💬 VIZZIO ChatBot - Python

Chatbot WhatsApp-style com notificações de build em tempo real.

## 🚀 Quick Start

```bash
# Instalar dependências
pip install -r requirements.txt

# Rodar
python chatbot.py

# Acessar
http://localhost:3001/chat
```

## 📡 API

### WebSocket
```
ws://localhost:3001/ws
```

### Notificar Build
```bash
curl -X POST "http://localhost:3001/api/chat/notify" \
  -H "Content-Type: application/json" \
  -d '{
    "buildId": "123-456",
    "workflow": "CI/CD",
    "status": "success",
    "details": {
      "testsRun": 50,
      "testsPassed": 50,
      "coverage": 85,
      "duration": 125
    }
  }'
```

### Mensagens
```bash
# Últimas 50 mensagens
GET http://localhost:3001/api/chat/messages?limit=50

# Não lidas
GET http://localhost:3001/api/chat/unread
```

## 🎯 Comandos do Bot

- `build status` → Status de builds
- `erros` → Lista de erros
- `success` → Builds bem-sucedidos
- `help` → Ajuda

## 🔗 Integração com CI/CD

Adicione ao workflow:

```yaml
- name: Notify ChatBot
  run: |
    curl -X POST "http://localhost:3001/api/chat/notify" \
      -H "Content-Type: application/json" \
      -d '{
        "buildId": "${{ github.run_id }}",
        "workflow": "${{ github.workflow }}",
        "status": "success",
        "details": {"testsRun": 50, "testsPassed": 50, "coverage": 85}
      }'
```

## 📦 Arquivos

- `chatbot.py` - Backend FastAPI + WebSocket
- `chat.html` - Interface WhatsApp-style
- `requirements.txt` - Dependências Python

**YOLO MODE**: Pronto para produção! 🎉
