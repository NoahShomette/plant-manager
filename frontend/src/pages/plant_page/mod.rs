//! A page that views a specific plant with customizable options for what parts of the plant to show.
//!
//! Includes Timeline and Edit views

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use thaw::{Button, ConfigProvider, Flex, Theme};
/// Default Home Page
#[component]
pub fn PlantPage() -> impl IntoView {
    // we can access the :id param reactively with `use_params_map`
    let params = use_params_map();
    let id = move || params.read().get("id").unwrap_or_default();

    let theme = Theme::use_rw_theme();

    view! {
        <ConfigProvider theme>

            <Flex vertical=true>
                <Button>{id}</Button>
                <Button>"3"</Button>
                <Button>"Create Plant"</Button>

            </Flex>

        </ConfigProvider>
    }
}
