//! A page that views a specific plant with customizable options for what parts of the plant to show.
//!
//! Includes Timeline and Edit views

use leptos::{prelude::*, reactive::spawn_local};
use reqwest::header::ACCESS_CONTROL_ALLOW_ORIGIN;
use thaw::{Button, ConfigProvider, Flex, Theme};

use crate::components::new_plant::NewPlant;
/// Default Home Page
#[component]
pub fn PlantPage() -> impl IntoView {
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
