use yew::prelude::*;
use crate::components::donutcanvas::Donutcanvas;

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct Props {
}

pub struct Home {
}

impl Component for Home {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="home">
                <h1 class="center main" >{ "In sterquiliniis invenitur" }</h1>
                <h2 class="center"> { "This is a rusty donut." } </h2>
                <Donutcanvas/>
                <p class="center"> {"🚧🚧🚧 The website is still under construction. In the meantime, please check the blogs section! 🚧🚧🚧" } </p>
            </div>
        }
    }

    // fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    // }
}
