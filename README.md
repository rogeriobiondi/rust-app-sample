# Rust App Sample

Frontend WebAssembly desenvolvido em Rust com Yew e Bulma CSS, consumindo a API `rust-api-sample`.

## Tecnologias

- **Rust** - Linguagem de programação
- **Yew** - Framework frontend reativo para WebAssembly
- **Bulma** - Framework CSS moderno e responsivo
- **Trunk** - Build tool para aplicações Rust/WASM
- **gloo-net** - Cliente HTTP para WASM

## Funcionalidades

- ✅ CRUD completo de itens (Create, Read, Update, Delete)
- ✅ Busca por ID ou nome do produto
- ✅ Ordenação clicável nas colunas (ID, Nome, Preço)
- ✅ Indicadores visuais de ordenação (▲/▼)
- ✅ Paginação com navegação por números de página
- ✅ Seleção de itens por página (5, 10, 20, 50)
- ✅ Interface responsiva com Bulma CSS
- ✅ Navegação com menu hamburger para mobile

## Pré-requisitos

- [Rust](https://rustup.rs/) (1.70+)
- [Trunk](https://trunkrs.dev/) - `cargo install trunk`
- Target WASM - `rustup target add wasm32-unknown-unknown`
- API [rust-api-sample](https://github.com/rogeriobiondi/rust-api-sample) rodando

## Instalação

1. Clone o repositório:
```bash
git clone git@github.com:rogeriobiondi/rust-app-sample.git
cd rust-app-sample
```

2. Instale o Trunk (se ainda não tiver):
```bash
cargo install trunk
```

3. Adicione o target WASM:
```bash
rustup target add wasm32-unknown-unknown
```

## Executando

### Modo desenvolvimento
```bash
trunk serve
```

A aplicação estará disponível em `http://localhost:8080`

> **Nota:** Certifique-se de que a API `rust-api-sample` está rodando em `http://localhost:3000`

### Build de produção
```bash
trunk build --release
```

Os arquivos serão gerados em `dist/`. Para servir:
```bash
cd dist
python3 -m http.server 4000
# ou
npx serve .
```

## Estrutura do projeto

```
rust-app-sample/
├── Cargo.toml      # Dependências e configuração
├── Trunk.toml      # Configuração do Trunk
├── index.html      # HTML principal com Bulma CSS
└── src/
    ├── main.rs     # Entrypoint da aplicação
    └── lib.rs      # Componente App com toda a lógica
```

## Screenshots

### Lista de itens
- Tabela com ordenação clicável
- Busca por ID ou nome
- Paginação com números de página
- Seletor de itens por página

### Formulário
- Criação e edição de itens
- Validação de campos
- Feedback visual de erros

## API

Esta aplicação consome a API REST disponível em:
- **Repositório:** https://github.com/rogeriobiondi/rust-api-sample
- **Endpoint padrão:** `http://localhost:3000`

## Licença

MIT
