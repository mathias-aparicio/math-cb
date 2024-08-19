// main.rs
mod markdown;
mod template;

use rocket::{self as rocket_lib, launch, routes, get};
use rocket_dyn_templates::{Template, context};

#[get("/")]
pub fn index() -> Template {
	Template::render("templates/index", context! {
		content: markdown::render_markdown("cours/index.md")
	})
}

#[launch]
fn rocket() -> _ {
	rocket_lib::build()
		.mount("/", routes![index])
		.attach(Template::fairing())
}



