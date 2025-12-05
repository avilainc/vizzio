# Script de Setup Automático - Projeto Avila
# Execução: .\setup-workspace.ps1

$ErrorActionPreference = "Stop"

Write-Host "═══════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "    🚀 AVILA WORKSPACE SETUP - Automação v1.0" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""

# ============================================================================
# CONFIGURAÇÃO
# ============================================================================

$WORKSPACE_ROOT = "d:\Vizzio\packages\avila"
$BACKUP_DIR = "$WORKSPACE_ROOT\backup-$(Get-Date -Format 'yyyyMMdd-HHmmss')"

# Lista de todos os 107 crates
$CRATES = @(
    "avila-dataframe",
    "aviladb",
    "avila-db",
    "aviladb-core",
    "avila-distributed-system",
    "avila-dns",
    "avila-election",
    "avila-error",
    "avila-errors-derive",
    "avila-fft",
    "avila-finite-fields",
    "avila-framework",
    "avila-frontend",
    "avila-future",
    "avila-geo",
    "avila-geo-workspace",
    "avila-gis-desktop",
    "avila-gltf",
    "avila-gossip",
    "avila-grpc",
    "avila-hash",
    "avila-http",
    "avila-id",
    "avila-ifc",
    "avila-image",
    "avila-jwt",
    "avila-kdf",
    "avila-lease",
    "avila-linalg",
    "avila-loadbalancer",
    "avila-location",
    "avila-lock",
    "avila-log",
    "avila-logger",
    "avila-mac",
    "avila-math",
    "avila-mesh",
    "avila-meta",
    "avila-metadata-extractor",
    "avila-metrics",
    "avila-ml",
    "avila-modular",
    "avila-molecule",
    "avila-monitor",
    "avila-mpc",
    "avila-ndarray",
    "avila-nucleus",
    "avila-numeric",
    "avila-oauth",
    "avila-onion-routing",
    "avila-optimizer",
    "avila-orchestrator",
    "avila-organ",
    "avila-organism",
    "avila-parallel",
    "avila-partition",
    "avila-pki",
    "avila-pool",
    "avila-post-quantum",
    "avila-prime",
    "avila-primitives",
    "avila-proxy",
    "avila-quantum",
    "avila-quic",
    "avila-quinn",
    "avila-raft",
    "avila-rand",
    "avila-random",
    "avila-rand-simple",
    "avila-rayon-simple",
    "avila-reduction",
    "avila-regex",
    "avila-replication",
    "avila-serde",
    "avila-serialize",
    "avila-serialize-derive",
    "avila-service-mesh",
    "avila-shard",
    "avila-signature",
    "avila-stealth",
    "avila-sync",
    "avila-tcp",
    "avila-telemetry",
    "avila-term",
    "avila-terminal",
    "avila-tesselation",
    "avila-threshold",
    "avila-time",
    "avila-tissue",
    "avila-tls",
    "avila-tokenizer",
    "avila-tracing",
    "avila-udp",
    "avila-validate",
    "avila-vec3d",
    "avila-vizzio",
    "avila-web",
    "avila-webframework",
    "avila-websocket",
    "avila-workflow",
    "avila-zkp"
)

# Crates deprecated
$DEPRECATED = @(
    "avila-error-old",
    "avila-serde-old"
)

# ============================================================================
# FUNÇÕES AUXILIARES
# ============================================================================

function Write-Step {
    param([string]$Message, [string]$Color = "Yellow")
    Write-Host ""
    Write-Host "▶ $Message" -ForegroundColor $Color
    Write-Host "─────────────────────────────────────────────────────" -ForegroundColor DarkGray
}

function Write-Success {
    param([string]$Message)
    Write-Host "✓ $Message" -ForegroundColor Green
}

function Write-Error {
    param([string]$Message)
    Write-Host "✗ $Message" -ForegroundColor Red
}

function Write-Warning {
    param([string]$Message)
    Write-Host "⚠ $Message" -ForegroundColor Yellow
}

function Test-Command {
    param([string]$Command)
    try {
        Get-Command $Command -ErrorAction Stop | Out-Null
        return $true
    } catch {
        return $false
    }
}

# ============================================================================
# ETAPA 1: PRÉ-REQUISITOS
# ============================================================================

Write-Step "Verificando pré-requisitos..." "Cyan"

# Verificar Rust
if (Test-Command "cargo") {
    $rustVersion = (cargo --version)
    Write-Success "Rust instalado: $rustVersion"
} else {
    Write-Error "Rust não encontrado! Instale via https://rustup.rs"
    exit 1
}

# Verificar Git
if (Test-Command "git") {
    $gitVersion = (git --version)
    Write-Success "Git instalado: $gitVersion"
} else {
    Write-Warning "Git não encontrado. Recomendado para controle de versão."
}

# Verificar diretório
if (Test-Path $WORKSPACE_ROOT) {
    Write-Success "Workspace root encontrado: $WORKSPACE_ROOT"
} else {
    Write-Error "Workspace root não encontrado: $WORKSPACE_ROOT"
    exit 1
}

