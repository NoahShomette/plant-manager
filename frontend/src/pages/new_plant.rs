//! A page for creating new plants

use leptos::prelude::*;
use thaw::{ConfigProvider, Theme};

use crate::components::new_plant::NewPlant;
/// Default Home Page
#[component]
pub fn NewPlantPage() -> impl IntoView {
    let theme = Theme::use_rw_theme();

    view! {
        <ConfigProvider theme>
            <div class="flex flex-row justify-center">
                <NewPlant />
            </div>
        </ConfigProvider>
    }
}
