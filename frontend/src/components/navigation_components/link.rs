use leptos::{either::Either, prelude::*};

#[component]
pub fn Link(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] href: Option<Signal<String>>,
    /// Whether the link is disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let link_disabled = Memo::new(move |_| disabled.get());

    let tabindex = Memo::new(move |_| if disabled.get() { Some("-1") } else { None });

    let class = match class.get() {
        Some(class) => class,
        None => "".to_string(),
    };

    if let Some(href) = href {
        Either::Left(view! {
            <a
                role="link"
                class=class
                href=href
                tabindex=tabindex
                aria-disabled=move || link_disabled.get().then_some("true")
            >
                {children()}
            </a>
        })
    } else {
        Either::Right(view! {
            <button
                class=class
                disabled=move || disabled.get().then_some("")
                aria-disabled=move || link_disabled.get().then_some("true")
            >
                {children()}
            </button>
        })
    }
}
