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

#[function_component(LoginFormTwo)]
pub fn login_form_two() -> Html {
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
        <section class="bg-gradient-to-r from-pink-600 via-orange-500 to-yellow-300 min-h-screen items-center justify-center text-white">
          <div class="container mx-auto">
            <div class="flex justify-center mb-5">
              <h2 class="text-3xl">{"Login Page"}</h2>
            </div>
            <div class="flex justify-center">
              <div class="w-full md:w-1/2 lg:w-1/3">
                <div class="bg-purple-800 rounded-lg p-8 shadow-md mt-10">
                  <div class="flex justify-center">
                    <div
                      class="w-24 h-24 bg-center bg-cover rounded-full"
                      style="
                        background-image: url(https://images.unsplash.com/photo-1690498705508-13f739ebb14b?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=687&q=80);
                      "
                      alt="Login Image"
                    ></div>
                  </div>
                  <h3 class="text-center text-lg font-semibold mt-4">
                    {"Welcome Back"}
                  </h3>
                  <p class="text-center text-sm text-opacity-75">
                    {"Fill in your credentials below to log in!"}
                  </p>
                  <form action="#" class="mt-6" onsubmit={onsubmit}>
                    if !error.is_empty() {
                      <div class="bg-red-600 text-white py-3 font-bold rounded-md text-center mb-4 text-lg">{error}</div>
                    }
                    <div class="mb-6">
                      <div class="relative">
                        <div
                          class="absolute inset-y-0 left-0 pl-3 flex items-center"
                        >
                          <span
                            class="bi bi-person-fill text-black"
                            aria-hidden="true"
                          ></span>
                        </div>
                        <input
                          type="text"
                          class="w-full px-4 py-2 pl-10 rounded-lg bg-opacity-50 placeholder-black text-black"
                          placeholder="Email"
                          ref={input_email_ref}
                          oninput={on_email_change}
                          aria-label="Email"
                        />
                      </div>
                    </div>
                    <div class="mb-6">
                      <div class="relative">
                        <div
                          class="absolute inset-y-0 left-0 pl-3 flex items-center"
                        >
                          <span
                            class="bi bi-lock-fill text-black"
                            aria-hidden="true"
                          ></span>
                        </div>
                        <input
                          type="password"
                          class="w-full px-4 py-2 pl-10 rounded-lg bg-opacity-50 placeholder-black text-black"
                          placeholder="Password"
                          aria-label="Password"
                          ref={input_password_ref}
                          oninput={on_password_change}
                        />
                      </div>
                    </div>
                    <div class="mb-6">
                      <div class="text-right">
                        <a
                          href="#"
                          class="text-sm text-blue-400"
                          aria-label="Forgot Password"
                          >{"Forgot Password"}</a
                        >
                      </div>
                    </div>
                    <div class="mb-6">
                      <button
                        type="submit"
                        class="w-full bg-pink-600 hover:bg-pink-500 text-white font-semibold rounded-lg py-3 transition duration-300"
                      >
                        {"Get Started"}
                      </button>
                    </div>
                  </form>
                  <div class="text-center mt-4 text-sm">
                    <p class="mb-1">{"Don't have an account?"}</p>
                    <a href="#" class="text-blue-400" aria-label="Sign Up"
                      >{"Sign Up"}</a
                    >
                  </div>
                  <div class="flex justify-center mt-8">
                    <div class="flex space-x-4">
                      <a
                        href="#"
                        class="w-10 h-10 flex items-center justify-center rounded-full bg-opacity-50 text-white hover:bg-opacity-75 transition duration-300"
                        aria-label="Facebook"
                      >
                        <i class="bi bi-facebook" aria-hidden="true"></i>
                      </a>
                      <a
                        href="#"
                        class="w-10 h-10 flex items-center justify-center rounded-full bg-opacity-50 text-white hover:bg-opacity-75 transition duration-300"
                        aria-label="LinkedIn"
                      >
                        <i class="bi bi-linkedin" aria-hidden="true"></i>
                      </a>
                      <a
                        href="#"
                        class="w-10 h-10 flex items-center justify-center rounded-full bg-opacity-50 text-white hover:bg-opacity-75 transition duration-300"
                        aria-label="Twitter"
                      >
                        <i class="bi bi-twitter" aria-hidden="true"></i>
                      </a>
                      <a
                        href="#"
                        class="w-10 h-10 flex items-center justify-center rounded-full bg-opacity-50 text-white hover:bg-opacity-75 transition duration-300"
                        aria-label="Instagram"
                      >
                        <i class="bi bi-instagram" aria-hidden="true"></i>
                      </a>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </section>
    }
}
