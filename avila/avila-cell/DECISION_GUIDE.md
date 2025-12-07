# 🚀 Stack Completo para Gmail SMTP com TLS

## 📦 O que você vai precisar implementar:

### 1. **avila-tls** (Pacote Principal - ~5.000 linhas)
Implementação TLS 1.3 nativa do zero.

**Componentes Core:**
- ✅ `record/` - TLS Record Layer Protocol
- ✅ `handshake/` - ClientHello, ServerHello, Finished
- ✅ `cipher_suite/` - AES-GCM, ChaCha20-Poly1305
- ✅ `extensions/` - SNI, ALPN, KeyShare
- ✅ `alert/` - TLS Alerts
- ✅ `session/` - Session resumption & 0-RTT

### 2. **avila-crypto** (Extensões - ~2.000 linhas)

**Adicionar:**
```rust
├── kdf/hkdf.rs              # HMAC-based Key Derivation (RFC 5869)
├── aead/aes_gcm.rs          # AES-GCM AEAD cipher
├── aead/chacha20_poly1305.rs # ChaCha20-Poly1305
└── mac/hmac.rs              # HMAC para HKDF
```

### 3. **avila-molecule** (Network - ~500 linhas)

**Adicionar suporte TLS:**
```rust
pub struct TcpClient {
    // ... existing
    tls: Option<TlsConnection>,
}

impl TcpClient {
    pub async fn connect_tls(addr, config) -> Result<Self>
    pub async fn upgrade_to_tls(&mut self) -> Result<()>  // Para STARTTLS
}
```

### 4. **avila-cell** (Já existe, atualizar ~200 linhas)

**Modificações no SMTP:**
```rust
impl SmtpClient {
    // Já temos isso, só precisamos ativar
    pub async fn connect_with_security(addr, SmtpSecurity::StartTls)
    pub async fn starttls(&mut self) -> Result<()>
}
```

---

## 📊 Estimativa de Esforço

### Opção 1: Implementação Completa (Zero-to-Hero)
**Tempo:** 6-8 semanas
**Linhas:** ~8.000
**Complexidade:** 🔥🔥🔥🔥🔥

**Você terá:**
- TLS 1.3 totalmente nativo
- Zero dependências externas para TLS
- Controle total sobre segurança
- Post-quantum ready
- Performance otimizada

**Ideal para:**
- Aprendizado profundo de TLS
- Controle total sobre stack
- Produto de longo prazo

### Opção 2: Híbrida (Usar rustls temporário)
**Tempo:** 1-2 semanas
**Linhas:** ~500
**Complexidade:** 🔥🔥

**Você terá:**
- SMTP funcional com Gmail AGORA
- TLS via rustls (battle-tested)
- Pode substituir depois

**Implementação:**
```toml
[dependencies]
rustls = "0.21"
tokio-rustls = "0.24"
```

```rust
use tokio_rustls::TlsConnector;

impl TcpClient {
    pub async fn upgrade_to_tls(&mut self) -> Result<()> {
        let connector = TlsConnector::from(Arc::new(config));
        self.stream = connector.connect(domain, self.stream).await?;
        Ok(())
    }
}
```

### Opção 3: Usar Windows Native (Schannel)
**Tempo:** 2-3 semanas
**Linhas:** ~1.000
**Complexidade:** 🔥🔥🔥

**Você terá:**
- TLS usando Schannel (Windows nativo)
- Zero dependências Rust
- Integração com OS

---

## 🎯 Recomendação Pragmática

### Para ter Gmail funcionando **HOJE**:

```rust
// Passo 1: Adicionar rustls temporariamente
[dependencies]
rustls = "0.21"
webpki-roots = "0.25"

// Passo 2: Implementar em avila-molecule
use rustls::{ClientConfig, ServerName};

pub async fn upgrade_to_tls(&mut self) -> Result<()> {
    let config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    // Wrap o TcpStream existente
    let connector = TlsConnector::from(Arc::new(config));
    let domain = ServerName::try_from(server_name)?;

    self.tls_stream = Some(connector.connect(domain, self.stream).await?);
    Ok(())
}

// Passo 3: Usar no SMTP
impl SmtpClient {
    pub async fn connect_gmail(email: &str, password: &str) -> Result<Self> {
        let mut client = Self::connect_with_security(
            NetworkAddress::new("smtp.gmail.com", 587),
            SmtpSecurity::StartTls
        ).await?;

        client.ehlo("avila.inc").await?;
        // STARTTLS já ativa automaticamente
        client.auth_plain(email, password).await?;

        Ok(client)
    }
}
```

### Depois, migrar gradualmente:

**Fase 1** (Mês 1-2): Usar rustls, funcional
**Fase 2** (Mês 3-4): Implementar avila-tls básico
**Fase 3** (Mês 5-6): Features avançadas (0-RTT, PQ)
**Fase 4** (Mês 7+): Performance tuning & security audit

---

## 💰 Trade-offs

### Opção 1 (Full Native):
**Prós:**
- ✅ Controle total
- ✅ Zero dependências
- ✅ Aprendizado máximo
- ✅ Customização infinita

**Contras:**
- ❌ 2+ meses de trabalho
- ❌ Complexidade altíssima
- ❌ Precisa de security audit
- ❌ Manutenção contínua

### Opção 2 (rustls):
**Prós:**
- ✅ Funciona em 1 semana
- ✅ Battle-tested
- ✅ Mantido pela comunidade
- ✅ Security audited

**Contras:**
- ❌ Dependência externa
- ❌ Menos controle
- ❌ ~500KB de código

### Opção 3 (Schannel):
**Prós:**
- ✅ Nativo do Windows
- ✅ Zero deps Rust
- ✅ Updates do OS

**Contras:**
- ❌ Windows-only
- ❌ API complexa
- ❌ Menos controle

---

## 🎬 Próxima Ação

**Você decide:**

**A)** "Vamos com rustls agora, Gmail funcionando hoje!"
→ Implemento em 30min

**B)** "Implementa TLS nativo do zero, tenho tempo"
→ Começo pelo Record Layer e Handshake

**C)** "Usa Schannel (Windows native)"
→ Implemento com FFI para Windows

**O que você prefere?** 🤔
