use web_sys::{WebGlBuffer, WebGl2RenderingContext};
use crate::game_object::traits::{Draw, Step};

const NUM_VERT: usize = 32;

pub struct Ball {
    radius: f32,
    x: f32,
    y: f32,
    velo_x: f32,
    velo_y: f32,

    vertices: [f32; (NUM_VERT + 1) * 2],
    idxs: [u32; NUM_VERT * 3],
    vbo: Option<WebGlBuffer>,
    ebo: Option<WebGlBuffer>
}

impl Ball {
    pub fn new(x: f32, y: f32, radius: f32) -> Ball {
        Ball {
            radius: radius,
            x: x,
            y: y,
            velo_x: -0.6,
            velo_y: 0.0,

            vertices: [0.0; (NUM_VERT + 1) * 2],
            idxs: [0; NUM_VERT * 3],
            vbo: None,
            ebo: None
        }
    }

    fn update_vertices(&mut self, win_width: f32, win_height: f32) {
        use std::f32::consts::PI;

        // Origin of the ball
        self.vertices[0] = 2.0*(self.x / win_width) - 1.0;
        self.vertices[1] = 2.0*(self.y / win_height) - 1.0;

        for i in (2..self.vertices.len()).step_by(2) {
            let theta = (((i-2) as f32)/((self.vertices.len()-2) as f32)) * 2.0 * PI;
            self.vertices[i] = (2.0*(self.radius * theta.cos() + self.x) / win_width) - 1.0;
            self.vertices[i+1] = (2.0*(self.radius * theta.sin() + self.y) / win_height) - 1.0;
        }
    }

    pub fn reset(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
        self.velo_x = -0.6;
        self.velo_y = 0.0;
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn bounce(&mut self, dy: f32) {
        self.velo_x *= -1.0;
        self.velo_y = dy * 4.0;
    }

    pub fn bounce_y(&mut self) {
        self.velo_y *= -1.0;
    }

    pub fn get_dir(&self) -> f32 {
        if self.velo_x <= 0.0 {
            -1.0
        }
        else {
            1.0
        }
    }

    pub fn get_y_dir(&self) -> f32 {
        if self.velo_y <= 0.0 {
            -1.0
        }
        else {
            1.0
        }
    }
    
}

impl Draw for Ball {
    fn init_gl(&mut self, context: &WebGl2RenderingContext, win_width: f32, win_height: f32) {
        // Vertices
        self.update_vertices(win_width, win_height);

        // Indices
        for i in (0..self.idxs.len()-3).step_by(3) {
            self.idxs[i] = 0;
            self.idxs[i+1] = (i/3 + 1) as u32;
            self.idxs[i+2] = (i/3 + 2) as u32;
        }
        self.idxs[self.idxs.len()-3] = 0;
        self.idxs[self.idxs.len()-2] = (self.idxs.len() as u32)/3;
        self.idxs[self.idxs.len()-1] = 1;

        // Create VBO and EBO
        self.vbo = context.create_buffer();
        self.ebo = context.create_buffer();

        // Bind and set VBO
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, self.vbo.as_ref());
        unsafe {
            let vbo_array = js_sys::Float32Array::view(&self.vertices);
            context.buffer_data_with_array_buffer_view(WebGl2RenderingContext::ARRAY_BUFFER, &vbo_array, WebGl2RenderingContext::DYNAMIC_DRAW);
        }

        // Bind and set EBO
        context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, self.ebo.as_ref());
        unsafe {
            let ebo_array = js_sys::Uint32Array::view(&self.idxs);
            context.buffer_data_with_array_buffer_view(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, &ebo_array, WebGl2RenderingContext::STATIC_DRAW);
        }

        // Vertex position
        context.vertex_attrib_pointer_with_i32(0, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);
        context.enable_vertex_attrib_array(0);

        // Unbind VBO
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
    }

    fn draw(&mut self, context: &WebGl2RenderingContext, win_width: f32, win_height: f32) {
        // Vertices
        self.update_vertices(win_width, win_height);

        // Bind and set VBO
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, self.vbo.as_ref());
        unsafe {
            let vbo_array = js_sys::Float32Array::view(&self.vertices);
            context.buffer_sub_data_with_i32_and_array_buffer_view(WebGl2RenderingContext::ARRAY_BUFFER, 0, &vbo_array);
        }

        // Bind EBO
        context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, self.ebo.as_ref());
        
        // Vertex position
        context.vertex_attrib_pointer_with_i32(0, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);
        context.enable_vertex_attrib_array(0);

        // Unbind VBO
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);

        // Draw
        context.draw_elements_with_i32(WebGl2RenderingContext::TRIANGLES, self.idxs.len() as i32, WebGl2RenderingContext::UNSIGNED_INT, 0);
    }


}

impl Step for Ball {
    fn step(&mut self, dt: f32) {
        self.x += self.velo_x * dt;
        self.y += self.velo_y * dt;
    }
}
