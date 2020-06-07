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



// #[get("/num?<number>")]
// fn hello(number :u8) -> String {
//     format!("The number is {}!", number+2)
// }

use rocket::request::Form;
use rocket::response::content::Html;

#[derive(FromForm)]
struct Number{
    number : i32,
}


#[post("/num", data = "<number>")]
fn hello(number : Form<Number>) -> Html<String> {
   let h1=format!("
   <div style='font-family: Helvetica, Arail; text-align: center;'>
   <h1 style='color:white; padding: 10px; background-color:#9e00c5;'>Making Rust Server</h1>
   <p style='font-size: 24px; color:#770494; font-weight: bolder;'>
   The number you entered is {} and the result is :
   {}</p>
   
   </div>",number.number, number.number + 2.0);
    Html(h1)
}

#[get("/num")]
fn add()-> Html<String>{
    let h1=format!("
    <div style='font-family: Helvetica, Arail; text-align: center;'>
        <h1 style='color:white; padding: 10px; background-color:#9e00c5;'>Making Rust Server</h1>
        <p style='font-size: 20px;'>Enter a Number, It will reply you the number add with 2.</p>
        <form style='margin: 100px 0px; margin-bottom: 80px;' action='/num' method='post'>
            <input style='padding: 5px 10px; border-radius: 3px;  height: 40px;
            width: 500px; font-size: 24px; border: 1px solid #9e00c5;' type='number' placeholder='Enter number'
                name='number' id='number'>
            <input style='font-size: 24px; padding:5px 12px; 
            border: none; height: 50px;
            border-radius: 3px;  color: white; background-color: #9e00c5

            ;' type='Submit'>
        </form>
        
    </div>");
    Html(h1)
}

#[get("/")]
fn home()-> Html<String>{
    let html=format!("
    <div
        style='position: relative; font-family: Helvetica, Arail; text-align: center; width: 100%; height:100vh; overflow: hidden;'>
        
        <div
            style='position: absolute; top:0; background-color: #9e00c5; width: inherit; height: inherit; opacity: 0.3;'>
        </div>
        <div style='position: absolute; top:40%; left: 40%;
        '>
            <h1
                style='margin-bottom:  10px; font-size: 4rem; font-weight: bolder; color: white;  text-shadow: 2px 2px 4px #000000;'>
                Rust Server</h1>
            <a href='/num' style='padding: 10px 20px; margin-top: 10px; 
            font-size: 24px; color: white; background-color: #9e00c5; text-decoration: none; border-radius: 5px;'>Get
                Started</a>
        </div>");
    Html(html)
}


fn main() {
    rocket::ignite().mount("/", routes![home,add,hello]).launch();
}

// openweathermapapi.com

// Web_Attributes or Web_Methods Get, Post, Put, Delete
// get request using rocket

// #[get("/<number>")]
// fn add(number : i8)->String{
//     format!("The number you give is : {}", number+2)
// }


// #[get("/home")]
// // [rocket::get("/")]
// fn hello() -> String {
//     let path = Path::new("api.json");
//     let display = path.display();
//     println!("{:?} {:?}", path, display);

//     let mut file = match File::create(path) {
//         Ok(file) => file,
//         Err(_) => panic!("File could not create."),
//     };

//     match reqwest::get("http://jsonplaceholder.typicode.com/users") {
//         Ok(mut response) => match response.text() {
//             Ok(text) => match file.write_all(text.as_bytes()) {
//                 Ok(_) => println!("Data write in file!"),
//                 Err(_) => println!("The error is "),
//             },
//             Err(_) => println!("The response is not come from the server."),
//         },
//         Err(_) => println!("Server could not established the connecton."),
//     }

//     let mut file = match File::open(&path) {
//         Ok(file) => file,
//         Err(_) => panic!("{} file open Error: e.description()"),
//     };

//     let mut buffer = String::new();
//     file.read_to_string(&mut buffer).unwrap(); // unmanagable errors and we can't write text in it. (unwrap)

//     let json = Json::from_str(&buffer).unwrap();
//     let result = format!("The apis is {}", json[0].find_path(&["name"]).unwrap());

//     // let result = format!(
//     //     "The temperature of Karachi is {}",
//     //     json.find_path(&["name"]).unwrap()
//     // );

//     result
// }


// use rocket::request::Form;



// #[derive(FromForm)]
// pub struct Todo {
//     description: String
// }

// #[post("/", data = "<todo_form>")]
// fn new(todo_form: Form<Todo>) -> Flash<Redirect> {
//     let todo = todo_form.into_inner();
//     if todo.description.is_empty() {
//         Flash::error(Redirect::to("/"), "Description cannot be empty.")
//     } else  {
//         Flash::success(Redirect::to("/"), "Todo successfully added.")
//     } 
// }


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

// fn main() {
//     rocket::ignite().mount("/add", rocket::routes![add]).launch();
// }
