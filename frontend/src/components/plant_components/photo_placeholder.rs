use leptos::prelude::*;
use leptos_use::{breakpoints_tailwind, use_breakpoints, BreakpointsTailwind};
use random_color::RandomColor;
use thaw::Icon;
use uuid::Uuid;
use BreakpointsTailwind::*;

/// Component to view a specific type of event
#[component]
pub fn PhotoPlaceholderDisplayComponent(use_color: Option<Uuid>) -> impl IntoView {
    let screen_width = use_breakpoints(breakpoints_tailwind());
    let larger_than_sm = screen_width.gt(Sm);
    let larger_than_md = screen_width.gt(Md);
    let icon_size = signal("75px");

    Effect::new(move || {
        if larger_than_md.get() {
            icon_size.1.set("150px");
        } else if larger_than_sm.get() {
            icon_size.1.set("100px");
        } else {
            icon_size.1.set("75px");
        }
    });
    let mut plant_color = "#353935".to_string();
    if let Some(plant_id) = use_color {
        let mut random_color = RandomColor::new();
        random_color.luminosity(random_color::options::Luminosity::Dark);
        random_color.seed(plant_id.to_string());
        plant_color = random_color.to_hex();
    }
    let plant_color = signal(plant_color);

    view! {
        {move || view! {<div class={format!("m-2 rounded-(--radius) max-w-[400px] aspect-square flex justify-center content-center")} style={ move || format!("background: {}", plant_color.0.get())}>
            <Icon icon=icondata::ChPlantPot width=icon_size.0.get() height=icon_size.0.get() class="text-(--foreground)"></Icon>
            </div>}}
    }
}
