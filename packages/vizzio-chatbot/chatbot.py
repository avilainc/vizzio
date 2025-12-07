"""
VIZZIO ChatBot - WhatsApp Style Notifications
Python + FastAPI + WebSocket
YOLO MODE - No tests, straight to production
"""

from fastapi import FastAPI, WebSocket, WebSocketDisconnect
from fastapi.staticfiles import StaticFiles
from fastapi.responses import HTMLResponse
from pydantic import BaseModel
from typing import List, Optional
import asyncio
import json
from datetime import datetime
import uuid

app = FastAPI(title="VIZZIO ChatBot")

# ============================================================================
# MODELS
# ============================================================================

class ChatMessage(BaseModel):
    id: str
    sender: str  # 'bot' or 'user'
    content: str
    timestamp: str
    read: bool
    status: str = "sent"
    type: str = "text"  # text, notification, alert, success
    metadata: Optional[dict] = None

class BuildNotification(BaseModel):
    buildId: str
    workflow: str
    status: str
    details: dict

# ============================================================================
# IN-MEMORY STORAGE (YOLO)
# ============================================================================

messages: List[ChatMessage] = []
active_connections: List[WebSocket] = []

# ============================================================================
# WEBSOCKET
# ============================================================================

@app.websocket("/ws")
async def websocket_endpoint(websocket: WebSocket):
    await websocket.accept()
    active_connections.append(websocket)
    print(f"💬 Cliente conectado. Total: {len(active_connections)}")

    try:
        while True:
            data = await websocket.receive_text()
            message_data = json.loads(data)

            if message_data.get("type") == "send-message":
                content = message_data.get("content", "")

                # Salvar mensagem do usuário
                user_msg = ChatMessage(
                    id=f"msg-{uuid.uuid4()}",
                    sender="user",
                    content=content,
                    timestamp=datetime.now().isoformat(),
                    read=True,
                    type="text"
                )
                messages.append(user_msg)

                # Broadcast para todos
                await broadcast(user_msg.dict())

                # Bot responde depois de 1s
                await asyncio.sleep(1)
                await handle_bot_response(content)

    except WebSocketDisconnect:
        active_connections.remove(websocket)
        print(f"💬 Cliente desconectado. Total: {len(active_connections)}")

async def broadcast(message: dict):
    """Enviar mensagem para todos os clientes conectados"""
    disconnected = []
    for connection in active_connections:
        try:
            await connection.send_json({"type": "new-message", "message": message})
        except:
            disconnected.append(connection)

    for conn in disconnected:
        active_connections.remove(conn)

# ============================================================================
# BOT RESPONSES
# ============================================================================

