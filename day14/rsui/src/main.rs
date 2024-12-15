use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use wasm_bindgen::{closure::Closure, JsCast};

use web_sys::{CanvasRenderingContext2d as C2D, HtmlCanvasElement, HtmlInputElement};

use instant::Instant;

use yew::prelude::*;

use day14::{robots, solve_1, solve_2, Robot, HEIGHT, INPUT, WIDTH};

const ZOOM: f64 = 2.0;

const STEPS: i32 = 1;

#[derive(Properties, PartialEq)]
pub struct ModelProps {
    pub input: String,
}

pub enum Msg {
    Run(String),
    Rewind,
    Forward,
    Reset,
    Play,
    Stop,
    Tick,
}

pub struct Model {
    input_ref: NodeRef,
    canvas_ref: NodeRef,
    part1: Option<usize>,
    part2: Option<usize>,
    input: String,
    elapsed_part_1: Option<Duration>,
    elapsed_part_2: Option<Duration>,
    elapsed_total: Option<Duration>,
    current: Option<i32>,
    play: bool,
}

static PLAY: AtomicBool = AtomicBool::new(false);

impl Model {
    fn window() -> web_sys::Window {
        web_sys::window().expect("no global `window` exists")
    }

    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        Self::window()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
    }

    fn start_animation(callback: Callback<()>) {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(Closure::new(move || {
            if !PLAY.load(Ordering::Relaxed) {
                Self::stop_animation();

                let _ = f.borrow_mut().take();

                return;
            }

            callback.emit(());

            Self::request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        PLAY.store(true, Ordering::Relaxed);

        Self::request_animation_frame(g.borrow().as_ref().unwrap());
    }

    fn stop_animation() {
        PLAY.store(false, Ordering::Relaxed);
    }

    fn start_tick(&mut self, callback: Callback<()>) {
        if !self.play {
            self.play = true;
            Self::start_animation(callback);
        }
    }

    fn stop_tick(&mut self) {
        if self.play {
            self.play = false;
            Self::stop_animation();
        }
    }

    fn render_solution(&self, i: Option<i32>) {
        let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();

        let g: C2D = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        g.clear_rect(0.0, 0.0, WIDTH as f64 * ZOOM, HEIGHT as f64 * ZOOM);
        if let Some(i) = i {
            for Robot {
                position: (px, py),
                velocity: (vx, vy),
            } in robots(&self.input)
            {
                let (px, py) = (
                    (px + vx * i).rem_euclid(WIDTH) as f64,
                    (py + vy * i).rem_euclid(HEIGHT) as f64,
                );

                g.fill_rect(px * ZOOM, py * ZOOM, ZOOM, ZOOM);
            }
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ModelProps;

    fn create(ctx: &Context<Self>) -> Self {
        let input = ctx.props().input.clone();

        Self {
            input_ref: NodeRef::default(),
            canvas_ref: NodeRef::default(),
            part1: None,
            part2: None,
            input,
            elapsed_part_1: None,
            elapsed_part_2: None,
            elapsed_total: None,
            current: None,
            play: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Run(input) => {
                self.stop_tick();

                let now_part_1 = Instant::now();
                self.part1 = Some(solve_1::<WIDTH, HEIGHT>(&input));
                self.elapsed_part_1 = Some(now_part_1.elapsed());

                let now_part_2 = Instant::now();
                self.part2 = Some(solve_2(&input));
                self.elapsed_part_2 = Some(now_part_2.elapsed());

                self.elapsed_total = Some(now_part_1.elapsed());

                self.input = input;

                self.current = self.part2.map(|v| v as i32);

                self.render_solution(self.current);

                true
            }
            Msg::Rewind => {
                self.stop_tick();

                if let Some(i) = self.current.as_mut() {
                    *i = (*i - STEPS).max(0);
                }

                self.render_solution(self.current);

                true
            }
            Msg::Forward => {
                self.stop_tick();

                if let Some(i) = self.current.as_mut() {
                    *i = (*i + STEPS).min(20000);
                }

                self.render_solution(self.current);

                true
            }
            Msg::Reset => {
                self.stop_tick();

                if let Some(i) = self.current.as_mut() {
                    *i = 0;
                }

                self.render_solution(self.current);

                true
            }
            Msg::Play => {
                self.start_tick(ctx.link().callback(|_| Msg::Tick));

                false
            }
            Msg::Stop => {
                self.stop_tick();

                false
            }
            Msg::Tick => {
                if let Some(i) = self.current.as_mut() {
                    *i = (*i + STEPS).min(20000);
                }

                self.render_solution(self.current);

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let input_ref = self.input_ref.clone();

        let onclick = link.batch_callback(move |_| {
            let input = input_ref.cast::<HtmlInputElement>();
            input.map(|input| Msg::Run(input.value()))
        });

        let rewind = link.callback(|_| Msg::Rewind);
        let forward = link.callback(|_| Msg::Forward);
        let reset = link.callback(|_| Msg::Reset);
        let play = link.callback(|_| Msg::Play);
        let stop = link.callback(|_| Msg::Stop);

        fn format_duration(elapsed: Option<Duration>) -> String {
            elapsed
                .map(|v| format!("{}ms ({}us)", v.as_millis(), v.as_micros()))
                .unwrap_or_else(|| "not run".to_string())
        }

        html! {
            <>
                <label for="input"> { "Input: " }
            <textarea id="input" ref={self.input_ref.clone()} rows="4" cols="50" value={self.input.clone()} />
                </label>
                <button onclick={onclick}>{ "\u{23F5}" }</button>
                <label for="results"> { "Results: " }
            <div id="results" class="output">
                <div class="result"><label> { "Part 1: " } </label> { self.part1 }</div>
                <div class="result"><label> { "Part 2: " } </label> { self.part2 }</div>
            </div>
            <div id="elapsed" class="output">
                <div class="result"><label> { "Part 1 Elapsed: " } </label> { format_duration(self.elapsed_part_1) }</div>
                <div class="result"><label> { "Part 2 Elapsed: " } </label> { format_duration(self.elapsed_part_2) }</div>
                <div class="result"><label> { "Elapsed: " } </label> { format_duration(self.elapsed_total) }</div>
            </div>
                </label>
                <div class="output">
                <label for="canvas"> { format!("View: {:>8}", self.current.unwrap_or_default()) }
            <canvas class="output" ref={self.canvas_ref.clone()} width={ ((WIDTH as f64 * ZOOM) as u32).to_string() } height={ ((HEIGHT as f64 * ZOOM) as u32).to_string() }>
                </canvas>
                </label>
                <div>
                <button onclick={rewind}>{ "<" }</button><button onclick={play}>{ "\u{25b6}" }</button><button onclick={stop}>{ "\u{25fb}" }</button><button onclick={forward}>{ ">" }</button><button onclick={reset}>{ "R" }</button>
                </div>

                </div>
                </>
        }
    }
}

fn main() {
    let model_props = ModelProps {
        input: INPUT.to_string(),
    };
    yew::Renderer::<Model>::with_props(model_props).render();
}
