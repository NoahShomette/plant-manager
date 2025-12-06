use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

use thaw::ConfigProvider;

// Modules
mod components;
mod data_storage;
mod pages;
mod server_helpers;
mod theme;

// Top-Level pages
use crate::{
    components::{footer::Footer, navbar::Navbar},
    data_storage::AppStorageComponent,
    pages::{gallery::Gallery, home::Home, new_plant::NewPlantPage, plant_page::PlantPage},
};

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let theme = RwSignal::new(theme::update_theme());

    view! {
        <ConfigProvider theme class="">
            <AppStorageComponent>
                <Stylesheet id="leptos" href="/style/output.css" />
                <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

                // sets the document title
                <Title text="House of Eden" />

                // injects metadata in the <head> of the page
                <Meta charset="UTF-8" />
                <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <div class="flex flex-col items-stretch min-h-screen">
                    <Navbar />
                    <div class="flex bg-background flex-col h-full mb-auto">
                        <Router>
                            <Routes fallback=|| view! { NotFound }>
                                <Route path=path!("/") view=Home />
                                <Route path=path!("/gallery") view=Gallery />
                                <Route path=path!("/plant/new") view=NewPlantPage />
                                <Route path=path!("/plant/:id/view") view=PlantPage />
                                <Route path=path!("/plant/:id/timeline") view=NewPlantPage />
                                <Route
                                    path=path!("/plant/:id/edit")
                                    view=|| view! { <p>edit</p> }
                                />

                            </Routes>
                        </Router>
                    </div>
                    <Footer />
                </div>
            </AppStorageComponent>
        </ConfigProvider>
    }
}
