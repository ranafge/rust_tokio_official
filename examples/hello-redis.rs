// Mini-Redis client library
use mini_redis::{client, Result};

// setting asyncronous runtime
#[tokio::main]
async fn main() ->Result<()> {
    //Open a connectin to the mini-redis address.
    // client::connect provide the mini-redis crate. It asynchronously established a  `TCP` connection with the specified remote address.
    // the client handle is returned.
    let mut client = client::connect("127.0.0.1:8080").await?;


    //Set the "hello" with the value "world"
    // client.set method expect a Bytes so need to convert it. using into
    client.set("hello", "ABCabc".into()).await?;

    // Get key `hello`
    // get method return future
    let result = client.get("hello").await?;
    println!("got value from the server; result={:?}", result.to_owned());

    // executing code form lib.res
    // let lib_result = my_redis::say_hello();
    // println!("Still my_redis::say_hello() does not exectute untill .await");
    // println!("Then execute and the result is: {:?}", lib_result.await);


    Ok(())
}

/* //////////////////////////////////////////////////////////////////////////////////////////////////////
 * 
 * syncronous programming a code sequencially execute hoy. mane ektar por r ekta , ekta execute na hoya 
 * porjonto porer line execute  hoy na.
 * 
 * kintu asyncronous programming a single thread a multipole task execute kara jay at the same time . ekta task execute karar pore
 * porer task ta execut hoy no 
 * 
 * mane fire and forget->  after completing the next task then go to the previous task
 * So we can get result fast
 * ///////////////////////////
 *  #[tokio::main]
 *  async fn main() {
 *  println!("hello")
 * }
 *  It gets transformed into
 *  fn main() {
 *  let mut rt = tokio::runtime::Runtime::new().unwrap();
 *  rt.block_on(
 *  async {
 *  println!("hello");
 * }
 * )
 * }
 * ////////////////////////////
 * 

 * 
 * ////////////////////////////////////////////////////////////////////////////////////////////////////////
 */