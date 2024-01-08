use base64::{engine::general_purpose, Engine as _};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct NoncedSecret {
    nonce: Vec<u8>,
    secret: Vec<u8>,
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
                    <Route path="/get/:id" view=RevealSecret/>
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
                            <a class="text-blue-600 text-medium" href="/">token.share</a>
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
            <div class="flex gap-2 mt-6 max-w-7xl lg:justify-center">
                <div class="mt-3 rounded-lg sm:mt-0">
                    <div class="items-center block px-10 py-3.5 text-base font-medium text-center text-blue-600 transition duration-500 ease-in-out transform border-2 border-white shadow-md rounded-xl focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500">based on <a href="https://www.fermyon.com/"> Fermyon </a> & <a href="https://leptos.dev/"> Leptos</a>.</div>
                </div>
            </div>
        </section>
        }
}

// Reveal token from URL
#[component]
fn RevealSecret() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());
    let (secret, set_secret) = create_signal("".to_string());
    spawn_local(async move {
        let secret_text = get_secret(id()).await.unwrap();
        set_secret.update(|text| *text = format!("{}", secret_text));
    });
    #[cfg(feature = "ssr")]
    view! {
        <section>
            <div class="relative items-center w-full px-5 py-12 mx-auto md:px-12 lg:px-24 max-w-7xl">
                <div class="grid grid-cols-1">
                <div class="w-full max-w-lg mx-auto my-4 bg-white shadow-xl rounded-xl">
                    <div class="p-6 lg:text-center">
                    <a class="text-blue-600 text-medium" href="/">token.share</a>
                    <h4 class="mt-8 text-2xl font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl">This is secret saved under passphrase</h4>
                    <p class="mt-3 text-base leading-relaxed text-gray-500">{id}</p>
                    <div class="justify-end mt-6">
                    <label for="secret" class="sr-only">Secret</label>
                    <div class="relative mt-1 rounded-md shadow-sm">
                      <div class="absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none">
                        <svg class="w-8 h-8 text-gray-400" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z"></path>
                        </svg>
                      </div>
                      <div type="text" name="secret" id="secret" class="w-full px-5 py-3 pl-10 text-base text-neutral-600 border-none rounded-lg bg-gray-50">{secret}</div>
                    </div>
                  </div>
                    <div class="flex gap-2 mt-6 max-w-7xl lg:justify-center">
                        <div class="mt-3 rounded-lg sm:mt-0">
                            <div class="items-center block px-10 py-3.5 text-base font-medium text-center text-blue-600 transition duration-500 ease-in-out transform border-2 border-white shadow-md rounded-xl focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500">based on <a href="https://www.fermyon.com/"> Fermyon </a> & <a href="https://leptos.dev/"> Leptos</a>.</div>
                        </div>
                    </div>
                    </div>
                </div>
                </div>
            </div>
            </section>
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

#[server(SaveSecret, "/set")]
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
        secret: ciphertext,
    };

    let store = spin_sdk::key_value::Store::open_default()
        .map_err(|e| ServerFnError::ServerError(format!("Failed to open store: {}", e)))?;

    store
        .set_json(&id, &nonce_secret)
        .map_err(|e| ServerFnError::ServerError(format!("Failed to set JSON in store: {}", e)))?;

    //println!("nonced{:#?}", nonce_secret);
    //println!("key{:#?}", key);

    Ok(keyandid)
}

#[server(RevealSecret, "/get")]
pub async fn get_secret(id: String) -> Result<String, ServerFnError> {
    let v: Vec<&str> = id.split("::").collect();
    let store = spin_sdk::key_value::Store::open_default()
        .map_err(|e| ServerFnError::ServerError(format!("Failed to open store: {}", e)))?;

    let nonce_secret = store
        .get_json::<NoncedSecret>(v[0])
        .map_err(|e| ServerFnError::ServerError(format!("Failed to get JSON from store: {}", e)))?;
    //println!("nonce{:#?}", nonce_secret);

    let nonce_secret =
        nonce_secret.ok_or_else(|| ServerFnError::ServerError("Secret not found".into()))?;

    let key = general_purpose::URL_SAFE
        .decode(v[1])
        .map_err(|e| ServerFnError::ServerError(format!("Failed to decode key: {}", e)))?;
    //println!("key{:#?}", key);

    let cipher = ChaCha20Poly1305::new(
        chacha20poly1305::aead::generic_array::GenericArray::from_slice(&key),
    );

    let nonce =
        chacha20poly1305::aead::generic_array::GenericArray::from_slice(&nonce_secret.nonce);

    let ciphertext = nonce_secret.secret;

    let value = cipher
        .decrypt(&nonce, ciphertext.as_ref())
        .map_err(|e| ServerFnError::ServerError(format!("Decryption failed: {}", e)))?;

    Ok(String::from_utf8(value).unwrap_or_else(|_| "Invalid UTF-8".to_string()))
}
