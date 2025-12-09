# 🤖 Avila AI Proxy

Servidor de IA **sem censura** com múltiplos modelos Ollama para uso público via API.

## 🎯 Modelos Disponíveis (SEM CENSURA)

- **dolphin-mistral** - Modelo principal sem alinhamento ou censura
- **wizard-vicuna-uncensored** - Sem filtros de conteúdo
- **neural-chat** - Conversacional sem restrições
- **deepseek-coder** - Especializado em código (opcional)
- **deepseek-chat** - Chat avançado (opcional)

## 🚀 Quick Start

### 1. Instalar Ollama

```powershell
# Windows (via winget)
winget install Ollama.Ollama

# Ou baixar de: https://ollama.ai/download
```

### 2. Instalar Modelos

```bash
# Linux/Mac
chmod +x install_models.sh
./install_models.sh

# Windows (PowerShell)
ollama pull dolphin-mistral
ollama pull wizard-vicuna-uncensored
ollama pull neural-chat
```

### 3. Instalar Dependências Python

```bash
pip install -r requirements.txt
```

### 4. Configurar Ambiente

```bash
cp .env.example .env
# Editar .env com suas configurações
```

### 5. Iniciar Servidor

```bash
python server.py
```

Servidor rodando em: `http://localhost:8000`

## 🔑 Gerenciamento de API Keys

### Criar Nova API Key (Admin)

```bash
curl -X POST http://localhost:8000/v1/keys \
  -H "Authorization: Bearer <ADMIN_KEY>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Usuario Publico",
    "tier": "free",
    "expires_days": 365
  }'
```

### Tiers Disponíveis

- **free**: 10 requests/minuto
- **paid**: 100 requests/minuto
- **admin**: 1000 requests/minuto

## 📡 API Endpoints

### Chat Completion (OpenAI-compatible)

```bash
curl -X POST http://localhost:8000/v1/chat/completions \
  -H "Authorization: Bearer <API_KEY>" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "dolphin-mistral",
    "messages": [
      {"role": "user", "content": "Explique como fazer..."}
    ],
    "temperature": 0.7,
    "max_tokens": 2000
  }'
```

### Code Completion (para IDEs)

```bash
curl -X POST http://localhost:8000/v1/code/completions \
  -H "Authorization: Bearer <API_KEY>" \
  -H "Content-Type: application/json" \
  -d '{
    "code": "def fibonacci(n):",
    "language": "python",
    "model": "dolphin-mistral",
    "max_tokens": 500
  }'
```

### WebSocket Streaming

```javascript
const ws = new WebSocket('ws://localhost:8000/ws');

ws.send(JSON.stringify({
  api_key: 'avila_...',
  model: 'dolphin-mistral',
  messages: [
    {role: 'user', content: 'Conte uma história...'}
  ]
}));

ws.onmessage = (event) => {
  console.log(event.data);
};
```

### Verificar Uso

```bash
curl http://localhost:8000/v1/usage \
  -H "Authorization: Bearer <API_KEY>"
```

## 🔌 Integração com Avila Copilot

### Configurar VS Code Extension

1. Abrir configurações do Avila Copilot
2. Definir **Server Path**: `http://localhost:8000`
3. Adicionar API Key nas configurações

### Modificar Engine (Próximo Passo)

Vou criar adaptador para conectar `avila-copilot-lsp` com este proxy.

## 🌐 Deploy Público

### Docker

```dockerfile
FROM python:3.11-slim

WORKDIR /app
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY . .

CMD ["python", "server.py"]
```

```bash
docker build -t avila-ai-proxy .
docker run -p 8000:8000 --env-file .env avila-ai-proxy
```

### Render/Railway/Fly.io

1. Conectar repositório
2. Definir variáveis de ambiente
3. Deploy automático

**IMPORTANTE**: Ollama precisa rodar em servidor separado com GPU.

## 🔒 Segurança

- ✅ API Keys com expiração
- ✅ Rate limiting por tier
- ✅ CORS configurado
- ✅ Validação de entrada
- ❌ **SEM CENSURA** - Use com responsabilidade

## 📊 Monitoramento

```bash
# Ver logs em tempo real
tail -f server.log

# Estatísticas de uso
curl http://localhost:8000/v1/usage -H "Authorization: Bearer <KEY>"
```

## 🎮 Exemplos de Uso

### Python Client

```python
import httpx

API_KEY = "avila_..."
BASE_URL = "http://localhost:8000"

async with httpx.AsyncClient() as client:
    response = await client.post(
        f"{BASE_URL}/v1/chat/completions",
        headers={"Authorization": f"Bearer {API_KEY}"},
        json={
            "model": "dolphin-mistral",
            "messages": [
                {"role": "user", "content": "Sua pergunta aqui"}
            ]
        }
    )
    print(response.json())
```

### JavaScript/TypeScript Client

```typescript
const response = await fetch('http://localhost:8000/v1/chat/completions', {
  method: 'POST',
  headers: {
    'Authorization': `Bearer ${API_KEY}`,
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    model: 'dolphin-mistral',
    messages: [
      {role: 'user', content: 'Sua pergunta aqui'}
    ]
  })
});

const data = await response.json();
console.log(data.choices[0].message.content);
```

## 🆘 Troubleshooting

### Ollama não conecta

```bash
# Verificar se Ollama está rodando
ollama list

# Reiniciar Ollama
# Windows: Reiniciar serviço
# Linux/Mac: killall ollama && ollama serve
```

### Rate limit excedido

- Verificar tier da sua API key
- Aguardar 1 minuto
- Ou criar key com tier superior

### Modelo não encontrado

```bash
# Listar modelos instalados
ollama list

# Instalar modelo faltando
ollama pull dolphin-mistral
```

## 📝 TODO

- [ ] Persistir API keys em MongoDB
- [ ] Dashboard web para gerenciar keys
- [ ] Integrar DeepSeek API
- [ ] Métricas com Prometheus
- [ ] Cache de respostas com Redis
- [ ] Load balancing entre múltiplas instâncias Ollama

## 📄 Licença

MIT - Uso público permitido com API key válida
