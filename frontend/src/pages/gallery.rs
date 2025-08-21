use leptos::prelude::*;
use thaw::{Button, ConfigProvider, Flex, Theme};
/// Default Home Page
#[component]
pub fn Gallery() -> impl IntoView {
    let theme = Theme::use_rw_theme();

    view! {
        <ConfigProvider theme>

            <Flex vertical=true>
                <Button>"1"</Button>
                <Button>"2"</Button>
                <Button>"3"</Button>
                <Button>"3"</Button>

            </Flex>

        </ConfigProvider>
    }
}
