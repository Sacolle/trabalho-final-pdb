# Playerdle
Este é o trabalho final da cadeira de Projeto de Banco de Dados do Instituto de Informática para o semestre 2023/1. 

A aplicação consiste em um jogo semelhante a worldle que deve se adivinhar qual o jogador secreto, tendo como feedback os atributos dos jogadores que se tentou adivinhar e errou. A cada dia há um novo jogador secreto para se adivinhar. O jogo faz track do progesso pelos cookies, ou seja, ele mantém o estado entre acessos.

Nesse trabahlo foram utilizadas as seguintes tecnologias:
 - [Rust](https://www.rust-lang.org/) para o backend
	- [Sqlx](https://lib.rs/crates/sqlx) para interagir com a DB
	- [Actix](https://actix.rs/) para o servidor HTTP
	- [Tera](https://keats.github.io/tera/) para os template
 - [MySql](https://www.mysql.com/) como o servidor relacional
 - [HTMX](https://htmx.org/) para realizar os request da página

Para executar o código, inialize uma variável de ambiente URL com o url de acesso a base de dados de acordo com a documentação da biblioteca sqlx.
