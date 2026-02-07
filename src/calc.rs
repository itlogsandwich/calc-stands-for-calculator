use serde::{ Serialize, Deserialize };
use std::sync::{ Arc, Mutex };

type CalcResult<T> = Result<T, crate::error::CalcError>;

#[derive(Debug)]
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
        .route("/clear-one", axum::routing::post(clear_one_display))
        .route("/percentage", axum::routing::post(percentage_value))
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
            "CE".to_string(), "C".to_string(), "%".to_string(), "/".to_string(),
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
    axum::extract::Form(payload): axum::Form<CalcRequest>, ) -> CalcResult<impl axum::response::IntoResponse> { println!("---> {:<12} - display_expression ", "HANDLER");

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
    println!("---> {:<12} - clear_all_expression ", "HANDLER");

    let mut expressions = state.expressions.lock().unwrap();

    expressions.clear();

    let template = crate::templates::ScreenTemplate 
    { 
        screen_content: expressions.to_vec()
    };

    Ok(crate::templates::HtmlTemplate(template))
}

async fn clear_one_display(
    axum::extract::State(state): axum::extract::State<CalcState>,
) -> CalcResult<impl axum::response::IntoResponse>
{
    println!("---> {:<12} - clear_one ", "HANDLER");

    let mut expressions = state.expressions.lock().unwrap();

    expressions.pop();

    let template = crate::templates::ScreenTemplate 
    { 
        screen_content: expressions.to_vec()
    };

    Ok(crate::templates::HtmlTemplate(template))
}

async fn percentage_value(
    axum::extract::State(state): axum::extract::State<CalcState>,
) -> CalcResult<impl axum::response::IntoResponse>
{
    println!("---> {:<12} - percentage_value ", "HANDLER");

    let mut expressions = state.expressions.lock().unwrap();

    let val = expressions.iter().next().unwrap();
    let result = val.parse::<f64>()? / 100.0;
    
    expressions.clear();
    expressions.push(result.to_string());

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
    println!("---> {:<12} - solve_expression ", "HANDLER");

    let mut expressions = state.expressions.lock().unwrap();

    let mut tokens: Vec<String> = Vec::new();
    let mut current_number = String::new();
    
    for char in expressions.iter()
    {
        if is_operator(char)
        {
            if !current_number.is_empty()
            {
                tokens.push(current_number.clone());
                current_number.clear();
            }
            tokens.push(char.to_string());
        }
        else
        {
            current_number.push_str(char);
        }
    }

    if !current_number.is_empty()
    {
        tokens.push(current_number);        
    }

    let mut iterator = tokens.iter();
    
    let first_token = iterator.next().unwrap();

    let mut result: f64 = first_token.parse::<f64>()?;

    while let Some(op_str) = iterator.next()
    {
        if let Some(num_str) = iterator.next() 
        {
            let num: f64 = num_str.parse::<f64>()?;
            let op = get_operation(op_str.to_string())?;

            result = match op 
            {
                Operations::Add => result + num,
                Operations::Subtract => result - num,
                Operations::Multiply => result * num,
                Operations::Divide => 
                {
                    if num == 0.0 { 0.0 } 
                    else { result / num }
                },
                Operations::NoneFound => result,
            };
        }
    }

    expressions.clear();
    expressions.push(result.to_string());
    let template = crate::templates::ScreenTemplate 
    { 
        screen_content: expressions.to_vec()
    };

    Ok(crate::templates::HtmlTemplate(template))
}

fn is_operator(val: &str) -> bool
{
    matches!(val, "+" | "-" | "*" | "/") 
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

