mod calc;
mod error;
mod templates;

#[tokio::main]
async fn main() 
{
    let app = crate::calc::create_app(crate::calc::CalcState {solved: String::from("")}).await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:6969")
        .await.unwrap();

    println!("Server runnning on http://localhost:6969");

    axum::serve(listener, app)
        .await.unwrap();
}
