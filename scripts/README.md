# 🛠️ Scripts e Utilitários

Scripts de build, teste e manutenção do VIZZIO.

## 📦 Build Scripts

### Windows PowerShell
```powershell
# Build completo
.\build.ps1 all

# Build seletivo
.\build.ps1 avila      # Apenas Rust (Avila)
.\build.ps1 avx        # Apenas Rust (Avx)
.\build.ps1 npm        # Apenas Node.js
```

### Linux/macOS Bash
```bash
# Build completo
bash build.sh all

# Build seletivo
bash build.sh avila
bash build.sh avx
bash build.sh npm
```

## 🧹 Limpeza

### Remove Profiles (Rust)
```powershell
# Windows
.\remove_profiles.ps1

# Linux/macOS
python remove_profiles.py
```

Limpa:
- `target/` directories
- Build artifacts
- Cache files

## 🧪 Testes

### Notificações
```powershell
.\test-notifications.ps1
```

Testa:
- Sistema de notificações
- Webhooks
- Integrações

## 📋 Arquivos Disponíveis

| Script | Plataforma | Função |
|--------|-----------|--------|
| `build.ps1` | Windows | Build automático |
| `build.sh` | Unix | Build automático |
| `remove_profiles.ps1` | Windows | Limpeza de build |
| `remove_profiles.py` | Unix/Windows | Limpeza alternativa |
| `test-notifications.ps1` | Windows | Testa notificações |

## 🚀 Fluxo Recomendado

```powershell
# 1. Setup inicial
cd scripts
.\build.ps1 all

# 2. Antes de commits
.\remove_profiles.ps1

# 3. Testar notificações
.\test-notifications.ps1

# 4. Deploy
cd ..
docker-compose up -d
```

---

**Última atualização:** 5 de dezembro de 2025
