use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    println!("Hello, world!");
    let resp = reqwest::get("https://www.baidu.com")
        .await?
        //.json::<HashMap<String,String>>()
        .text()
        .await?;
    println!("{resp:?}");
    Ok(())
}
