use rocket_dyn_templates::{context, Template};
use rocket::{get, routes};

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        context! {
            name: "Hello",
            items: vec!["Example", "List", "Of", "Five", "Items"],
        },
    )
}

#[get("/<id>")]
pub fn get_post(id: usize) -> String {
    format!("Displaying post with id: {}", id)
}

// Export a vector of the route handlers
pub fn root() -> Vec<rocket::Route> {
    routes![index, get_post]
}
