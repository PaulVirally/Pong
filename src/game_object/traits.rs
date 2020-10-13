use web_sys::WebGl2RenderingContext;

pub trait Draw {
    fn init_gl(&mut self, context: &WebGl2RenderingContext, win_width: f32, win_height: f32);
    fn draw(&mut self, context: &WebGl2RenderingContext, win_width: f32, win_height: f32);
}

pub trait Step {
    fn step(&mut self, dt: f32);
}
