# Budget Calculator — Regra 50/30/20

<img src="https://www.rust-lang.org/logos/rust-logo-blk.svg" alt="Rust Logo" width="180" height="180">

<img src="https://rust-lang.org/static/images/ferris.gif" alt="Rust Learn" width="500" height="180">

Aplicação em Rust que calcula a distribuição de orçamento mensal com base na regra 50/30/20:
- 50% Necessidades
- 30% Desejos
- 20% Poupança

Inclui uma API HTTP (Axum) e uma interface estática simples servida pela própria aplicação.

## Visão Geral

- Backend: `axum`, `tokio`, `serde`, `tower-http`
- Configuração: `dotenvy` (variáveis de ambiente via `.env`)
- Front-end: arquivos estáticos em `static/` (`index.html`, `styles.css`, `app.js`)
- Binário: executável via `cargo run`
- Rotas:
  - GET `/calculate?income=<valor>`
  - POST `/api/budget` com JSON `{"income": <valor>}`
  - Interface web em `/` (serve `static/index.html`)

## Requisitos

- Rust (toolchain estável): `rustup` + `cargo`
- Opcional: arquivo `.env` para configurar porta/host

## Como Rodar

1. Instale dependências do Rust:
   ```bash
   rustup update