# Task Management Script
# Helper para gerenciar TODOs do projeto

# Buscar todos os TODOs no código
function Find-AllTodos {
    Write-Host "🔍 Buscando TODOs no código..." -ForegroundColor Cyan
    Get-ChildItem -Path "src" -Recurse -Filter "*.rs" |
        Select-String -Pattern "TODO|FIXME|XXX|HACK" |
        ForEach-Object {
            Write-Host "$($_.Filename):$($_.LineNumber): " -NoNewline -ForegroundColor Yellow
            Write-Host $_.Line.Trim()
        }
}

# Contar TODOs por arquivo
function Count-Todos {
    Write-Host "📊 Contagem de TODOs por arquivo:" -ForegroundColor Cyan
    Get-ChildItem -Path "src" -Recurse -Filter "*.rs" | ForEach-Object {
        $count = (Select-String -Path $_.FullName -Pattern "TODO" | Measure-Object).Count
        if ($count -gt 0) {
            Write-Host "  $($_.Name): $count TODOs" -ForegroundColor Yellow
        }
    }
}

# Listar próximas tarefas prioritárias do TODO.md
function Show-NextTasks {
    Write-Host "🎯 Próximas Tarefas (Alta Prioridade):" -ForegroundColor Green
    Write-Host ""
    Write-Host "1. Serde Integration (src/serde.rs)" -ForegroundColor Yellow
    Write-Host "   - Implementar serialização real"
    Write-Host ""
    Write-Host "2. Thread Safety (src/concurrent.rs)" -ForegroundColor Yellow
    Write-Host "   - Substituir RefCell por Mutex"
    Write-Host ""
    Write-Host "3. Real Timestamp (src/ttl.rs)" -ForegroundColor Yellow
    Write-Host "   - Implementar time source real"
    Write-Host ""
    Write-Host "Para detalhes completos, veja: TODO.md"
}

# Rodar testes
function Run-Tests {
    Write-Host "🧪 Rodando testes..." -ForegroundColor Cyan
    cargo test
}

# Verificar formatação
function Check-Format {
    Write-Host "✨ Verificando formatação..." -ForegroundColor Cyan
    cargo fmt --check
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Código está formatado!" -ForegroundColor Green
    } else {
        Write-Host "❌ Execute 'cargo fmt' para formatar" -ForegroundColor Red
    }
}

# Rodar clippy
function Run-Clippy {
    Write-Host "🔍 Rodando clippy..." -ForegroundColor Cyan
    cargo clippy -- -D warnings
}

# Checklist completo antes de PR
function Pre-PR-Check {
    Write-Host "📋 Checklist Pre-PR" -ForegroundColor Magenta
    Write-Host "==================" -ForegroundColor Magenta
    Write-Host ""

    Write-Host "1/4 Compilação..." -ForegroundColor Yellow
    cargo build
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Falha na compilação" -ForegroundColor Red
        return
    }
    Write-Host "✅ Compilação OK" -ForegroundColor Green
    Write-Host ""

    Write-Host "2/4 Testes..." -ForegroundColor Yellow
    cargo test
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Testes falharam" -ForegroundColor Red
        return
    }
    Write-Host "✅ Testes OK" -ForegroundColor Green
    Write-Host ""

    Write-Host "3/4 Formatação..." -ForegroundColor Yellow
    cargo fmt --check
    if ($LASTEXITCODE -ne 0) {
        Write-Host "⚠️ Execute 'cargo fmt' para formatar" -ForegroundColor Yellow
    } else {
        Write-Host "✅ Formatação OK" -ForegroundColor Green
    }
    Write-Host ""

    Write-Host "4/4 Clippy..." -ForegroundColor Yellow
    cargo clippy -- -D warnings
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Clippy encontrou problemas" -ForegroundColor Red
        return
    }
    Write-Host "✅ Clippy OK" -ForegroundColor Green
    Write-Host ""

    Write-Host "🎉 Tudo pronto para PR!" -ForegroundColor Green
}

# Estatísticas do projeto
function Show-Stats {
    Write-Host "📊 Estatísticas do Projeto" -ForegroundColor Cyan
    Write-Host "=========================" -ForegroundColor Cyan
    Write-Host ""

    $rsFiles = Get-ChildItem -Path "src" -Recurse -Filter "*.rs"
    $totalLines = ($rsFiles | Get-Content | Measure-Object -Line).Lines
    $testLines = ($rsFiles | Select-String -Pattern "#\[test\]" | Measure-Object).Count

    Write-Host "📁 Arquivos Rust: $($rsFiles.Count)"
    Write-Host "📝 Linhas totais: $totalLines"
    Write-Host "🧪 Testes: $testLines"
    Write-Host ""

    Write-Host "Módulos implementados:" -ForegroundColor Yellow
    Get-ChildItem -Path "src" -Filter "*.rs" | ForEach-Object {
        Write-Host "  - $($_.BaseName)" -ForegroundColor Gray
    }
}

# Help
function Show-Help {
    Write-Host "🛠️  Avila Cache - Task Manager" -ForegroundColor Cyan
    Write-Host "==============================" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Comandos disponíveis:" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "  Find-AllTodos      - Buscar todos os TODOs no código"
    Write-Host "  Count-Todos        - Contar TODOs por arquivo"
    Write-Host "  Show-NextTasks     - Mostrar próximas tarefas prioritárias"
    Write-Host "  Run-Tests          - Rodar testes"
    Write-Host "  Check-Format       - Verificar formatação"
    Write-Host "  Run-Clippy         - Rodar clippy"
    Write-Host "  Pre-PR-Check       - Checklist completo antes de PR"
    Write-Host "  Show-Stats         - Estatísticas do projeto"
    Write-Host "  Show-Help          - Mostrar esta ajuda"
    Write-Host ""
    Write-Host "Exemplos:" -ForegroundColor Yellow
    Write-Host "  PS> Find-AllTodos"
    Write-Host "  PS> Pre-PR-Check"
    Write-Host "  PS> Show-NextTasks"
}

# Exportar funções
Export-ModuleMember -Function @(
    'Find-AllTodos',
    'Count-Todos',
    'Show-NextTasks',
    'Run-Tests',
    'Check-Format',
    'Run-Clippy',
    'Pre-PR-Check',
    'Show-Stats',
    'Show-Help'
)

# Mostrar help por padrão
Write-Host ""
Show-Help
