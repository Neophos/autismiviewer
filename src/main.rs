pub mod rocket_server;
pub mod tera_templater;
pub mod routes;

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let _tera = tera_templater::create_templates();

    return rocket_server::launch();
}
