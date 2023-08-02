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

#[function_component(ContactFormOne)]
pub fn contact_form_one() -> Html {
    let error_handle = use_state(String::default);
    let error = (*error_handle).clone();

    let email_valid_handle = use_state(|| true);
    let email_valid = (*email_valid_handle).clone();

    let name_valid_handle = use_state(|| true);
    let name_valid = (*name_valid_handle).clone();

    let subject_valid_handle = use_state(|| true);
    let subject_valid = (*subject_valid_handle).clone();

    let message_valid_handle = use_state(|| true);
    let message_valid = (*message_valid_handle).clone();

    let input_email_ref = use_node_ref();
    let input_email_handle = use_state(String::default);
    let input_email = (*input_email_handle).clone();

    let input_name_ref = use_node_ref();
    let input_name_handle = use_state(String::default);
    let input_name = (*input_name_handle).clone();

    let input_subject_ref = use_node_ref();
    let input_subject_handle = use_state(String::default);
    let input_subject = (*input_subject_handle).clone();

    let input_message_ref = use_node_ref();
    let input_message_handle = use_state(String::default);
    let input_message= (*input_message_handle).clone();

    let validate_email = |email: &str| {
        let pattern = Regex::new(r"^[^ ]+@[^ ]+\.[a-z]{2,3}$").unwrap();
        pattern.is_match(email)
    };

    let validate_input = |field: &str| !field.is_empty();

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

    let on_name_change = {
        let input_name_ref = input_name_ref.clone();

        Callback::from(move |_| {
            let input = input_name_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_name_handle.set(value);
                name_valid_handle.set(validate_input(&input.value()));
            }
        })
    };

    let on_subject_change = {
        let input_subject_ref = input_subject_ref.clone();

        Callback::from(move |_| {
            let input = input_subject_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_subject_handle.set(value);
                subject_valid_handle.set(validate_input(&input.value()));
            }
        })
    };

    let on_message_change = {
        let input_message_ref = input_message_ref.clone();

        Callback::from(move |_| {
            let input = input_message_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_message_handle.set(value);
                message_valid_handle.set(validate_input(&input.value()));
            }
        })
    };

    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();

        let email_ref = input_email.clone();
        let name_ref = input_name.clone();
        let subject_ref = input_subject.clone();
        let message_ref = input_message.clone();
        let error_handle = error_handle.clone();
        console::log_1(&format!("Email: {}, Name: {}, Subject: {}, Message: {}", input_email, input_name, input_subject, input_message).into());

        spawn_local(async move {
            let email_val = email_ref.clone();
            let _name_val = name_ref.clone();
            let subject_val = subject_ref.clone();
            let _message_val = message_ref.clone();

            let error_handle = error_handle.clone();
            if email_valid && name_valid && subject_valid && message_valid {
              // TODO: create a contact us endpoint
              let response = login_user(email_val, subject_val).await;
              match response {
                  Ok(_) => {
                      console::log_1(&"success".into());
                      let window: Window = web_sys::window().expect("window not available");
                      let location = window.location();
                      let _ = location.set_href("/home");
                  }
                  Err(err) => {
                      error_handle.set(err);
                  }
              }
            }
            else {
              error_handle.set("Please provide valid contact information!".into());
            }
        });
    });

    html! {
        <section class="bg-gradient-to-r from-purple-500 to-yellow-700 h-screen">
          <div class="flex items-center justify-center h-full">
            <div
              class="w-full max-w-5xl bg-white rounded-lg overflow-hidden shadow-lg p-8 mx-4 md:mx-8 md:grid md:grid-cols-2 md:gap-10"
            >
              <div
                class="contact1-pic hidden md:block w-full mb-6 md:mb-0 md:mr-6 flex justify-center items-center"
              >
                <img
                  src="https://cdn.pixabay.com/photo/2023/06/09/19/37/letter-8052497_960_720.png"
                  alt="Contact Image"
                  class="w-full max-w-2xl h-auto"
                  role="img"
                  aria-label="Contact Image"
                />
              </div>
              <form class="w-full flex-grow" aria-label="Contact Form" onsubmit={onsubmit}>
                if !error.is_empty() {
                  <div class="mb-3 error bg-red-600 text-white px-4 py-3 font-semibold rounded-md text-center text-base">{error}</div>
                }
                <span
                  class="text-3xl font-bold text-gray-900 block text-center mb-6"
                >
                  {"Contact US"}
                </span>
                <div class="wrap-input1 validate-input mb-6">
                  <input
                    type="text"
                    class="w-full bg-gray-200 focus:bg-white border border-transparent rounded-lg px-4 py-3 placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-teal-600 text-gray-900"
                    id="name"
                    name="name"
                    aria-required="true"
                    placeholder="Your Name"
                    required={true}
                    ref={input_name_ref}
                    oninput={on_name_change}
                    aria-label="Your Name"
                  />
                </div>
                if !name_valid {
                    <div class="error-txt text-red-500 text-sm mb-2">{"Name can't be blank"}</div>
                }
                <div class="validate-input mb-6">
                  <input
                    type="text"
                    class="input1 w-full bg-gray-200 focus:bg-white border border-transparent rounded-lg px-4 py-3 placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-teal-600 text-gray-900"
                    id="email"
                    name="email"
                    aria-required="true"
                    placeholder="Your Email"
                    required={true}
                    ref={input_email_ref}
                    oninput={on_email_change}
                    aria-label="Your Email"
                  />
                </div>
                if !email_valid {
                    <div class="error-txt text-red-500 text-sm mb-2">{"Enter a valid email address"}</div>
                }
                <div class="validate-input mb-6">
                  <input
                    type="text"
                    class="input1 w-full bg-gray-200 focus:bg-white border border-transparent rounded-lg px-4 py-3 placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-teal-600 text-gray-900"
                    id="subject"
                    name="subject"
                    aria-required="true"
                    placeholder="Your Subject"
                    required={true}
                    ref={input_subject_ref}
                    oninput={on_subject_change}
                    aria-label="Your Subject"
                  />
                </div>
                if !subject_valid {
                    <div class="text-red-500 text-sm mb-2">{"Subject can't be blank"}</div>
                }
                <div class="wrap-input1 validate-input mb-6">
                  <textarea
                    class="w-full bg-gray-200 focus:bg-white border border-transparent rounded-lg px-4 py-3 placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-teal-600 text-gray-900"
                    id="message"
                    name="message"
                    aria-required="true"
                    placeholder="Your Message"
                    required={true}
                    ref={input_message_ref}
                    oninput={on_message_change}
                    aria-label="Your Message"
                  ></textarea>
                </div>
                if !message_valid {
                    <div class="text-red-500 text-sm mb-2">{"Message can't be blank"}</div>
                }
                <div class="container-contact1-form-btn">
                  <button
                    class="w-full bg-green-500 hover:bg-green-600 text-white font-bold py-3 rounded-lg transition duration-300"
                    type="submit"
                    aria-label="Submit"
                  >
                    <span>
                      {"Send Email"}
                      <i class="fa fa-long-arrow-right ml-2" aria-hidden="true"></i>
                    </span>
                  </button>
                </div>
              </form>
            </div>
          </div>
        </section>
    }
}
