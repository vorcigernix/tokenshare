use base64::{engine::general_purpose, Engine as _};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305,
};
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct NoncedSecret {
    nonce: Vec<u8>,
    secret: Vec<u8>,
}

// Reveal token from URL
#[component]
pub fn GetSecret() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());
    let (secret, set_secret) = create_signal("".to_string());
    spawn_local(async move {
        let secret_text = get_secret(id())
            .await
            .unwrap_or_else(|_| "Not found, sorry.".to_string());
        set_secret.update(|text| *text = format!("{}", secret_text));
    });

    view! {
        <section>
            <div class="relative items-center w-full px-5 py-12 mx-auto md:px-12 lg:px-24 max-w-7xl">
                <div class="grid grid-cols-1">
                    <div class="w-full max-w-lg mx-auto my-4 bg-white shadow-xl rounded-xl">
                        <div class="p-6 lg:text-center">
                            <a class="text-blue-600 text-medium flex" href="/">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    viewBox="0 0 24 24"
                                    fill="currentColor"
                                    class="w-6 h-6 pr-2"
                                >
                                    <path
                                        fill-rule="evenodd"
                                        d="M15.75 1.5a6.75 6.75 0 0 0-6.651 7.906c.067.39-.032.717-.221.906l-6.5 6.499a3 3 0 0 0-.878 2.121v2.818c0 .414.336.75.75.75H6a.75.75 0 0 0 .75-.75v-1.5h1.5A.75.75 0 0 0 9 19.5V18h1.5a.75.75 0 0 0 .53-.22l2.658-2.658c.19-.189.517-.288.906-.22A6.75 6.75 0 1 0 15.75 1.5Zm0 3a.75.75 0 0 0 0 1.5A2.25 2.25 0 0 1 18 8.25a.75.75 0 0 0 1.5 0 3.75 3.75 0 0 0-3.75-3.75Z"
                                        clip-rule="evenodd"
                                    ></path>
                                </svg>
                                token.share
                            </a>
                            <h4 class="mt-8 text-2xl font-semibold leading-none tracking-tighter text-neutral-600 lg:text-3xl">
                                This is secret saved under passphrase
                            </h4>
                            <p class="mt-3 text-base leading-relaxed text-gray-500 break-all">
                                {id}
                            </p>

                            <div class="justify-end mt-6">
                                <label for="secret" class="sr-only">
                                    Secret
                                </label>
                                <div class="relative mt-1 rounded-md shadow-sm">
                                    <div class="absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none">
                                        <svg
                                            class="w-8 h-8 text-gray-400"
                                            xmlns="http://www.w3.org/2000/svg"
                                            fill="none"
                                            viewBox="0 0 24 24"
                                            stroke="currentColor"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="1.5"
                                                d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z"
                                            ></path>
                                        </svg>
                                    </div>
                                    <div
                                        type="text"
                                        name="secret"
                                        id="secret"
                                        class="w-full px-5 py-3 pl-10 text-base text-neutral-600 border-none rounded-lg bg-gray-50"
                                    >
                                        {secret}
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[server(GetSecret, "/api")]
pub async fn get_secret(id: String) -> Result<String, ServerFnError> {
    let v: Vec<&str> = id.split("::").collect();
    let store = spin_sdk::key_value::Store::open_default()
        .map_err(|e| ServerFnError::ServerError(format!("Failed to open store: {}", e)))?;

    let nonce_secret = store
        .get_json::<NoncedSecret>(v[0])
        .map_err(|e| ServerFnError::ServerError(format!("Failed to get JSON from store: {}", e)))?;

    let nonce_secret =
        nonce_secret.ok_or_else(|| ServerFnError::ServerError("Secret not found".into()))?;

    let key = general_purpose::URL_SAFE
        .decode(v[1])
        .map_err(|e| ServerFnError::ServerError(format!("Failed to decode key: {}", e)))?;

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
