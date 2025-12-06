use gloo_net::http::{Request, RequestBuilder};
use leptos::{
    prelude::{Get, GetUntracked},
    server::codee::string::FromToStringCodec,
};
use leptos_use::storage::use_local_storage;

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
    let (state, _set_state, _) = use_local_storage::<String, FromToStringCodec>("SERVER_ADDR");
    //let addr: &'static str = env!("SERVER_ADDR");
    let addr: &'static str = "localhost:8080";

    format!("http://{}", state.get_untracked())
}
