use base64::{engine::general_purpose, Engine as _};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305,
};
use leptos::{ev::MouseEvent, *};
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
    let on_click = move |_event: MouseEvent| {
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
                    <a class="text-blue-600 text-medium flex" href="/">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-6 h-6 pr-2">
                    <path fill-rule="evenodd" d="M15.75 1.5a6.75 6.75 0 0 0-6.651 7.906c.067.39-.032.717-.221.906l-6.5 6.499a3 3 0 0 0-.878 2.121v2.818c0 .414.336.75.75.75H6a.75.75 0 0 0 .75-.75v-1.5h1.5A.75.75 0 0 0 9 19.5V18h1.5a.75.75 0 0 0 .53-.22l2.658-2.658c.19-.189.517-.288.906-.22A6.75 6.75 0 1 0 15.75 1.5Zm0 3a.75.75 0 0 0 0 1.5A2.25 2.25 0 0 1 18 8.25a.75.75 0 0 0 1.5 0 3.75 3.75 0 0 0-3.75-3.75Z" clip-rule="evenodd" />
                    </svg>token.share</a>
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
    <div class="px-5 py-12 mx-auto bg-gray-50 max-w-7xl sm:px-6 md:flex md:items-center md:justify-between lg:px-20">
        <div class="flex justify-center mb-8 space-x-6 md:order-last md:mb-0">
        <a href="https://www.fermyon.com/" class="text-gray-400 hover:text-gray-500">
            <span class="sr-only">Fermyon</span>
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-6 h-6" aria-hidden="true">
            <path fill-rule="evenodd" d="M3 6a3 3 0 0 1 3-3h12a3 3 0 0 1 3 3v12a3 3 0 0 1-3 3H6a3 3 0 0 1-3-3V6Zm14.25 6a.75.75 0 0 1-.22.53l-2.25 2.25a.75.75 0 1 1-1.06-1.06L15.44 12l-1.72-1.72a.75.75 0 1 1 1.06-1.06l2.25 2.25c.141.14.22.331.22.53Zm-10.28-.53a.75.75 0 0 0 0 1.06l2.25 2.25a.75.75 0 1 0 1.06-1.06L8.56 12l1.72-1.72a.75.75 0 1 0-1.06-1.06l-2.25 2.25Z" clip-rule="evenodd" />
        </svg>
        </a>

        <a href="https://leptos.dev/" class="text-gray-400 hover:text-gray-500">
            <span class="sr-only">Leptos</span>
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-6 h-6" aria-hidden="true">
                    <path fill-rule="evenodd" d="M3 6a3 3 0 0 1 3-3h12a3 3 0 0 1 3 3v12a3 3 0 0 1-3 3H6a3 3 0 0 1-3-3V6Zm14.25 6a.75.75 0 0 1-.22.53l-2.25 2.25a.75.75 0 1 1-1.06-1.06L15.44 12l-1.72-1.72a.75.75 0 1 1 1.06-1.06l2.25 2.25c.141.14.22.331.22.53Zm-10.28-.53a.75.75 0 0 0 0 1.06l2.25 2.25a.75.75 0 1 0 1.06-1.06L8.56 12l1.72-1.72a.75.75 0 1 0-1.06-1.06l-2.25 2.25Z" clip-rule="evenodd" />
                </svg>
        </a>

        <a href="https://twitter.com/vorcigernix" class="text-gray-400 hover:text-gray-500">
            <span class="sr-only">Twitter</span>
            <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
            <path d="M8.29 20.251c7.547 0 11.675-6.253 11.675-11.675 0-.178 0-.355-.012-.53A8.348 8.348 0 0022 5.92a8.19 8.19 0 01-2.357.646 4.118 4.118 0 001.804-2.27 8.224 8.224 0 01-2.605.996 4.107 4.107 0 00-6.993 3.743 11.65 11.65 0 01-8.457-4.287 4.106 4.106 0 001.27 5.477A4.072 4.072 0 012.8 9.713v.052a4.105 4.105 0 003.292 4.022 4.095 4.095 0 01-1.853.07 4.108 4.108 0 003.834 2.85A8.233 8.233 0 012 18.407a11.616 11.616 0 006.29 1.84"></path>
            </svg>
        </a>

        <a href="https://github.com/vorcigernix/tokenshare" class="text-gray-400 hover:text-gray-500">
            <span class="sr-only">GitHub</span>
            <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
            <path fill-rule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" clip-rule="evenodd"></path>
            </svg>
        </a>
        </div>

        <div class="mt-8 md:mt-0 md:order-1">
        <span class="mt-2 text-sm font-light text-gray-500">
            Copyleft 2024
            <a href="https://github.com/vorcigernix/tokenshare" class="mx-2 text-wickedblue hover:text-gray-500" rel="noopener noreferrer">@vorcigernix</a>. Since 1974
        </span>
        </div>
    </div>
    </section>
    }
}


// Reveal token from URL
#[component]
fn RevealSecret() -> impl IntoView {

    let params = use_params_map();
    let token = move || params.with(|params| params.get("id").cloned().unwrap_or_default());
    //println!("params{:#?}", params);
    let (secret, set_secret) = create_signal("".to_string());
    let _ = spawn_local(async move {
        let secret_text = get_secret(token()).await.unwrap();
        set_secret.update(|text| *text = format!("{}", secret_text));
    });

    view! {
        <section>
            <div class="relative items-center w-full px-5 py-12 mx-auto md:px-12 lg:px-24 max-w-7xl">
                <div class="grid grid-cols-1">
                    <div class="w-full max-w-lg mx-auto my-4 bg-white shadow-xl rounded-xl">
                        <div class="p-6 lg:text-center">
                        <a class="text-blue-600 text-medium flex" href="/">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-6 h-6 pr-2">
                        <path fill-rule="evenodd" d="M15.75 1.5a6.75 6.75 0 0 0-6.651 7.906c.067.39-.032.717-.221.906l-6.5 6.499a3 3 0 0 0-.878 2.121v2.818c0 .414.336.75.75.75H6a.75.75 0 0 0 .75-.75v-1.5h1.5A.75.75 0 0 0 9 19.5V18h1.5a.75.75 0 0 0 .53-.22l2.658-2.658c.19-.189.517-.288.906-.22A6.75 6.75 0 1 0 15.75 1.5Zm0 3a.75.75 0 0 0 0 1.5A2.25 2.25 0 0 1 18 8.25a.75.75 0 0 0 1.5 0 3.75 3.75 0 0 0-3.75-3.75Z" clip-rule="evenodd" />
                        </svg>token.share</a>
                        <h4 class="mt-8 text-2xl font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl">This is secret saved under passphrase</h4>
                        <p class="mt-3 text-base leading-relaxed text-gray-500 break-all">{token}</p>
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

#[server(GetSecret, "/api")]
pub async fn get_secret(id: String) -> Result<String, ServerFnError> {
    //println!("id{:#?}", id);
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
