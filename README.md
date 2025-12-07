# 🚀 VIZZIO - Enterprise Automation Platform

**Plataforma unificada de automação empresarial integrando Rust, Node.js e serviços distribuídos.**

## 📍 Início Rápido

```powershell
# Windows
cd scripts
.\build.ps1 all
npm run dev:all
```

```bash
# Linux/macOS
cd scripts
bash build.sh all
npm run dev:all
```

**Acesse:** http://localhost:3001

## 📂 Estrutura do Repositório

```
d:\Vizzio\
├── 📖 docs/                    # Documentação completa
│   ├── START_HERE.md           # Comece aqui
│   ├── ARCHITECTURE.md         # Design técnico
│   ├── ROADMAP.md              # Plano de desenvolvimento
│   ├── CONFIGURATION.md        # Guia de configuração
│   └── ... (12+ documentos)
│
├── 📦 scripts/                 # Scripts de build e utilidades
│   ├── build.ps1               # Build Windows
│   ├── build.sh                # Build Unix
│   ├── remove_profiles.ps1     # Limpeza Windows
│   └── remove_profiles.py      # Limpeza Python
│
├── 🦀 avila/                   # 130+ crates Rust (core)
├── 🦀 avx/                     # 23 crates Rust (extensões)
├── 🎯 vizzio-deploy-config/    # Configuração de deploy
├── src/                        # Código-fonte unificado (reserved)
│
├── Cargo.toml                  # Workspace Rust
├── package.json                # Workspace Node.js
├── tsconfig.json               # Configuração TypeScript
├── Dockerfile.*                # Containers
└── .gitignore, .env.*          # Configuração Git
```

## 🏗️ Componentes

### Avila (Rust Foundation)
130+ crates com:
- Criptografia, Hash, Segurança
- Estruturas de dados, Álgebra linear
- Sistemas distribuídos, gRPC
- Machine Learning, Otimização
- GIS/Geolocalização
- E muito mais...

### Avx (Rust Extensions)
23 crates especializados em:
- Processamento de imagem
- Análise de dados
- Aceleração numérica
- Integrações externas

### Node.js Packages
13 pacotes para:
- Frontend/UI
- APIs REST
- WebSockets
- Automação empresarial

## 📚 Documentação

| Documento | Propósito |
|-----------|-----------|
| [START_HERE.md](docs/START_HERE.md) | Guia de boas-vindas e instruções iniciais |
| [ARCHITECTURE.md](docs/ARCHITECTURE.md) | Design técnico e decisões arquiteturais |
| [ROADMAP.md](docs/ROADMAP.md) | Plano de 6 meses de desenvolvimento |
| [CONFIGURATION.md](docs/CONFIGURATION.md) | Guia detalhado de configuração |
| [INDEX.md](docs/INDEX.md) | Índice completo de recursos |

## 🛠️ Build & Deploy

**Windows (PowerShell):**
```powershell
cd scripts
.\build.ps1 all        # Compila tudo (~40 min na primeira vez)
.\build.ps1 avila      # Apenas Rust Avila
.\build.ps1 avx        # Apenas Rust Avx
.\build.ps1 npm        # Apenas Node.js
```

**Linux/macOS (Bash):**
```bash
cd scripts
bash build.sh all
bash build.sh avila
bash build.sh avx
bash build.sh npm
```

## 🐳 Docker

```bash
docker-compose up -d              # Inicia stack completo
docker-compose logs -f            # Vê logs
docker-compose down               # Para e remove
```

## 📊 Status

- ✅ Unificação de projetos completa
- ✅ Workspace Rust (153+ crates)
- ✅ Workspace Node.js (13 pacotes)
- ✅ Scripts de build automático
- ✅ Documentação abrangente
- 🔄 CI/CD em desenvolvimento

## 🤝 Contribuindo

1. Leia [ARCHITECTURE.md](docs/ARCHITECTURE.md)
2. Escolha uma tarefa de [ROADMAP.md](docs/ROADMAP.md)
3. Crie branch: `git checkout -b feature/sua-feature`
4. Faça commits descritivos
5. Push e abra PR

## 📝 Licença

Veja LICENSE file para detalhes.

---

**Última atualização:** 5 de dezembro de 2025
