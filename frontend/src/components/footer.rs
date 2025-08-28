use leptos::prelude::*;
use thaw::{Divider, Flex, FlexAlign, FlexJustify, Link};

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <Flex
            vertical=false
            justify=FlexJustify::SpaceBetween
            class="py-3 px-5 color-sidebar-accent"
        >
            <Flex vertical=false align=FlexAlign::Center justify=FlexJustify::Center>
                <Link href="https://github.com/NoahShomette/plant-manager">"Github"</Link>
            </Flex>
        </Flex>
    }
}
