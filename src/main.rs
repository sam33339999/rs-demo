use std::thread;
use std::time::Duration;
use redis::Commands;
#[tokio::main]
async fn main() -> Result<(), ()>{

    let mut rdb_cli = redis::Client::open("redis://127.0.0.1").unwrap();

    for i in 0 .. 5 {
        let _thread_handle = thread::spawn(move || {
            loop {
                println!("YEEEEEEEEEEEEEEE ~{}", i);
                thread::sleep(Duration::from_secs(1));
            }
        });
    }
    println!("{:#?}", rdb_cli);

    // for i in 0..10 {
    //     // if i == 10 {
    //         println!("hi main thread: {}", i);
    //     // }
    //     thread::sleep(Duration::from_secs(1));
    // }
    // thread_handle.join().unwrap();
    
    loop {
        let buf = rdb_cli.clone().blpop::<String, Vec<String>>("LIST_QUEUE".to_string(), 4);
        println!("buf: {:#?}", buf);
    }
}

