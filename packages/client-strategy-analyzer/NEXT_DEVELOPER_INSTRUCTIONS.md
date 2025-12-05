# 🚀 INSTRUÇÕES PARA O PRÓXIMO DESENVOLVEDOR

**Data**: 2024
**Estado da Plataforma**: Arquitetura Completa - Pronto para Desenvolvimento
**Quem**: Qualquer desenvolvedor/engenheiro que continuar este projeto

---

## 📖 Leia Primeiro

1. **START_HERE.md** (5 min) - Overview rápido
2. **MONOREPO_STRUCTURE.md** (10 min) - Arquitetura completa
3. **EXECUTIVE_SUMMARY.md** (10 min) - Visão estratégica
4. Este arquivo (10 min) - Instruções práticas

---

## 🛠️ Setup Local (15 minutos)

### Requisitos
- Node.js 18+
- Docker Desktop instalado
- Git configurado

### Passo a Passo

```bash
# 1. Clonar ou abrir o repositório
cd d:\Vizzio\packages

# 2. Instalar dependências (vai linkar os workspaces)
npm install

# 3. Compilar TypeScript
npm run build

# 4. Subir Docker containers
docker-compose up -d

# 5. Verificar status
docker-compose ps

# 6. Acessar serviços
# Dashboard: http://localhost:3001
# API: http://localhost:3000/api
# RabbitMQ: http://localhost:15672 (admin/password123)
# MongoDB: mongodb://localhost:27017
# Redis: localhost:6379
```

---

## 📂 Onde Encontrar O Quê

### 📦 Pacotes Monorepo

| Pacote | Pasta | O Quê | Começar Com |
|--------|-------|-------|------------|
| Core | `packages/core/src/` | Types, Interfaces | `types.ts` |
| Workflows | `packages/workflows/src/engine/` | Bull Queue, Async Jobs | `WorkflowEngine.ts` |
| Email | `packages/email-service/src/smtp/` | SMTP, SendGrid, Templates | `EmailService.ts` |
| Finance | `packages/finance-tools/src/invoicing/` | Faturas, Pagamentos | `FinanceTools.ts` |
| Marketing | `packages/marketing-automation/src/campaigns/` | Campanhas, Leads | `MarketingAutomation.ts` |
| Sales | `packages/sales-pipeline/src/deals/` | Deals, Propostas | (Criar) |
| Shortcuts | `packages/shortcuts/src/keyboard/` | Atalhos, Voice | `ShortcutManager.ts` |
| Integrations | `packages/integrations/src/salesforce/` | Salesforce, Slack, HubSpot | `Integrations.ts` |
| AI Assistant | `packages/ai-assistant/src/copilot/` | Copilot, Suggestions | (Criar) |
| Backend | `packages/backend/src/` | Express API | `index.ts` |
| Frontend | `packages/frontend/` | Next.js Dashboard | `pages/` |
| Mobile | `packages/mobile/` | React Native App | (Criar) |
| CLI | `packages/cli/src/` | Command Line | `index.ts` |

### 🔧 Configurações

| Arquivo | Localização | Para Quê |
|---------|------------|---------|
| Root config | `package.json` | Workspaces, scripts |
| TS Config | `tsconfig.json` | Paths, compilação |
| ESLint | `.eslintrc.js` | Linting rules |
| Prettier | `.prettierrc` | Formatting |
| Docker | `docker-compose.yml` | Services (6) |
| Env vars | `.env` | Configuração (criar) |

### 📚 Documentação

```
START_HERE.md                    ← Comece aqui!
MONOREPO_STRUCTURE.md            ← Workflows e estrutura
EXECUTIVE_SUMMARY.md             ← Visão estratégica
COMPLETION_CHECKLIST.md          ← O que foi feito
NEXT_DEVELOPER_INSTRUCTIONS.md   ← Este arquivo
```

---

## 🎯 Tarefas Imediatas (Próximas 2 Semanas)

### Semana 1: Setup & Validação
```
[ ] npm install (resolver dependências)
[ ] npm run build (verificar compilação)
[ ] docker-compose up -d (subir containers)
[ ] Verificar conexões:
    [ ] MongoDB: mongosh admin -u admin -p password123 --eval "db.version()"
    [ ] Redis: redis-cli ping
    [ ] Backend API: curl http://localhost:3000/health
    [ ] Frontend: http://localhost:3001
```

