#!/usr/bin/env bash
# build.sh - Script unificado de build para VIZZIO
# Coordena compilação de Rust + Node.js

set -e

echo "═══════════════════════════════════════════════════════════════"
echo "🚀 VIZZIO - Build Sistema Unificado"
echo "═══════════════════════════════════════════════════════════════"
echo ""

# Cores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Funções
print_step() {
    echo -e "${BLUE}→${NC} $1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

# Verificar pré-requisitos
check_requirements() {
    print_step "Verificando pré-requisitos..."

    # Rust
    if ! command -v cargo &> /dev/null; then
        print_error "Rust não está instalado. Instale em https://rustup.rs"
        exit 1
    fi
    print_success "Rust encontrado ($(cargo --version))"

    # Node.js
    if ! command -v node &> /dev/null; then
        print_error "Node.js não está instalado"
        exit 1
    fi
    print_success "Node.js encontrado ($(node --version))"

    # npm
    if ! command -v npm &> /dev/null; then
        print_error "npm não está instalado"
        exit 1
    fi
    print_success "npm encontrado ($(npm --version))"

    echo ""
}

# Build Rust
build_rust() {
    print_step "Compilando Rust workspace (Avila + Avx)..."
    echo "  Isso pode levar 10-30 minutos na primeira vez..."
    echo ""

    cargo build --workspace --release \
        --jobs 4 \
        --quiet

    print_success "Rust compilation concluída"
    echo ""
}

# Build Node
build_node() {
    print_step "Compilando Node.js packages..."

    npm install
    npm run build --workspaces

    print_success "Node.js compilation concluída"
    echo ""
}

# Test Rust
test_rust() {
    print_step "Testando Rust crates..."
    cargo test --workspace --quiet
    print_success "Rust tests passaram"
    echo ""
}

# Test Node
test_node() {
    print_step "Testando Node.js packages..."
    npm run test --workspaces
    print_success "Node.js tests passaram"
    echo ""
}

# Lint
lint_all() {
    print_step "Linting Rust..."
    cargo clippy --workspace --quiet -- -D warnings
    print_success "Rust lint OK"

    print_step "Linting Node.js..."
    npm run lint --workspaces
    print_success "Node.js lint OK"
    echo ""
}

# Main
main() {
    COMMAND=${1:-all}

    case $COMMAND in
        check)
            check_requirements
            ;;
        rust)
            check_requirements
            build_rust
            ;;
        node)
            check_requirements
            build_node
            ;;
        all)
            check_requirements
            build_rust
            build_node
            print_success "Build completo concluído!"
            ;;
        test)
            check_requirements
            test_rust
            test_node
            print_success "Todos os testes passaram!"
            ;;
        lint)
            check_requirements
            lint_all
            print_success "Linting concluído!"
            ;;
        clean)
            print_step "Limpando..."
            cargo clean
            npm run clean
            print_success "Limpeza concluída"
            ;;
        help)
            echo "Uso: ./build.sh [comando]"
            echo ""
            echo "Comandos:"
            echo "  check   - Verifica pré-requisitos"
            echo "  rust    - Compila apenas Rust"
            echo "  node    - Compila apenas Node.js"
            echo "  all     - Compila tudo (padrão)"
            echo "  test    - Executa testes"
            echo "  lint    - Executa linting"
            echo "  clean   - Limpa build"
            echo "  help    - Mostra esta mensagem"
            ;;
        *)
            print_error "Comando desconhecido: $COMMAND"
            echo "Use './build.sh help' para ajuda"
            exit 1
            ;;
    esac
}

main "$@"
