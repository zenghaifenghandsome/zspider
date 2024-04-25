use reqwest::header;

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    println!("Hello, world!");

    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT, header::HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36"));
    headers.insert(header::REFERER, header::HeaderValue::from_static("https://www.xiaohongshu.com/"));
    let client = reqwest::Client::builder().default_headers(headers).build()?;
    
    let resp = client.get("https://www.xiaohongshu.com/explore")
        .send().await?
        //.json::<HashMap<String,String>>()
        .text()
        .await?;
    // println!("{resp:?}");

// parse html
    let dom = tl::parse(&resp, tl::ParserOptions::default())?;
   let parser = dom.parser();

    // let element = dom.query_selector("img[src]").unwrap().next().unwrap();
    let element = dom.get_element_by_id("app")
    .expect("failed to find element")
    .get(parser)
    .unwrap();
    // for i in element {
    //     println!("{i:?}");
    // }
    //let a = element.get(parser).unwrap();
    println!("{element:?}");

    Ok(())
}
