#[macro_use]
extern crate rocket;
use rocket::fs::{FileServer, relative};
use std::path::PathBuf;
use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::Template;
use rocket_dyn_templates::context;

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!("/", hello(name = "Your Name")))
}
#[get("/hello/<name>")]
pub fn hello(name: &str) -> Template {
    Template::render("templates/index", context! {
        title: "Hello",
        name: Some(name),
        items: vec!["One", "Two", "Three"],
    })
}
#[post("/clicked")]
pub fn button_clicked() -> Template {
    Template::render("templates/no_button", context! {
        message: "You Replaced the Button"
    })
}

#[post("/button")]
pub fn a_clicked() -> Template {
    Template::render("templates/button", context! {
        message: "You Got the button back"
    })
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render("templates/error/404", context! {
        uri: req.uri()
    })
}


// https://docs.rs/rocket/0.5.0-rc.2/rocket/fs/struct.FileServer.html#example
#[shuttle_runtime::main]
async fn rocket(#[shuttle_static_folder::StaticFolder(folder="templates")] _static_folder: PathBuf) -> shuttle_rocket::ShuttleRocket {

    let rocket = rocket::build().mount("/hello", routes![index])
        .mount("/", routes![index, hello, button_clicked, a_clicked])
        .mount("/", FileServer::from(relative!("templates/public")))
        .register("/", catchers![not_found])
        .attach(Template::fairing())
        ;
    Ok(rocket.into())
}
