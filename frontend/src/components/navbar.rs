use leptos::prelude::*;
use thaw::{Divider, Flex, FlexAlign, FlexJustify};

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <Flex
            vertical=false
            justify=FlexJustify::SpaceBetween
            class="py-3 px-5 bg-(--sidebar) border-b border-(--border)"
        >
            <Flex vertical=false align=FlexAlign::Center>
                <a
                    class="text-(--secondary) hover:text-(--secondary)/80 text-xl font-extrabold tracking-wider"
                    href="/"
                >
                    "House of Eden"
                </a>
                <Divider vertical=true></Divider>
            </Flex>

            <Flex vertical=false align=FlexAlign::Center justify=FlexJustify::Center>
                <a
                    class="text-(--foreground) hover:bg-(--accent) p-1 px-2 rounded-(--radius) text-lg font-bold tracking-wide"
                    href="/plant/new"
                >
                    "New Plant"
                </a>
                <Divider vertical=true></Divider>
                <a
                    class="text-(--foreground) hover:bg-(--accent) p-1 px-2 rounded-(--radius) text-lg font-bold tracking-wide"
                    href="/gallery"
                >
                    "Plants"
                </a>
                <a
                    href="/settings"
                    class="text-(--foreground) hover:bg-(--accent) p-1 px-2 rounded-(--radius) text-lg font-bold tracking-wide"
                >
                    "Settings"
                </a>
            </Flex>
        </Flex>
    }
}
