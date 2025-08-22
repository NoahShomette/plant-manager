use leptos::prelude::*;
use thaw::{Divider, Flex, FlexAlign, FlexJustify, Link};

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <Flex vertical=false justify=FlexJustify::SpaceBetween class="py-3 px-5">
            <Flex vertical=false align=FlexAlign::Center>
                <Link href="/">"House of Eden"</Link>
                <Divider vertical=true></Divider>
            </Flex>

            <Flex vertical=false align=FlexAlign::Center justify=FlexJustify::Center>
                <Link href="/plant/new">"New Plant"</Link>
                <Divider vertical=true></Divider>
                <Link href="/gallery">"Plants"</Link>
                <Link href="/settings">"Settings"</Link>
            </Flex>
        </Flex>
    }
}
