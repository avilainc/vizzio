/**
 * VIZZIO ChatBot - WhatsApp Style Notifications
 * Backend com Socket.IO para notificações em tempo real
 */

import express from 'express';
import { Server } from 'socket.io';
import cors from 'cors';
import { MongoClient, Db, Collection } from 'mongodb';
import dotenv from 'dotenv';

dotenv.config();

const app = express();
const PORT = process.env.CHATBOT_PORT || 3001;

app.use(cors());
app.use(express.json());

let db: Db;
let messagesCollection: Collection;
let usersCollection: Collection;

interface ChatMessage {
  _id?: string;
  id: string;
  sender: 'bot' | 'user';
  content: string;
  timestamp: Date;
  read: boolean;
  status?: 'sending' | 'sent' | 'delivered' | 'read';
  type: 'text' | 'notification' | 'alert' | 'success';
  metadata?: {
    buildId?: string;
    workflow?: string;
    branch?: string;
    details?: any;
  };
}

interface User {
  _id?: string;
  email: string;
  name: string;
  avatar: string;
  lastSeen: Date;
  notifications: boolean;
}

const server = require('http').createServer(app);
const io = new Server(server, {
  cors: { origin: '*' },
  transports: ['websocket', 'polling'],
});

// ============================================================================
// WEBSOCKET
// ============================================================================

io.on('connection', (socket) => {
  console.log('💬 User connected:', socket.id);

  // Enviar mensagens recentes
  socket.on('load-chat', async () => {
    try {
      const messages = await messagesCollection
        .find({})
        .sort({ timestamp: -1 })
        .limit(50)
        .toArray();

      socket.emit('chat-history', messages.reverse());
    } catch (error) {
      console.error('Error loading chat:', error);
    }
  });

  // Receber mensagem do usuário
  socket.on('send-message', async (data) => {
    try {
      const message: ChatMessage = {
        id: `msg-${Date.now()}`,
        sender: 'user',
        content: data.content,
        timestamp: new Date(),
        read: true,
        status: 'sent',
        type: 'text',
      };

      await messagesCollection.insertOne(message);
      io.emit('new-message', message);

      // Resposta automática do bot
      setTimeout(() => {
        handleBotResponse(data.content);
      }, 1000);
    } catch (error) {
      console.error('Error sending message:', error);
    }
  });

  // Marcar mensagem como lida
  socket.on('mark-read', async (messageId) => {
    await messagesCollection.updateOne(
      { id: messageId },
      { $set: { read: true, status: 'read' } }
    );
  });

  socket.on('typing', () => {
    io.emit('bot-typing', true);
  });

  socket.on('disconnect', () => {
    console.log('💬 User disconnected:', socket.id);
  });
});

// ============================================================================
// HANDLE BOT RESPONSES
// ============================================================================

async function handleBotResponse(userMessage: string) {
  const message = userMessage.toLowerCase();
  let response = '';
  let type: 'text' | 'notification' | 'alert' | 'success' = 'text';

  if (message.includes('build') || message.includes('status')) {
    response = '🔍 Verificando status dos builds...';
    type = 'notification';

    // Simular resposta com dados
    setTimeout(async () => {
      const botMsg: ChatMessage = {
        id: `msg-${Date.now()}`,
        sender: 'bot',
        content: `📊 **Status de Builds**\n\n✅ CI/CD Pipeline (main): SUCCESS\n⏱️ Duração: 2m 15s\n🧪 Testes: 50/50 passed\n📈 Coverage: 85%\n\n✅ Release (v1.0.0): SUCCESS\n⏱️ Publicado no crates.io\n\n⚠️ Deploy (develop): RUNNING...\n⏳ Tempo decorrido: 1m 30s`,
        timestamp: new Date(),
        read: false,
        status: 'delivered',
        type: 'notification',
        metadata: {
          buildId: 'build-123',
          workflow: 'CI/CD Pipeline',
        },
      };

      await messagesCollection.insertOne(botMsg);
      io.emit('new-message', botMsg);
    }, 2000);
  } else if (message.includes('erro') || message.includes('fail')) {
    response = '⚠️ Procurando erros recentes...';
    type = 'alert';

    setTimeout(async () => {
      const botMsg: ChatMessage = {
        id: `msg-${Date.now()}`,
        sender: 'bot',
        content: `❌ **Erros Detectados**\n\n🔴 Deploy (feature/new-api): FAILED\n\nErro:\n\`\`\`\nTest failed: authentication_test\nTimeout after 5000ms\n\`\`\`\n\nArquivo: src/auth.rs:145\n\nSolução sugerida:\nAjuste o timeout para 10000ms ou revise a lógica de autenticação.`,
        timestamp: new Date(),
        read: false,
        status: 'delivered',
        type: 'alert',
      };

      await messagesCollection.insertOne(botMsg);
      io.emit('new-message', botMsg);
    }, 2000);
  } else if (message.includes('sucesso') || message.includes('success')) {
    response = '🎉 Buscando boas notícias...';
    type = 'success';

    setTimeout(async () => {
      const botMsg: ChatMessage = {
        id: `msg-${Date.now()}`,
        sender: 'bot',
        content: `✅ **Todos os Builds Passaram!**\n\n🏆 Taxa de Sucesso: 96.67%\n📈 Tendência: +2.3% desde ontem\n\n🚀 Últimas releases:\n• v2.1.0 - Released 2h ago\n• v2.0.9 - Released 1d ago\n• v2.0.8 - Released 2d ago\n\nParabéns! 🎉`,
        timestamp: new Date(),
        read: false,
        status: 'delivered',
        type: 'success',
      };

      await messagesCollection.insertOne(botMsg);
      io.emit('new-message', botMsg);
    }, 1500);
  } else if (
    message.includes('help') ||
    message.includes('ajuda') ||
    message.includes('?')
  ) {
    response = '📚 Aqui estão os comandos disponíveis:\n\n• "build status" - Ver status dos builds\n• "erros" - Listar erros recentes\n• "success" - Mostrar builds bem-sucedidos\n• "deploy" - Status de deployments\n• "metrics" - Métricas gerais';
    type = 'text';
  } else {
    response =
      '👋 Oi! Sou o VIZZIO Bot. Posso ajudar com:\n\n🔍 Status de builds\n⚠️ Erros e falhas\n✅ Sucessos\n📊 Métricas\n🚀 Deployments\n\nTente: "build status", "erros", "success"';
    type = 'text';
  }

  const botMsg: ChatMessage = {
    id: `msg-${Date.now()}`,
    sender: 'bot',
    content: response,
    timestamp: new Date(),
    read: false,
    status: 'delivered',
    type,
  };

  await messagesCollection.insertOne(botMsg);
  io.emit('new-message', botMsg);
}

