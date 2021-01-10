#[tokio::main]
async fn main() {
    let err = match reddit_bot::run().await {
        Err(it) => it,
        _ => return,
    };
    println!("error running example: {}", err);
}
