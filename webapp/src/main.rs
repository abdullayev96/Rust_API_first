#[macro_use] extern crate rocket;

use std::fs::OpenOptions;
use std::io::Write; // .write() metodidan foydalanish uchun kerak
use rocket::serde::{Deserialize, json::Json};


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


#[get("/index")]
fn index1() -> &'static str {
    "Hello Rust web tizim "
}


#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Task<'r> {
    item: &'r str
}

#[post("/addtask", data="<task>")]
fn add_task(task: Json<Task<'_>>) -> &'static str {
    let mut tasks = OpenOptions::new()
                    .read(true)
                    .append(true)
                    .create(true)
                    .open("tasks.txt")
                    .expect("unable to access tasks.txt");   
    let task_item_string = format!("{}\n", task.item);
    let task_item_bytes = task_item_string.as_bytes();
    tasks.write(task_item_bytes).expect("unable to write to tasks.txt");
    "Task added succesfully"
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, index1, add_task])
   
}