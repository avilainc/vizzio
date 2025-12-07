/**
 * VIZZIO Build Monitor - Backend Server
 * Real-time monitoring com WebSocket e Email
 */

import express, { Request, Response } from 'express';
import { Server } from 'socket.io';
import cors from 'cors';
import helmet from 'helmet';
import nodemailer from 'nodemailer';
import { MongoClient, Db, Collection } from 'mongodb';
import dotenv from 'dotenv';

dotenv.config();

const app = express();
const PORT = process.env.PORT || 3000;

// Middleware
app.use(helmet());
app.use(cors());
app.use(express.json());

// MongoDB
let db: Db;
let buildsCollection: Collection;
let eventsCollection: Collection; // eslint-disable-line @typescript-eslint/no-unused-vars

// Email transporter
const transporter = nodemailer.createTransport({
  host: process.env.SMTP_HOST || 'smtp.gmail.com',
  port: parseInt(process.env.SMTP_PORT || '587'),
  secure: process.env.SMTP_SECURE === 'true',
  auth: {
    user: process.env.SMTP_USER,
    pass: process.env.SMTP_PASS,
  },
});

interface BuildEvent {
  _id?: string;
  buildId: string;
  workflow: string;
  status: 'running' | 'success' | 'failure' | 'cancelled';
  branch: string;
  commit: string;
  author: string;
  timestamp: Date;
  duration?: number;
  details: {
    testsRun?: number;
    testsPassed?: number;
    testsFailed?: number;
    coverage?: number;
    issues?: Array<{
      type: 'error' | 'warning';
      message: string;
      file?: string;
      line?: number;
    }>;
  };
  logs: Array<{
    timestamp: Date;
    level: 'info' | 'warn' | 'error';
    message: string;
    step?: string;
  }>;
}

// WebSocket
const server = require('http').createServer(app);
const io = new Server(server, {
  cors: { origin: '*' },
});

io.on('connection', (socket: any) => {
  console.log('✅ Cliente conectado:', socket.id);

  socket.on('disconnect', () => {
    console.log('❌ Cliente desconectado:', socket.id);
  });

  // Enviar builds recentes ao conectar
  socket.on('request-recent-builds', async () => {
    try {
      const builds = await buildsCollection
        .find({})
        .sort({ timestamp: -1 })
        .limit(50)
        .toArray();
      socket.emit('recent-builds', builds);
    } catch (error) {
      console.error('Erro ao buscar builds:', error);
    }
  });
});

// ============================================================================
// API ROUTES
// ============================================================================

/**
 * POST /api/builds/start
 * Inicia um novo build
 */
app.post('/api/builds/start', async (req: Request, res: Response) => {
  try {
    const {
      buildId,
      workflow,
      branch,
      commit,
      author,
      message,
    } = req.body;

    const buildEvent: BuildEvent = {
      buildId,
      workflow,
      status: 'running',
      branch,
      commit,
      author,
      timestamp: new Date(),
      details: {},
      logs: [
        {
          timestamp: new Date(),
          level: 'info',
          message: `Build iniciado: ${message || 'N/A'}`,
          step: 'initialization',
        },
      ],
    };

    await buildsCollection.insertOne(buildEvent);
    io.emit('build-started', buildEvent);

    res.json({ success: true, buildId });
  } catch (error) {
    res.status(500).json({ error: String(error) });
  }
});

/**
 * POST /api/builds/:buildId/log
 * Adiciona log a um build em andamento
 */
app.post('/api/builds/:buildId/log', async (req: Request, res: Response) => {
  try {
    const { buildId } = req.params;
    const { level, message, step } = req.body;

    const logEntry = {
      timestamp: new Date(),
      level,
      message,
      step,
    };

    await buildsCollection.updateOne(
      { buildId },
      { $push: { logs: logEntry } }
    );

    io.emit('build-log', { buildId, log: logEntry });
    res.json({ success: true });
  } catch (error) {
    res.status(500).json({ error: String(error) });
  }
});

/**
 * POST /api/builds/:buildId/complete
 * Completa um build
 */
app.post('/api/builds/:buildId/complete', async (req: Request, res: Response) => {
  try {
    const { buildId } = req.params;
    const { status, duration, details } = req.body;

    const completedTime = new Date();

    await buildsCollection.updateOne(
      { buildId },
      {
        $set: {
          status,
          duration,
          details,
          completedAt: completedTime,
        },
      }
    );

    const build = await buildsCollection.findOne({ buildId });

    // Enviar email
    if (build) {
      await sendBuildEmail(build as BuildEvent);
    }

    io.emit('build-completed', build);
    res.json({ success: true });
  } catch (error) {
    res.status(500).json({ error: String(error) });
  }
});

