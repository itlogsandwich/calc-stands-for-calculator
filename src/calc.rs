use serde::{ Serialize, Deserialize };
use std::sync::{ Arc, Mutex };

type CalcResult<T> = Result<T, crate::error::CalcError>;
pub enum Operations
{
    Add,
    Subtract,
    Multiply,
    Divide,
    NoneFound,
}

#[derive(Clone)]
pub struct CalcState
{   
    pub expressions: Arc<Mutex<Vec<String>>>
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
        .route("/solve", axum::routing::post(solve_expression))
        .route("/display", axum::routing::post(display_expression))
        .route("/clear", axum::routing::post(clear_display))
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
            "C".to_string(), "()".to_string(), "%".to_string(), "/".to_string(),
            "7".to_string(), "8".to_string(), "9".to_string(), "*".to_string(),
            "4".to_string(), "5".to_string(), "6".to_string(), "-".to_string(),
            "1".to_string(), "2".to_string(), "3".to_string(), "+".to_string(),
            ".".to_string(), "0".to_string(), ".".to_string(), "=".to_string(),
        ],

        screen_content: state.expressions.lock().unwrap().to_vec()
    };
    Ok(crate::templates::HtmlTemplate(template))
}

async fn display_expression(
    axum::extract::State(state): axum::extract::State<CalcState>,
    axum::extract::Form(payload): axum::Form<CalcRequest>, 
) -> CalcResult<impl axum::response::IntoResponse>
{
    println!("---> {:<12} - display_expression ", "HANDLER");

    let mut expressions = state.expressions.lock().unwrap();

    expressions.push(payload.expression);
    let template = crate::templates::ScreenTemplate 
    { 
        screen_content: expressions.to_vec()
    };

    Ok(crate::templates::HtmlTemplate(template))
}

async fn clear_display(
    axum::extract::State(state): axum::extract::State<CalcState>,
) -> CalcResult<impl axum::response::IntoResponse>
{
    println!("---> {:<12} - display_expression ", "HANDLER");

    let mut expressions = state.expressions.lock().unwrap();

    expressions.clear();

    let template = crate::templates::ScreenTemplate 
    { 
        screen_content: expressions.to_vec()
    };

    Ok(crate::templates::HtmlTemplate(template))
}

async fn solve_expression(
    axum::extract::State(state): axum::extract::State<CalcState>,
) -> CalcResult<impl axum::response::IntoResponse>
{
    println!("---> {:<12} - add_expression ", "HANDLER");

    let mut expressions = state.expressions.lock().unwrap();

    let mut expr_one = String::new();
    let mut expr_two = String::new();
    
    let operator= expressions.iter()
        .position(|opr| opr == "+" || opr == "-" || opr == "*" || opr == "/").unwrap();

    for (index, val) in expressions.iter().enumerate()
    {
        if index != operator && index < operator
        {
            expr_one.push_str(val);
        }
        else if index != operator && index > operator
        {
            expr_two.push_str(val);     
        }
        continue
    }

    let sign = expressions.get(operator).unwrap();

    let operation = get_operation(sign.to_string())?;

    let solved_expr = match operation
    {
        Operations::Add => expr_one.parse::<i64>()? + expr_two.parse::<i64>()?,
        Operations::Subtract => expr_one.parse::<i64>()? - expr_two.parse::<i64>()?,
        Operations::Multiply => expr_one.parse::<i64>()? * expr_two.parse::<i64>()?,
        Operations::Divide => expr_one.parse::<i64>()? / expr_two.parse::<i64>()?,
        Operations::NoneFound=> 0,
    };

    expressions.clear();

    expressions.push(solved_expr.to_string());

    let template = crate::templates::ScreenTemplate 
    { 
        screen_content: expressions.to_vec()
    };

    Ok(crate::templates::HtmlTemplate(template))
}

fn get_operation(sign: String) -> CalcResult<Operations>
{
    Ok(match sign.as_str() 
    {
        "+" => Operations::Add,
        "-" => Operations::Subtract,
        "*" => Operations::Multiply,
        "/" => Operations::Divide,
        _ => Operations::NoneFound,
    })
}

