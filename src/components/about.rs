use yew::prelude::*;

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct Props {
}

pub struct About {
}

impl Component for About {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="about">
                <p>{"About page still in progress."}</p>
            </div>
        }
    }

    // fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    // }
}