/**
 * GET /api/builds
 * Lista todos os builds
 */
app.get('/api/builds', async (req: Request, res: Response) => {
  try {
    const limit = parseInt(req.query.limit as string) || 50;
    const builds = await buildsCollection
      .find({})
      .sort({ timestamp: -1 })
      .limit(limit)
      .toArray();
    res.json(builds);
  } catch (error) {
    res.status(500).json({ error: String(error) });
  }
});

/**
 * GET /api/builds/:buildId
 * Detalhes de um build específico
 */
app.get('/api/builds/:buildId', async (req: Request, res: Response) => {
  try {
    const { buildId } = req.params;
    const build = await buildsCollection.findOne({ buildId });
    if (!build) return res.status(404).json({ error: 'Build not found' });
    res.json(build);
  } catch (error) {
    res.status(500).json({ error: String(error) });
  }
});

/**
 * GET /api/stats
 * Estatísticas gerais
 */
app.get('/api/stats', async (_req: Request, res: Response) => {
  try {
    const total = await buildsCollection.countDocuments();
    const successful = await buildsCollection.countDocuments({
      status: 'success',
    });
    const failed = await buildsCollection.countDocuments({
      status: 'failure',
    });

    const avgDuration = await buildsCollection
      .aggregate([
        { $match: { duration: { $exists: true } } },
        { $group: { _id: null, avg: { $avg: '$duration' } } },
      ])
      .toArray();

    res.json({
      total,
      successful,
      failed,
      successRate: ((successful / total) * 100).toFixed(2) + '%',
      avgDuration: avgDuration[0]?.avg || 0,
    });
  } catch (error) {
    res.status(500).json({ error: String(error) });
  }
});

// ============================================================================
// EMAIL UTILS
// ============================================================================

async function sendBuildEmail(build: BuildEvent) {
  const statusEmoji: Record<string, string> = {
    running: '🔄',
    success: '✅',
    failure: '❌',
    cancelled: '⚠️',
  };
  const emoji = statusEmoji[build.status] || '❓';

  const htmlContent = generateEmailHTML(build);

  try {
    await transporter.sendMail({
      from: process.env.SMTP_FROM || 'build-monitor@vizzio.dev',
      to: process.env.EMAIL_RECIPIENTS || 'nicolas@avila.inc',
      subject: `${emoji} VIZZIO Build ${build.status.toUpperCase()} - ${build.workflow}`,
      html: htmlContent,
    });

    console.log('📧 Email enviado com sucesso');
  } catch (error) {
    console.error('❌ Erro ao enviar email:', error);
  }
}

