mod hashing;

use rocket::response::content::RawHtml;
use url::{Url, ParseError, form_urlencoded};
use urlencoding::decode;
use rocket::{Build, Rocket};



#[macro_use]
extern crate rocket;
extern crate core;

#[get("/")]
fn index() -> RawHtml<String> {
    RawHtml(include_str!("index.html").to_string())
}

#[post("/", data = "<url>")]
fn submit(url: String) -> RawHtml<String> {
    let decoded = decode(&url).unwrap().to_string();
    let mut splitted: Vec<&str> = decoded.split("url=").collect();
    let hash_code = hashing::generate_hash();
    hashing::add_link(hash_code.to_string(), splitted[1].to_string());
    let href = format!("<a href = https://link-shortener-shuttle.shuttleapp.rs/{}>Der kurze Link</a>", hash_code.to_string());
    let link = format!("<p>https://link-shortener-shuttle.shuttleapp.rs/{}</p>", hash_code.to_string());
    RawHtml(href.as_str().to_owned() + link.as_str())
}


#[get("/<link>")]
fn resolve_short_link(link: String) -> RawHtml<String> {
    let real_link = hashing::load_link(link.to_string());
    RawHtml(format!("<meta http-equiv='refresh' content='0; URL={}'>", real_link.as_str()))
}

#[shuttle_service::main]
async fn rocket() -> Result<Rocket<Build>, shuttle_service::Error> {
    Ok(
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![resolve_short_link])
        .mount("/", routes![submit])
    )
}


