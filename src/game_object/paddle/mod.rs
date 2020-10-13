use web_sys::{WebGlBuffer, WebGl2RenderingContext};
use crate::game_object::traits::{Draw, Step};

pub struct Paddle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    dir: f32,

    vertices: [f32; 4*2],
    idxs: [u32; 6],
    vbo: Option<WebGlBuffer>,
    ebo: Option<WebGlBuffer>
}

impl Paddle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Paddle {
        Paddle {
            x: x,
            y: y,
            width: width,
            height: height,
            dir: 0.0,

            vertices: [0.0; 4*2],
            idxs: [0; 6],
            vbo: None,
            ebo: None
        }
    }

    fn update_vertices(&mut self, win_width: f32, win_height: f32) {
        self.vertices[0] = (2.0*(self.x + self.width/2.0)/win_width) - 1.0;
        self.vertices[1] = (2.0*(self.y - self.height/2.0)/win_height) - 1.0;

        self.vertices[2] = (2.0*(self.x - self.width/2.0)/win_width) - 1.0;
        self.vertices[3] = (2.0*(self.y - self.height/2.0)/win_height) - 1.0;

        self.vertices[4] = (2.0*(self.x - self.width/2.0)/win_width) - 1.0;
        self.vertices[5] = (2.0*(self.y + self.height/2.0)/win_height) - 1.0;

        self.vertices[6] = (2.0*(self.x + self.width/2.0)/win_width) - 1.0;
        self.vertices[7] = (2.0*(self.y + self.height/2.0)/win_height) - 1.0;
    }

    pub fn set_dir(&mut self, dir: f32) {
        self.dir = dir;
    }


    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }
}

impl Draw for Paddle {
    fn init_gl(&mut self, context: &WebGl2RenderingContext, win_width: f32, win_height: f32) {
        // Vertices
        self.update_vertices(win_width, win_height);

        // Indices
        self.idxs[0] = 0;
        self.idxs[1] = 1;
        self.idxs[2] = 2;
        self.idxs[3] = 0;
        self.idxs[4] = 2;
        self.idxs[5] = 3;

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

impl Step for Paddle {
    fn step(&mut self, dt: f32) {
        self.y += self.dir * 0.4 * dt;
    }
}
