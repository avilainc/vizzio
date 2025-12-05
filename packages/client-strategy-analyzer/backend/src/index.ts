import express, { Express } from 'express';
import cors from 'cors';
import dotenv from 'dotenv';
import mongoose from 'mongoose';
import casesRoutes from './routes/cases';

dotenv.config();

const app: Express = express();
const port = process.env.PORT || 3000;

// Middlewares
app.use(cors());
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

// Conectar ao MongoDB
const connectDatabase = async () => {
  try {
    await mongoose.connect(process.env.MONGODB_URI!);
    console.log('✓ Conectado ao MongoDB com sucesso');
  } catch (error) {
    console.error('✗ Erro ao conectar ao MongoDB:', error);
    process.exit(1);
  }
};

// Rotas
app.use('/api', casesRoutes);

// Health check
app.get('/health', (req, res) => {
  res.json({
    status: 'ok',
    timestamp: new Date().toISOString(),
  });
});

// Iniciar servidor
const startServer = async () => {
  await connectDatabase();

  app.listen(port, () => {
    console.log(`
╔════════════════════════════════════════════╗
║  Client Strategy Analyzer Backend          ║
║  Servidor rodando em: http://localhost:${port}  ║
╚════════════════════════════════════════════╝
    `);
  });
};

startServer();
