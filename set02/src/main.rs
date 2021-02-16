mod async_join;

#[tokio::main]
async fn main() {
    println!("====================");
    println!(" Asnyc Join");
    println!("====================");
    async_join::run().await;
    println!("\n");
}
