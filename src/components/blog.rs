use yew::prelude::*;
use crate::mdblogs::BLOGS;
// use crate::highlight;

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct Props {
}

pub struct Blog {
}

impl Component for Blog {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Blog {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let parsed = Html::from_html_unchecked(AttrValue::from(BLOGS[0].post));
        html! {
            <div class="blog">
                <h1 class="center main">{BLOGS[0].title}</h1>
                <h3 class="center"> {format!("Date created: {}", BLOGS[0].date_created)} </h3>
                { parsed }
            </div>
        }
    }

    // fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    // }
    // fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        // highlight();
    // }
}