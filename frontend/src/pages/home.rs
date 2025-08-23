use leptos::prelude::*;
use thaw::{Button, ConfigProvider, Flex, Theme};

use crate::components::new_plant::NewPlant;
/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let theme = Theme::use_rw_theme();

    view! {
        <div class="bg-background">
            <Flex vertical=true>
                <NewPlant />
                <Button>"2"</Button>
                <Button>"3"</Button>
                <Button>"3"</Button>

            </Flex>
        </div>
    }
}
