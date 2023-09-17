use lambda_http::{run, Error};
use axum::{
    response::Html,
    Router,
    routing::get,
};
use std::fs;

// async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
//     // Extract some useful information from the request
//     let who = event
//         .query_string_parameters_ref()
//         .and_then(|params| params.first("name"))
//         .unwrap_or("world");
//     let message = format!("Hello {who}, this is an AWS Lambda HTTP request");

//     // Return something that implements IntoResponse.
//     // It will be serialized to the right response event automatically by the runtime
//     let resp = Response::builder()
//         .status(200)
//         .header("content-type", "text/html")
//         .body(message.into())
//         .map_err(Box::new)?;
//     Ok(resp)
// }

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let app = Router::new()
    .route("/", get(serve_html));

    run(app).await
    // run(service_fn(function_handler)).await
}

async fn serve_html() -> Html<String> {
    let content = fs::read_to_string("assets/index.html").unwrap_or_else(|_| "Error reading index.html".to_string());
    Html(content)
}
