use leptos::{prelude::*, reactive::spawn_local};
use reactive_stores::Store;
use thaw::{Button, Divider, Flex, FlexAlign, FlexJustify, Link};

use crate::FrontEndState;

#[component]
pub fn NewPlant() -> impl IntoView {
    let health_check_state = RwSignal::new("Unknown".to_string());
    let reqwest_client: Store<FrontEndState> = expect_context::<Store<FrontEndState>>();

    let click = move |_| spawn_local(health_check(health_check_state, reqwest_client.get()));
    view! {
        <Flex vertical=false justify=FlexJustify::SpaceBetween class="py-3 px-5">
            <Button on_click=click>{move || health_check_state.get()}</Button>
        </Flex>
    }
}

async fn health_check(health_check: RwSignal<String>, reqwest_client: FrontEndState) {
    let Some(response) = reqwest_client
        .client
        .get("http://localhost:8080/")
        .send()
        .await
        .map_err(|e| log::error!("{e}"))
        .ok()
    else {
        *health_check.write() = "ERROR".to_string();

        return;
    };
    let Some(body_text) = response.text().await.ok() else {
        *health_check.write() = "ERROR".to_string();
        return;
    };

    *health_check.write() = body_text;
}
