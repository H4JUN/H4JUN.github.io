use std::f32::consts::PI;

use yew::prelude::*;
use crate::donut::*;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};


const PARTICLE: u32 = 7;

pub enum Msg {
    RenderDonut
}

#[derive(Properties, PartialEq)]
pub struct Props {
}

pub struct Donutcanvas {
    canvas: NodeRef,
    donut: Donut,
    canvas_width : u32,
    canvas_height : u32,
    a : f32,
    b : f32,
    callback : Closure<dyn FnMut()>
}

impl Component for Donutcanvas {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let donut = Donut::new(1.0, 2.5, 0.07, 0.02);
        let ctx_link_copy = ctx.link().clone();
        let callback = Closure::new(Box::new(move || ctx_link_copy.send_message(Msg::RenderDonut)) as Box<dyn FnMut()>);
        Self {
            canvas: NodeRef::default(),
            donut,
            canvas_width : PARTICLE * SCREEN_WIDTH as u32,
            canvas_height : PARTICLE * SCREEN_HEIGHT as u32,
            a : 0.0,
            b : 0.0,
            callback
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <canvas id="donut" width={self.canvas_width.to_string()} height={self.canvas_height.to_string()} ref={self.canvas.clone()}></canvas>
            </>
        }
    }


    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::RenderDonut => {
                self.render_donut();
                false
            }
        }
    }
    
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_message(Msg::RenderDonut)
        }
    }
}

impl Donutcanvas {
    fn render_donut(&mut self) {
        let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
        let ctx : CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().unchecked_into();
        let new_vec = self.donut.draw(self.a, self.b);
        ctx.clear_rect(0.0, 0.0, self.canvas_width as f64, self.canvas_height as f64);
        for y in 0..SCREEN_HEIGHT as u32{
            for x in 0..SCREEN_WIDTH as u32{
                let idx = self.donut.get_index(x, y);
                if new_vec[idx] > 0 {
                    // ctx.set_fill_style(&format!("rgba(133,58,25,{})", 1.0 - ( new_vec[idx] as f64 / 255.0)).into());
                    ctx.set_fill_style(&format!("rgba(255,255,255,{})", new_vec[idx] as f64 / 255.0).into());
                }
                else {
                    ctx.set_fill_style(&"rgba(0,0,0,0)".into());
                }
                ctx.fill_rect(
                    (x * PARTICLE) as f64,
                    (y * PARTICLE) as f64,
                    PARTICLE as f64,
                    PARTICLE as f64
                );
            }
        }
        self.a += 0.03;
        self.b += 0.01;
        // Prevent overflow
        self.a %= 2.0 * PI;
        self.b %= 2.0 * PI;
        window().unwrap()
            .request_animation_frame(self.callback.as_ref().unchecked_ref()).unwrap();
    }
}


