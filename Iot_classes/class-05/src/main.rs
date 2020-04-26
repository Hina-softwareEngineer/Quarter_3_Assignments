use std::thread;
use std::time::Duration;
use futures::executor::block_on;
use async_std::task;

mod file;

fn main(){
    block_on(load_data());
}

async fn load_f1(){
    let r1=file::read_file("t1.txt").await;
    println!("file 1 {}", r1.unwrap().len());
}

async fn load_f2(){
    let r2=file::read_file("t2.txt").await;
    println!("file 2 {}", r2.unwrap().len());
}

async fn load_data(){
    futures::join!(load_f1(), load_f2());
}

// async fn learning_song(){
//     task::sleep(Duration::from_secs(6)).await;
//     println!("learning Songs taare zameen par");
// }

// async fn singing_song(){
//     task::sleep(Duration::from_secs(3)).await;
//     println!("Singing Song");
// }

// async fn dance(){
//     task::sleep(Duration::from_secs(3)).await;
//     println!("dance");
// }

// async fn learn_and_sing(){
//    let f1=learning_song().await;
//    let f2=singing_song().await; 
   
//     println!("learn and sing");
// }

// async fn async_main(){
//     let f1 = learn_and_sing();
//     let f2= dance();

//     futures::join!(f1, f2);
// }

// fn main(){
//     block_on(async_main());
//     println!("Heeyy");
// }


// await makes sure that asynchronous functions
// behave like synchronous

// executors -> join.await

