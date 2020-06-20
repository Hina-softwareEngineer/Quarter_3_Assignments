#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive; // 
#[macro_use] extern crate rocket_contrib; // maintain cors (attach)
#[macro_use] extern crate lazy_static; // store coming data


#[macro_use] extern crate rocket_cors; // originable

use std::sync::{Arc, Mutex}; // capture data coming through lazy statics
use std::collections::HashMap; // store data in hashmap
use rocket_contrib::json::{Json, JsonValue}; // store in json format
use rocket::http::Method; // Html Attributes (Http methods get, post)

// two different platforms
use rocket_cors::{
    AllowedHeaders,// wo kiya data le k aa raha hai kahan say aa rahi hai, 
    AllowedOrigins, Error,
    Cors, CorsOptions, // headers tells from where the request came
};

use rocket::State; // tells about server condition

type ID = usize;    // declaring globally
#[derive(Debug, PartialEq, Eq, Deserialize)] // data in the form of bytes, deserialization
struct Message{
    id : ID,
    contents: String
}


fn make_cors()->Cors{
    let allowed_origins=AllowedOrigins::some_exact(&[
        "https://best-ball.surge.sh/",// allow request from these
        "http://127.0.0.1:5500" // allow from local machine
    ]);
    CorsOptions{
        allowed_origins, 
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        allowed_headers:AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",
        ]),
        allow_credentials: true, // without user name and password
        ..Default::default()
    }
    .to_cors() // convert to cross origin
    .expect("Error while building the Cros")
}


#[get("/")]
fn hello()->JsonValue{
    json!([
        {
            "id":"01",
            "name":"Hina"
        },
        {
            "id":"02",
            "name":"Hina2"
        },
        {
            "id":"03",
            "name":"Hina3"
        },
        {
            "id":"04",
            "name":"Hina4"
        }

    ])
}


// Mutex for real time store data on server.
type MessageMap = Mutex<HashMap<ID, String>>;


#[post("/add", data="<user_input>")]
fn helloPost(user_input: Json<Message>, map:State<'_, MessageMap>)
{
    println!("{:?}", user_input.0.contents);
}

fn rocket()->rocket::Rocket{
    rocket::ignite()
        .mount("/", routes![hello, helloPost]).attach(make_cors())
        .manage(Mutex::new(HashMap::<ID, String>::new()))
}

fn main(){
    rocket().launch();
}



// What to do => Structures bases and object Oriented

// How to do => Functional Language ; SQL, Rust Closure


// Stateless protocol


// Stateful protocol