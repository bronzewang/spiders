use leptos::*;
use leptos_meta::*;
use leptos_router::*;
// use serde::{Deserialize, Serialize};
// use thiserror::Error;
use crate::page::{
    snooper::SnooperView,
    toolkit::ToolkitView,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct State {
    pub debug: RwSignal<bool>,
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_app_context();

    // let fallback = || view! { "Page not found." }.into_view();

    view! {
        <Router trailing_slash=TrailingSlash::Exact>
            <SiteHeader />
            <main>
                <Routes>
                    <Route path="/snooper" view=SnooperView />
                    <Route path="/toolkit" view=ToolkitView />
                    // <Route path="/*any" view=NotFound />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn SiteHeader() -> impl IntoView {
    view! {
        <header class="header">
            <nav class="inner">
                <A href="/snooper">
                    <strong>"Snooper"</strong>
                </A>
                <span>" "</span>
                <A href="/toolkit">
                    <strong>"Toolkit"</strong>
                </A>
            </nav>
            // <div class="badges">
            //     <p>
            //         <a rel="external" href="https://github.com/feral-dot-io/leptos-chartistry">
            //             <img src="https://img.shields.io/badge/github-blue?logo=github&style=for-the-badge" alt="GitHub" height="28px" />
            //         </a>
            //     </p>
            // </div>
        </header>
    }
}

// #[component]
// fn NotFound() -> impl IntoView {
//     view! {
//         <article id="status404">
//             <p class="background-box">
//                 <h1 class="connect-heading">"Page not found"</h1>
//                 "The page you are looking for does not exist."
//             </p>
//         </article>
//     }
// }

pub fn provide_app_context() {
    provide_context(State::default());
}

pub fn use_app_context() -> State {
    use_context::<State>().unwrap()
}
