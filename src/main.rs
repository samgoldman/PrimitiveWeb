#![feature(proc_macro_hygiene, decl_macro)]

mod views;
mod api;
mod primitive_request;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;

extern crate nanoid;
extern crate rocket_multipart_form_data;
extern crate rocket_raw_response;
extern crate serde_json;

use crossbeam_queue::SegQueue;
use rocket::{Request, Rocket};
use std::collections::HashMap;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use rocket::response::Redirect;
use std::{fs, env, thread};
use rocket::fairing::AdHoc;
use std::time::{Duration, SystemTime};
use primitive_request::PrimitiveRequest;
use primitive_image::primitive_image::PrimitiveImage;

const FILE_LIFETIME: u64 = 60;

pub const VALID_SHAPES: [&str; 6] = ["TRIANGLE", "CUBIC", "QUADRATIC", "RECTANGLE", "ELLIPSE", "MIXED"];
pub const MAX_IMAGE_SIZE: u64 = 32; // MB

pub const NUM_SHAPES_DEFAULT: u32 = 500;
pub const MAX_AGE_DEFAULT: u32 = 100;
pub const SCALE_TO_DEFAULT: u32 = 100;
pub const SEED_DEFAULT: u32 = 0;
pub const SHAPE_DEFAULT: &str = VALID_SHAPES[0];

pub const NUM_SHAPES_MAX: u32 = 2000;
pub const MAX_AGE_MAX: u32 = 200;

lazy_static! {
    pub static ref Q: SegQueue<PrimitiveRequest> = SegQueue::<PrimitiveRequest>::new();
}

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

fn primitive_worker() {
    loop {
        match (Q).pop() {
            Ok(request) => {
                let mut image = PrimitiveImage::from_path(request.input_file_path.clone(), request.scale_to, None);

                primitive_image::runner::run(&mut image, request.num_shapes, request.max_age, request.seed.into(), request.shape);

                image.save_to_svg(env::temp_dir().join("primitive_web").join("output").join(request.request_id.clone() + ".svg"));

                fs::remove_file(request.input_file_path.clone()).unwrap();
            }
            Err(_err) => {}
        }

        thread::sleep(Duration::from_millis(100));
    }
}

fn cleanup_worker() {
    loop {
        let paths = fs::read_dir(env::temp_dir().join("primitive_web").join("output")).unwrap();
        let now = SystemTime::now();
        for path_result in paths {
            let path = path_result.unwrap().path();
            let metadata = fs::metadata(path.clone()).unwrap();

            if let Ok(time) = metadata.created() {
                if let Ok(duration) = now.duration_since(time) {

                    if duration.as_secs() > FILE_LIFETIME * 60 {
                        fs::remove_file(path.clone()).unwrap();
                    }

                }
            } else {
                println!("Created is not supported!");
            }
        }

        thread::sleep(Duration::from_secs(60));
    }
}

fn main() {
    let r: Rocket = rocket::ignite()
        .attach(AdHoc::on_launch("Initialize temp dirs", |_| {
            let _res_input = fs::create_dir_all(env::temp_dir().join("primitive_web").join("input"));
            let _res_output = fs::create_dir_all(env::temp_dir().join("primitive_web").join("output"));
        }))
        .mount("/", routes![base_redirect])
        .mount("/view/", routes![views::home, views::result])
        .mount("/static/", StaticFiles::from("static"))
        .mount("/api/", routes![api::submit, api::check_status, api::get_result, api::queue_size])
        .attach(Template::fairing())
        .manage(&Q)
        .register(catchers![not_found]);

    thread::spawn(primitive_worker);
    thread::spawn(cleanup_worker);

    r.launch();
}