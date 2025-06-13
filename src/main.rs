use bunny::serve_bunny;

#[tokio::main]
async fn main() {
    serve_bunny().await;
}
