use web_sys::WebGlRenderingContext;

pub trait Draw {
    fn init_gl(&mut self, context: &WebGlRenderingContext, win_width: f32, win_height: f32);
    fn draw(&mut self, context: &WebGlRenderingContext, win_width: f32, win_height: f32);
}

pub trait Step {
    fn step(&mut self, dt: f32);
}
