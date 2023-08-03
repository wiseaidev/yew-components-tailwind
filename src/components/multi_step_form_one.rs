use web_sys::HtmlInputElement;
use yew::prelude::*;
use regex::Regex;

#[function_component(MultiStepFormOne)]
pub fn multi_step_form_one() -> Html {
    // TODO:  Figure out how to refactor all of this mess!
    let error_handle = use_state(String::default);
    let error = (*error_handle).clone();

    let email_valid_handle = use_state(|| false);
    let email_valid = (*email_valid_handle).clone();

    let full_name_valid_handle = use_state(|| false);
    let full_name_valid = (*full_name_valid_handle).clone();

    let phone_number_valid_handle = use_state(|| false);
    let phone_number_valid = (*phone_number_valid_handle).clone();

    let address_valid_handle = use_state(|| false);
    let address_valid = (*address_valid_handle).clone();

    let birthday_valid_handle = use_state(|| false);
    let birthday_valid = (*birthday_valid_handle).clone();

    let gender_valid_handle = use_state(|| false);
    let gender_valid = (*gender_valid_handle).clone();

    let username_valid_handle = use_state(|| false);
    let username_valid = (*username_valid_handle).clone();

    let password_valid_handle = use_state(|| false);
    let password_valid = (*password_valid_handle).clone();

    let input_email_ref = use_node_ref();
    let input_email_handle = use_state(String::default);
    let input_email = (*input_email_handle).clone();

    let input_full_name_ref = use_node_ref();
    let input_full_name_handle = use_state(String::default);
    let input_full_name = (*input_full_name_handle).clone();

    let input_subject_ref = use_node_ref();
    let input_subject_handle = use_state(String::default);
    let input_subject = (*input_subject_handle).clone();

    let input_phone_number_ref = use_node_ref();
    let input_phone_number_handle = use_state(String::default);
    let input_phone_number = (*input_phone_number_handle).clone();

    let input_address_ref = use_node_ref();
    let input_address_handle = use_state(String::default);
    let input_address = (*input_address_handle).clone();

    let input_birthday_ref = use_node_ref();
    let input_birthday_handle = use_state(String::default);
    let input_birthday_number = (*input_birthday_handle).clone();

    let input_gender_ref = use_node_ref();
    let input_gender_handle = use_state(String::default);
    let input_gender = (*input_gender_handle).clone();

    let input_username_ref = use_node_ref();
    let input_username_handle = use_state(String::default);
    let input_username_number = (*input_username_handle).clone();

    let input_password_ref = use_node_ref();
    let input_password_handle = use_state(String::default);
    let input_password_number = (*input_password_handle).clone();

    let current_step_handle = use_state(|| 0);
    let current_step = (*current_step_handle).clone();

    let on_next = {
        let error_handle = error_handle.clone();
        let counter = current_step_handle.clone();
        move |_| {
            match *counter {
                0 => {
                  if full_name_valid && email_valid {
                        let value = *counter + 1;
                        counter.set(value);
                        error_handle.set("".to_string());
                  }
                  else {
                        error_handle.set("Please provide a valid full name and email address!".to_string());
                  }
                },
                1 => {
                  if phone_number_valid && address_valid {
                        let value = *counter + 1;
                        counter.set(value);
                        error_handle.set("".to_string());
                  }
                  else {
                        error_handle.set("Please provide a valid phone number and address!".to_string());
                  }
                },
                2 => {
                  if birthday_valid && gender_valid {
                        let value = *counter + 1;
                        counter.set(value);
                        error_handle.set("".to_string());
                  }
                  else {
                        error_handle.set("Please provide a valid birth date and gender!".to_string());
                  }
                },
                _ => println!("Ain't special"),
            }
        }
    };

    let on_previous = {
        let counter = current_step_handle.clone();
        move |_| {
            let value = *counter - 1;
            counter.set(value);
        }
    };

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

    let on_full_name_change = {
        let input_full_name_ref = input_full_name_ref.clone();

        Callback::from(move |_| {
            let input = input_full_name_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_full_name_handle.set(value);
                full_name_valid_handle.set(validate_input(&input.value()));
            }
        })
    };

    let on_phone_number_change = {
        let input_phone_number_ref = input_phone_number_ref.clone();

        Callback::from(move |_| {
            let input = input_phone_number_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_phone_number_handle.set(value);
                phone_number_valid_handle.set(validate_input(&input.value()));
            }
        })
    };

    let on_address_change = {
        let input_address_ref = input_address_ref.clone();

        Callback::from(move |_| {
            let input = input_address_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_address_handle.set(value);
                address_valid_handle.set(validate_input(&input.value()));
            }
        })
    };

    let on_birthday_change = {
        let input_birthday_ref = input_birthday_ref.clone();

        Callback::from(move |_| {
            let input = input_birthday_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_birthday_handle.set(value);
                birthday_valid_handle.set(validate_input(&input.value()));
            }
        })
    };

    let on_gender_change = {
        let input_gender_ref = input_gender_ref.clone();

        Callback::from(move |_| {
            let input = input_gender_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_gender_handle.set(value);
                gender_valid_handle.set(validate_input(&input.value()));
            }
        })
    };

    let on_username_change = {
        let input_username_ref = input_username_ref.clone();

        Callback::from(move |_| {
            let input = input_username_ref.cast::<HtmlInputElement>();

            if let Some(input) = input {
                let value = input.value();
                input_username_handle.set(value);
                username_valid_handle.set(validate_input(&input.value()));
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
                password_valid_handle.set(validate_input(&input.value()));
            }
        })
    };

    // TODO: add an onclick handler on last button

    let render_progress_item = |(index, _): (usize, &str)| {
        // TODO: Refactor and convert to a for loop/map, make this tag show up: <span class="check bg-pink-800 z-50"></span>
        html! {
            <div class="flex items-center text-left justify-center my-10">
                <div class="step">
                    <p class="font-semibold mb-2 text-pink-800">{"Step 1"}</p>
                    <div class="flex items-center">
                        <div class="outer-check bullet border-2 border-pink-800 rounded-full h-7 w-7 flex items-center justify-center">
                            <span class="step-index font-semibold text-pink-800">{"1"}</span>
                            if index > 0 {
                                 <span class="check bg-pink-800 z-50"></span>
                            }
                        </div>
                        if index > 0 {
                            <span class="line bg-pink-800 h-1 w-10"></span>
                        }
                        else {
                          <span class="line bg-grey-400 h-1 w-10"></span>
                        }
                    </div>
                </div>
                <div class="step">
                    <p class="font-semibold mb-2 text-pink-800">{"Step 2"}</p>
                    <div class="flex items-center">
                        <div class="outer-check bullet border-2 border-pink-800 rounded-full h-7 w-7 flex items-center justify-center">
                            <span class="step-index font-semibold text-pink-800">{"2"}</span>
                            if index > 1 {
                                 <span class="check bg-pink-800"></span>
                            }
                        </div>
                        if index > 1 {
                            <span class="line bg-pink-800 h-1 w-10"></span>
                        }
                        else {
                          <span class="line bg-grey-400 h-1 w-10"></span>
                        }
                    </div>
                </div>
                <div class="step">
                    <p class="font-semibold mb-2 text-pink-800">{"Step 3"}</p>
                    <div class="flex items-center">
                        <div class="outer-check bullet border-2 border-pink-800 rounded-full h-7 w-7 flex items-center justify-center">
                            <span class="step-index font-semibold text-pink-800">{"3"}</span>
                            if index > 2 {
                                 <span class="check bg-pink-800"></span>
                            }
                        </div>
                        if index > 2 {
                            <span class="line bg-pink-800 h-1 w-10"></span>
                        }
                        else {
                          <span class="line bg-grey-400 h-1 w-10"></span>
                        }
                    </div>
                </div>
                <div class="step">
                    <p class="font-semibold mb-2 text-pink-800">{"Step 4"}</p>
                    <div class="flex items-center">
                        <div class="outer-check bullet border-2 border-pink-800 rounded-full h-7 w-7 flex items-center justify-center">
                            <span class="step-index font-semibold text-pink-800">{"4"}</span>
                        </div>
                    </div>
                </div>
            </div>
        }
    };

    let current_step_content = match current_step {
        0 => html! {
            <div class="page ml-0 transition-transform duration-300">
                <div class="title text-left text-xl font-semibold mb-4">{"Personal Information"}</div>
                <div class="field mb-6">
                    <div class="label font-semibold text-pink-800">{"Full Name"}</div>
                    <input
                      id="full-name"
                      class="w-full border border-pink-800 rounded px-4 py-2"
                      type="text"
                      name="full-name"
                      placeholder="Full Name"
                      aria-required="true"
                      required={true}
                      ref={input_full_name_ref}
                      oninput={on_full_name_change}
                    />
                </div>
                if !full_name_valid {
                   <div class="error-txt text-red-500 text-sm my-2">{"Full name can't be blank!"}</div>
                }
                <div class="field mb-6">
                    <div class="label font-semibold text-pink-800">{"Email Address"}</div>
                    <input
                      id="email"
                      class="w-full border border-pink-800 rounded px-4 py-2"
                      type="email"
                      name="email"
                      placeholder="Email"
                      aria-required="true"
                      required={true}
                      ref={input_email_ref}
                      oninput={on_email_change}
                    />
                </div>
                if !email_valid {
                    <div class="error-txt text-red-500 text-sm my-2">{"Enter a valid email address"}</div>
                }
                <button class="next bg-purple-800 hover:bg-purple-700 text-white rounded px-4 mt-10 py-2 w-full font-semibold mb-4" onclick={on_next}>{ "Next" }</button>
            </div>
        },
        1 => html! {
            <div class="page transition-transform duration-300">
                <div class="title text-left text-xl font-semibold mb-4">{"Contact Details"}</div>
                <div class="field mb-6">
                    <div class="label font-semibold text-pink-800">{"Phone Number"}</div>
                    <input
                      id="phone-number"
                      class="w-full border border-pink-800 rounded px-4 py-2"
                      type="tel"
                      name="phone-number"
                      placeholder="Phone Number"
                      aria-required="true"
                      required={true}
                      ref={input_phone_number_ref}
                      oninput={on_phone_number_change}
                    />
                </div>
                if !phone_number_valid {
                   <div class="error-txt text-red-500 text-sm my-2">{"Phone number can't be blank!"}</div>
                }
                <div class="field mb-6">
                    <div class="label font-semibold text-pink-800">{"Address"}</div>
                    <input
                      id="address"
                      class="w-full border border-pink-800 rounded px-4 py-2"
                      type="text"
                      name="address"
                      placeholder="Address"
                      aria-required="true"
                      required={true}
                      ref={input_address_ref}
                      oninput={on_address_change}
                    />
                </div>
                if !address_valid {
                   <div class="error-txt text-red-500 text-sm my-2">{"Address can't be blank!"}</div>
                }
                <div class="field btns text-center space-x-5 mt-10">
                    <button class="prev bg-pink-800 hover:bg-pink-700 text-white rounded px-4 py-2 font-semibold" onclick={on_previous}>{ "Previous" }</button>
                    <button class="next bg-purple-800 hover:bg-purple-700 text-white rounded px-4 py-2 font-semibold" onclick={on_next}>{ "Next" }</button>
                </div>
            </div>
        },
        2 => html! {
            <div class="page transition-transform duration-300">
                <div class="title text-left text-xl font-semibold mb-4">{"Date of Birth"}</div>
                <div class="field mb-6">
                    <div class="label font-semibold text-pink-800">{"Date of Birth"}</div>
                    <input
                      id="birthday"
                      class="w-full border border-pink-800 rounded px-4 py-2"
                      type="date"
                      name="birthday"
                      placeholder="Birthday"
                      aria-required="true"
                      required={true}
                      ref={input_birthday_ref}
                      oninput={on_birthday_change}
                    />
                </div>
                if !birthday_valid {
                   <div class="error-txt text-red-500 text-sm my-2">{"Birthday can't be blank!"}</div>
                }
                <div class="field mb-6">
                    <div class="label font-semibold text-pink-800">{"Gender"}</div>
                    <select
                      class="w-full border border-pink-800 rounded px-4 py-2"
                      id="gender"
                      name="gender"
                      ref={input_gender_ref}
                      aria-required="true"
                      required={true}
                      placeholder="Gender"
                      oninput={on_gender_change}
                    >
                        <option>{"Male"}</option>
                        <option>{"Female"}</option>
                        <option>{"Non-binary"}</option>
                        <option>{"Other"}</option>
                    </select>
                </div>
                if !gender_valid {
                   <div class="error-txt text-red-500 text-sm my-2">{"Gender can't be blank!"}</div>
                }
                <div class="field btns text-center space-x-5 mt-10">
                    <button class="prev bg-pink-800 hover:bg-pink-700 text-white rounded px-4 py-2 font-semibold" onclick={on_previous}>{ "Previous" }</button>
                    <button class="next bg-purple-800 hover:bg-purple-700 text-white rounded px-4 py-2 font-semibold" onclick={on_next}>{ "Next" }</button>
                </div>
            </div>
        },
        3 => html! {
              <div class="page transition-transform duration-300">
                  <div class="title text-left text-xl font-semibold mb-4">{"Account Details"}</div>
                  <div class="field mb-4">
                      <div class="label font-semibold text-pink-800">{"Username"}</div>
                      <input
                        id="username"
                        class="border border-pink-800 rounded px-3 py-2 w-full"
                        type="text"
                        name="username"
                        placeholder="Username"
                        aria-required="true"
                        required={true}
                        ref={input_username_ref}
                        oninput={on_username_change}
                      />
                  </div>
                  if !username_valid {
                     <div class="error-txt text-red-500 text-sm my-2">{"Username can't be blank!"}</div>
                  }
                  <div class="field mb-4">
                      <div class="label font-semibold text-pink-800">{"Password"}</div>
                      <input
                        id="password"
                        class="border border-pink-800 rounded px-3 py-2 w-full"
                        type="password"
                        name="password"
                        placeholder="Password"
                        aria-required="true"
                        required={true}
                        ref={input_password_ref}
                        oninput={on_password_change}
                      />
                  </div>
                  if !password_valid {
                     <div class="error-txt text-red-500 text-sm my-2">{"Password can't be blank!"}</div>
                  }
                  <div class="field btns text-center space-x-5 mt-10">
                      <button class="prev bg-pink-800 hover:bg-pink-700 text-white rounded px-4 py-2 font-semibold" onclick={on_previous}>{ "Previous" }</button>
                      <button class="submit bg-purple-800 hover:bg-purple-700 text-white rounded px-4 py-2 font-semibold">{ "Submit" }</button>
                  </div>
              </div>
        },
        _ => html! {},
    };

    html! {
        <div
          class="min-h-screen bg-gradient-to-tr from-indigo-500 to-pink-500 flex items-center justify-center"
        >
        <div class="container mx-auto p-10 bg-white text-center rounded w-2/3 md:w-1/3 lg:w-1/3">
            if !error.is_empty() {
              <div class="error bg-red-600 text-white px-4 py-3 mb-5 font-semibold rounded-md text-center text-base">{error}</div>
            }
            <header class="text-4xl font-semibold mb-3">{"Multi-Step Form"}</header>
            <div class="flex items-center text-left justify-center my-3">
              { render_progress_item((current_step, ""))}
            </div>
            <div class="form-outer slide-page  text-left">
                    { current_step_content }
            </div>
        </div>
        </div>
    }
}