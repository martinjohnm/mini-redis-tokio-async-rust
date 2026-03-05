



use mini_redis::{client, Result};


#[tokio::main]
async fn main() -> Result<()> {

    // open a connection to mini-redis address
    let mut client = client::connect("127.0.0.1:6379").await?;

    // set the new key "hello" with value world
    client.set("hello", "world".into()).await?;
    // get the key "hello"
    let result = client.get("hello").await?;
    println!("here 2");
    println!("got value from the server; result = {:?}", result);

    Ok(())

}
