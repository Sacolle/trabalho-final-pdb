use pdb;

select * from jogadores;

select * from jogadores WHERE jogador = "Himad Abdelli";

SELECT jogador from jogadores Where jogador LIKE 'Himad Abdelli%' LIMIT 100;


INSERT INTO jogador_diario (jogador_id, dia) VALUES(123, CURDATE());

SELECT curdate() + 0 as date;

SELECT * FROM jogador_diario JOIN jogadores ON jogador_diario.jogador_id = jogadores.id  WHERE dia = CURDATE();

SELECT * FROM jogador_diario;

INSERT INTO usuarios (cookie) VALUES(uuid());

SELECT * FROM usuarios;

INSERT INTO tentativas (usuario_id, jogador_diario_id, jogador_chutado_id) VALUES(2, 6, 1651);

SELECT 
	jogadores.id as id,
    jogadores.jogador as jogador,
	nacionalidade,
	posicao,
	equipe,
	competicao,
	idade,
	partidas_jogadas,
	gols_feitos
FROM 
	usuarios 
		JOIN tentativas ON usuarios.id = tentativas.usuario_id
		JOIN jogador_diario ON jogador_diario.id = tentativas.jogador_diario_id
        JOIN jogadores ON jogadores.id = tentativas.jogador_chutado_id
	WHERE
		jogador_diario.id = 3 AND
        usuarios.cookie = "e34bc2e9-93f5-4711-a4ff-ccc4e73fec29";

SELECT 
		jogador_diario.id as day_player_id,
		jogadores.id as id,
		jogadores.jogador as jogador,
		nacionalidade,
		posicao,
		equipe,
		competicao,
		idade,
		partidas_jogadas,
		gols_feitos
	FROM 
		jogador_diario JOIN jogadores ON jogador_diario.jogador_id = jogadores.id
	ORDER BY
		jogador_diario.id DESC
	LIMIT 1;

UPDATE usuarios SET acertou = 1 WHERE id = 1;
