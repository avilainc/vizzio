#!/usr/bin/env pwsh
# Script para testar notificação de email via avila-cell
# Usa credenciais do Porkbun SMTP

# Carregar variáveis do .env.local
if (Test-Path ".\.env.local") {
    Get-Content ".\.env.local" | Where-Object { $_ -notmatch "^#" -and $_.Trim() } | ForEach-Object {
        $key, $value = $_ -split "=", 2
        [Environment]::SetEnvironmentVariable($key.Trim(), $value.Trim())
    }
    Write-Host "✅ Variáveis de ambiente carregadas de .env.local" -ForegroundColor Green
}

# Verificar se variáveis estão definidas
$required = @("SMTP_HOST", "SMTP_PORT", "SMTP_USER", "SMTP_PASSWORD", "PARTNER_1_EMAIL", "PARTNER_1_NAME")
$missing = @()

foreach ($var in $required) {
    if (-not (Test-Path env:\$var)) {
        $missing += $var
    }
}

if ($missing.Count -gt 0) {
    Write-Host "❌ Variáveis de ambiente faltando: $($missing -join ', ')" -ForegroundColor Red
    exit 1
}

# Exibir configuração (sem expor senha)
Write-Host ""
Write-Host "📧 Configuração SMTP:" -ForegroundColor Cyan
Write-Host "   Host: $env:SMTP_HOST"
Write-Host "   Port: $env:SMTP_PORT"
Write-Host "   User: $env:SMTP_USER"
Write-Host "   Pass: ****** (escondida)"
Write-Host ""
Write-Host "👥 Sócios:" -ForegroundColor Cyan
Write-Host "   1. $env:PARTNER_1_NAME ($env:PARTNER_1_EMAIL)"
Write-Host "   2. $env:PARTNER_2_NAME ($env:PARTNER_2_EMAIL)"
Write-Host "   3. $env:PARTNER_3_NAME ($env:PARTNER_3_EMAIL)"
Write-Host ""

# Executar o exemplo
Write-Host "🚀 Executando exemplo de notificações..." -ForegroundColor Yellow
Write-Host ""

cd packages/avila/avila-cell
cargo run --example partner_notifications

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "✅ Notificações enviadas com sucesso!" -ForegroundColor Green
} else {
    Write-Host ""
    Write-Host "❌ Erro ao enviar notificações" -ForegroundColor Red
    exit 1
}
