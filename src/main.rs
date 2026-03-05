use tokio::net::{TcpListener, TcpStream};






#[tokio::main]
async fn main() {
    
    // bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (socket, ip) = listener.accept().await.unwrap();
        println!("{:?}", ip);
        process(socket).await;

    }
    
}

async fn process(socket : TcpStream) {
    
}
