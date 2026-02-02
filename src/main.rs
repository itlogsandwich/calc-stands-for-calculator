use std::sync::{ Arc, Mutex };

mod calc;
mod error;
mod templates;
#[tokio::main]

async fn main() -> Result<(), crate::error::CalcError>
{
    let state = crate::calc::CalcState 
    {
        expressions: Arc::new(Mutex::new(Vec::new()))
    };

    let app = crate::calc::create_app(state).await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:6969")
        .await?;

    println!("Server runnning on http://localhost:6969");

    axum::serve(listener, app)
        .await?;

    Ok(())
}
