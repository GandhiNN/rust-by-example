mod http_handlers;

#[tokio::main]
async fn main() {
    // handle requests
    match http_handlers::headers_for_requests().await {
        Ok(_) => (),
        Err(e) => eprintln!("error: {}", e),
    }

    // handle error
    match http_handlers::handle_error().await {
        Ok(_) => (),
        Err(e) => eprintln!("error: {}", e),
    }

    // http get request
    match http_handlers::http_get_request().await {
        Ok(_) => (),
        Err(e) => eprintln!("error: {}", e),
    }

    // http post request
    match http_handlers::http_post_request().await {
        Ok(_) => (),
        Err(e) => eprintln!("error: {}", e),
    }
}
