# Vizzio Unified Platform

A Vizzio é uma plataforma soberana construída para operar **todo o stack de dados, IA, BIM, GIS, segurança e alto desempenho** sem depender de bibliotecas externas. Este monorepositório reúne os blocos fundamentais em Rust – divididos entre as famílias `avila-*`, `avx-*` e `avl-*` – além de documentação completa, artefatos de build e dados de exemplo.

> "Não dependemos de ninguém. Construímos tudo do zero, do bit ao pixel."
> — Blueprint Avila Inc.

---

## ✨ Destaques

- **128 crates Rust** publicados dentro do workspace (`cargo metadata` contabiliza tudo automaticamente).
  - `avila-*`: 108 crates de base matemática, criptografia, redes, dados, ML e GIS.
  - `avx-*`: 11 crates voltadas para GPU, computação vetorial e orquestração de alto desempenho.
  - `avl-*`: 7 crates de serviços de plataforma (auth, storage, filas, observabilidade, etc.).
- **Zero dependências externas por padrão** – os módulos se referenciam apenas entre si.
- **Documentação extensa** em `docs/`, cobrindo arquitetura, configuração, roadmap e guias de integração.
- **Dados IFC reais** no repositório para validar pipelines BIM.
- **Blueprint completo** (`COPILOT_BLUEPRINT_COMPLETE_STACK.md`) guiando a expansão da stack para 100% de cobertura.

---

## 🗂️ Organização Geral

```
d:\Vizzio
├── Cargo.toml                 # Workspace Rust com todos os crates
├── Cargo.lock
├── crates/                    # 128 crates agrupados por famílias (avila, avx, avl)
├── avila/                     # Espaços experimentais específicos (ex.: avila-vizzio)
├── docs/                      # Documentação oficial da plataforma
├── target/                    # Artefatos de build (gitignored)
├── .env / .env.example        # Variáveis de ambiente padrão
├── .github/                   # Pipelines e automações
├── .vscode/                   # Configurações de editor
├── ELE - ... .ifc             # Arquivos IFC elétricos de referência
└── VZZ086_... .ifc            # Arquivo IFC estrutural completo
```

### 📚 Documentação Interna

A pasta `docs/` traz tudo o que você precisa para navegar pelo ecossistema:

- `START_HERE.md`, `00_LEIA_PRIMEIRO.md` – guias de entrada.
- `ARCHITECTURE.md`, `STRUCTURE_VISUAL.md` – visão arquitetural e mapa visual do repo.
- `CONFIGURATION.md`, `INTEGRATION_GUIDE.md` – preparação de ambiente e integrações.
- `ROADMAP.md`, `IMPLEMENTATION_COMPLETE.md` – planejamento e status por camada.
- `ENV_VARS.md` – catálogo de variáveis de ambiente.

Use `docs/README.md` como índice navegável.

### 🧭 Blueprint estratégico

`COPILOT_BLUEPRINT_COMPLETE_STACK.md` detalha item a item a meta de implementar uma stack 100% soberana. Cada bloco descreve objetivos, critérios de qualidade, prioridades de sprint e filosofia da Avila Inc.

---

## 🧱 Pillars do Código

| Área | Crates de Referência | O que oferecem |
|------|----------------------|----------------|
| **Fundação & Runtime** | `avila-alloc`, `avila-atom`, `avila-buffer`, `avila-error`, `avila-future`, `avila-log`, `avila-sync` | Primitivos de memória, mutabilidade interior, sistema de erros, runtime async e infraestrutura de logging. |
| **Criptografia & Segurança** | `avila-crypto`, `avila-hash`, `avila-jwt`, `avila-kdf`, `avila-mac`, `avila-pki`, `avila-post-quantum`, `avila-signature`, `avila-zkp`, `avila-onion-routing`, `avila-mpc` | Criptografia clássica e pós-quântica, autenticação, PKI, zero-knowledge, MPC e roteamento seguro. |
| **Rede & Sistemas Distribuídos** | `avila-async`, `avila-http`, `avila-grpc`, `avila-dns`, `avila-proxy`, `avila-distributed-system`, `avila-gossip`, `avila-election`, `avila-partition`, `avila-lease`, `avila-lock` | Pilha de comunicação completa (HTTP/2, gRPC, DNS, proxy), primitivos de coordenação e consenso. |
| **Dados & Serialização** | `avila-serde`, `avila-codec`, `avila-arrow`, `avila-dataframe`, `avila-db`, `avila-compress`, `avila-crdt`, `avila-optimizer` | Engine de serialização, compressão, bancos de dados, dataframes e replicação livre de conflitos. |
| **Matemática, ML & HPC** | `avila-math`, `avila-linalg`, `avila-ndarray`, `avila-fft`, `avila-ml`, `avila-optimizer`, `avila-finite-fields`, `avila-prime`, `avila-bignum` | Núcleo científico para álgebra linear, FFT, autograd, otimização e aritmética de alta precisão. |
| **Geo, BIM & Visualização** | `avila-geo`, `avila-image`, `avila-gis-desktop`, `avila-vision`, `avila-gltf`, `avila-mesh`, arquivos IFC em `root` | GIS completo com projeções, pipelines de renderização, visão computacional e suporte a dados 3D. |
| **Avx – GPU & Orquestração** | `avx-gpu/*`, `avx-runtime`, `avx-http`, `avx-events`, `avx-api-core`, `avx-config`, `avx-telemetry`, `avx-intelligence`, `avx-mcp`, `avx-cli` | Núcleo de computação vetorial, compilação de shaders, runtime GPU, telemetria e ferramentas de orquestração. |
| **Avl – Serviços de Plataforma** | `avl-auth`, `avl-storage`, `avl-queue`, `avl-loadbalancer`, `avl-observability`, `avl-secrets`, `avl-console` | Serviços utilitários (auth, storage S3-like, filas, LB, observabilidade, secret management). |

