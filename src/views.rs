use rocket_contrib::templates::Template;
use std::collections::HashMap;
use crate::VALID_SHAPES;

#[derive(Serialize)]
struct HomeContext {
    page_title: &'static str,
    shapes: Vec<&'static str>
}

#[get("/home")]
pub fn home() -> Template {
    let context = HomeContext {
        page_title: "Primitive Web",
        shapes: VALID_SHAPES.to_vec()
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