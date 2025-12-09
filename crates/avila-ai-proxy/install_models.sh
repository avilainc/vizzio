#!/bin/bash
# Script para instalar modelos no Ollama

echo "🚀 Instalando modelos no Ollama..."
echo ""

# Mistral - Modelo base recomendado (4.1GB)
echo "📦 Instalando Mistral 7B..."
ollama pull mistral

# Magister (se disponível no Ollama Hub)
# Nota: Magister pode não estar disponível oficialmente
# Alternativas: mistral-openorca, mistral-nemo
echo "📦 Instalando Mistral OpenOrca (variante otimizada)..."
ollama pull mistral-openorca

# Dolphin Mistral - Modelo sem censura
echo "📦 Instalando Dolphin Mistral (sem censura)..."
ollama pull dolphin-mistral

# Wizard Vicuna Uncensored
echo "📦 Instalando Wizard Vicuna Uncensored..."
ollama pull wizard-vicuna-uncensored

# Neural Chat
echo "📦 Instalando Neural Chat..."
ollama pull neural-chat

# Llama 3.2 (mais recente)
echo "📦 Instalando Llama 3.2 3B..."
ollama pull llama3.2

# Verificar modelos instalados
echo ""
echo "✅ Modelos instalados:"
ollama list

echo ""
echo "🎉 Instalação completa!"
echo "📡 Inicie o servidor: python server.py"
echo "🧪 Teste: curl http://localhost:8000/health"