Cada crate mantém seu próprio `Cargo.toml` minimalista e segue o princípio de **não depender de crates externos** a não ser que explicitado.

---

## 🚀 Primeiros Passos

1. **Requisitos**
   - Rust toolchain 1.75+ (`rustup` recomendado).
   - PowerShell 5.1+ (já incluso no Windows) ou um shell equivalente.
   - `git` para versionamento.
2. **Clonar o repositório**
   - `git clone https://.../vizzio.git` (URL da Avila Inc.).
3. **Configurar variáveis**
   - Copie `.env.example` para `.env` e ajuste valores conforme necessário.
   - Consulte `docs/ENV_VARS.md` para a lista completa.
4. **Build completo**
   - No PowerShell: `cargo build --workspace --release` (o primeiro build compila todos os 128 crates).
5. **Testes**
   - `cargo test --workspace` executa as suítes unitárias. Alguns módulos possuem benchmarks específicos em `benches/`.
6. **Exemplos práticos**
   - Geo: `cargo run -p avila-geo --example world_map`
   - GPU: consulte `crates/avx-gpu/examples/` para pipelines CUDA/Vulkan.
   - Storage: `cargo run -p avl-storage --example basic_upload` (exemplo de S3 compatível).

> **Dica:** para builds iterativos, utilize `cargo check -p <crate>` ou `cargo test -p <crate>` focando apenas no módulo em desenvolvimento.

---

## 🧪 Dados de Referência BIM/GIS

Dois arquivos IFC completos acompanham o repositório e podem ser usados em pipelines de parsing, renderização e análise (ver `docs/ENV_VARS.md` para configurações padrão):

- `ELE - VZZ086_25 - Magnussão - Res. Heitor - REV01-4.ifc`
- `VZZ086_25 Magnussão - Res. Heitor - Estrutural Executivo - Rev08.ifc`

Esses insumos ajudam a validar as bibliotecas de geometria (`avila-geo`, `avila-gis-desktop`) e quaisquer pipelines de BIM que você montar com os crates Avila/Avx.

---

## 🧭 Fluxo de Desenvolvimento Recomendado

1. Leia `docs/START_HERE.md` e `docs/ARCHITECTURE.md` para se situar.
2. Abra `COPILOT_BLUEPRINT_COMPLETE_STACK.md` para entender prioridades de implementação.
3. Escolha um crate dentro de `crates/` e confira o `src/` correspondente.
4. Desenvolva com foco em:
   - Cobertura de testes (>80% quando viável).
   - Documentação (`//!` headers + `cargo doc`).
   - Benchmarks (quando aplicável, via Criterion ou harness próprio).
5. Utilize `docs/IMPLEMENTATION_COMPLETE.md` para marcar status de entrega.

Cada crate deve permanecer **100% auditável**, com foco em segurança, performance e previsibilidade.

---

## 📈 Roadmap e Status

- Consulte `docs/ROADMAP.md` para o planejamento macro (sprints temáticos por camada).
- `docs/IMPLEMENTATION_COMPLETE.md` e `docs/UNIFIED_COMPLETE.md` rastreiam o progresso de implementação por módulo.
- As prioridades seguem a sequência do blueprint (primitivos → segurança → networking → dados → ML/BIM → UI).

---

## 🤝 Contribuindo

1. Crie uma branch (`git checkout -b feature/minha-feature`).
2. Desenvolva seguindo as diretrizes acima (sem deps externas, testes + docs).
3. Execute `cargo fmt` e `cargo clippy` caso estejam configurados no crate.
4. Abra um Pull Request descrevendo o objetivo e linkando a seção relevante do blueprint/roadmap.

Para dúvidas internas, contate a equipe Avila via canais padrão ou consulte `docs/README_VIZZIO.md`.

---

## 📜 Licença

Este repositório segue a política interna da Avila Inc. Consulte os arquivos de licença individuais em cada crate (`LICENSE-APACHE`, `LICENSE-MIT` ou equivalentes) quando aplicável.

---

**Vizzio Unified Platform** – stack soberana, performática e extensível. Desenvolva, analise e opere todo o ciclo de dados e inteligência sem depender de terceiros.
