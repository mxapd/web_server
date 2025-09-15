use crate::html::html::Html;
use crate::html::html_builder::HtmlBuilder;
use crate::http::http_response::HttpResponse;
use crate::http::http_status::HttpStatus;

use std::error::Error;

pub fn home_page() -> Result<HttpResponse, Box<dyn Error>> {
    let html_string = HtmlBuilder::new()
        .title("Test Title")
        .add_h1("Heading1")
        .add_p("this is a paragraph")
        .add_hr()
        .add_h1("Heading2")
        .add_p("this is another paragraph")
        .build();

    let html = Html::from_string(html_string);

    let response = HttpResponse::from_html(html, HttpStatus::Ok);

    println!("built response");

    Ok(response)
}
