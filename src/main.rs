use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};



/*
#[tokio::main] a.k.a =>     
let mut runtime = tokio::runtime::Runtime::new().unwrap();
let future = main();
runtime.block_on(future);
*/
#[tokio::main] 
async fn main(){

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop{
        //the second item _ contains IP and port of the new connection
        let (socket, _) = listener.accept().await.unwrap();
        process(socket).await;
    }


}

async fn process(socket: TcpStream){
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap(){
        println!("GOT Frame from Sock Stream: {:?}", frame);

        // Error response
        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}