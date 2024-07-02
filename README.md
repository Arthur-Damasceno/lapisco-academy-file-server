# Lapisco Academy - File Server

## 🚀 Introdução
O **Lapisco Academy** é uma plataforma de cursos online, na qual professores do *Lapisco* podem publicar videoaulas. Neste repositório, reside a *API* responsável por lidar com o upload e download de arquivos.

## 📦 Requisitos
O projeto é desenvolvido na linguagem de programação [Rust](https://www.rust-lang.org/) com a biblioteca [Axum](https://github.com/tokio-rs/axum/). Para executar este projeto, é necessário ter:
 - [Rust](https://www.rust-lang.org/): O conjunto de compilador, gerenciador de pacotes e outras ferramentas da linguagem.
 - [Sqlx CLI](https://github.com/launchbadge/sqlx/): CLI para executar as migrations do banco de dados.
 - [Docker](https://www.docker.com): Para automatizar a execução e deploy do projeto

## 🔥 Executando
### Localmente
**1. Clone o projeto**
```bash
git clone https://github.com/lapisco/lapisco-academy-file-server.git
```
**2. Copie as variáveis de ambiente**
```
cp .env.example .env
```
**3. Execute as migrations do SQLite**
```bash
sqlx database create
sqlx migrate run
```
**4. Execute a aplicação**
```bash
cargo run
```

### Via Docker
**1. Clone o projeto**
```bash
git clone https://github.com/lapisco/lapisco-academy-file-server.git
```
**2. Execute a aplicação via Docker**
```bash
docker compose up -d
```
