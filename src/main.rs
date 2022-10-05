use std::thread;
use std::time::Duration;
use redis::Commands;
#[tokio::main]
async fn main() -> Result<(), ()>{

    let rdb_cli = redis::Client::open("redis://127.0.0.1").unwrap();

    for i in 0 .. 5 {
        if i == 0 {
            loop {
                let buf = rdb_cli.clone().blpop::<String, Vec<String>>("LIST_QUEUE".to_string(), 4);
                println!("buf: {:#?}", buf);
            }
        }
    }
    return ();
}

