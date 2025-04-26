#![allow(non_snake_case)]

pub async fn Client() {
    let mut client = mini_redis::client::connect("127.0.0.1:6379").await.unwrap();
    client.set("but", "nobody came...".into()).await.unwrap();

    let stuff = client.get("but").await.unwrap();
    println!("Got: {:?}", stuff);
}
