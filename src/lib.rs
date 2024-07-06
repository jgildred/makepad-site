use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

/// A simple Spin HTTP component.
#[http_component]
fn handle_makepad_site(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling API request to {:?}", req.header("spin-full-url"));
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("API for this site is not yet defined.")
        .build())
}
