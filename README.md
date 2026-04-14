# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

##  Descrição

Sistema de busca eficiente para e-commerce utilizando HashMap e Grafos para indexação e recomendação de produtos.

##  Tecnologias

* Rust
* HashMap
* Grafos (BFS)

## Como executar

```bash
cargo run
```

## Testes

```bash
cargo test
```

## Exemplos

* Buscar: "notebook"
* Recomendar produtos relacionados

## Arquitetura

* index.rs → busca rápida
* graph.rs → recomendações
* search.rs → lógica principal

## Performance

* Busca: O(1)
* Recomendação: O(n)