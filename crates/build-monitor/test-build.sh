#!/bin/bash

###############################################################################
# VIZZIO Build Monitor - Script de Teste
# Simula um build completo e envia notificações
###############################################################################

set -e

# Cores
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Configuração
MONITOR_URL="${MONITOR_URL:-http://localhost:3000}"
BUILD_ID="test-$(date +%s)"
WORKFLOW="Test Build"
BRANCH="main"
COMMIT="$(git rev-parse --short HEAD 2>/dev/null || echo 'abc12345')"
AUTHOR="$(git config user.name 2>/dev/null || echo 'Test User')"

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}VIZZIO Build Monitor - Teste${NC}"
echo -e "${BLUE}========================================${NC}\n"

echo -e "${YELLOW}Configuração:${NC}"
echo "  Monitor URL: $MONITOR_URL"
echo "  Build ID:    $BUILD_ID"
echo "  Workflow:    $WORKFLOW"
echo "  Branch:      $BRANCH"
echo "  Commit:      $COMMIT"
echo "  Autor:       $AUTHOR"
echo ""

# ============================================================================
# 1. INICIAR BUILD
# ============================================================================

echo -e "${BLUE}[1/5] Iniciando build...${NC}"

curl -s -X POST "$MONITOR_URL/api/builds/start" \
  -H "Content-Type: application/json" \
  -d "{
    \"buildId\": \"$BUILD_ID\",
    \"workflow\": \"$WORKFLOW\",
    \"branch\": \"$BRANCH\",
    \"commit\": \"$COMMIT\",
    \"author\": \"$AUTHOR\",
    \"message\": \"Build iniciado para teste\"
  }" > /dev/null

echo -e "${GREEN}✅ Build iniciado${NC}\n"

# ============================================================================
# 2. SIMULAR TESTES
# ============================================================================

echo -e "${BLUE}[2/5] Executando testes...${NC}"

TESTS=(
  "Configurando ambiente Rust..."
  "Instalando dependências..."
  "Compilando código..."
  "Rodando testes unitários..."
  "Teste 1/50 - auth_test... PASS ✓"
  "Teste 2/50 - db_test... PASS ✓"
  "Teste 3/50 - api_test... PASS ✓"
  "Teste 4/50 - crypto_test... PASS ✓"
  "Teste 5/50 - utils_test... PASS ✓"
)

for test in "${TESTS[@]}"; do
  echo -e "  ${GREEN}→${NC} $test"

  curl -s -X POST "$MONITOR_URL/api/builds/$BUILD_ID/log" \
    -H "Content-Type: application/json" \
    -d "{
      \"level\": \"info\",
      \"message\": \"$test\",
      \"step\": \"test-rust\"
    }" > /dev/null

  sleep 0.5
done

echo -e "${GREEN}✅ Testes completados${NC}\n"

# ============================================================================
# 3. ANALISAR COBERTURA
# ============================================================================

echo -e "${BLUE}[3/5] Analisando cobertura...${NC}"

curl -s -X POST "$MONITOR_URL/api/builds/$BUILD_ID/log" \
  -H "Content-Type: application/json" \
  -d "{
    \"level\": \"info\",
    \"message\": \"Coverage analysis: 85% (42/50 files)\",
    \"step\": \"coverage\"
  }" > /dev/null

echo -e "  ${GREEN}→${NC} Coverage analysis: 85% (42/50 files)"
echo -e "${GREEN}✅ Cobertura analisada${NC}\n"

# ============================================================================
# 4. LINT & FORMAT
# ============================================================================

echo -e "${BLUE}[4/5] Verificando estilo de código...${NC}"

curl -s -X POST "$MONITOR_URL/api/builds/$BUILD_ID/log" \
  -H "Content-Type: application/json" \
  -d "{
    \"level\": \"info\",
    \"message\": \"Running rustfmt...\",
    \"step\": \"format\"
  }" > /dev/null

curl -s -X POST "$MONITOR_URL/api/builds/$BUILD_ID/log" \
  -H "Content-Type: application/json" \
  -d "{
    \"level\": \"info\",
    \"message\": \"Running clippy...\",
    \"step\": \"clippy\"
  }" > /dev/null

echo -e "  ${GREEN}→${NC} rustfmt check passed"
echo -e "  ${GREEN}→${NC} clippy lint passed"
echo -e "${GREEN}✅ Estilo de código validado${NC}\n"

# ============================================================================
# 5. COMPLETAR BUILD
# ============================================================================

echo -e "${BLUE}[5/5] Completando build...${NC}"

DURATION=$((RANDOM % 300 + 60))

curl -s -X POST "$MONITOR_URL/api/builds/$BUILD_ID/complete" \
  -H "Content-Type: application/json" \
  -d "{
    \"status\": \"success\",
    \"duration\": $DURATION,
    \"details\": {
      \"testsRun\": 50,
      \"testsPassed\": 50,
      \"testsFailed\": 0,
      \"coverage\": 85,
      \"issues\": []
    }
  }" > /dev/null

echo -e "  ${GREEN}→${NC} Build concluído com sucesso"
echo -e "  ${GREEN}→${NC} Duração: $((DURATION / 60))m $((DURATION % 60))s"
echo -e "  ${GREEN}→${NC} Email enviado para nicolas@avila.inc"
echo -e "${GREEN}✅ Build finalizado${NC}\n"

# ============================================================================
# RESUMO
# ============================================================================

echo -e "${BLUE}========================================${NC}"
echo -e "${GREEN}✅ TESTE CONCLUÍDO COM SUCESSO!${NC}"
echo -e "${BLUE}========================================${NC}\n"

echo -e "📊 Dashboard: ${BLUE}http://localhost:3000/dashboard${NC}"
echo -e "📝 Build ID:  ${BLUE}$BUILD_ID${NC}"
echo -e "📧 Email:     ${BLUE}nicolas@avila.inc${NC}\n"

echo -e "${YELLOW}Dicas:${NC}"
echo "  • Abra http://localhost:3000/dashboard para ver em tempo real"
echo "  • Verifique seu email para a notificação de build"
echo "  • Os logs aparecem conforme são enviados"
echo ""
