use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/home")]
pub fn home() -> Template {
    let mut map = HashMap::new();
    map.insert("page_title", "Primitive Web");

    Template::render("home", &map)
}

#[get("/result/<request_id>")]
pub fn result(request_id: String) -> Template {
    let mut map = HashMap::new();
    map.insert("page_title", "Primitive Web Result");
    map.insert("request_id", request_id.as_ref());

    Template::render("result", &map)
}