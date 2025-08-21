use leptos::{prelude::*, reactive::spawn_local};
use reqwest::header::ACCESS_CONTROL_ALLOW_ORIGIN;
use thaw::{Button, ConfigProvider, Flex, Theme};

use crate::components::new_plant::NewPlant;
/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let theme = Theme::use_rw_theme();

    view! {
        <ConfigProvider theme>

            <Flex vertical=true>
                <NewPlant />
                <Button>"2"</Button>
                <Button>"3"</Button>
                <Button>"3"</Button>

            </Flex>

        </ConfigProvider>
    }
}
