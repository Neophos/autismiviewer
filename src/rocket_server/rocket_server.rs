use rocket_dyn_templates::Template;

use crate::routes;

pub fn launch() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes::root())
        .register("/", routes::errors())
        .attach(Template::fairing());

    Ok(rocket.into())
}
