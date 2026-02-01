use serde::{ Serialize, Deserialize };
use std::sync::{ Arc, Mutex };

type CalcResult<T> = Result<T, crate::error::CalcError>;
pub enum Operations
{
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Clone)]
pub struct CalcState
{   
    pub expression: Arc<Mutex<Vec<&'static str>>>
}

#[derive(Serialize, Deserialize)]
pub struct CalcRequest
{
    pub expression: String
}

pub async fn create_app(state: CalcState) -> axum::Router
{
    axum::Router::new()
        .route("/", axum::routing::get(index))
        .route("/add", axum::routing::get(solve_expression))
        .fallback_service(tower_http::services::ServeDir::new("assets"))
        .with_state(state)
}

async fn index(
    axum::extract::State(state): axum::extract::State<CalcState>,
) -> CalcResult<impl axum::response::IntoResponse>
{
    println!("---> {:<12} - index - ", "HANDLER");

    let template = crate::templates::IndexTemplate 
    { 
        calc_input: vec![
            "C", "()", "%", "/",
            "7", "8", "9", "*",
            "4", "5", "6", "-",
            "1", "2", "3", "+",
            ".", "0", ".", "=",
        ],

        screen_content: state.expression.lock().unwrap().to_vec()
    };
    Ok(crate::templates::HtmlTemplate(template))
}

async fn solve_expression(
    axum::extract::State(state): axum::extract::State<CalcState>,
    // axum::Form(payload): axum::Form<CalcRequest>,
) -> CalcResult<impl axum::response::IntoResponse>
{
    println!("---> {:<12} - add_expression ", "HANDLER");
    
    let mut default_val: f64 = 0.0;
    let default_opr = String::from("+");
    
    // let vec: Vec<&str> = payload.expression.split("+").collect();
    // let vec = Vec::<f64>::from([5.0,10.0,16.0]);
    let vec= "10+20+30".split("+").collect::<Vec<&str>>();

    for x in vec
    {
        // default_val += x.parse::<f64>().unwrap();
        default_val += x.parse::<f64>().unwrap();
    }

    let template = crate::templates::IndexTemplate 
    { 
        calc_input: vec![
            "C", "()", "%", "/",
            "7", "8", "9", "*",
            "4", "5", "6", "-",
            "1", "2", "3", "+",
            ".", "0", ".", "=",
        ],

        screen_content: state.expression.lock().unwrap().to_vec()
    };
    Ok(crate::templates::HtmlTemplate(template))
}

