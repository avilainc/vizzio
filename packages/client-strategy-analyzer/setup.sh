#!/bin/bash

# Script de inicialização rápida

echo "╔════════════════════════════════════════════╗"
echo "║  Client Strategy Analyzer - Startup       ║"
echo "╚════════════════════════════════════════════╝"
echo ""

# Cores
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Verificar Node.js
echo -e "${BLUE}Verificando Node.js...${NC}"
if ! command -v node &> /dev/null; then
    echo "Node.js não está instalado!"
    exit 1
fi
echo -e "${GREEN}✓ Node.js $(node --version)${NC}"

# Verificar MongoDB
echo -e "${BLUE}Verificando MongoDB...${NC}"
if ! command -v mongod &> /dev/null; then
    echo "⚠ MongoDB não está instalado. Usando MongoDB Cloud?"
fi

# Backend
echo ""
echo -e "${BLUE}Instalando Backend...${NC}"
cd backend
npm install
cp .env.example .env 2>/dev/null || true
echo -e "${GREEN}✓ Backend instalado${NC}"

# Frontend
echo ""
echo -e "${BLUE}Instalando Frontend...${NC}"
cd ../frontend
npm install
echo -e "${GREEN}✓ Frontend instalado${NC}"

# Instruções finais
echo ""
echo "╔════════════════════════════════════════════╗"
echo "║  Instalação Completa!                     ║"
echo "╚════════════════════════════════════════════╝"
echo ""
echo "Próximos passos:"
echo ""
echo -e "${BLUE}1. Configure variáveis de ambiente:${NC}"
echo "   - backend/.env (MongoDB, OpenAI, Email)"
echo ""
echo -e "${BLUE}2. Inicie o Backend:${NC}"
echo "   cd backend && npm run dev"
echo ""
echo -e "${BLUE}3. Em outro terminal, inicie o Frontend:${NC}"
echo "   cd frontend && npm run dev"
echo ""
echo -e "${BLUE}4. Acesse o Dashboard:${NC}"
echo "   http://localhost:3001"
echo ""
echo "Documentação:"
echo "   - QUICKSTART.md     (este arquivo)"
echo "   - INSTALLATION.md   (instalação detalhada)"
echo "   - ARCHITECTURE.md   (arquitetura do sistema)"
echo ""
