# Avila Copilot VS Code Extension

AI-powered code completion with **zero censorship** using Ollama, DeepSeek, and OpenAI.

## 🎯 Features

- ✅ **Inline completions** - Code suggestions as you type
- ✅ **Bug detection** - Automatic bug analysis
- ✅ **Documentation** - Generate docs from code
- ✅ **Test generation** - Create unit tests
- ✅ **Chat interface** - Ask questions about your code
- ✅ **Multiple models** - Dolphin, Wizard Vicuna, Neural Chat, GPT-OSS 120B
- ✅ **No censorship** - Uncensored models for maximum freedom

## 🚀 Quick Start

### 1. Install Extension

```bash
cd avila/avila-copilot-vscode
npm install
npm run compile
```

### 2. Start AI Proxy

```bash
cd avila/avila-ai-proxy
cargo run --release
# Copy the Admin API Key from console
```

### 3. Configure VS Code

Open Settings (Ctrl+,) and search for "Avila Copilot":

- **Server Path**: `http://localhost:8000`
- **API Key**: `avila_...` (from step 2)
- **Model**: `dolphin-mistral` (or `gpt-oss:120b-cloud`)

### 4. Use

- **Auto-complete**: Type code, wait for suggestions
- **Manual complete**: `Ctrl+Space` (Cmd+Space on Mac)
- **Bug detection**: `Ctrl+Shift+P` → "Avila: Detect Bugs"
- **Generate docs**: `Ctrl+Shift+P` → "Avila: Generate Documentation"
- **Generate tests**: `Ctrl+Shift+P` → "Avila: Generate Tests"
- **Chat**: `Ctrl+Shift+A` (Cmd+Shift+A on Mac)

## 📦 Publish to Marketplace

### 1. Get Personal Access Token (PAT)

1. Go to: https://dev.azure.com/avila/_usersSettings/tokens
2. Create new token with **Marketplace** → **Manage** permission
3. Copy token

### 2. Create Publisher

```bash
npx vsce create-publisher avila
```

### 3. Login

```bash
npx vsce login avila
# Paste your PAT
```

### 4. Publish

```bash
npm run package
npx vsce publish
```

## 🔧 Development

```bash
# Install dependencies
npm install

# Compile
npm run compile

# Watch mode
npm run watch

# Package VSIX
npm run package

# Test locally
code --install-extension avila-copilot-1.0.0.vsix
```

## 🎮 Commands

- `avilacopilot.toggle` - Toggle Avila Copilot on/off
- `avilacopilot.complete` - Trigger code completion
- `avilacopilot.detectBugs` - Detect bugs in current file
- `avilacopilot.generateDocs` - Generate documentation
- `avilacopilot.generateTests` - Generate unit tests
- `avilacopilot.chat` - Open Avila Chat

## ⚙️ Configuration

```jsonc
{
  // Enable/disable Avila Copilot
  "avilacopilot.enabled": true,

  // AI Proxy server URL
  "avilacopilot.serverPath": "http://localhost:8000",

  // API Key
  "avilacopilot.apiKey": "avila_...",

  // Enable auto-complete
  "avilacopilot.autoComplete": true,

  // Enable bug detection
  "avilacopilot.bugDetection": true,

  // Maximum latency (ms)
  "avilacopilot.maxLatencyMs": 50000,

  // Show inline completions
  "avilacopilot.showInlineCompletions": true,

  // AI model
  "avilacopilot.model": "dolphin-mistral"
}
```

## 📚 Links

- **Website**: https://avila.inc
- **Docs**: https://docs.avila.inc
- **Support**: https://support.avila.inc
- **GitHub**: https://github.com/avilaops/vizzio
- **LinkedIn**: https://linkedin.com/company/avila-devops

## 📄 License

MIT - Copyright (c) 2025 Avila Team
