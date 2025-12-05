Write-Host "╔════════════════════════════════════════════╗" -ForegroundColor Green
Write-Host "║  Client Strategy Analyzer - Startup       ║" -ForegroundColor Green
Write-Host "╚════════════════════════════════════════════╝" -ForegroundColor Green
Write-Host ""

# Verificar Node.js
Write-Host "Verificando Node.js..." -ForegroundColor Cyan
if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
    Write-Host "Node.js não está instalado!" -ForegroundColor Red
    exit 1
}
$nodeVersion = & node --version
Write-Host "✓ Node.js $nodeVersion" -ForegroundColor Green

# Backend
Write-Host ""
Write-Host "Instalando Backend..." -ForegroundColor Cyan
Set-Location backend
npm install
if (-not (Test-Path .env)) {
    Copy-Item .env.example .env -ErrorAction SilentlyContinue
    Write-Host "⚠ Arquivo .env criado. Edite com suas credenciais!" -ForegroundColor Yellow
}
Write-Host "✓ Backend instalado" -ForegroundColor Green

# Frontend
Write-Host ""
Write-Host "Instalando Frontend..." -ForegroundColor Cyan
Set-Location ../frontend
npm install
Write-Host "✓ Frontend instalado" -ForegroundColor Green

# Instruções finais
Write-Host ""
Write-Host "╔════════════════════════════════════════════╗" -ForegroundColor Green
Write-Host "║  Instalação Completa!                     ║" -ForegroundColor Green
Write-Host "╚════════════════════════════════════════════╝" -ForegroundColor Green
Write-Host ""
Write-Host "Próximos passos:" -ForegroundColor Green
Write-Host ""
Write-Host "1. Configure variáveis de ambiente:" -ForegroundColor Cyan
Write-Host "   - backend/.env (MongoDB, OpenAI, Email)" -ForegroundColor White
Write-Host ""
Write-Host "2. Inicie o Backend em um terminal:" -ForegroundColor Cyan
Write-Host "   cd backend`npm run dev" -ForegroundColor White
Write-Host ""
Write-Host "3. Em outro terminal, inicie o Frontend:" -ForegroundColor Cyan
Write-Host "   cd frontend`npm run dev" -ForegroundColor White
Write-Host ""
Write-Host "4. Acesse o Dashboard:" -ForegroundColor Cyan
Write-Host "   http://localhost:3001" -ForegroundColor White
Write-Host ""
Write-Host "Documentação:" -ForegroundColor Green
Write-Host "   - QUICKSTART.md     (início rápido)" -ForegroundColor White
Write-Host "   - INSTALLATION.md   (instalação detalhada)" -ForegroundColor White
Write-Host "   - ARCHITECTURE.md   (arquitetura do sistema)" -ForegroundColor White
Write-Host ""
