// extern crate reqwest;

// fn main() {
//     match reqwest::get("https://httpbin.org/ip") {
//         Ok(mut res) => {
//             match res.text() {
//             Ok(text) => println!("The response is {}", text),
//             Err(_) => print!("The error"),
//             }
//         }
//         Err(_) => print!("Last error"),
//             // Multi-threaded code-----------------------
//     }
// }

// use reqwest;
// use std::collections::HashMap;
// use async_std::task;

// fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
//     task::block_on(async {
//         let res= reqwest::blocking::get("https://google.com")?
//             .json::<HashMap<String, String>>()?;

//             // Single Threaded code

//         println!("{:#?}", res);
//         Ok(())
//     })
// }

// use std::collections::HashMap;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let resp = reqwest::get("http://www.google.com")
//         .await?
//         .json::<HashMap<String, String>>()
//         .await?;
//     println!("{:#?}", resp);
//     Ok(())
// }

// ---------------------Class 08------------
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

extern crate reqwest;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use std::path::Path;
extern crate rustc_serialize;
use rustc_serialize::json::Json;

use std::io::Read;

// openweathermapapi.com

// Web_Attributes or Web_Methods Get, Post, Put, Delete
// get request using rocket

#[get("/")]
// [rocket::get("/")]
fn hello() -> String {
    let path = Path::new("api.json");
    let display = path.display();
    println!("{:?} {:?}", path, display);

    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(_) => panic!("File could not create."),
    };

    match reqwest::get("http://jsonplaceholder.typicode.com/users") {
        Ok(mut response) => match response.text() {
            Ok(text) => match file.write_all(text.as_bytes()) {
                Ok(_) => println!("Data write in file!"),
                Err(_) => println!("The error is "),
            },
            Err(_) => println!("The response is not come from the server."),
        },
        Err(_) => println!("Server could not established the connecton."),
    }

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => panic!("{} file open Error: e.description()"),
    };

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap(); // unmanagable errors and we can't write text in it. (unwrap)

    let json = Json::from_str(&buffer).unwrap();
    let result = format!("The apis is {}", json[0].find_path(&["name"]).unwrap());

    // let result = format!(
    //     "The temperature of Karachi is {}",
    //     json.find_path(&["name"]).unwrap()
    // );

    result
}


use rocket::request::Form;



#[derive(FromForm)]
pub struct Todo {
    description: String
}

#[post("/", data = "<todo_form>")]
fn new(todo_form: Form<Todo>) -> Flash<Redirect> {
    let todo = todo_form.into_inner();
    if todo.description.is_empty() {
        Flash::error(Redirect::to("/"), "Description cannot be empty.")
    } else  {
        Flash::success(Redirect::to("/"), "Todo successfully added.")
    } 
}


// use rocket::Data;

// #[derive(FromForm)]
// struct Task{
//     description : String,
//     completed: bool
// }

// #[post("/", data="<task>")]
// fn new(task : Data)-> Result<String, std::io::Error> {
//     task.stream_to_file("upload.txt").map(|n| n.to_string())
// }

fn main() {
    rocket::ignite().mount("/", rocket::routes![hello]).launch();
}
