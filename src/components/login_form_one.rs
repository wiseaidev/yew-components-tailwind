use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, HtmlInputElement, Window};
use yew::prelude::*;

use crate::api::auth::login_user;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct LoginUserSchema {
    email: String,
    password: String,
}

#[function_component(LoginFormOne)]
pub fn login_form_one() -> Html {
    let error_handle = use_state(String::default);
    let error = (*error_handle).clone();

    let input_email_ref = use_node_ref();
    let input_email_handle = use_state(String::default);
    let input_email = (*input_email_handle).clone();

    let input_password_ref = use_node_ref();
    let input_password_handle = use_state(String::default);
    let input_password = (*input_password_handle).clone();

    let on_email_change = {
        let input_email_ref = input_email_ref.clone();

        Callback::from(move |_| {
            let input = input_email_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_email_handle.set(value);
            }
        })
    };

    let on_password_change = {
        let input_password_ref = input_password_ref.clone();

        Callback::from(move |_| {
            let input = input_password_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_password_handle.set(value);
            }
        })
    };

    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();

        let email_ref = input_password.clone();
        let password_ref = input_password.clone();
        let error_handle = error_handle.clone();
        console::log_1(&format!("Email: {}, Password: {}", input_email, input_password).into());

        spawn_local(async move {
            let email_val = email_ref.clone();
            let password_val = password_ref.clone();
            let error_handle = error_handle.clone();
            let response = login_user(email_val, password_val).await;
            match response {
                Ok(_) => {
                    console::log_1(&"success".into());
                    let window: Window = web_sys::window().expect("window not available");
                    let location = window.location();
                    let _ = location.set_href("/error");
                }
                Err(err) => {
                    error_handle.set(err);
                }
            }
        });
    });

    html! {
        <div
          class="min-h-screen bg-gradient-to-tr from-blue-500 to-purple-600 flex items-center justify-center"
        >
          <div class="w-96 bg-white rounded-lg shadow-lg p-8">
            <form class="flex flex-col space-y-6" onsubmit={onsubmit}>
              if !error.is_empty() {
                <div class="error">{error}</div>
              }
              <span class="text-4xl text-gray-800 pb-6">{"Login"}</span>
              <div>
                <label for="username" class="text-base text-gray-800"
                  >{"Username"}</label
                >
                <div class="relative mt-6">
                  <i
                    class="bi bi-person text-gray-500 absolute left-3 top-1/2 transform -translate-y-1/2"
                  ></i>
                  <input
                    id="username"
                    class="input w-full px-4 py-3 rounded-lg border border-gray-300 bg-gray-100 pl-12"
                    type="text"
                    name="username"
                    placeholder="Email"
                    aria-required="true"
                    ref={input_email_ref}
                    oninput={on_email_change}
                  />
                </div>
              </div>
              <div class="flex items-center justify-between">
                <label for="pass" class="text-base text-gray-800">{"Password"}</label>
                <a href="#" class="text-base text-gray-900">{"Forgot?"}</a>
              </div>
              <div class="relative mt-1">
                <i
                  class="bi bi-lock text-gray-500 absolute left-3 top-1/2 transform -translate-y-1/2"
                ></i>
                <input
                  id="pass"
                  class="input w-full px-4 py-3 rounded-lg border border-gray-300 bg-gray-100 pl-12"
                  type="password"
                  name="pass"
                  aria-required="true"
                      placeholder="Password"
                  ref={input_password_ref}
                  oninput={on_password_change}
                />
              </div>
              <div class="w-full">
                <button
                  class="btn-social bg-indigo-600 hover:bg-indigo-700 text-white w-full py-3 rounded-lg text-base font-medium tracking-wide"
                  type="submit"
                >
                  <i class="bi bi-box-arrow-in-right me-2"></i>
                  {"Sign In"}
                </button>
              </div>
              <div class="w-full text-center">
                <span class="text-base text-gray-800">{"Not a member?"}</span>
                <a href="#" class="text-base text-gray-800">{"Sign Up Now"}</a>
              </div>
              <div class="mt-8">
                <div class="relative text-center text-gray-800 mt-4">
                  <span class="px-4 bg-white relative z-10">{"Or Sign In With"}</span>
                  <div class="absolute inset-0 flex items-center">
                    <div class="w-full h-px bg-gray-400"></div>
                  </div>
                </div>
                <div class="flex justify-center space-x-4 mt-12">
                  <button
                    class="btn-social bg-blue-600 hover:bg-blue-700 text-white w-12 h-12 rounded-lg text-xl leading-12"
                    aria-label="Sign in with Facebook"
                  >
                    <i class="bi bi-facebook"></i>
                  </button>
                  <button
                    class="btn-social bg-red-600 hover:bg-red-700 text-white w-12 h-12 rounded-lg text-xl leading-12"
                    aria-label="Sign in with Google"
                  >
                    <i class="bi bi-google"></i>
                  </button>
                  <button
                    class="btn-social bg-blue-400 hover:bg-blue-500 text-white w-12 h-12 rounded-lg text-xl leading-12"
                    aria-label="Sign in with Twitter"
                  >
                    <i class="bi bi-twitter"></i>
                  </button>
                  <button
                    class="btn-social bg-blue-900 hover:bg-blue-800 text-white w-12 h-12 rounded-lg text-xl leading-12"
                    aria-label="Sign in with LinkedIn"
                  >
                    <i class="bi bi-linkedin"></i>
                  </button>
                </div>
              </div>
            </form>
          </div>
        </div>
    }
}
