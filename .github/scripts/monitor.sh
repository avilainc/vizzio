#!/bin/bash

##############################################################################
# VIZZIO Build Monitor - Integração com GitHub Actions
# Envia eventos em tempo real para o dashboard
##############################################################################

set -e

# Configuração
MONITOR_URL="${MONITOR_URL:-http://localhost:3000}"
BUILD_ID="${GITHUB_RUN_ID}-${GITHUB_RUN_NUMBER}"
WORKFLOW="${GITHUB_WORKFLOW}"
BRANCH="${GITHUB_REF#refs/heads/}"
COMMIT="${GITHUB_SHA:0:8}"
AUTHOR="${GITHUB_ACTOR}"

# Cores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

##############################################################################
# FUNÇÕES
##############################################################################

log_info() {
  echo -e "${BLUE}ℹ️  $1${NC}"
}

log_success() {
  echo -e "${GREEN}✅ $1${NC}"
}

log_error() {
  echo -e "${RED}❌ $1${NC}"
}

log_warning() {
  echo -e "${YELLOW}⚠️  $1${NC}"
}

# Notificar início do build
notify_build_start() {
  log_info "Notificando início do build..."

  local payload=$(cat <<EOF
{
  "buildId": "${BUILD_ID}",
  "workflow": "${WORKFLOW}",
  "branch": "${BRANCH}",
  "commit": "${COMMIT}",
  "author": "${AUTHOR}",
  "message": "Build iniciado"
}
EOF
)

  if curl -s -X POST "${MONITOR_URL}/api/builds/start" \
    -H "Content-Type: application/json" \
    -d "${payload}" > /dev/null 2>&1; then
    log_success "Build notificado"
  else
    log_warning "Falha ao notificar início do build"
  fi
}

# Notificar log
notify_log() {
  local level="$1"
  local message="$2"
  local step="$3"

  if [ -z "$step" ]; then
    step="build"
  fi

  local payload=$(cat <<EOF
{
  "level": "${level}",
  "message": "${message}",
  "step": "${step}"
}
EOF
)

  if ! curl -s -X POST "${MONITOR_URL}/api/builds/${BUILD_ID}/log" \
    -H "Content-Type: application/json" \
    -d "${payload}" > /dev/null 2>&1; then
    : # Falha silenciosa
  fi
}

# Notificar conclusão
notify_build_complete() {
  local status="$1"
  local duration="$2"
  local details="$3"

  log_info "Notificando conclusão do build..."

  local payload=$(cat <<EOF
{
  "status": "${status}",
  "duration": ${duration},
  "details": ${details}
}
EOF
)

  if curl -s -X POST "${MONITOR_URL}/api/builds/${BUILD_ID}/complete" \
    -H "Content-Type: application/json" \
    -d "${payload}" > /dev/null 2>&1; then
    log_success "Build finalizado"
  else
    log_warning "Falha ao notificar conclusão do build"
  fi
}

##############################################################################
# VARIÁVEIS DE TESTE
##############################################################################

export -f notify_build_start
export -f notify_log
export -f notify_build_complete
export -f log_info
export -f log_success
export -f log_error
export -f log_warning
export MONITOR_URL BUILD_ID WORKFLOW BRANCH COMMIT AUTHOR
export RED GREEN YELLOW BLUE NC
