#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

extern crate nanoid;
extern crate rocket_multipart_form_data;
extern crate rocket_raw_response;

use rocket::Request;
use std::collections::HashMap;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use rocket::response::Redirect;
use std::{fs, env};
use rocket::fairing::AdHoc;

mod views;
mod api;

#[catch(404)]
fn not_found(req: &Request<'_>) -> Template {
    let mut map = HashMap::new();
    map.insert("page_title", "404 Not Found");
    map.insert("path", req.uri().path());
    Template::render("errors/404", &map)
}

#[get("/")]
fn base_redirect() -> Redirect {
    Redirect::to("/view/home")
}

fn main() {
    rocket::ignite()
        .attach(AdHoc::on_launch("Initialize temp dirs", |_| {
            let _res_input = fs::create_dir_all(env::temp_dir().join("primitive_web").join("input"));
            let _res_output = fs::create_dir_all(env::temp_dir().join("primitive_web").join("output"));
        }))
        .mount("/", routes![base_redirect])
        .mount("/view/", routes![views::home, views::result])
        .mount("/static/", StaticFiles::from("static"))
        .mount("/api/", routes![api::submit, api::is_ready, api::get_result, api::queue_size])
        .attach(Template::fairing())
        .register(catchers![not_found])
        .launch();
}