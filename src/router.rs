use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::error::Error;
use crate::pages::login_page_one::LoginPageOne;
use crate::pages::login_page_two::LoginPageTwo;
use crate::pages::login_page_three::LoginPageThree;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/error")]
    Error,
    #[at("/1")]
    LoginPageOne,
    #[at("/2")]
    LoginPageTwo,
    #[at("/3")]
    LoginPageThree
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::LoginPageOne => html! { <LoginPageOne /> },
        Route::LoginPageTwo => html! { <LoginPageTwo /> },
        Route::LoginPageThree => html! { <LoginPageThree /> },
        Route::Error => html! { <Error /> },
    }
}