async def handle_bot_response(user_message: str):
    """Bot responde automaticamente baseado na mensagem"""
    msg_lower = user_message.lower()

    if "build" in msg_lower or "status" in msg_lower:
        response = """📊 **Status de Builds**

✅ CI/CD Pipeline (main): SUCCESS
⏱️ Duração: 2m 15s
🧪 Testes: 50/50 passed
📈 Coverage: 85%

✅ Release (v1.0.0): SUCCESS
⏱️ Publicado no crates.io

⚠️ Deploy (develop): RUNNING...
⏳ Tempo decorrido: 1m 30s"""
        msg_type = "notification"

    elif "erro" in msg_lower or "fail" in msg_lower:
        response = """❌ **Erros Detectados**

🔴 Deploy (feature/new-api): FAILED

Erro:
```
Test failed: authentication_test
Timeout after 5000ms
```

Arquivo: src/auth.rs:145

Solução sugerida:
Ajuste o timeout ou revise a lógica de autenticação."""
        msg_type = "alert"

    elif "sucesso" in msg_lower or "success" in msg_lower:
        response = """✅ **Todos os Builds Passaram!**

🏆 Taxa de Sucesso: 96.67%
📈 Tendência: +2.3% desde ontem

🚀 Últimas releases:
• v2.1.0 - Released 2h ago
• v2.0.9 - Released 1d ago
• v2.0.8 - Released 2d ago

Parabéns! 🎉"""
        msg_type = "success"

    elif "help" in msg_lower or "ajuda" in msg_lower or "?" in user_message:
        response = """📚 Aqui estão os comandos disponíveis:

• **build status** - Ver status dos builds
• **erros** - Listar erros recentes
• **success** - Mostrar builds bem-sucedidos
• **deploy** - Status de deployments
• **metrics** - Métricas gerais"""
        msg_type = "text"

    else:
        response = """👋 Oi! Sou o VIZZIO Bot. Posso ajudar com:

🔍 **build status** - Ver status dos builds
⚠️ **erros** - Listar erros recentes
✅ **success** - Mostrar builds bem-sucedidos
📊 **metrics** - Métricas gerais
🚀 **deploy** - Status de deployments

Tente: "build status", "erros", "success" """
        msg_type = "text"

    bot_msg = ChatMessage(
        id=f"msg-{uuid.uuid4()}",
        sender="bot",
        content=response,
        timestamp=datetime.now().isoformat(),
        read=False,
        status="delivered",
        type=msg_type
    )

    messages.append(bot_msg)
    await broadcast(bot_msg.dict())

# ============================================================================
# API ROUTES
# ============================================================================

@app.post("/api/chat/notify")
async def notify_build(notification: BuildNotification):
    """Recebe notificação de build e envia para o chat"""

    emoji = {
        "success": "✅",
        "failure": "❌",
        "running": "🔄",
        "cancelled": "⚠️"
    }.get(notification.status, "📢")

    content = f"""{emoji} **{notification.workflow}** - {notification.status.upper()}

📋 Build ID: {notification.buildId}
⏱️ Duração: {notification.details.get('duration', 'N/A')}s
🧪 Testes: {notification.details.get('testsPassed', 0)}/{notification.details.get('testsRun', 0)} passed
📈 Coverage: {notification.details.get('coverage', 0)}%"""

    msg_type = {
        "failure": "alert",
        "success": "success",
    }.get(notification.status, "notification")

    bot_msg = ChatMessage(
        id=f"notif-{uuid.uuid4()}",
        sender="bot",
        content=content,
        timestamp=datetime.now().isoformat(),
        read=False,
        status="delivered",
        type=msg_type,
        metadata={
            "buildId": notification.buildId,
            "workflow": notification.workflow
        }
    )

    messages.append(bot_msg)
    await broadcast(bot_msg.dict())

    return {"success": True}

@app.get("/api/chat/messages")
async def get_messages(limit: int = 50):
    """Retorna últimas mensagens"""
    return messages[-limit:]

@app.get("/api/chat/unread")
async def get_unread():
    """Conta mensagens não lidas"""
    unread = sum(1 for msg in messages if not msg.read and msg.sender == "bot")
    return {"unread": unread}

# ============================================================================
# SERVE HTML
# ============================================================================

@app.get("/chat", response_class=HTMLResponse)
async def serve_chat():
    """Serve o HTML do chat"""
    with open("chat.html", "r", encoding="utf-8") as f:
        return f.read()

@app.get("/")
async def root():
    return {
        "status": "online",
        "service": "VIZZIO ChatBot",
        "endpoints": {
            "chat": "/chat",
            "websocket": "/ws",
            "notify": "/api/chat/notify",
            "messages": "/api/chat/messages",
            "unread": "/api/chat/unread"
        }
    }

# ============================================================================
# STARTUP
# ============================================================================

if __name__ == "__main__":
    import uvicorn
    print("\n💬 VIZZIO ChatBot Python")
    print("🚀 http://localhost:3001/chat")
    print("📡 WebSocket: ws://localhost:3001/ws\n")

    uvicorn.run(
        app,
        host="0.0.0.0",
        port=3001,
        log_level="info"
    )
