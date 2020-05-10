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


use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://www.google.com")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}