#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::Template;

fn main() {
    rocket::ignite().attach(Template::fairing()).launch();
}
