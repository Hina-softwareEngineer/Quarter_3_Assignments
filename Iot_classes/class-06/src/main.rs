
use async_std::task;
use surf;
use futures::try_join;

// async fn fetch(url: &str) -> Result <String,surf::Exception>{
//     // surf.get("").receive_string()   Synchronous
//     surf::get(url).recv_string().await  // Asynchronous
// }

// async fn exce() {
//     match fetch("https://www.piaic.org/").await{  //synchronously
//         Ok (res) => print!("{}", res),
//         Err (e) => println!("Error {}",e),
//     }
// }

// fn main() {
//     task::block_on(exce());
//     println!("Hello, world!");
// }


fn main()->Result<(),Box<dyn std::error::Error + Send + Sync>>{
    task::block_on(
        async{
            let request01 = surf::get("https://www.google.com").recv_string();
            let request02 = surf::get("https://www.google.com").recv_string();

            let (response_01, response_02) = futures::future::try_join(request01, request02).await?;
            // dbg!(":?", response_01);
            // dbg!(":?", response_02);

            println!("request 01 {:?} request 02 {:?}",response_01, response_02);

            Ok(())
        }
    )
}