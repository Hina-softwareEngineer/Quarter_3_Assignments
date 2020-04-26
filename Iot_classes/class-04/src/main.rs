use std::thread;
use std::time::Duration;
use futures::executor::block_on;

// fn get_two_sites(){
//     let thread_one=thread::spawn(||{
//         thread::sleep(Duration::from_secs(5));
//         println!("thread one");
//     });

   
//     let thread_two=thread::spawn(||{
//         thread::sleep(Duration::from_secs(3));
//         println!("thread two");
//     });

//     thread_one.join();
//     thread_two.join();
// }

// fn main(){
//     get_two_sites();
// }
// async fn t1(){
//     thread::sleep(Duration::from_secs(7));
//     println!("thread one");
// }

// fn main(){
//     block_on(get_two_sites_async());
// }

// async fn t2(){
//     thread::sleep(Duration::from_secs(4));
//     println!("thread two");
// }

// async fn get_two_sites_async(){
//     let thread_one=t1();
//     let thread_two=t2();

//     futures::join!(thread_one, thread_two);
// }


async fn learning_song()->String{
    thread::sleep(Duration::from_secs(2));
    "Songs taare zameen par".to_string()
}

async fn singing_song(song:String){
    println!("{},", song);
}

async fn dance(){
    println!("dance");
}

async fn learn_and_sing(){
    let song = learning_song().await;
    singing_song(song).await;
    println!("learn and sing");
}

async fn async_main(){
    let f1 = learn_and_sing();
    let f2= dance();

    futures::join!(f1, f2);
}

fn main(){
    block_on(async_main());
    println!("Heeyy");
}
