use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::error::Error;
use crate::pages::login_page_one::TailwindCssLoginPageOne;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/error")]
    Error,
    #[at("/")]
    TailwindCssLoginPageOne
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::TailwindCssLoginPageOne => html! { <TailwindCssLoginPageOne /> },
        Route::Error => html! { <Error /> },
    }
}
