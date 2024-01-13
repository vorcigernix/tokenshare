use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};


pub(crate) mod getsecret;
pub(crate) mod savesecret;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/tokenshare.css"/>

        // sets the document title
        <Title text="Share your secrets and tokens securely"/>

        <Link
            rel="icon"
            type_="image/svg+xml"
            href="data:image/svg+xml;base64,PHN2ZyB4bWxucz0naHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmcnIHZpZXdCb3g9JzAgMCAyNCAyNCcgZmlsbD0nI2ZhZmFmYSc+PHBhdGggZmlsbC1ydWxlPSdldmVub2RkJyBkPSdNMTUuNzUgMS41YTYuNzUgNi43NSAwIDAgMC02LjY1MSA3LjkwNmMuMDY3LjM5LS4wMzIuNzE3LS4yMjEuOTA2bC02LjUgNi40OTlhMyAzIDAgMCAwLS44NzggMi4xMjF2Mi44MThjMCAuNDE0LjMzNi43NS43NS43NUg2YS43NS43NSAwIDAgMCAuNzUtLjc1di0xLjVoMS41QS43NS43NSAwIDAgMCA5IDE5LjVWMThoMS41YS43NS43NSAwIDAgMCAuNTMtLjIybDIuNjU4LTIuNjU4Yy4xOS0uMTg5LjUxNy0uMjg4LjkwNi0uMjJBNi43NSA2Ljc1IDAgMSAwIDE1Ljc1IDEuNVptMCAzYS43NS43NSAwIDAgMCAwIDEuNUEyLjI1IDIuMjUgMCAwIDEgMTggOC4yNWEuNzUuNzUgMCAwIDAgMS41IDAgMy43NSAzLjc1IDAgMCAwLTMuNzUtMy43NVonIGNsaXAtcnVsZT0nZXZlbm9kZCcgLz48L3N2Zz4="
        />

        <Router>
            <main>
                <Routes>
                    <Route path="" view=savesecret::SaveSecret/>
                    <Route path="/get/:id" view=getsecret::GetSecret/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_spin::ResponseOptions>();
        resp.set_status(404);
    }

    view! { <h1>"Not Found"</h1> }
}
