# Script PowerShell para instalar modelos no Ollama
# Autor: Avila Inc.

Write-Host "🚀 Instalando modelos no Ollama..." -ForegroundColor Cyan
Write-Host ""

# Verificar se Ollama está rodando
try {
    $null = Invoke-RestMethod -Uri "http://localhost:11434/api/tags" -ErrorAction Stop
    Write-Host "✅ Ollama está rodando" -ForegroundColor Green
} catch {
    Write-Host "⚠️ Ollama não está rodando. Iniciando..." -ForegroundColor Yellow
    Start-Process "ollama" -ArgumentList "serve" -WindowStyle Hidden
    Start-Sleep -Seconds 3
}

# Função para instalar modelo
function Install-Model {
    param([string]$ModelName, [string]$Description)

    Write-Host ""
    Write-Host "📦 Instalando $ModelName - $Description" -ForegroundColor Yellow

    try {
        ollama pull $ModelName
        Write-Host "✅ $ModelName instalado com sucesso!" -ForegroundColor Green
    } catch {
        Write-Host "❌ Erro ao instalar $ModelName : $_" -ForegroundColor Red
    }
}

# Instalar modelos
Install-Model "mistral" "Mistral 7B - Modelo base recomendado (4.1GB)"
Install-Model "dolphin-mistral" "Mistral sem censura - Ideal para pesquisa"
Install-Model "llama3.2" "Llama 3.2 3B - Rápido e eficiente (2GB)"
Install-Model "mistral-openorca" "Mistral OpenOrca - Otimizado para instruções"

# Modelos opcionais (comentados por padrão)
# Install-Model "wizard-vicuna-uncensored" "Wizard Vicuna - Sem filtros"
# Install-Model "neural-chat" "Neural Chat - Intel otimizado"
# Install-Model "codellama" "Code Llama - Especializado em código"

Write-Host ""
Write-Host "📊 Modelos instalados:" -ForegroundColor Cyan
ollama list

Write-Host ""
Write-Host "🎉 Instalação completa!" -ForegroundColor Green
Write-Host ""
Write-Host "📡 Próximos passos:" -ForegroundColor Yellow
Write-Host "1. Instalar dependências Python: pip install -r requirements.txt"
Write-Host "2. Iniciar servidor: python server.py"
Write-Host "3. Testar: curl http://localhost:8000/health"
Write-Host ""
Write-Host "💡 Para usar Mistral:" -ForegroundColor Cyan
Write-Host '   curl -X POST http://localhost:8000/v1/chat/completions \'
Write-Host '     -H "Content-Type: application/json" \'
Write-Host '     -d ''{"model":"mistral","messages":[{"role":"user","content":"Olá!"}]}'''
