cd..
# 🗺️ VIZZIO - Mapa de Navegação

Guia rápido para navegar e entender a estrutura do repositório VIZZIO.

## 🚀 Para Começar Agora

| Caso de Uso | Arquivo | Tempo |
|-------------|---------|-------|
| **Primeira vez aqui?** | [docs/START_HERE.md](docs/START_HERE.md) | 5 min |
| **Setup rápido** | Execute `scripts/build.ps1 all` | 40 min |
| **Entender a arquitetura** | [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | 15 min |
| **Configurar ambiente** | [docs/CONFIGURATION.md](docs/CONFIGURATION.md) | 10 min |

## 📂 Estrutura Lógica

```
VIZZIO/
├─ 📖 Documentação
│  └─ docs/                     ← Leia aqui!
│     ├─ START_HERE.md          ✅ Comece aqui
│     ├─ ARCHITECTURE.md        Decisões técnicas
│     ├─ ROADMAP.md             Plano 6 meses
│     └─ ... (15 documentos)
│
├─ 🛠️ Scripts & Build
│  └─ scripts/                  ← Execute aqui!
│     ├─ build.ps1 / build.sh   Compilar tudo
│     ├─ remove_profiles.*      Limpar artifacts
│     └─ test-notifications.ps1 Testar notificações
│
├─ 🦀 Código Rust (Core)
│  ├─ avila/                    130+ crates (foundation)
│  └─ avx/                      23 crates (extensões)
│
├─ 💾 Configuração
│  ├─ Cargo.toml                Workspace Rust
│  ├─ package.json              Workspace Node.js
│  ├─ tsconfig.json             Tipos TypeScript
│  ├─ Dockerfile.*              Containers
│  └─ .env.local                Variáveis
│
└─ 📦 Deploy
   ├─ vizzio-deploy-config/     Configurações
   └─ docker-compose.yml        Stack completo
```

## 🎯 Casos de Uso

### 1️⃣ Sou Novo Aqui

```
1. Leia: docs/START_HERE.md (5 min)
2. Instale: scripts/build.ps1 all (40 min)
3. Acesse: http://localhost:3001
4. Leia: docs/ARCHITECTURE.md (15 min)
```

### 2️⃣ Vou Desenvolver

```
1. Leia: docs/CONFIGURATION.md (10 min)
2. Execute: scripts/build.ps1 avila (15 min)
3. Estude: docs/ARCHITECTURE.md (20 min)
4. Comece: escolha uma task do ROADMAP
```

### 3️⃣ Vou Fazer Deploy

```
1. Leia: docs/CONFIGURATION.md (10 min)
2. Prepare: .env.local com variáveis
3. Execute: docker-compose up -d
4. Verifique: docker-compose logs -f
5. Acesse: http://localhost:3001
```

### 4️⃣ Vou Integrar Módulos

```
1. Leia: docs/INTEGRATION_GUIDE.md (15 min)
2. Entenda: docs/ARCHITECTURE.md (20 min)
3. Estude: docs/INDEX.md → procure seu módulo
4. Implemente: seguindo padrões da arquitetura
```

### 5️⃣ Preciso Ajuda

```
→ Erro de build?     Veja: docs/CONFIGURATION.md
→ Não entendo arq?   Leia: docs/ARCHITECTURE.md
→ Preciso de feature? Veja: docs/ROADMAP.md
→ Tudo indexado em:   Consulte: docs/INDEX.md
```

## 📚 Documentos por Tipo

### Onboarding
- [START_HERE.md](docs/START_HERE.md) - Bem-vindas
- [CONFIGURATION.md](docs/CONFIGURATION.md) - Setup

### Técnico
- [ARCHITECTURE.md](docs/ARCHITECTURE.md) - Design
- [INTEGRATION_GUIDE.md](docs/INTEGRATION_GUIDE.md) - Integração
- [INDEX.md](docs/INDEX.md) - Índice completo

### Planejamento
- [ROADMAP.md](docs/ROADMAP.md) - 6 meses ahead
- [SOLUTIONS_COMPARISON.md](docs/SOLUTIONS_COMPARISON.md) - Comparação

### Notificações
- [NOTIFICATION_SETUP.md](docs/NOTIFICATION_SETUP.md) - Setup
- [PARTNER_NOTIFICATIONS_GUIDE.md](docs/PARTNER_NOTIFICATIONS_GUIDE.md) - Para partners
- [TEST_NOTIFICATION.md](docs/TEST_NOTIFICATION.md) - Testes

### Status & Referência
- [VIZZIO_COMPLETE.md](docs/VIZZIO_COMPLETE.md) - Documentação
- [VIZZIO_UNIFIED.md](docs/VIZZIO_UNIFIED.md) - Unificação
- [IMPLEMENTATION_COMPLETE.md](docs/IMPLEMENTATION_COMPLETE.md) - Status
- [UNIFIED_COMPLETE.md](docs/UNIFIED_COMPLETE.md) - Unificação status

## 🔍 Como Encontrar Algo

**Procuro por:**
- **Componente específico?** → `docs/INDEX.md`
- **Como fazer X?** → `docs/CONFIGURATION.md`
- **Arquitetura?** → `docs/ARCHITECTURE.md`
- **Plano futuro?** → `docs/ROADMAP.md`
- **Integração?** → `docs/INTEGRATION_GUIDE.md`
- **Status projeto?** → `docs/IMPLEMENTATION_COMPLETE.md`

## ⚡ Atalhos Úteis

### Build Rápido (Windows)
```powershell
cd scripts
.\build.ps1 all          # Tudo (40 min)
.\build.ps1 avila        # Só Rust (15 min)
.\build.ps1 npm          # Só Node.js (2 min)
```

### Build Rápido (Unix)
```bash
cd scripts
bash build.sh all        # Tudo (40 min)
bash build.sh avila      # Só Rust (15 min)
bash build.sh npm        # Só Node.js (2 min)
```

### Docker
```bash
docker-compose up -d     # Inicia
docker-compose logs -f   # Logs em tempo real
docker-compose down      # Para
```

### Acessar
- **Frontend:** http://localhost:3001
- **API:** http://localhost:3000
- **Documentação:** Leia `docs/`

## 📊 Estatísticas do Projeto

- **Linguagens:** Rust, TypeScript/JavaScript, Python
- **Crates Rust:** 153+ (avila + avx)
- **Pacotes Node:** 13+
- **Documentação:** 15+ arquivos
- **Linhas de código:** 500K+

## 🤝 Contribuindo

1. Leia [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)
2. Escolha task em [docs/ROADMAP.md](docs/ROADMAP.md)
3. Crie branch: `git checkout -b feature/sua-feature`
4. Comita: `git commit -m "type: descrição"`
5. Push: `git push origin feature/sua-feature`
6. PR com descrição clara

## ❓ FAQ Rápido

**P: Por onde começo?**
R: `docs/START_HERE.md`

**P: Como instalo?**
R: `scripts/build.ps1 all` (Windows) ou `scripts/build.sh all` (Unix)

**P: Quanto tempo leva?**
R: ~40 minutos na primeira vez (download + compilação)

**P: Qual é a URL da app?**
R: http://localhost:3001 (após fazer docker-compose up -d)

**P: Como testo notificações?**
R: `scripts/test-notifications.ps1`

**P: Como faço deploy?**
R: Veja `docs/CONFIGURATION.md`

---

**Estrutura organizada em:** 5 de dezembro de 2025

**Próximos passos:** Leia [docs/START_HERE.md](docs/START_HERE.md)