# ============================================================================
# ETAPA 2: BACKUP
# ============================================================================

Write-Step "Criando backup..." "Cyan"

try {
    New-Item -ItemType Directory -Path $BACKUP_DIR -Force | Out-Null

    # Backup apenas de Cargo.toml se existir
    if (Test-Path "$WORKSPACE_ROOT\Cargo.toml") {
        Copy-Item "$WORKSPACE_ROOT\Cargo.toml" "$BACKUP_DIR\Cargo.toml.bak"
        Write-Success "Backup criado em: $BACKUP_DIR"
    } else {
        Write-Warning "Nenhum Cargo.toml encontrado para backup"
    }
} catch {
    Write-Error "Falha ao criar backup: $_"
    exit 1
}

# ============================================================================
# ETAPA 3: CRIAR WORKSPACE CARGO.TOML
# ============================================================================

Write-Step "Criando Cargo.toml workspace..." "Cyan"

# Verificar se Cargo.toml.example existe
if (-not (Test-Path "$WORKSPACE_ROOT\Cargo.toml.example")) {
    Write-Error "Cargo.toml.example não encontrado!"
    Write-Host "Execute primeiro os comandos de criação dos arquivos de documentação."
    exit 1
}

# Copiar exemplo para Cargo.toml
Copy-Item "$WORKSPACE_ROOT\Cargo.toml.example" "$WORKSPACE_ROOT\Cargo.toml" -Force
Write-Success "Cargo.toml criado a partir do exemplo"

# ============================================================================
# ETAPA 4: VALIDAR ESTRUTURA DE CRATES
# ============================================================================

Write-Step "Validando estrutura de crates..." "Cyan"

$missingCrates = @()
$foundCrates = 0

foreach ($crate in $CRATES) {
    $cratePath = Join-Path $WORKSPACE_ROOT $crate
    if (Test-Path $cratePath) {
        $foundCrates++
        # Verificar se tem Cargo.toml
        if (-not (Test-Path "$cratePath\Cargo.toml")) {
            Write-Warning "Crate $crate existe mas sem Cargo.toml"
        }
    } else {
        $missingCrates += $crate
    }
}

Write-Host ""
Write-Host "Estatísticas:" -ForegroundColor Cyan
Write-Host "  Total esperado: $($CRATES.Count) crates"
Write-Host "  Encontrados: $foundCrates crates" -ForegroundColor Green
Write-Host "  Faltando: $($missingCrates.Count) crates" -ForegroundColor $(if ($missingCrates.Count -gt 0) { "Yellow" } else { "Green" })

if ($missingCrates.Count -gt 0) {
    Write-Warning "Crates faltando:"
    $missingCrates | ForEach-Object { Write-Host "    - $_" }
}

# ============================================================================
# ETAPA 5: VALIDAR BUILD
# ============================================================================

Write-Step "Testando build do workspace..." "Cyan"
Write-Host "Isto pode levar alguns minutos..." -ForegroundColor Gray

Push-Location $WORKSPACE_ROOT

try {
    Write-Host ""
    Write-Host "Executando: cargo check --workspace" -ForegroundColor Gray

    $checkOutput = cargo check --workspace 2>&1

    if ($LASTEXITCODE -eq 0) {
        Write-Success "Build bem-sucedido!"
    } else {
        Write-Error "Build falhou. Veja erros acima."
        Write-Warning "Possíveis causas:"
        Write-Host "  1. Dependências faltando em algum Cargo.toml"
        Write-Host "  2. Crates com código incompleto"
        Write-Host "  3. Paths incorretos no workspace"
    }
} catch {
    Write-Error "Erro ao executar cargo check: $_"
} finally {
    Pop-Location
}

# ============================================================================
# ETAPA 6: MARCAR DEPRECATED
# ============================================================================

Write-Step "Marcando crates deprecated..." "Cyan"

foreach ($crate in $DEPRECATED) {
    $cratePath = Join-Path $WORKSPACE_ROOT $crate
    if (Test-Path $cratePath) {
        $readmePath = Join-Path $cratePath "README_DEPRECATED.md"

        $deprecatedContent = @"
# ⚠️ DEPRECATED - $crate

Este crate foi marcado como **DEPRECATED** e não deve ser usado em novos projetos.

## Motivo
Funcionalidade consolidada em outro crate mais recente.

## Alternativa Recomendada
Veja o workspace principal para o crate substituto.

## Status
- **Manutenção:** Apenas bug fixes críticos
- **Novas features:** ❌ Não serão adicionadas
- **Documentação:** Mínima
- **Remoção prevista:** v2.0

## Migração
Consulte o guia de migração no README principal do workspace.

---
*Marcado como deprecated em: $(Get-Date -Format "yyyy-MM-dd")*
"@

        $deprecatedContent | Out-File -FilePath $readmePath -Encoding UTF8
        Write-Success "README_DEPRECATED.md criado em $crate"
    }
}

