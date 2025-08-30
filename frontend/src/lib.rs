use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};
use reactive_stores::Store;
use reqwest::{
    header::{self, ACCESS_CONTROL_ALLOW_ORIGIN},
    Client,
};
use thaw::ConfigProvider;

// Modules
mod components;
mod data_storage;
mod pages;
mod theme;

// Top-Level pages
use crate::{
    components::{footer::Footer, navbar::Navbar},
    data_storage::AppStorageComponent,
    pages::{gallery::Gallery, home::Home, new_plant::NewPlantPage, plant_page::PlantPage},
};

#[derive(Clone, Debug, Default, Store)]
struct FrontEndState {
    client: Client,
}

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let mut headers = header::HeaderMap::new();
    headers.insert(
        ACCESS_CONTROL_ALLOW_ORIGIN,
        header::HeaderValue::from_static("http://localhost"),
    );
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .expect("Reqwest Client Build failed");

    provide_context(Store::new(FrontEndState { client }));

    let theme = RwSignal::new(theme::update_theme());

    view! {
        <ConfigProvider theme>
            <AppStorageComponent>
                <Stylesheet id="leptos" href="/style/output.css" />
                <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

                // sets the document title
                <Title text="Household of Eden" />

                // injects metadata in the <head> of the page
                <Meta charset="UTF-8" />
                <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <div class="flex flex-col h-screen justify-stretch">
                    <Navbar />
                    <div class="bg-(--background) h-full">
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