// ============================================================================
// API ROUTES
// ============================================================================

app.post('/api/chat/notify', async (req, res) => {
  try {
    const { buildId, workflow, status, details } = req.body;

    const statusEmoji = {
      success: '✅',
      failure: '❌',
      running: '🔄',
      cancelled: '⚠️',
    }[status] || '📢';

    const message: ChatMessage = {
      id: `notif-${Date.now()}`,
      sender: 'bot',
      content: `${statusEmoji} **${workflow}** - ${status.toUpperCase()}\n\n📋 Build ID: ${buildId}\n⏱️ Duração: ${details.duration}s\n🧪 Testes: ${details.testsPassed}/${details.testsRun} passed\n📈 Coverage: ${details.coverage}%`,
      timestamp: new Date(),
      read: false,
      status: 'delivered',
      type: status === 'failure' ? 'alert' : status === 'success' ? 'success' : 'notification',
      metadata: {
        buildId,
        workflow,
        details,
      },
    };

    await messagesCollection.insertOne(message);
    io.emit('new-message', message);
    io.emit('notification-received', {
      icon: statusEmoji,
      title: `${workflow} - ${status.toUpperCase()}`,
      description: `Build ${buildId}: ${details.testsPassed}/${details.testsRun} tests passed`,
    });

    res.json({ success: true });
  } catch (error) {
    res.status(500).json({ error: String(error) });
  }
});

app.get('/api/chat/messages', async (req, res) => {
  try {
    const limit = parseInt(req.query.limit as string) || 50;
    const messages = await messagesCollection
      .find({})
      .sort({ timestamp: -1 })
      .limit(limit)
      .toArray();
    res.json(messages.reverse());
  } catch (error) {
    res.status(500).json({ error: String(error) });
  }
});

app.get('/api/chat/unread', async (req, res) => {
  try {
    const count = await messagesCollection.countDocuments({ read: false });
    res.json({ unread: count });
  } catch (error) {
    res.status(500).json({ error: String(error) });
  }
});

// ============================================================================
// STARTUP
// ============================================================================

async function start() {
  try {
    const client = new MongoClient(process.env.MONGODB_URI || 'mongodb://localhost:27017');
    await client.connect();
    db = client.db('vizzio-chat');
    messagesCollection = db.collection('messages');
    usersCollection = db.collection('users');

    await messagesCollection.createIndex({ timestamp: -1 });
    await messagesCollection.createIndex({ sender: 1 });

    console.log('✅ MongoDB conectado');

    server.listen(PORT, () => {
      console.log(`\n💬 VIZZIO ChatBot rodando em http://localhost:${PORT}`);
      console.log(`💬 Chat: http://localhost:${PORT}/chat`);
    });
  } catch (error) {
    console.error('❌ Erro:', error);
    process.exit(1);
  }
}

start();

export default app;
