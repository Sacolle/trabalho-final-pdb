use pdb;

create table jogadores(
	id INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
    jogador VARCHAR(100) NOT NULL,
	nacionalidade VARCHAR(3) NOT NULL,
	posicao VARCHAR(10) NOT NULL,
	equipe VARCHAR(100) NOT NULL,
	competicao VARCHAR(100) NOT NULL,
	idade INT NOT NULL,
	partidas_jogadas INT NOT NULL,
	gols_feitos INT NOT NULL
);

create table jogador_diario(
	id INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
    jogador_id INT NOT NULL,
    dia DATE NOT NULL UNIQUE,
    foreign key(jogador_id) references jogadores(id)
);

CREATE TABLE usuarios(
	id INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
    cookie VARCHAR(36) NOT NULL UNIQUE,
    acertou INT NOT NULL
);


CREATE TABLE tentativas(
	id INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
	usuario_id INT NOT NULL,
    jogador_diario_id INT NOT NULL,
    jogador_chutado_id INT NOT NULL,
    FOREIGN KEY(usuario_id) REFERENCES usuarios(id),
    FOREIGN KEY(jogador_diario_id) REFERENCES jogador_diario(id),
    FOREIGN KEY(jogador_chutado_id) REFERENCES jogadores(id)
);

