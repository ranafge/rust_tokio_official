use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

// tokio runtime
#[tokio::main]

async fn main() {
    //Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    // infinite loop for continously listening or catch the frame data
    loop {
        // socket is tcpstream and add is Socket Address(ip and port)
        let (socket, _) = listener.accept().await.unwrap();

        // process(socket).await;
        // a new task is spawned for each inbound socket and socket move to the new task and processed there.
        tokio::spawn(async move {
            process(socket).await;
        });
        // let joinhandle = tokio::spawn(async move {
        //     process(socket);
        // });

        // let out = joinhandle.await.unwrap();
        // println!("GOT {:?}", out);

        // tokio::spawn return joinhandle
    }

}

async fn process(socket: TcpStream) {
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    // hasmap used to store value as key value pair
    let mut db = HashMap::new();


    // The connection let's us read/write redis *frames* instead of bytes streams. The connection type is defined by mini-redis
    let mut connection = Connection::new(socket);
 
    // #TODO replace with while let
    // if let Some(frame) = connection.read_frame().await.unwrap() {
    //     println!("GOT: {:?}", frame);

    //     // Response with an error
    //     let response = Frame::Error("implemented".to_string());
    //     connection.write_frame(&response).await.unwrap();
    // }


    // GOT: Array([Bulk(b"set"), Bulk(b"hello"), Bulk(b"world")])

    while let Some(frame) = connection.read_frame().await.unwrap()  {
    //frame = Array([Bulk(b"set"), Bulk(b"hello"), Bulk(b"world")]);
        // Command::from_frame(frame) ruturn a enum type Frame
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                // The value is stored as  `Vec<u8>`
                db.insert(cmd.key().to_string(), cmd.value().to_vec() );
                Frame::Simple("OK".to_string())
                // this is mendetory for redis for successfull set operation
            }
            Get(cmd) => {
                if let Some(vaue) = db.get(cmd.key())  {
                    Frame::Bulk(vaue.clone().into())
                }else {
                    Frame::Null
                }
            }
            // cmd will be panic other Command except Set and Get command
            cmd => panic!("unimplemented {:?}", cmd),
        };
        // Write the response to the client

        connection.write_frame(&response).await.unwrap();
    }
    println!("{:?}", db);
}