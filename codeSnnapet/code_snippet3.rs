use tokio::task::yield_now;
use std::rc::Rc;


#[tokio::main] 

async fn main( ) {
    tokio::spawn(async {
        let rc = Rc::new("hello");
        yield_now().await;

        println!("{}", rc);
    });
}