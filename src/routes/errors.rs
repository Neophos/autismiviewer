use rocket_dyn_templates::{context, Template};
use rocket::{catch, catchers};

#[catch(500)]
fn error_500() -> Template {
    Template::render(
        "error",
        context! {
            error: "500 gg"
        },
    )
}

#[catch(404)]
fn error_404() -> Template {
    Template::render(
        "error",
        context! {
            error: "404 gg"
        },
    )
}

#[catch(default)]
fn error_unspecified() -> Template {
    Template::render(
        "error",
        context! {
            error: "no idea lol gg"
        },
    )
}


// Export a vector of the route handlers
pub fn errors() -> Vec<rocket::Catcher> {
    catchers![error_500, error_404, error_unspecified]
}
