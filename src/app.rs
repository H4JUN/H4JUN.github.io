use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::{
    navbar::Navbar,
    home::Home,
    blog::Blog,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/blog")]
    Blog,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes : Route) -> Html {
    match routes {
        Route::Home => html! {
            <>
            <Navbar name = {"Home"} />
            <Home/>
            </>
        },
        Route::Blog => html! {
            <>
            <Navbar name = {"Blog"} />
            <Blog/> 
            </>
        },
        Route::About => html! {
            <>
            <Navbar name = {"About"} />
            <h1> {"About"} </h1> 
            </>
        },
        Route::NotFound => html! { <h1> { "404:Error" } </h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
        </>
    }
}
