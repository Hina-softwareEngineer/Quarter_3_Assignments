use async_std::io;
use async_std::fs::File;
use async_std::prelude::*;

pub async fn read_file(path: &str) -> io::Result<String>{
    let mut file= File::open(path).await?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).await?;

    Ok(buffer)
}