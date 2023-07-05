use yew::prelude::*;
use crate::mdblogs::BLOGS;
use crate::app::Route;
use yew_router::prelude::Link;

pub enum Msg {
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or("TOC".to_string())]
    pub blog_title: String,
}

pub struct Blog {
}

impl Component for Blog {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Blog {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if &ctx.props().blog_title == "TOC" {
            let mut blog_list = vec![];
            for (_, blog) in &BLOGS {
                let blog_html = html! {
                    <Link<Route> to={Route::Blog {title: blog.title.to_string()}}>
                    <div class="item">
                        <div class="title">{blog.title}</div>
                        <div class="date">{blog.date_created}</div>
                    </div>
                    </Link<Route>>
                };
                blog_list.push(blog_html);
            }
            html! {
            <div class="blog">
            { for blog_list }
            </div>
            }
        }
        else {
            let valid_title = str::replace(&ctx.props().blog_title, "%20", " ");
            let blog = &BLOGS[&valid_title];
            let parsed = Html::from_html_unchecked(AttrValue::from(blog.post));
            html! {
                <div class="blog">
                <h1 class="center main">{blog.title}</h1>
                <h3 class="center"> {format!("Date created: {}", blog.date_created)} </h3>
                { parsed }
                </div>
            }
        }
    }

    // fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    // }
    // fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
    // }
}