function generateEmailHTML(build: BuildEvent): string {
  const statusColor: Record<string, string> = {
    running: '#3b82f6',
    success: '#10b981',
    failure: '#ef4444',
    cancelled: '#f59e0b',
  };
  const color = statusColor[build.status] || '#6b7280';

  const statusText: Record<string, string> = {
    running: 'EM EXECUÇÃO 🔄',
    success: 'SUCESSO ✅',
    failure: 'FALHA ❌',
    cancelled: 'CANCELADO ⚠️',
  };
  const text = statusText[build.status] || 'DESCONHECIDO ❓';

  return `
<!DOCTYPE html>
<html lang="pt-BR">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <style>
    body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; color: #1f2937; }
    .container { max-width: 600px; margin: 0 auto; padding: 20px; }
    .header { background: ${color}; color: white; padding: 20px; border-radius: 8px; margin-bottom: 20px; }
    .header h1 { margin: 0; font-size: 24px; }
    .info { background: #f9fafb; padding: 15px; border-radius: 6px; margin: 10px 0; border-left: 4px solid ${color}; }
    .info-row { display: flex; justify-content: space-between; padding: 8px 0; }
    .label { font-weight: 600; color: #6b7280; }
    .value { color: #1f2937; }
    .stats { display: grid; grid-template-columns: repeat(4, 1fr); gap: 10px; margin: 15px 0; }
    .stat-box { background: white; padding: 12px; border-radius: 6px; border: 1px solid #e5e7eb; text-align: center; }
    .stat-number { font-size: 24px; font-weight: bold; color: ${color}; }
    .stat-label { font-size: 12px; color: #6b7280; margin-top: 5px; }
    .logs { background: #1f2937; color: #10b981; padding: 15px; border-radius: 6px; font-family: 'Courier New', monospace; font-size: 12px; max-height: 300px; overflow-y: auto; }
    .log-line { margin: 5px 0; }
    .error { color: #ef4444; }
    .warn { color: #f59e0b; }
    .footer { text-align: center; margin-top: 20px; color: #9ca3af; font-size: 12px; }
    a { color: ${color}; text-decoration: none; }
  </style>
</head>
<body>
  <div class="container">
    <div class="header">
      <h1>${text}</h1>
      <p style="margin: 10px 0 0 0;">${build.workflow} • ${build.branch}</p>
    </div>

    <div class="info">
      <div class="info-row">
        <span class="label">Workflow:</span>
        <span class="value">${build.workflow}</span>
      </div>
      <div class="info-row">
        <span class="label">Branch:</span>
        <span class="value">${build.branch}</span>
      </div>
      <div class="info-row">
        <span class="label">Commit:</span>
        <span class="value" style="font-family: monospace; font-size: 12px;">${build.commit.substring(0, 8)}</span>
      </div>
      <div class="info-row">
        <span class="label">Autor:</span>
        <span class="value">${build.author}</span>
      </div>
      <div class="info-row">
        <span class="label">Duração:</span>
        <span class="value">${build.duration ? formatDuration(build.duration) : 'N/A'}</span>
      </div>
      <div class="info-row">
        <span class="label">Data:</span>
        <span class="value">${new Date(build.timestamp).toLocaleString('pt-BR')}</span>
      </div>
    </div>

    ${
      build.details.testsRun
        ? `
    <div class="stats">
      <div class="stat-box">
        <div class="stat-number">${build.details.testsRun}</div>
        <div class="stat-label">Testes</div>
      </div>
      <div class="stat-box">
        <div class="stat-number" style="color: #10b981;">${build.details.testsPassed || 0}</div>
        <div class="stat-label">Passaram</div>
      </div>
      <div class="stat-box">
        <div class="stat-number" style="color: #ef4444;">${build.details.testsFailed || 0}</div>
        <div class="stat-label">Falharam</div>
      </div>
      <div class="stat-box">
        <div class="stat-number" style="color: #3b82f6;">${build.details.coverage || 0}%</div>
        <div class="stat-label">Coverage</div>
      </div>
    </div>
    `
        : ''
    }

    ${
      build.logs && build.logs.length > 0
        ? `
    <div style="margin: 20px 0;">
      <h3 style="margin: 0 0 10px 0;">Últimos Logs:</h3>
      <div class="logs">
        ${build.logs
          .slice(-20)
          .map(
            (log) =>
              `<div class="log-line ${log.level === 'error' ? 'error' : log.level === 'warn' ? 'warn' : ''}">
          [${new Date(log.timestamp).toLocaleTimeString()}] ${log.step ? `[${log.step}] ` : ''}${log.message}
        </div>`
          )
          .join('')}
      </div>
    </div>
    `
        : ''
    }

    <div class="footer">
      <p>VIZZIO Build Monitor • Monitoramento automático de builds</p>
      <p><a href="${process.env.DASHBOARD_URL || 'http://localhost:3000'}/builds/${build.buildId}">Ver detalhes completos no dashboard</a></p>
    </div>
  </div>
</body>
</html>
  `;
}

function formatDuration(seconds: number): string {
  const mins = Math.floor(seconds / 60);
  const secs = seconds % 60;
  if (mins > 0) return `${mins}m ${secs}s`;
  return `${secs}s`;
}

// ============================================================================
// STARTUP
// ============================================================================

async function start() {
  try {
    // Conectar MongoDB
    const client = new MongoClient(process.env.MONGODB_URI || 'mongodb://localhost:27017');
    await client.connect();
    db = client.db('vizzio-builds');
    buildsCollection = db.collection('builds');
    eventsCollection = db.collection('events');

    // Criar índices
    await buildsCollection.createIndex({ buildId: 1 }, { unique: true });
    await buildsCollection.createIndex({ timestamp: -1 });
    await buildsCollection.createIndex({ status: 1 });

    console.log('✅ MongoDB conectado');

    server.listen(PORT, () => {
      console.log(`\n🚀 VIZZIO Build Monitor rodando em http://localhost:${PORT}`);
      console.log(`📊 Dashboard: http://localhost:${PORT}/dashboard`);
      console.log(`📧 Emails: ${process.env.EMAIL_RECIPIENTS || 'nicolas@avila.inc'}`);
    });
  } catch (error) {
    console.error('❌ Erro ao iniciar:', error);
    process.exit(1);
  }
}

start();

export default app;