# ============================================================================
# ETAPA 7: CRIAR ESTRUTURA DE DIRETÓRIOS ADICIONAL
# ============================================================================

Write-Step "Criando estrutura de diretórios..." "Cyan"

$directories = @(
    ".github\workflows",
    "docs",
    "examples",
    "tests\integration",
    "benches",
    "tools",
    "scripts"
)

foreach ($dir in $directories) {
    $fullPath = Join-Path $WORKSPACE_ROOT $dir
    if (-not (Test-Path $fullPath)) {
        New-Item -ItemType Directory -Path $fullPath -Force | Out-Null
        Write-Success "Criado: $dir"
    }
}

# ============================================================================
# ETAPA 8: GERAR RELATÓRIO
# ============================================================================

Write-Step "Gerando relatório..." "Cyan"

$reportPath = Join-Path $WORKSPACE_ROOT "SETUP_REPORT.md"

$report = @"
# Relatório de Setup - Workspace Avila

**Data:** $(Get-Date -Format "yyyy-MM-dd HH:mm:ss")

## Resumo

- **Total de crates:** $($CRATES.Count)
- **Crates encontrados:** $foundCrates
- **Crates faltando:** $($missingCrates.Count)
- **Crates deprecated:** $($DEPRECATED.Count)

## Status do Build

$(if ($LASTEXITCODE -eq 0) { "✅ **BUILD SUCESSO**" } else { "❌ **BUILD FALHOU**" })

## Crates Faltando

$(if ($missingCrates.Count -gt 0) {
    $missingCrates | ForEach-Object { "- $_" }
} else {
    "Nenhum crate faltando ✅"
})

## Próximos Passos

1. [ ] Revisar erros de compilação (se houver)
2. [ ] Adicionar documentação (README.md principal)
3. [ ] Configurar CI/CD (.github/workflows/ci.yml)
4. [ ] Resolver TODOs críticos
5. [ ] Adicionar testes

## Backup

Backup criado em: `$BACKUP_DIR`

## Comandos Úteis

```powershell
# Build completo
cargo build --workspace

# Testes
cargo test --workspace

# Documentação
cargo doc --workspace --no-deps --open

# Clippy
cargo clippy --workspace --all-features

# Format
cargo fmt --all
```

---
*Gerado automaticamente por setup-workspace.ps1*
"@

$report | Out-File -FilePath $reportPath -Encoding UTF8
Write-Success "Relatório salvo em: SETUP_REPORT.md"

# ============================================================================
# ETAPA 9: PRÓXIMOS PASSOS
# ============================================================================

Write-Host ""
Write-Host "═══════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "    ✓ SETUP COMPLETO!" -ForegroundColor Green
Write-Host "═══════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""

Write-Host "📋 Próximos Passos:" -ForegroundColor Yellow
Write-Host ""
Write-Host "1. Revisar o relatório:" -ForegroundColor White
Write-Host "   cat .\SETUP_REPORT.md" -ForegroundColor Gray
Write-Host ""
Write-Host "2. Corrigir erros de build (se houver):" -ForegroundColor White
Write-Host "   cargo build --workspace" -ForegroundColor Gray
Write-Host ""
Write-Host "3. Rodar testes:" -ForegroundColor White
Write-Host "   cargo test --workspace" -ForegroundColor Gray
Write-Host ""
Write-Host "4. Gerar documentação:" -ForegroundColor White
Write-Host "   cargo doc --workspace --no-deps --open" -ForegroundColor Gray
Write-Host ""
Write-Host "5. Seguir o plano de ação:" -ForegroundColor White
Write-Host "   cat .\ACTION_PLAN_IMMEDIATE.md" -ForegroundColor Gray
Write-Host ""

Write-Host "📚 Documentação Criada:" -ForegroundColor Yellow
Write-Host "  - BLUEPRINT_AVILA_v1.0-v10.0.md" -ForegroundColor Cyan
Write-Host "  - EXECUTIVE_SUMMARY.md" -ForegroundColor Cyan
Write-Host "  - ACTION_PLAN_IMMEDIATE.md" -ForegroundColor Cyan
Write-Host "  - Cargo.toml (workspace)" -ForegroundColor Cyan
Write-Host "  - SETUP_REPORT.md" -ForegroundColor Cyan
Write-Host ""

if ($LASTEXITCODE -ne 0) {
    Write-Warning "Build teve problemas. Revise os erros acima e consulte SETUP_REPORT.md"
    Write-Host ""
    Write-Host "Dicas rápidas:" -ForegroundColor Yellow
    Write-Host "  - Compile crates individualmente para identificar problemas"
    Write-Host "  - Verifique dependências em cada Cargo.toml"
    Write-Host "  - Consulte os logs de erro detalhados"
} else {
    Write-Host "🎉 Tudo certo! Workspace configurado com sucesso!" -ForegroundColor Green
}

Write-Host ""
Write-Host "═══════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""
