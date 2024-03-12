#[macro_use] extern crate rocket;
pub mod lib;
pub mod api_routes;
use lib::*;
pub mod chat;
use crate::chat::*;
use crate::api_routes::message_api;
pub fn all() -> Vec<rocket::Route> {
    routes![
        message_api::events,
        message_api::post,
        message_api::get_room_messages,
    ]
}

// 2do
// update app state from the event stream/main? (construct app state) vs doing it on the f.e
// send app_state to db periodically
// seed app state from db when server starts

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let chat_state = Arc::new(Mutex::new(ChatState {
        rooms: vec![Room{name: "lobby".to_string(), messages: Vec::new()}],
    }));

    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors()?;

    let _ = rocket::build()
        .manage(chat_state.clone())
        .manage(channel::<Message>(1024).0)
        .mount("/", all())
        .attach(cors)
        .launch()
        .await?;

    Ok(())
}