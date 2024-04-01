extern crate chain_mail;
use tokio::runtime::Runtime;

fn main() {
    let runtime = Runtime::new().unwrap();
    let processor = chain_mail::ProfileRequest::new(String::from("davidonuh1@gmail.com"));

    runtime.block_on(async {
        let payload = processor.build().await;

        println!("payload from processor: {:?}", payload)
    });

    println!("hellow world");
}
