use rocket_contrib::templates::Template;
use std::collections::HashMap;
use crate::{VALID_SHAPES, MAX_IMAGE_SIZE, NUM_SHAPES_DEFAULT, NUM_SHAPES_MAX, MAX_AGE_DEFAULT, MAX_AGE_MAX, SCALE_TO_DEFAULT, SEED_DEFAULT};

#[derive(Serialize)]
struct HomeContext {
    page_title: &'static str,
    shapes: Vec<&'static str>,
    max_image_size: u64,
    default_num_shapes: u32,
    max_num_shapes: u32,
    default_max_age: u32,
    max_max_age: u32,
    default_scale_to: u32,
    default_seed: u64
}

#[get("/home")]
pub fn home() -> Template {
    let context = HomeContext {
        page_title: "Primitive Web",
        shapes: VALID_SHAPES.to_vec(),
        max_image_size: MAX_IMAGE_SIZE,
        default_num_shapes: NUM_SHAPES_DEFAULT,
        max_num_shapes: NUM_SHAPES_MAX,
        default_max_age: MAX_AGE_DEFAULT,
        max_max_age: MAX_AGE_MAX,
        default_scale_to: SCALE_TO_DEFAULT,
        default_seed: SEED_DEFAULT
    };

    Template::render("home", &context)
}

#[get("/result/<request_id>")]
pub fn result(request_id: String) -> Template {
    let mut map = HashMap::new();
    map.insert("page_title", "Primitive Web Result");
    map.insert("request_id", request_id.as_ref());

    Template::render("result", &map)
}