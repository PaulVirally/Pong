use std::rc::Rc;
use std::cell::RefCell;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlShader, WebGlRenderingContext};
use js_sys::Date;

use crate::game_object::{ball, paddle, traits::{Draw, Step}};
use ball::Ball;
use paddle::Paddle;

extern crate rand;
use rand::prelude::*;

pub struct GameManager {
    context: WebGlRenderingContext,
    win_width: f32,
    win_height: f32,

    p1: Rc<RefCell<paddle::Paddle>>,
    p2: Rc<RefCell<paddle::Paddle>>,
    ball: Rc<RefCell<ball::Ball>>,
    p1_score: u32,
    p2_score: u32
}

impl GameManager {
    pub fn new(context: WebGlRenderingContext, vert_shader_src: &str, frag_shader_src: &str, win_width: f32, win_height: f32) -> Result<GameManager, JsValue> {
        let vert_shader = Self::compile_shader(&context, WebGlRenderingContext::VERTEX_SHADER, vert_shader_src)?;
        let frag_shader = Self::compile_shader(&context, WebGlRenderingContext::FRAGMENT_SHADER, frag_shader_src)?;
        let program = Self::link_program(&context, &vert_shader, &frag_shader)?;
        context.use_program(Some(&program));
        context.delete_shader(Some(&vert_shader));
        context.delete_shader(Some(&frag_shader));

        context.bind_attrib_location(&program, 0, "attr_position");

        let paddle_width = win_width.min(win_height)/50.0;
        let paddle_height = win_width.min(win_height)/5.0;

        let p1 = Rc::new(RefCell::new(Paddle::new(paddle_width/2.0, win_height/2.0, paddle_width, paddle_height)));
        p1.borrow_mut().init_gl(&context, win_width, win_height);

        let p2 = Rc::new(RefCell::new(Paddle::new(win_width - paddle_width/2.0, win_height/2.0, paddle_width, paddle_height)));
        p2.borrow_mut().init_gl(&context, win_width, win_height);

        let ball = Rc::new(RefCell::new(Ball::new(win_width/2.0, win_height/2.0, paddle_width/2.0)));
        ball.borrow_mut().init_gl(&context, win_width, win_height);

        let p1_score: u32 = 0;
        let p2_score: u32 = 0;

        Ok(GameManager {
            context: context,
            win_width: win_width,
            win_height: win_height,

            p1: p1,
            p2: p2,
            ball: ball,
            p1_score: p1_score,
            p2_score: p2_score
        })
    }

