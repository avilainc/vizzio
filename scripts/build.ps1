# build.ps1 - Script unificado de build para VIZZIO (Windows PowerShell)
# Coordena compilação de Rust + Node.js

param(
    [Parameter(Position = 0)]
    [string]$Command = "all"
)

$ErrorActionPreference = "Stop"

Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Blue
Write-Host "🚀 VIZZIO - Build Sistema Unificado (Windows)" -ForegroundColor Blue
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Blue
Write-Host ""

# Cores
function Print-Step {
    param([string]$Message)
    Write-Host "→ $Message" -ForegroundColor Cyan
}

function Print-Success {
    param([string]$Message)
    Write-Host "✓ $Message" -ForegroundColor Green
}

function Print-Error {
    param([string]$Message)
    Write-Host "✗ $Message" -ForegroundColor Red
}

# Verificar pré-requisitos
function Check-Requirements {
    Print-Step "Verificando pré-requisitos..."

    # Rust
    $rustVersion = & cargo --version 2>$null
    if ($LASTEXITCODE -ne 0) {
        Print-Error "Rust não está instalado. Instale em https://rustup.rs"
        exit 1
    }
    Print-Success "Rust encontrado ($rustVersion)"

    # Node.js
    $nodeVersion = & node --version 2>$null
    if ($LASTEXITCODE -ne 0) {
        Print-Error "Node.js não está instalado"
        exit 1
    }
    Print-Success "Node.js encontrado ($nodeVersion)"

    # npm
    $npmVersion = & npm --version 2>$null
    if ($LASTEXITCODE -ne 0) {
        Print-Error "npm não está instalado"
        exit 1
    }
    Print-Success "npm encontrado ($npmVersion)"

    Write-Host ""
}

# Build Rust
function Build-Rust {
    Print-Step "Compilando Rust workspace (Avila + Avx)..."
    Write-Host "  Isso pode levar 10-30 minutos na primeira vez..."
    Write-Host ""

    & cargo build --workspace --release --jobs 4 --quiet
    if ($LASTEXITCODE -ne 0) {
        Print-Error "Falha na compilação Rust"
        exit 1
    }

    Print-Success "Rust compilation concluída"
    Write-Host ""
}

# Build Node
function Build-Node {
    Print-Step "Compilando Node.js packages..."

    & npm install
    if ($LASTEXITCODE -ne 0) {
        Print-Error "npm install falhou"
        exit 1
    }

    & npm run build --workspaces
    if ($LASTEXITCODE -ne 0) {
        Print-Error "npm build falhou"
        exit 1
    }

    Print-Success "Node.js compilation concluída"
    Write-Host ""
}

# Test Rust
function Test-Rust {
    Print-Step "Testando Rust crates..."
    & cargo test --workspace --quiet
    if ($LASTEXITCODE -ne 0) {
        Print-Error "Rust tests falharam"
        exit 1
    }
    Print-Success "Rust tests passaram"
    Write-Host ""
}

# Test Node
function Test-Node {
    Print-Step "Testando Node.js packages..."
    & npm run test --workspaces
    if ($LASTEXITCODE -ne 0) {
        Print-Error "Node.js tests falharam"
        exit 1
    }
    Print-Success "Node.js tests passaram"
    Write-Host ""
}

# Lint
function Lint-All {
    Print-Step "Linting Rust..."
    & cargo clippy --workspace --quiet -- -D warnings
    if ($LASTEXITCODE -ne 0) {
        Print-Error "Rust clippy falhou"
        exit 1
    }
    Print-Success "Rust lint OK"

    Print-Step "Linting Node.js..."
    & npm run lint --workspaces
    if ($LASTEXITCODE -ne 0) {
        Print-Error "Node.js lint falhou"
        exit 1
    }
    Print-Success "Node.js lint OK"
    Write-Host ""
}

# Main
switch ($Command) {
    "check" {
        Check-Requirements
    }
    "rust" {
        Check-Requirements
        Build-Rust
    }
    "node" {
        Check-Requirements
        Build-Node
    }
    "all" {
        Check-Requirements
        Build-Rust
        Build-Node
        Print-Success "Build completo concluído!"
    }
    "test" {
        Check-Requirements
        Test-Rust
        Test-Node
        Print-Success "Todos os testes passaram!"
    }
    "lint" {
        Check-Requirements
        Lint-All
        Print-Success "Linting concluído!"
    }
    "clean" {
        Print-Step "Limpando..."
        & cargo clean
        & npm run clean
        Print-Success "Limpeza concluída"
    }
    "help" {
        Write-Host "Uso: .\build.ps1 [comando]"
        Write-Host ""
        Write-Host "Comandos:"
        Write-Host "  check   - Verifica pré-requisitos"
        Write-Host "  rust    - Compila apenas Rust"
        Write-Host "  node    - Compila apenas Node.js"
        Write-Host "  all     - Compila tudo (padrão)"
        Write-Host "  test    - Executa testes"
        Write-Host "  lint    - Executa linting"
        Write-Host "  clean   - Limpa build"
        Write-Host "  help    - Mostra esta mensagem"
    }
    default {
        Print-Error "Comando desconhecido: $Command"
        Write-Host "Use '.\build.ps1 help' para ajuda"
        exit 1
    }
}
