use serde::Serialize;
use sqlx::mysql;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Guess{
	id: i32,
    jogador: String,
	nacionalidade: String,
	posicao: String,
	equipe: String,
	competicao: String,
	idade: i32,
	partidas_jogadas: i32,
	gols_feitos: i32
}
impl Guess{
	pub fn get_id(&self) -> i32 { self.id }
}

#[derive(sqlx::FromRow, Serialize)]
pub struct NameQuery{
	jogador: String
}

pub async fn query_player(pool: &mysql::MySqlPool, name: &str) -> Result<Guess, sqlx::Error>{
	sqlx::query_as::<_,Guess>("select * from jogadores WHERE jogador = ?;")
		.bind(name)
		.fetch_one(pool).await
}

pub async fn query_player_names(pool: &mysql::MySqlPool, prefix: &str) -> Result<Vec<NameQuery>, sqlx::Error>{
	sqlx::query_as::<_, NameQuery>("select jogador from jogadores WHERE jogador LIKE ? LIMIT 10;")
		.bind(&format!("{}%", prefix))
		.fetch_all(pool).await
}

#[derive(sqlx::FromRow)]
pub struct Usuario{
	id: i32,
	acertou: i32
}
impl Usuario{
	pub fn get_id(&self) -> i32 { 
		self.id
	}
	pub fn daily_acertou(&self) -> bool {
		self.acertou != 0
	}
}

pub async fn query_user(pool: &mysql::MySqlPool, uuid: &str) -> Result<Usuario, sqlx::Error>{
	sqlx::query_as::<_, Usuario>("SELECT id, acertou FROM usuarios WHERE cookie = ?;")
		.bind(uuid)
		.fetch_one(pool)
		.await
}

pub async fn insert_user(pool: &mysql::MySqlPool, uuid: &str) -> Result<(), sqlx::Error>{
	sqlx::query("INSERT INTO usuarios (cookie, acertou) VALUES(?, 0);")
		.bind(uuid)
		.execute(pool)
		.await
		.map(|_| ())
}

pub async fn update_user_acertou(pool: &mysql::MySqlPool, id: i32) -> Result<(), sqlx::Error>{
	sqlx::query("UPDATE usuarios SET acertou = 1 WHERE id = ?;")
		.bind(id)
		.execute(pool)
		.await
		.map(|_| ())
}


pub async fn insert_guess(pool: &mysql::MySqlPool, user_id: i32, daily_id: i32, guess_id: i32) -> Result<(), sqlx::Error>{
	sqlx::query("INSERT INTO tentativas (usuario_id, jogador_diario_id, jogador_chutado_id) VALUES(?, ?, ?);")
		.bind(user_id)
		.bind(daily_id)
		.bind(guess_id)
		.execute(pool)
		.await
		.map(|_| ())
}


pub async fn get_user_daily_guesses(pool: &mysql::MySqlPool, day_player_id: i32, uuid: &str) -> Result<Vec<Guess>, sqlx::Error>{
	sqlx::query_as::<_, Guess>(
		"
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
			jogador_diario.id = ? AND
			usuarios.cookie = ?;
		"
	)
		.bind(day_player_id)
		.bind(uuid)
		.fetch_all(pool)
		.await
}


#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct DayPlayer{
	day_player_id: i32,
	id: i32,
    jogador: String,
	nacionalidade: String,
	posicao: String,
	equipe: String,
	competicao: String,
	idade: i32,
	partidas_jogadas: i32,
	gols_feitos: i32
}
impl DayPlayer{
	pub fn get_id(&self) -> i32 { self.day_player_id }
	pub fn get_player_id(&self) -> i32 { self.id }
}


pub async fn query_day_player(pool: &mysql::MySqlPool) -> Result<DayPlayer, sqlx::Error>{
	sqlx::query_as::<_, DayPlayer>(
		"
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
		"
	)
		.fetch_one(pool)
		.await
}

#[cfg(test)]
mod tests{
	use super::*;

	#[derive(Serialize, sqlx::FromRow)]
	pub struct Date{
		date: i32
	}

	#[actix_web::test]
	async fn corret_curr_date(){
		let db_url = std::env::var("URL").expect("Failed to parse URL from env");

		let pool_connection = mysql::MySqlPoolOptions::new()
			.max_connections(5)
			.connect(&db_url)
			.await
			.expect("Failed to connect to db");

		let date = sqlx::query_as::<_, Date>("SELECT curdate() + 0 as date;")
			.fetch_one(&pool_connection)
			.await;

		assert_eq!(20230911, date.unwrap().date);
	}
}