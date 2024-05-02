use axum::{response::Html, routing::get, Router};



#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get (handler));

    let listener = tokio::net::TcpListener::bind("localhost", 5173)
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}



async fn handler() -> Html<&'static str> {
    return Html("<h1>Hello, _soder!</h1>")
}
