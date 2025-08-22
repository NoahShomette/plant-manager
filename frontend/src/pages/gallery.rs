use leptos::{prelude::*, reactive::spawn_local};
use reactive_stores::Store;
use thaw::{Button, ConfigProvider, Flex, Label, Theme};

use crate::FrontEndState;
/// Default Home Page
#[component]
pub fn Gallery() -> impl IntoView {
    let theme = Theme::use_rw_theme();
    let value = RwSignal::new("".to_string());
    let submit_response = RwSignal::new("Unknown".to_string());
    let reqwest_client: Store<FrontEndState> = expect_context::<Store<FrontEndState>>();

    view! {
        <ConfigProvider theme>

            <Flex vertical=true>
                <Button>"1"</Button>
                <Button>"2"</Button>
                <Button>"3"</Button>
                <Button>"3"</Button>
                <Label>{move || submit_response.get()}</Label>

            </Flex>

        </ConfigProvider>
    }
}
