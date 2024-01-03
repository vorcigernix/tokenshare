use base64::{engine::general_purpose, Engine as _};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct NoncedSecret {
    nonce: Vec<u8>,
    secret: String,
}

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
                    <Route path="/get/:id" view=RevealToken/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (token, set_token) = create_signal("".to_string());
    let (url, set_url) = create_signal("".to_string());
    let on_click = move |_| {
        spawn_local(async move {
            let secret_url = save_secret(token.get().to_string()).await.unwrap();
            //todo: use host from request
            set_url.update(|url| {
                *url = format!("https://tokenshare-ngosnw7s.fermyon.app/get/{}", secret_url)
            });
        });
    };

    view! {

        <section class="h-screen flex flex-col justify-center">
        <div class="flex min- overflow-hidden">
            <div class="flex flex-col justify-center flex-1 px-4 py-12 sm:px-6 lg:flex-none lg:px-20 xl:px-24">
                <div class="w-full max-w-xl mx-auto lg:w-96">
                    <div>
                        <a class="text-blue-600 text-medium" href="/groups/login/">token.share</a>
                        <h2 class="mt-6 text-3xl font-extrabold text-neutral-600">Share your secrets and tokens</h2>
                    </div>

                    <div class="mt-8">
                        <div class="mt-6">
                            <div class="space-y-6">
                                <div>
                                    <label for="token" class="block text-sm font-medium text-neutral-600"> Token or secret </label>
                                    <div class="mt-1">
                                        <textarea id="token" prop:value=token class="textarea" prop:value=token
                                        on:input=move |ev| {
                                            set_token.update(|token| *token = event_target_value(&ev));
                                        }
                                        placeholder="Type your secret here" class="block w-full px-5 py-3 text-base placeholder-gray-300 transition duration-500 ease-in-out transform border border-transparent rounded-lg text-neutral-600 bg-gray-50 focus:outline-none focus:border-transparent focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-300"/>
                                    </div>
                                </div>

                                <div>
                                    <button on:click=on_click class="flex items-center justify-center w-full px-10 py-4 text-base font-medium text-center text-white transition duration-500 ease-in-out transform bg-blue-600 rounded-xl hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">Generate</button>
                                </div>
                            </div>
                            <div class="relative my-4">
                                <div class="absolute inset-0 flex items-center">
                                    <div class="w-full border-t border-gray-300"></div>
                                </div>
                                <div class="relative flex justify-center text-sm">
                                    <span class="px-2 bg-white text-neutral-600">Your unique URL</span>
                                </div>
                            </div>
                            <div>
                                <a href=url class="text-blue-600 text-medium break-all">
                                    {move || url.get()}
                                </a>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <div class="relative flex-1 hidden w-0 overflow-hidden lg:block">
                <img class="absolute inset-0 object-cover w-full h-full" src="https://images.unsplash.com/photo-1483706600674-e0c87d3fe85b?q=80&w=2407&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D" alt=""/>
            </div>
        </div>
    </section>
    <section>
        <div class=" px-5 py-24 mx-auto lg:px-16">
            <div class="flex flex-col w-full mb-8 text-center">
                <span class="mb-4 text-sm font-medium tracking-wide text-gray-500 uppercase">
            Yet another secret sharing service?
            <a href="/about.html" class="font-semibold text-blue-600 lg:mb-0 hover:text-blue-500">Our difference</a>
                </span>
            </div>
            <div class="mx-auto text-center">
            <footer class="bg-white" aria-labelledby="footer-heading">
            <h2 id="footer-heading" class="sr-only">Footer</h2>

            <div class="px-4 py-12 mx-auto bg-gray-50 max-w-7xl sm:px-6 lg:px-16">
              <div class="flex flex-wrap items-baseline lg:justify-center">
                <span class="mt-2 text-sm font-light text-gray-500">
                  Built on
                  <a href="https://developer.fermyon.com/" class="mx-2 text-blue hover:text-gray-500" rel="noopener noreferrer">@Fermyon</a> and Leptos
                </span>
              </div>
            </div>
          </footer>

            </div>
        </div>
    </section>
    }
}

// Reveal token from URL
#[component]
fn RevealToken() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());
    let (secret, set_secret) = create_signal("".to_string());
    spawn_local(async move {
        let secret_text = get_secret(id()).await.unwrap();
        set_secret.update(|text| *text = format!("{}", secret_text));
    });
    #[cfg(feature = "ssr")]
    view! {
        <h1>"Data"{id}</h1>
        <h2>"Encrypted"{secret}</h2>
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

    view! {
        <h1>"Not Found"</h1>
    }
}

#[server(SaveSecret, "/api")]
pub async fn save_secret(token: String) -> Result<String, ServerFnError> {
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let id = Uuid::new_v4().to_string();
    let keyencoded: String = general_purpose::URL_SAFE.encode(&key);
    let keyandid = format!("{}::{}", id, keyencoded);

    let ciphertext = cipher
        .encrypt(&nonce, token.as_ref())
        .map_err(|e| ServerFnError::ServerError(format!("Encryption failed: {}", e)))?;

    let nonce_secret: NoncedSecret = NoncedSecret {
        nonce: nonce.to_vec(),
        secret: format!("{:?}", ciphertext),
    };

    let store = spin_sdk::key_value::Store::open_default()
        .map_err(|e| ServerFnError::ServerError(format!("Failed to open store: {}", e)))?;

    store
        .set_json(&id, &nonce_secret)
        .map_err(|e| ServerFnError::ServerError(format!("Failed to set JSON in store: {}", e)))?;

    println!("nonced{:#?}", nonce_secret);
    println!("key{:#?}", key);

    Ok(keyandid)
}

#[server(RevealToken, "/get")]
pub async fn get_secret(id: String) -> Result<String, ServerFnError> {
    let v: Vec<&str> = id.split("::").collect();
    let store = spin_sdk::key_value::Store::open_default()
        .map_err(|e| ServerFnError::ServerError(format!("Failed to open store: {}", e)))?;

    let nonce_secret = store
        .get_json::<NoncedSecret>(v[0])
        .map_err(|e| ServerFnError::ServerError(format!("Failed to get JSON from store: {}", e)))?;
    println!("nonce{:#?}", nonce_secret);

    let nonce_secret =
        nonce_secret.ok_or_else(|| ServerFnError::ServerError("Secret not found".into()))?;

    let key = general_purpose::URL_SAFE
        .decode(v[1])
        .map_err(|e| ServerFnError::ServerError(format!("Failed to decode key: {}", e)))?;
    println!("key{:#?}", key);

    let cipher = ChaCha20Poly1305::new(
        chacha20poly1305::aead::generic_array::GenericArray::from_slice(&key),
    );

    let nonce =
        chacha20poly1305::aead::generic_array::GenericArray::from_slice(&nonce_secret.nonce);

    let ciphertext = nonce_secret.secret.as_bytes();

    let value = cipher
        .decrypt(&nonce, ciphertext)
        .map_err(|e| ServerFnError::ServerError(format!("Decryption failed: {}", e)))?;

    Ok(String::from_utf8(value).unwrap_or_else(|_| "Invalid UTF-8".to_string()))
}
