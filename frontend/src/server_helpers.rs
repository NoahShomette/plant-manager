use gloo_net::http::{Request, RequestBuilder};

pub fn post_request(request: &str) -> RequestBuilder {
    let request = Request::post(&format!("{}{}", base_server_addr(), request));
    let request = default_http_request(request);
    request
}

pub fn get_request(request: &str) -> RequestBuilder {
    let request = Request::get(&format!("{}{}", base_server_addr(), request));
    let request = default_http_request(request);
    request
}

pub fn default_http_request(request: RequestBuilder) -> RequestBuilder {
    let request = request.header("Access-Control-Allow-Origin", "http://localhost");
    request
}

pub fn base_server_addr() -> String {
    let addr: &'static str = env!("SERVER_ADDR");
    format!("http://{}", addr)
}
