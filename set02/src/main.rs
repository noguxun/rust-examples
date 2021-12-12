mod spawn_join;
mod concurrent_join;

#[tokio::main]
async fn main() {
    spawn_join::run().await;

    println!("\n");

    concurrent_join::speak().await;
}