### Semana 2: Primeira Implementação
```
[ ] Implementar /health endpoint no backend
[ ] Conectar backend a MongoDB
[ ] Conectar backend a Redis
[ ] Criar 1º modelo Mongoose (User)
[ ] Criar 1º endpoint REST (/api/users)
[ ] Testar endpoint com Postman/Insomnia
[ ] Criar 1º teste unitário
```

---

## 🔨 Desenvolvimento Diário

### Ambiente de Desenvolvimento

```bash
# Terminal 1: Watch backend
cd packages/backend
npm run dev

# Terminal 2: Watch frontend
cd packages/frontend
npm run dev

# Terminal 3: Docker logs
docker-compose logs -f

# Terminal 4: CLI commands
cd packages/cli
npm run dev
```

### Scripts Úteis

```bash
# Compilar tudo
npm run build

# Rodar testes
npm run test

# Verificar linting
npm run lint

# Formatar código
npm run format

# Documentação TypeDoc
npm run docs

# Limpar tudo
npm run clean

# Reinstalar
npm install
```

---

## 📝 Convenções de Código

### TypeScript
- ✅ Use `strict: true` no tsconfig
- ✅ Nunca use `any` (use `unknown` e faça type guard)
- ✅ Sempre tipifique inputs/outputs de funções
- ✅ Use interfaces para dados, types para unions

```typescript
// ✅ BOM
interface User {
  id: string;
  email: string;
  name: string;
}

function getUser(id: string): Promise<User> {
  // ...
}

// ❌ RUIM
function getUser(id) {
  // ...
}

const data: any = response.data;
```

### Nomes
- 📦 Pacotes: `@vizzio/feature-name` (kebab-case)
- 📁 Pastas: `lowercase` ou `camelCase`
- 📄 Arquivos: `PascalCase.ts` para classes, `camelCase.ts` para utils
- 🔧 Funções: `camelCase`
- 🏗️ Interfaces: `PascalCase`

```typescript
// Estrutura de arquivo
src/
├── types/          # Interfaces globais
├── services/       # Business logic
├── utils/          # Helper functions
├── models/         # MongoDB schemas
├── controllers/    # API handlers
├── middleware/     # Express middleware
└── routes/         # Route definitions
```

### Commits

```bash
# Formato: type(scope): message

git commit -m "feat(email-service): add template rendering"
git commit -m "fix(workflows): resolve Bull queue timeout"
git commit -m "docs(readme): update installation steps"
git commit -m "test(finance-tools): add invoice generation tests"

# Types: feat, fix, docs, test, refactor, chore, perf, ci
```

---

## 🚀 Deployando Features

### 1. Branch Strategy
```bash
# Dev local
git checkout -b feature/your-feature

# Commit
git add .
git commit -m "feat(package): description"

# Push
git push origin feature/your-feature

# PR → Code Review → Merge
```

### 2. Pull Request Checklist
- [ ] Testes passando (`npm run test`)
- [ ] Código formatado (`npm run format`)
- [ ] Sem linting errors (`npm run lint`)
- [ ] Documentação atualizada
- [ ] CHANGELOG.md atualizado
- [ ] TypeScript compila sem erros (`npm run build`)

### 3. Deployment
```bash
# Staging
git push origin develop  # Trigger CI/CD

# Production
git push origin main     # Trigger deploy.yml
```

---

## 🐛 Debugging

### Logs

```bash
# Backend logs
docker-compose logs -f backend

# MongoDB logs
docker-compose logs -f mongodb

# Redis logs
docker-compose logs -f redis

# Frontend logs (console do browser)
Open DevTools (F12) → Console
```

### Problemas Comuns

| Problema | Solução |
|----------|---------|
| `Cannot find module '@vizzio/core'` | Run `npm install` na root |
| `MongoDB connection refused` | Check `docker-compose ps`, restart: `docker-compose down && up -d` |
| `Port 3000 already in use` | Kill process: `lsof -i :3000` then `kill -9 <PID>` |
| `TypeScript errors` | Run `npm run build` to see all errors |
| `Node modules corrupted` | `npm run clean && npm install` |

---

## 📚 Aprender Mais

