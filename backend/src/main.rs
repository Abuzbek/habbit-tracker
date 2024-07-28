#[macro_use] extern crate rocket;
use rocket::fairing::AdHoc;
use surrealdb::Surreal;
use surrealdb::engine::local::Db;

fn main() {
    println!("Hello, world!");
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::on_ignite("Database Config", |rocket| async {
            let db = Surreal::new::<Db>("mem").await.unwrap();
            db.use_ns("habittracker").use_db("main").await.unwrap();
            rocket.manage(db)
        }))
        .mount("/", routes![index])
}

