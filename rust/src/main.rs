use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path::end()
        .map(|| warp::reply::html("Hello, Warp!"));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

