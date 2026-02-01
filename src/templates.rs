use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]

pub struct IndexTemplate
{
    pub calc_input: Vec<&'static str>,
    pub screen_content: Vec<&'static str>
}

pub struct HtmlTemplate<T>(pub T);

impl <T> axum::response::IntoResponse for HtmlTemplate<T>
    where T: Template,
{
    fn into_response(self) -> axum::response::Response
    {
        match self.0.render()
        {
            Ok(html) => axum::response::Html(html).into_response(),
            Err(err) => 
            {
                println!("Error: {err}");
                err.to_string().into_response()
            }
        }
    }

}
