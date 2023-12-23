use tokio::task::yield_now;
use std::rc::Rc; 

#[tokio::main] 

async fn main() {
    tokio::spawn(async {
        {
            let rc = Rc::new("hello");
            println!("{}", rc);
        }// rc is droped after this line

        yield_now().await; // task yields to the scheduler 
    })
}


