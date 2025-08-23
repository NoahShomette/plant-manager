use std::collections::HashMap;

use leptos::{prelude::*, server::codee::string::JsonSerdeCodec};
use leptos_meta::*;
use leptos_router::{components::*, path};
use leptos_use::storage::use_local_storage;
use reactive_stores::Store;
use reqwest::{
    header::{self, ACCESS_CONTROL_ALLOW_ORIGIN},
    Client,
};
use thaw::{ConfigProvider, Theme};

// Modules
mod components;
mod pages;
mod plant_storage;
mod theme;

// Top-Level pages
use crate::{
    components::{footer::Footer, navbar::Navbar},
    pages::{gallery::Gallery, home::Home, new_plant::NewPlantPage, plant_page::PlantPage},
    plant_storage::{PlantStorage, PlantStorageContext},
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
    let (state, set_state, _) = use_local_storage::<PlantStorage, JsonSerdeCodec>("my-plants");
    provide_context(PlantStorageContext {
        get: state,
        write: set_state,
    });

    let theme = RwSignal::new(theme::update_theme());

    view! {
        <ConfigProvider theme>
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
                            <ParentRoute path=path!("/plant") view=|| view! { <Outlet /> }>
                                <Route path=path!("/new") view=NewPlantPage />
                                <ParentRoute path=path!("/view/:id") view=PlantPage>
                                    <Route path=path!("") view=|| view! {} />
                                // <Route path=path!("conversations") view=|| view! {} /> // Example of having a sub path to the id url - use this for the edit/timeline pages?
                                </ParentRoute>
                            // <Route path=path!("") view=Gallery />
                            </ParentRoute>

                        </Routes>
                    </Router>
                </div>
                <Footer />
            </div>
        </ConfigProvider>
    }
}
