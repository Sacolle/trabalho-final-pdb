DROP PROCEDURE IF EXISTS r_player;
DROP PROCEDURE IF EXISTS proc_daily;
DROP EVENT IF EXISTS day_event;

DELIMITER //
CREATE PROCEDURE r_player(OUT i INT) NOT DETERMINISTIC
	BEGIN
		SELECT COUNT(id)*RAND() into i from pdb.jogadores;
	END //

DELIMITER //
CREATE PROCEDURE proc_daily() NOT DETERMINISTIC
	BEGIN
		CALL r_player(@i);
		INSERT INTO jogador_diario (jogador_id, dia) VALUES(@i, CURDATE());
		UPDATE usuarios SET acertou = 0;
	END //

CREATE EVENT day_event ON SCHEDULE EVERY 24 HOUR DO CALL proc_daily();

#see if the event schedules is on
SHOW PROCESSLIST;