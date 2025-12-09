# AVL Cloud Platform · Meta Crate

Meta pacote oficial da Ávila Engineering que organiza as features do ecossistema cloud soberano.

> ⚠️ Enquanto os componentes individuais não são liberados no crates.io, este crate fornece **módulos placeholder** e uma função de inventário (`active_components`) para ver quais features foram compiladas.

## Instalação

```toml
[dependencies]
avila = "0.1.1"
```

Para habilitar determinados domínios:

```toml
[dependencies]
avila = { version = "0.1.1", features = ["full"] }
```

## API principal

- `avila::VERSION` – versão do meta pacote.
- `avila::platform::*` – metadados da plataforma.
- `avila::active_components()` – lista dinâmica de componentes habilitados.
- Módulos opcionais (ex.: `avila::compress`) expõem `SUMMARY` e `version()`.

## Ferramenta GUI

Execute o painel HTML para inspecionar as features ativas:

```bash
cargo run --bin active_components_gui --all-features
```

Isso gera `target/active_components.html` e abre o navegador automaticamente.

## Licença

MIT OR Apache-2.0, escolha a seu critério.
