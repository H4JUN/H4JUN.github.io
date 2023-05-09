use yew::prelude::*;
use yew_router::prelude::*;
use crate::app::Route;

// Change values accordingly
// Syntax is "Icon\sName"
static NAVITEMS:[&str; 3] = ["🏡 Home", "📝 Blog", "📜 About"];

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: AttrValue,
}

pub enum Msg {
}

pub struct Navbar {
    items: Vec<AttrValue>,
}

impl Component for Navbar {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            items : NAVITEMS.iter().map(|&item| item.into()).collect(),
        }
    }
    #[allow(clippy::redundant_clone)]
    fn view(&self, ctx: &Context<Self>) -> Html {
        let navigator = ctx.link().navigator().unwrap();
    
        let on_click_home = {
            let navigator = navigator.clone();
            Callback::from(move |_| navigator.push(&Route::Home))
        };
        let on_click_blog = {
            let navigator = navigator.clone();
            Callback::from(move |_| navigator.push(&Route::Blog))
        };
        let on_click_about = {
            let navigator = navigator.clone();
            Callback::from(move |_| navigator.push(&Route::About))
        };

        let on_click_arr = [on_click_home, on_click_blog, on_click_about];

        // Converting static navitems to html tags
        let elements = self.items.iter().enumerate().map(|(idx, item)| {
            let name = item.clone().split(' ').collect::<Vec<_>>()[1].to_string().to_lowercase(); // 🏡 Home -> Home
            let mut class = vec!["nav_item".to_string()];
            if name == *ctx.props().name {
                class.push("active".to_string());
            }
            html! {<div class={class}><button onclick={on_click_arr[idx].clone()} > { item } </button></div> }
            // if name == "home" {
            //     html! { <a href='/'> {item} </a> }
            // } else {
            //     html! {<a href={name} > {item} </a> }
            // } 
        });

        html! {
                <div class="navbar">
                    <img id="logo" src="/assets/img/13th.png"/>
                    { for elements }
                </div>
        }
    }
}

// Equivalent funtional component

// #[function_component(Navbar)]
// pub fn navbar(props: &Props) -> Html {
//     let items:Vec<AttrValue> = NAVITEMS.iter().map(|&item| item.into()).collect();
//     let navigator = use_navigator().unwrap();

//     let on_click_home = {
//         let navigator = navigator.clone();
//         Callback::from(move |_| navigator.push(&Route::Home))
//     };
//     let on_click_blog = {
//         let navigator = navigator.clone();
//         Callback::from(move |_| navigator.push(&Route::Blog))
//     };
//     let on_click_about = {
//         let navigator = navigator.clone();
//         Callback::from(move |_| navigator.push(&Route::About))
//     };
    
//     let on_click_arr = [on_click_home, on_click_blog, on_click_about];
//     // Converting static navitems to html tags
//     let elements = items.iter().enumerate().map(|(idx, item)| {
//         let mut class = vec!["nav_item".to_string()];
//         let name = format!("{}", item.clone().split(" ").collect::<Vec<_>>()[1]); // 🏡 Home -> Home
//         if name == *props.name {
//             class.push("active".to_string());
//         }
//         html! {<div id={idx.to_string()} class={class}><button onclick={on_click_arr[idx].clone()}> { item } </button></div> }
//     });
//     html! {
//             <div class="navbar">
//                 <img id="logo" src="img/13th.png"/>
//                 { for elements }
//             </div>
//     }
// }

// // Just for info about how to access clicked element
// fn view(&self, ctx:&Context<self>) -> Html {
// // Callback function on click
//         let on_click = ctx.link().batch_callback(move |e: MouseEvent| {
//             let target = e.target_dyn_into::<HtmlAnchorElement>(); // TargetCast, safer
//             if let Some(target) = target {
//                 let parent_div = target.parent_element();
//                 if let Some(parent_div) = parent_div {
//                     let id = parent_div.id();
//                     let curr_ind = id.parse::<u8>();
//                     match curr_ind {
//                             Ok(curr_ind) if curr_ind == curr_active_ind => None,
//                             Ok(curr_ind) => Some(Msg::SetActiveIndex(curr_ind)),
//                             Err(_) => panic!()
//                     }
//                 } else {
//                     None
//                 }
//             } else {
//                 None
//             }
//         });
//     ...
 // let elements = self.items.iter().enumerate().map(|(idx, item)| {
 //            let href_name = format!("#{}", item.clone().split(" ").collect::<Vec<_>>()[1].to_lowercase()); // 🏡 Home -> home
 //            let mut class = vec!["nav_item".to_string()];
 //            if href_name == ctx.props().name {
 //                class.push("active".to_string());
 //            }
 //            html! {<div id={idx.to_string()} class={class}><a href={ href_name } onclick={on_click.clone()> { item } </a></div> }
 //        });

// }