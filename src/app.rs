use http::status;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        <Stylesheet id="leptos" href="/pkg/tokenshare.css"/>

        // sets the document title
        <Title text="Share your secrets and tokens securely"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    //let (token, set_token) = create_signal("");
    let (token, set_token) = create_signal("".to_string());
    let (status, set_status) = create_signal(0);
    let on_click = move |_| {
        set_status.update(|status| *status = 1);
        spawn_local(async move {
            save_secret(token.get().to_string()).await.unwrap();
        });
    };

    view! {
      <div class="div">
      <div class="div-2">
        <div class="div-3">
          <div class="div-4">
            <div class="column">
              <div class="div-5">
                <div class="div-6"></div>
                <div class="div-7">Share a secret</div>
                <div class="div-8">
                  Type your key, token or secret below and press Generate button
                </div>

                <div class="div-10">
                    <textarea class="textarea" prop:value=token
                    on:input=move |ev| {
                        set_token.update(|token| *token = event_target_value(&ev));
                    }
                    placeholder="Type your secret here"/>
                </div>
                {move || if status.get() == 0 {
                    view! {
                        <button class="div-11" on:click=on_click>"Generate"</button>
                    }
                } else {
                    view! {
                        <button class="div-11" on:click=on_click>"Generate again"</button>
                    }
                }}
                <div class="div-10">

                </div>
              </div>
            </div>
            <div class="column-2">
              <img
                loading="lazy"
                src="https://cdn.builder.io/api/v1/image/assets/TEMP/a396a2b99a1be7361dffcb33e135ed11e8ec084d779d6c23ac65b9509bf6487a?apiKey=873a6acc1f864ebfbf772f9af3bc2381&"
                class="img"
              />
            </div>
          </div>
        </div>
        <div class="div-12">
          <div class="div-13">Help</div>
          <div class="div-14">Privacy</div>
          <div class="div-15">Terms</div>
        </div>
      </div>
    </div>

      }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_spin::ResponseOptions>();
        resp.set_status(404);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}

#[server(SaveSecret, "/api")]
pub async fn save_secret(token: String) -> Result<(), ServerFnError> {
    println!("Saving value {token}");
    let store = spin_sdk::key_value::Store::open_default()?;
    store
        .set_json("tokenshare_count", &token)
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    Ok(())
}
