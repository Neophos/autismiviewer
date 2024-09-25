use rocket_dyn_templates::tera::Tera;

pub fn create_templates() -> Tera {
    let mut tera = match Tera::new("../templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    tera.autoescape_on(vec![".html", ".sql"]);
    //tera.register_filter("do_nothing", do_nothing_filter);
    tera
}
