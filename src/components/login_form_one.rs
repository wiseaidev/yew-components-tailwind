use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, HtmlInputElement, Window};
use yew::prelude::*;
use regex::Regex;

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

    let email_valid_handle = use_state(|| true);
    let email_valid = (*email_valid_handle).clone();

    let eye_active_handle = use_state(|| false);
    let eye_active = (*eye_active_handle).clone();

    let password_valid_handle = use_state(|| true);
    let password_valid = (*password_valid_handle).clone();

    let password_type_handle = use_state(|| "text");
    let password_type = (*password_type_handle).clone();

    let input_email_ref = use_node_ref();
    let input_email_handle = use_state(String::default);
    let input_email = (*input_email_handle).clone();

    let input_password_ref = use_node_ref();
    let input_password_handle = use_state(String::default);
    let input_password = (*input_password_handle).clone();

    let validate_email = |email: &str| {
        let pattern = Regex::new(r"^[^ ]+@[^ ]+\.[a-z]{2,3}$").unwrap();
        pattern.is_match(email)
    };

    let validate_password = |password: &str| !password.is_empty();

    let on_email_change = {
        let input_email_ref = input_email_ref.clone();

        Callback::from(move |_| {
            let input = input_email_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_email_handle.set(value);
                email_valid_handle.set(validate_email(&input.value()));
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
                password_valid_handle.set(validate_password(&input.value()));
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
            if email_valid && password_valid {
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
            }
            else {
              error_handle.set("Please provide a valid email and password!".into());
            }
        });
    });

    let on_toggle_password = {
        Callback::from(move |_| {
            if eye_active {
                password_type_handle.set("password".into())
            }
            else {
                password_type_handle.set("text".into())
            }
            eye_active_handle.set(!eye_active);
        })
    };

    html! {
        <div
          class="min-h-screen bg-gradient-to-tr from-blue-500 to-purple-600 flex items-center justify-center"
        >
          <div class="w-96 bg-white rounded-lg shadow-lg p-8">
            <form class="flex flex-col space-y-6" onsubmit={onsubmit}>
              if !error.is_empty() {
                <div class="mb-3 error bg-red-600 text-white px-4 py-3 font-semibold rounded-md text-center text-base">{error}</div>
              }
              <span class="text-4xl text-gray-800 pb-6">{"Login"}</span>
              <div>
                <label for="username" class="text-base text-gray-800"
                  >{"Username"}</label
                >
                <div class="relative mt-6">
                  <i
                    class="text-2xl fa fa-person text-gray-500 absolute left-3 top-1/2 transform -translate-y-1/2"
                  ></i>
                  <input
                    id="username"
                    class="input w-full px-4 py-3 rounded-lg border border-gray-300 bg-gray-100 pl-12"
                    type="text"
                    name="username"
                    placeholder="Email"
                    aria-required="true"
                    required={true}
                    ref={input_email_ref}
                    oninput={on_email_change}
                  />
                </div>
              </div>
              if !email_valid {
                  <div class="error-txt text-red-500 text-sm my-2">{"Enter a valid email address"}</div>
              }
              <div class="flex items-center justify-between">
                <label for="password" class="text-base text-gray-800">{"Password"}</label>
                <a href="#" class="text-base text-gray-900">{"Forgot?"}</a>
              </div>
              <div class="relative mt-1">
                <i
                  class="text-2xl fa fa-lock text-gray-500 absolute left-3 top-1/2 transform -translate-y-1/2"
                ></i>
                <input
                  id="password"
                  class="input w-full px-4 py-3 rounded-lg border border-gray-300 bg-gray-100 pl-12"
                  type={&*password_type}
                  required={true}
                  name="password"
                  aria-required="true"
                      placeholder="Password"
                  ref={input_password_ref}
                  oninput={on_password_change}
                />
                <i
                  class={format!("cursor-pointer absolute right-4 top-1/2 transform -translate-y-1/2 text-2xl text-gray-600 toggle-button fa {}", if eye_active { "fa-eye" } else { "fa-eye-slash " })}
                  aria-hidden="true"
                  role="presentation"
                  onclick={on_toggle_password}
                ></i>
              </div>
              if !password_valid {
                 <div class="error-txt text-red-500 text-sm my-2">{"Password can't be blank"}</div>
              }
              <div class="w-full">
                <button
                  class="btn-social bg-indigo-600 hover:bg-indigo-700 text-white w-full py-3 rounded-lg text-base font-medium tracking-wide"
                  type="submit"
                >
                  <i class="fa fa-box-arrow-in-right me-2"></i>
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
                    <i class="fa fa-facebook"></i>
                  </button>
                  <button
                    class="btn-social bg-red-600 hover:bg-red-700 text-white w-12 h-12 rounded-lg text-xl leading-12"
                    aria-label="Sign in with Google"
                  >
                    <i class="fa fa-google"></i>
                  </button>
                  <button
                    class="btn-social bg-blue-400 hover:bg-blue-500 text-white w-12 h-12 rounded-lg text-xl leading-12"
                    aria-label="Sign in with Twitter"
                  >
                    <i class="fa fa-twitter"></i>
                  </button>
                  <button
                    class="btn-social bg-blue-900 hover:bg-blue-800 text-white w-12 h-12 rounded-lg text-xl leading-12"
                    aria-label="Sign in with LinkedIn"
                  >
                    <i class="fa fa-linkedin"></i>
                  </button>
                </div>
              </div>
            </form>
          </div>
        </div>
    }
}
