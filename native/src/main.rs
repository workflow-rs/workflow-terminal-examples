use cli::example_terminal;

#[tokio::main]
async fn main() {
    let result = example_terminal().await;
    if let Err(err) = result {
        println!("{}", err);
    }
}
