#![allow(clippy::map_clone)]
#[macro_use]
extern crate rocket;
mod clone;
mod git;
mod index;
mod repository;
mod utils;

use utils::config;

use rocket::fs::relative;
use std::path::Path;

use crate::repository::archive;
use crate::repository::blame;
use crate::repository::commit;
use crate::repository::log;
use crate::repository::raw;
use crate::repository::refs;
use crate::repository::summary;
use crate::repository::tree;
use crate::utils::own_pathbuf::PathBufWithDotfiles;
use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                file_server,
                robots,
                favicon,
                index::index,
                summary::repository,
                tree::tree_main,
                tree::tree,
                clone::clone,
                raw::raw,
                log::log_main,
                log::log,
                blame::blames,
                commit::commit,
                commit::patch,
                refs::refs,
                refs::refs_id,
                archive::archive
            ],
        )
        .attach(Template::fairing())
}

#[get("/static/<path..>")]
async fn file_server(path: PathBufWithDotfiles) -> Option<rocket::fs::NamedFile> {
    let path = Path::new(relative!("static")).join(path.get());
    rocket::fs::NamedFile::open(path).await.ok()
}

#[get("/robots.txt")]
async fn robots() -> Option<rocket::fs::NamedFile> {
    let path = Path::new(relative!("static")).join(Path::new("robots.txt"));
    rocket::fs::NamedFile::open(path).await.ok()
}

#[get("/favicon.ico")]
async fn favicon() -> Option<rocket::fs::NamedFile> {
    let mut path = dirs::config_dir().expect("Could not get the config directory");
    path.push("selfhut");
    path.push("favicon.ico");
    rocket::fs::NamedFile::open(path).await.ok()
}