    pub fn init_event_handlers(&mut self, document: &web_sys::Document) -> Result<(), JsValue> {
        let p1_clone1 = self.p1.clone();
        let key_down_closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            if event.key() == "ArrowUp" {
                p1_clone1.borrow_mut().set_dir(1.0);
            }
            else if event.key() == "ArrowDown" {
                p1_clone1.borrow_mut().set_dir(-1.0);
            }
        }) as Box<dyn FnMut(_)>);
        document.add_event_listener_with_callback("keydown", key_down_closure.as_ref().unchecked_ref())?;
        key_down_closure.forget();

        let p1_clone2 = self.p1.clone();
        let key_up_closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            if event.key() == "ArrowUp" {
                p1_clone2.borrow_mut().set_dir(0.0);
            }
            else if event.key() == "ArrowDown" {
                p1_clone2.borrow_mut().set_dir(0.0);
            }
        }) as Box<dyn FnMut(_)>);
        document.add_event_listener_with_callback("keyup", key_up_closure.as_ref().unchecked_ref())?;
        key_up_closure.forget();

        Ok(())
    }

    pub fn start_game(&mut self) -> Result<(), JsValue> {
        let p1_clone = self.p1.clone();
        let p2_clone = self.p2.clone();
        let ball_clone = self.ball.clone();
        let context_clone = self.context.clone();
        let p1_html = web_sys::window().expect("Could not get window").document().unwrap().get_element_by_id("p1-score").unwrap();
        let mut p1_score = self.p1_score.clone();
        let p2_html = web_sys::window().expect("Could not get window").document().unwrap().get_element_by_id("p2-score").unwrap();
        let mut p2_score = self.p2_score.clone();
        let win_width = self.win_width.clone();
        let win_height = self.win_height.clone();

        let mut rng = rand::thread_rng();
        let mut p2_target = rng.gen_range(-p2_clone.borrow().get_height()/2.0, p2_clone.borrow().get_height()/2.0);

        let mut prev_time = Date::now();

        let window1 = web_sys::window().expect("Could not get window");
        let window2 = web_sys::window().expect("Could not get window");
        let window3 = web_sys::window().expect("Could not get window");

        let animate_callback = Rc::new(RefCell::new(Closure::wrap(Box::new(move || {}) as Box<dyn FnMut()>))); // We will initialize the callback in just a sec
        let animate_callback_clone = animate_callback.clone();
        let animate_callback_clone2 = animate_callback.clone();

        let timeout_callback = Rc::new(RefCell::new(Closure::wrap(Box::new(move || {
            window1.request_animation_frame(animate_callback_clone2.borrow_mut().as_ref().unchecked_ref()).unwrap();
        }) as Box<dyn FnMut()>)));
        let timeout_callback_clone = timeout_callback.clone();

        *animate_callback_clone.borrow_mut() = Closure::wrap(Box::new(move || {
            // dt
            let curr_time = Date::now();
            let dt = (curr_time - prev_time) as f32;
            prev_time = curr_time;

            // Step
            p1_clone.borrow_mut().step(dt);
            p2_clone.borrow_mut().step(dt);
            ball_clone.borrow_mut().step(dt);

            // Physics

            // Out of bounds
            if ball_clone.borrow().get_dir() < 0.0 && ball_clone.borrow().get_x() <= 0.0 - p1_clone.borrow().get_width()/2.0 {
                ball_clone.borrow_mut().reset(win_width/2.0, win_height/2.0);
                p2_score += 1;
                p1_html.set_inner_html(&p1_score.to_string()[..]);
                p2_html.set_inner_html(&p2_score.to_string()[..]);
            }
            if ball_clone.borrow().get_dir() > 0.0 && ball_clone.borrow().get_x() >= win_width + p2_clone.borrow().get_width()/2.0 {
                ball_clone.borrow_mut().reset(win_width/2.0, win_height/2.0);
                p1_score += 1;
                p1_html.set_inner_html(&p1_score.to_string()[..]);
                p2_html.set_inner_html(&p2_score.to_string()[..]);
            }

            // Walls
            if ball_clone.borrow().get_y_dir() < 0.0 && ball_clone.borrow().get_y() <= p1_clone.borrow().get_width()/2.0 {
                ball_clone.borrow_mut().bounce_y();
            }
            if ball_clone.borrow().get_y_dir() > 0.0 && ball_clone.borrow().get_y() >= win_height - p1_clone.borrow().get_width()/2.0 {
                ball_clone.borrow_mut().bounce_y();
            }

            // Player 1 paddle collision
            if ball_clone.borrow().get_dir() < 0.0 && ball_clone.borrow().get_x() <= p1_clone.borrow().get_width()*3.0/2.0 && Self::within(ball_clone.borrow().get_y(), p1_clone.borrow().get_y(), p1_clone.borrow().get_height()/2.0) {
                let dy = (ball_clone.borrow().get_y() - p1_clone.borrow().get_y())/p1_clone.borrow().get_height()/2.0;
                ball_clone.borrow_mut().bounce(dy);
            }
            // Player 2 paddle collision
            else if ball_clone.borrow().get_dir() > 0.0 && ball_clone.borrow().get_x() >= win_width - p2_clone.borrow().get_width()*3.0/2.0 && Self::within(ball_clone.borrow().get_y(), p2_clone.borrow().get_y(), p2_clone.borrow().get_height()/2.0) {
                let dy = (ball_clone.borrow().get_y() - p2_clone.borrow().get_y())/p2_clone.borrow().get_height()/2.0;
                ball_clone.borrow_mut().bounce(dy);
                p2_target = rng.gen_range(-p2_clone.borrow().get_height()/2.0, p2_clone.borrow().get_height()/2.0);
            }

            // Player 2 AI
            if !Self::within(p2_clone.borrow().get_y() + p2_target, ball_clone.borrow().get_y(), p2_clone.borrow().get_height()/50.0) {
                if p2_clone.borrow().get_y() + p2_target < ball_clone.borrow().get_y() {
                    p2_clone.borrow_mut().set_dir(1.0);
                }
                else {
                    p2_clone.borrow_mut().set_dir(-1.0);
                }
            }
            else {
                p2_clone.borrow_mut().set_dir(0.0);
            }

            // Clear
            context_clone.clear_color(0.0, 0.0, 0.0, 1.0);
            context_clone.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

            // Draw
            ball_clone.borrow_mut().draw(&context_clone, win_width, win_height);
            p1_clone.borrow_mut().draw(&context_clone, win_width, win_height);
            p2_clone.borrow_mut().draw(&context_clone, win_width, win_height);

            window2.set_timeout_with_callback_and_timeout_and_arguments_0(timeout_callback_clone.borrow_mut().as_ref().unchecked_ref(), 1000 / 240).unwrap();
        }) as Box<dyn FnMut()>);
        window3.request_animation_frame(animate_callback_clone.borrow_mut().as_ref().unchecked_ref())?;

        Ok(())
    }

    fn within(x: f32, y: f32, tolerance: f32) -> bool {
        x < y+tolerance && x > y-tolerance
    }

    fn compile_shader(context: &WebGlRenderingContext, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
        let shader = context.create_shader(shader_type).ok_or_else(|| String::from("Unable to create shader object"))?;
        context.shader_source(&shader, &source);
        context.compile_shader(&shader);

        if context.get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS).as_bool().unwrap_or(false) {
            Ok(shader)
        }
        else {
            Err(context.get_shader_info_log(&shader).unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }

    fn link_program(context: &WebGlRenderingContext, vert_shader: &WebGlShader, frag_shader: &WebGlShader) -> Result<WebGlProgram, String> {
        let program = context.create_program().ok_or_else(|| String::from("Unable to create program object"))?;
        context.attach_shader(&program, vert_shader);
        context.attach_shader(&program, frag_shader);
        context.link_program(&program);

        if context.get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS).as_bool().unwrap_or(false) {
            Ok(program)
        }
        else {
            Err(context.get_program_info_log(&program).unwrap_or_else(|| String::from("Unknown error creating program")))
        }
    }
}
