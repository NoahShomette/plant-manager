use leptos::prelude::*;
use thaw::{
    Button, ButtonAppearance, Divider, DrawerBody, DrawerHeader, DrawerHeaderTitle,
    DrawerHeaderTitleAction, DrawerPosition, DrawerSize, Flex, FlexAlign, FlexJustify,
    InlineDrawer, OverlayDrawer,
};

#[component]
pub fn Navbar() -> impl IntoView {
    let open = RwSignal::new(false);
    let open_mobile_nav = move |_| {
        // Note: Since `show` changes are made in real time,
        // please put it in front of `show.set(true)` when changing `placement`.
        open.set(!open.get_untracked());
    };
    view! {
        <Flex
            vertical=false
            justify=FlexJustify::SpaceBetween
            class="py-3 px-5 bg-sidebar border-b border-border shrink-0"
        >
            <Flex vertical=false align=FlexAlign::Center>
                <a
                    class="text-secondary hover:text-(--secondary)/80 text-xl font-extrabold tracking-wider"
                    href="/"
                >
                    "House of Eden"
                </a>
            </Flex>
            <div class="md:hidden">
                <Button icon=icondata::ChMenuHamburger on_click=open_mobile_nav></Button>
            </div>

            <OverlayDrawer open=open position=DrawerPosition::Left>
                <DrawerHeader>
                    <DrawerHeaderTitle>
                        <a
                            class="text-secondary hover:text-(--secondary)/80 text-3xl font-extrabold tracking-wider"
                            on:click=open_mobile_nav
                            href="/"
                        >
                            "House of Eden"
                        </a>
                    </DrawerHeaderTitle>
                </DrawerHeader>
                <DrawerBody>
                    <div class="flex flex-col content-center justify-center">
                        <Divider class="py-2" vertical=false />

                        <a
                            class="text-foreground hover:bg-accent p-1 px-2 rounded-(--radius) text-2xl font-bold tracking-wide"
                            on:click=open_mobile_nav
                            href="/plant/new"
                        >
                            "New Plant"
                        </a>
                        <a
                            class="text-foreground hover:bg-accent p-1 px-2 rounded-(--radius) text-2xl font-bold tracking-wide"
                            on:click=open_mobile_nav
                            href="/gallery"
                        >
                            "Plants"
                        </a>
                        <a
                            href="/settings"
                            on:click=open_mobile_nav
                            class="text-foreground hover:bg-accent p-1 px-2 rounded-(--radius) text-2xl font-bold tracking-wide"
                        >
                            "Settings"
                        </a>
                    </div>
                </DrawerBody>
            </OverlayDrawer>

            <div class="hidden md:flex content-center justify-center">
                <a
                    class="text-foreground hover:bg-accent p-1 px-2 rounded-(--radius) text-lg font-bold tracking-wide"
                    href="/plant/new"
                >
                    "New Plant"
                </a>
                <Divider vertical=true></Divider>
                <a
                    class="text-foreground hover:bg-accent p-1 px-2 rounded-(--radius) text-lg font-bold tracking-wide"
                    href="/gallery"
                >
                    "Plants"
                </a>
                <a
                    href="/settings"
                    class="text-foreground hover:bg-accent p-1 px-2 rounded-(--radius) text-lg font-bold tracking-wide"
                >
                    "Settings"
                </a>
            </div>
        </Flex>
    }
}
