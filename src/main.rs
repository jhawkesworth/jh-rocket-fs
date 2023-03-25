#[macro_use]
extern crate rocket;
use rocket::fs::{FileServer, relative};
use std::path::PathBuf;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


// https://docs.rs/rocket/0.5.0-rc.2/rocket/fs/struct.FileServer.html#example
#[shuttle_runtime::main]
async fn rocket(#[shuttle_static_folder::StaticFolder] _static_folder: PathBuf) -> shuttle_rocket::ShuttleRocket {

    let rocket = rocket::build().mount("/hello", routes![index])
        //.mount("/public", FileServer::from("/opt/shuttle/shuttle-builds/jh-rocket-fs/static"))
        // ^ if do this, must first symlink the folder so you can cargo shuttle run locally
        .mount("/public", FileServer::from(relative!("static")));

    Ok(rocket.into())
}