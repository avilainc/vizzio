# 📧 Guia de Configuração - Notificações para Sócios

Este guia mostra como configurar as notificações automatizadas de email para os sócios do projeto Vizzio Platform.

## 🎯 Opções Disponíveis

### ✅ Opção 1: GitHub Actions (Recomendado - Gratuito)

**Vantagens:**
- ✨ Nativo do GitHub
- 💰 Totalmente gratuito (2000 minutos/mês)
- 🔒 Integrado com segurança
- ⚡ Sem necessidade de servidor externo

**Desvantagens:**
- Requer configuração de credenciais SMTP

---

### Opção 2: GitHub App Customizado

**Vantagens:**
- 🎨 Totalmente customizável
- 🚀 Mais flexível
- 🔧 Integração profunda com GitHub

**Desvantagens:**
- Requer hospedagem externa
- Mais complexo de configurar

---

### Opção 3: WebHook + Servidor Externo

**Vantagens:**
- 📱 Controle total
- 🛠️ Fácil de adaptar

**Desvantagens:**
- Precisa de servidor 24/7
- Custo de hospedagem

---

## 🚀 Configuração - GitHub Actions (Recomendado)

### Passo 1: Configurar Secrets no GitHub

1. Vá para: **Settings → Secrets and variables → Actions**

2. Clique em **"New repository secret"** e adicione:

```
SMTP_HOST = smtp.gmail.com
SMTP_PORT = 587
SMTP_USER = seu_email@gmail.com
SMTP_PASSWORD = sua_senha_aplicacao
SEND_FROM = noreply@vizzio.dev
PARTNER_1_EMAIL = socio1@example.com
PARTNER_1_NAME = Sócio 1
PARTNER_2_EMAIL = socio2@example.com
PARTNER_2_NAME = Sócio 2
```

### Passo 2: Configurar as Credenciais

#### 🔵 Usando Gmail com 2FA

1. Ative 2FA na sua conta Google
2. Gere uma **"Senha de Aplicação"**:
   - https://myaccount.google.com/apppasswords
   - Selecione "Mail" e "Windows Computer"
   - Copie a senha gerada (16 caracteres)

3. Use essa senha como `SMTP_PASSWORD`

#### 🔴 Usando Outlook/Microsoft

```
SMTP_HOST = smtp.office365.com
SMTP_PORT = 587
SMTP_USER = seu_email@outlook.com
SMTP_PASSWORD = sua_senha
```

#### 🟡 Usando SendGrid (Recomendado para Produção)

```
SMTP_HOST = smtp.sendgrid.net
SMTP_PORT = 587
SMTP_USER = apikey
SMTP_PASSWORD = SG.sua_chave_api
SEND_FROM = noreply@seudominio.com
```

[Obter SendGrid API Key](https://sendgrid.com)

### Passo 3: Adicionar os Arquivos

✅ Os arquivos já foram criados:
- `.github/workflows/notify-partners.yml`
- `.github/scripts/send-emails.js`

### Passo 4: Testando

Para testar o workflow:

```bash
# Fazer um push para ativar o workflow
git push origin master

# Ou criar um Pull Request

# Verificar logs em: Actions → notify-partners
```

---

## 📧 Template de Email

O email é enviado em **HTML+CSS** com:

- ✨ Design responsivo e profissional
- 🎨 Cores do Vizzio Platform
- 📱 Funciona em todos os clientes de email
- 🔗 Link direto para o commit/PR/issue
- 📊 Informações detalhadas do evento

### Tipos de Eventos Suportados

1. **📤 PUSH** - Novo commit
   - Autor do commit
   - Branch
   - Mensagem de commit
   - Link direto para o commit

2. **🔀 PULL REQUEST** - Novo PR ou atualização
   - Número e título do PR
   - Status (aberto, fechado, atualizado)
   - Autor do PR
   - Branches (de/para)

3. **⚠️ ISSUE** - Nova issue ou fechamento
   - Número e título da issue
   - Status (aberta, fechada)
   - Quem reportou
   - Descrição

---

## 🔧 Personalizar os Sócios

### Método 1: Via GitHub Settings (Recomendado)

No arquivo `.github/workflows/notify-partners.yml`:

```yaml
env:
  PARTNER_1_EMAIL: novo_socio1@example.com
  PARTNER_1_NAME: Nome Sócio 1
  PARTNER_2_EMAIL: novo_socio2@example.com
  PARTNER_2_NAME: Nome Sócio 2
```

### Método 2: Adicionar mais Sócios

Edite `.github/scripts/send-emails.js`:

```javascript
const PARTNERS = [
  {
    email: 'socio1@example.com',
    name: 'Sócio 1'
  },
  {
    email: 'socio2@example.com',
    name: 'Sócio 2'
  },
  {
    email: 'socio3@example.com',    // Novo sócio
    name: 'Sócio 3'
  }
];
```

---

## 🎯 Personalizações Avançadas

### Filtrar por Tipo de Evento

Edite `.github/workflows/notify-partners.yml`:

```yaml
on:
  push:
    branches: [ master, main ]  # Apenas master e main
  pull_request:
    types: [ opened, closed ]    # Apenas abertos e fechados
  issues:
    types: [ opened ]            # Apenas issues novas
```

### Notificar apenas em Pushes Críticos

```yaml
on:
  push:
    branches: [ master ]
    paths:
      - 'src/**'        # Apenas mudanças em src/
      - '.github/**'    # E em .github/
```

### Sem Notificação em Commits de Merge

No script `send-emails.js`:

```javascript
if (eventPayload.head_commit.message.includes('Merge branch')) {
  console.log('⏭️  Ignorando merge commit');
  process.exit(0);
}
```

---

## ✅ Checklist de Configuração

- [ ] Criar secrets no GitHub
- [ ] Configurar SMTP_HOST e SMTP_PORT
- [ ] Adicionar SMTP_USER e SMTP_PASSWORD
- [ ] Adicionar emails dos sócios
- [ ] Testar com um push/PR/issue
- [ ] Verificar logs em Actions
- [ ] Validar recebimento de email

---

## 🐛 Troubleshooting

### ❌ "Repository not found"

**Solução:** Verifique se o workflow está no branch correto (master).

### ❌ "SMTP Authentication failed"

**Solução:**
- Verifique as credenciais
- Se usar Gmail, verifique a "Senha de Aplicação"
- Desabilite 2FA temporariamente para testar

### ❌ "Email not sent"

**Solução:**
- Verifique em `Actions → notify-partners → Logs`
- Teste o SMTP com um cliente de teste
- Verifique filtros de spam

### ❌ "Workflow not triggered"

**Solução:**
- Push para o branch configurado (master)
- Verifique em `Actions` se há erros
- O workflow aparecerá automaticamente após o push

---

## 📊 Monitoramento

### Ver Status do Workflow

1. Vá para `Code → Actions`
2. Selecione `notify-partners`
3. Veja o histórico de execuções

### Logs Detalhados

Clique em um workflow → "Send Notification Emails" para ver:
- ✅ Emails enviados com sucesso
- ❌ Erros de envio
- 📊 Detalhes de cada tentativa

---

## 🚀 Próximos Passos

- [ ] Implementar notificações para Slack
- [ ] Adicionar digest semanal de atividades
- [ ] Criar dashboard de notificações
- [ ] Adicionar preferências de notificação por sócio
- [ ] Integrar com Discord

---

## 📞 Suporte

Para dúvidas ou problemas:

1. Verificar os logs do workflow
2. Testar com um email pessoal
3. Validar as credenciais SMTP

---

**Criado em:** 2025-12-05
**Status:** ✅ Pronto para uso
