#!/bin/bash
# Script para instalar modelos sem censura no Ollama

echo "🚀 Instalando modelos sem censura no Ollama..."
echo ""

# Dolphin Mistral - Principal modelo sem censura
echo "📦 Instalando Dolphin Mistral (sem censura)..."
ollama pull dolphin-mistral

# Wizard Vicuna Uncensored
echo "📦 Instalando Wizard Vicuna Uncensored..."
ollama pull wizard-vicuna-uncensored

# Neural Chat
echo "📦 Instalando Neural Chat..."
ollama pull neural-chat

# Verificar modelos instalados
echo ""
echo "✅ Modelos instalados:"
ollama list

echo ""
echo "🎉 Instalação completa! Execute: python server.py"
