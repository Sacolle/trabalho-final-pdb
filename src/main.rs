mod queries;

use std::env;

use actix_files::NamedFile;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, HttpRequest, Result, cookie};
use lazy_static::lazy_static;
use tera::{Tera, Context};
use sqlx::{mysql::{self, MySql}, Pool};
use uuid::Uuid;

lazy_static! {
	pub static ref TEMPLATES: Tera = {
        let tera = match Tera::new("page/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera
    };
}
#[derive(serde::Deserialize)]
struct NameGuess{
	name: String
}

#[post("/guess")]
async fn guess_value(req: HttpRequest, web::Form(info): web::Form<NameGuess>, pool: web::Data<Pool<MySql>>) -> actix_web::Result<HttpResponse> {
	let Some(cookie) = req.cookie("playerdle") else {
		return Err(actix_web::error::ErrorBadRequest("user does not have the cookies"));
	};

	let user = queries::query_user(&pool, &cookie.value()).await
		.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

	if user.daily_acertou() {
		return Ok(HttpResponse::Ok().append_header(("HX-Trigger-After-Settle", "ja_acertou")).body(""));
	}

	let guess = queries::query_player(&pool, &info.name).await
		.map_err(|e| actix_web::error::ErrorBadRequest(e.to_string()))?;

	let day_player = queries::query_day_player(&pool).await
		.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

	let acerto = guess.get_id() == day_player.get_player_id();
	if acerto {
		let _ = queries::update_user_acertou(&pool, user.get_id()).await
			.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;
	}
	
	let _ = queries::insert_guess(&pool, user.get_id(), day_player.get_id(), guess.get_id()).await 
		.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

	let mut ctx = Context::new();
	ctx.insert("guess", &guess);
	ctx.insert("day_player", &day_player);

	match TEMPLATES.render("guess_row.html", &ctx){
		Ok(page) => Ok(
			if acerto {
				HttpResponse::Ok().append_header(("HX-Trigger-After-Settle", "acerto")).body(page)
			}else{
				HttpResponse::Ok().body(page)
			}
		),
		Err(err) => Err(actix_web::error::ErrorInternalServerError(err.to_string()))
	}
}

#[get("/autocomplete")]
async fn autocomplete(web::Query(info): web::Query<NameGuess>, pool: web::Data<Pool<MySql>>) -> actix_web::Result<HttpResponse>{
	let results = queries::query_player_names(&pool, &info.name).await;
	match results {
		Ok(res) => {
			let mut ctx = Context::new();
			ctx.insert("results", &res);

			match TEMPLATES.render("autocomplete_results.html", &ctx){
				Ok(page) => Ok(HttpResponse::Ok().body(page)),
				Err(err) => Err(actix_web::error::ErrorInternalServerError(err.to_string()))
			}
		},
		Err(err) => {
			Err(actix_web::error::ErrorBadRequest(err.to_string()))
		}
	}
}


#[get("/")]
async fn root(req: HttpRequest, pool: web::Data<Pool<MySql>>) -> actix_web::Result<HttpResponse> {
	if let Some(cookie) = req.cookie("playerdle"){
		//NOTE: validate the cookie?
		let day_player = queries::query_day_player(&pool).await
			.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

		let guesses = queries::get_user_daily_guesses(&pool, day_player.get_id(), cookie.value()).await 
			.map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

		let mut ctx = Context::new();
		ctx.insert("guesses", &guesses);
		ctx.insert("day_player", &day_player);

		match TEMPLATES.render("index.html", &ctx){
			Ok(page) => Ok(HttpResponse::Ok().body(page)),
			Err(err) => Err(actix_web::error::ErrorInternalServerError(err.to_string()))
		}
	}else{
		let ctx = Context::new();
		let user_cookie = Uuid::new_v4().to_string();

		// insert into the user table the new user
		if let Err(err) = queries::insert_user(&pool, &user_cookie).await {
			return Err(actix_web::error::ErrorInternalServerError(err.to_string()))
		}

		match TEMPLATES.render("index.html", &ctx){
			Ok(page) => Ok(
				HttpResponse::Ok()
					.cookie(cookie::Cookie::new("playerdle", user_cookie))
					.body(page)
				),
			Err(err) => Err(actix_web::error::ErrorInternalServerError(err.to_string()))
		}
	}
}

#[get("/styles.css")]
async fn serve_css() -> Result<NamedFile> {
	Ok(NamedFile::open("./page/styles.css")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let db_url = env::var("URL").expect("Failed to parse URL from env");

	let pool_connection = mysql::MySqlPoolOptions::new()
		.max_connections(5)
		.connect(&db_url)
		.await
		.expect("Failed to connect to db");

	HttpServer::new(move || {
        App::new()
			.app_data(
				web::Data::new(pool_connection.clone())
			)
            .service(root)
            .service(serve_css)
			.service(guess_value)
			.service(autocomplete)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests{
	use super::*;
	#[actix_web::test]
	async fn read_db(){
		let db_url = env::var("URL").expect("Failed to parse URL from env");

		let pool_connection = mysql::MySqlPoolOptions::new()
			.max_connections(5)
			.connect(&db_url)
			.await
			.expect("Failed to connect to db");

		assert!(queries::query_player(&pool_connection, "Himad Abdelli").await.is_ok());
	}

	#[actix_web::test]
	async fn gen_template(){
		let db_url = env::var("URL").expect("Failed to parse URL from env");

		let pool_connection = mysql::MySqlPoolOptions::new()
			.max_connections(5)
			.connect(&db_url)
			.await
			.expect("Failed to connect to db");

		let player_row = queries::query_player(&pool_connection, "Himad Abdelli").await;
		assert!(player_row.is_ok());
		let player_row = player_row.unwrap();

		let mut ctx = Context::new();
		ctx.insert("guess", &player_row);
		ctx.insert("day_player", &player_row);

		let gen = TEMPLATES.render("guess_row.html", &ctx);
		assert!(gen.is_ok());
	}
}
