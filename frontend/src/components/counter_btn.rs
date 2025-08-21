use leptos::prelude::*;
use reqwest::header::ACCESS_CONTROL_ALLOW_ORIGIN;

/// A parameterized incrementing button
#[component]
pub fn Button() -> impl IntoView {
    let (database_result, set_database_result) = signal(Some(String::from("Test")));

    view! {
        <button on:click=move |_| {
            let database_result = LocalResource::new(move || health_check());
            
            if database_result.get().is_some(){
                set_database_result.set(database_result.get().unwrap());
            }
        }>

        "Result: " {move || database_result.get()}

        </button>
    }
}

async fn health_check() -> Option<String> {
    let client = reqwest::Client::new();
    Some(
        client
            .get("http://localhost:8080/")
            .header(ACCESS_CONTROL_ALLOW_ORIGIN, "http://localhost")
            .send()
            .await
            .map_err(|e| log::error!("{e}"))
            .ok()?
            .text()
            .await
            .ok()?,
    )
}
