# 🔐 Configuração de Secrets - GitHub Actions

**Guia para configurar credenciais de email no GitHub**

---

## 📋 Secrets a Configurar

Vá para: **https://github.com/avilainc/vizzio/settings/secrets/actions**

Clique em **"New repository secret"** e adicione cada um dos secrets abaixo:

### 📧 SMTP Configuration (Porkbun)

```
Name: SMTP_HOST
Value: smtp.porkbun.com
```

```
Name: SMTP_PORT
Value: 587
```

```
Name: SMTP_USER
Value: dev@avila.inc
```

```
Name: SMTP_PASSWORD
Value: 7Aciqgr7@3278579
```

---

### 👥 Parceiro 1 - Nicolas

```
Name: PARTNER_1_EMAIL
Value: nicolas@avila.inc
```

```
Name: PARTNER_1_NAME
Value: Nicolas
```

---

### 👥 Parceiro 2 - Marcelo Savazzi

```
Name: PARTNER_2_EMAIL
Value: marcelosavazzi1@gmail.com
```

```
Name: PARTNER_2_NAME
Value: Marcelo Savazzi
```

---

### 👥 Parceiro 3 - Rafael Ochiussi

```
Name: PARTNER_3_EMAIL
Value: rafaelochiussi@hotmail.com
```

```
Name: PARTNER_3_NAME
Value: Rafael Ochiussi
```

---

## ✅ Verificar Configuração

Após adicionar todos os secrets:

1. Vá para: **https://github.com/avilainc/vizzio/settings/secrets/actions**
2. Você deve ver **8 secrets** listados:
   - ✅ SMTP_HOST
   - ✅ SMTP_PORT
   - ✅ SMTP_USER
   - ✅ SMTP_PASSWORD
   - ✅ PARTNER_1_EMAIL
   - ✅ PARTNER_1_NAME
   - ✅ PARTNER_2_EMAIL
   - ✅ PARTNER_2_NAME
   - ✅ PARTNER_3_EMAIL
   - ✅ PARTNER_3_NAME

---

## 🚀 Testar Notificações

### Local (via avila-cell)

```powershell
# Carregar .env.local e testar
cd D:\Vizzio
.\test-notifications.ps1
```

### Automático (GitHub Actions)

```bash
# Fazer um push para disparar
git add .
git commit -m "Test: GitHub Actions notification workflow"
git push origin master

# Verificar em: https://github.com/avilainc/vizzio/actions
```

---

## 📞 Suporte

Se houver erro:

1. **"Connection refused"** → Verificar SMTP_HOST e SMTP_PORT
2. **"Authentication failed"** → Verificar SMTP_USER e SMTP_PASSWORD
3. **"Email not received"** → Verificar PARTNER_*_EMAIL (formato correto)
4. **Workflow não executa** → Verificar se secrets estão visíveis em Actions → Workflow run

---

**Data:** 5 de dezembro de 2025  
**Status:** Pronto para configurar