### Tecnologias Principais

| Tech | Tutorial | Docs |
|------|----------|------|
| TypeScript | [TS Handbook](https://www.typescriptlang.org/docs/) | [Docs](https://www.typescriptlang.org/) |
| Express | [Express Guide](https://expressjs.com/en/guide/routing.html) | [Docs](https://expressjs.com/) |
| MongoDB | [Mongo University](https://university.mongodb.com/) | [Docs](https://docs.mongodb.com/) |
| React | [React Docs](https://react.dev) | [Tutorial](https://react.dev/learn) |
| Next.js | [Next Guide](https://nextjs.org/learn) | [Docs](https://nextjs.org/docs) |
| Bull Queue | [Bull Docs](https://github.com/OptimalBits/bull) | [API](https://github.com/OptimalBits/bull/blob/master/API.md) |
| Redis | [Redis Guide](https://redis.io/docs/about/) | [Docs](https://redis.io/docs/) |

### Arquitetura

- [Microservices Pattern](https://microservices.io/)
- [Domain-Driven Design](https://www.domainlanguage.com/ddd/)
- [SOLID Principles](https://en.wikipedia.org/wiki/SOLID)
- [Clean Code](https://www.oreilly.com/library/view/clean-code-a/9780136083238/)

---

## 👥 Comunicação

### Equipe
- **Tech Lead**: [Name]
- **Product Owner**: [Name]
- **DevOps**: [Name]
- **QA**: [Name]

### Standups
- **Daily**: 10:00 AM (15 min)
- **Sprint Planning**: Monday 14:00 (1h)
- **Sprint Review**: Friday 16:00 (1h)
- **Retrospective**: Friday 17:00 (45 min)

### Channels
- **#engineering** - Slack channel
- **#vizzio-general** - General discussion
- **#deployments** - Deployment notifications
- **#bugs** - Bug reports

---

## 📊 Métricas & Monitoring

### O que Monitorar

```bash
# Performance
- API response time (target: <200ms)
- Build time (target: <5min)
- Test coverage (target: >80%)
- Uptime (target: >99.9%)

# Quality
- ESLint errors: 0
- TypeScript errors: 0
- Test failures: 0
- Code coverage gaps

# Resources
- CPU usage
- Memory usage
- Disk space
- Network bandwidth
```

### Tools

- **Monitoring**: New Relic / DataDog
- **Error Tracking**: Sentry
- **Performance**: Lighthouse / Grafana
- **Analytics**: Google Analytics / Segment

---

## 🎓 Onboarding Checklist

- [ ] Ler este arquivo
- [ ] Ler START_HERE.md
- [ ] Setup local (npm install, docker-compose up)
- [ ] Verificar tudo rodando
- [ ] Fazer primeiro commit
- [ ] Ler MONOREPO_STRUCTURE.md
- [ ] Explorar packages/core/src/types.ts
- [ ] Implementar 1º endpoint simples
- [ ] Criar 1º teste
- [ ] Fazer 1º PR
- [ ] Reunião com tech lead

**Tempo estimado: 1 semana**

---

## 🆘 Precisa de Ajuda?

### Recursos

1. **Documentação Local**
   - `START_HERE.md` - Início
   - `MONOREPO_STRUCTURE.md` - Arquitetura
   - `packages/*/README.md` - Por pacote

2. **Online**
   - Stack Overflow
   - GitHub Issues
   - Official docs

3. **Comunidade**
   - Slack engineering channel
   - Daily standup
   - Tech lead 1:1

4. **Problemas Técnicos**
   - Issues GitHub
   - Wiki interna
   - Tech lead (@name)

---

## 🎯 Sua Primeira Tarefa

```
TODO: Implementar /health endpoint no Backend

1. Abra: packages/backend/src/index.ts
2. Adicione rota GET /health
3. Retorne: { status: 'ok', timestamp: Date.now() }
4. Teste: curl http://localhost:3000/health
5. Commit: git commit -m "feat(backend): add health check endpoint"
6. Push: git push origin your-branch
7. Crie PR
```

---

**Bem-vindo ao time! 🚀**

*Desenvolvido com ❤️ para automatizar todas as operações empresariais.*

---

**Últimas atualizações**: 2024
**Próxima revisão**: Quando novo dev onboard
