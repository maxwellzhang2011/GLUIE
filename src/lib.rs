use glfw::{Context, Action, MouseButton};
use std::collections::HashSet;

mod shader;
use shader as sdr;

mod objects;
use objects as obj;

pub type Rec = obj::Rec;

use std::fs;

type KeyBoard = glfw::Key;

pub struct Gluie{
    pub window: glfw::PWindow,
    pub events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    pub glfw_init: glfw::Glfw,
    event_buffer: Vec<glfw::WindowEvent>,

    //shaders
    pub shape_shader: Option<u32>,

    //shapes_recs
    pub rec_uniform_window_size: i32,
    pub rec_uniform_color: i32
}

impl Gluie{
    ///init for everything like window create and init variable and load openGL
    pub fn new(size: (u32, u32), name: &str) -> Gluie{
        let mut glfw_init = glfw::init(glfw::fail_on_errors).unwrap();
        let (mut window, events) = glfw_init.create_window(size.0, size.1, name, glfw::WindowMode::Windowed).unwrap();
        window.make_current();
        glfw_init.set_swap_interval(glfw::SwapInterval::None);
        window.set_key_polling(true);
        gl::load_with(|symbol| {
            window.get_proc_address(symbol).map_or(std::ptr::null(), |p| p as *const _)
        });

        Gluie {window: window, events: events, event_buffer: Vec::new(), glfw_init: glfw_init, shape_shader: None, rec_uniform_window_size: 0, rec_uniform_color: 0}
    }

    ///undate screen
    pub fn show(&mut self){
        self.window.swap_buffers();
    }

    ///close screen set
    pub fn should_close(&self) -> bool{
        self.window.should_close()
    }
    
    ///set close screen
    pub fn set_should_close(&mut self){
        self.window.set_should_close(true);
    }

    ///if the exit button is clicked
    pub fn click_exit(&self) -> bool{
        for event in &self.event_buffer{
           match event{
                glfw::WindowEvent::Close => return true,
                _ => {},
            }
        }
        false
    }

    ///check if window size changed
    fn resized(&self) -> bool{
        for event in &self.event_buffer{
            match event{
                glfw::WindowEvent::Size(_, _) => return false,
                _ => {}
            }
        }
        true
    }

    ///flush events so we can get window input
    pub fn flush_event(&mut self){
        self.event_buffer.clear();
        self.glfw_init.poll_events();
        for (_, event) in glfw::flush_messages(&self.events){
            self.event_buffer.push(event);
        }
    }

    ///set a color for the screen
    pub fn set_screen_color(&self, r: u8, g: u8, b: u8){
        unsafe{
            gl::ClearColor((r as f32) / 255.0, (g as f32) / 255.0, (b as f32) / 255.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        };
    }
    
    ///return window's size
    pub fn window_size(&self) -> (u32, u32){
        let (x, y) = self.window.get_size();
        (x as u32, y as u32)
    }
    
    ///set window's size into an other size
    pub fn set_window_size(&mut self, w: u32, h: u32){
        self.window.set_size(w as i32, h as i32);
    }
    
    ///get mouse's pos
    pub fn mouse_cord(&self) -> (u32, u32){
        let (x, y) = self.window.get_cursor_pos();
        (x as u32, y as u32)
    }

    ///check if a button is pressed support up to 1-8 buttons
    pub fn mouse_button(&self, button: u8) -> bool{
        let button = match button{
            1 => MouseButton::Button1,
            2 => MouseButton::Button2,
            3 => MouseButton::Button3,
            4 => MouseButton::Button4,
            5 => MouseButton::Button5,
            6 => MouseButton::Button6,
            7 => MouseButton::Button7,
            8 => MouseButton::Button8,
            _ => return false
        };
        self.window.get_mouse_button(button) == Action::Press
    }

    ///check scroll direction
    pub fn mouse_scroll(&self, x: &mut f32, y: &mut f32){
        for event in &self.event_buffer{
            match event{
                glfw::WindowEvent::Scroll(xoff, yoff) =>{
                    *x = *xoff as f32;
                    *y = *yoff as f32;
                    return
                }
                _ => {}
            }
        }
        *x = 0.0;
        *y = 0.0;
    }

    ///get keyboard input
    pub fn key(&self, keys: &mut Vec<KeyBoard>){
        let mut keys_prestore: HashSet<KeyBoard> = keys.iter().copied().collect();
        keys.clear();
        for event in &self.event_buffer{
            if let glfw::WindowEvent::Key(key, _, glfw::Action::Press, _) = *event{
                keys_prestore.insert(key as KeyBoard);
            }
            
            if let glfw::WindowEvent::Key(key, _, glfw::Action::Release, _) = *event{
                keys_prestore.remove(&(key as KeyBoard));
            }
        }
        keys_prestore.iter().for_each(|key|{
            keys.push(*key);
        });
    }

    ///update viewport (used for resizing)
    pub fn update_view(&mut self){
        if self.resized(){
            let (x, y) = self.window_size();
            unsafe{ gl::Viewport(0, 0, x as i32, y as i32) };

            unsafe{ gl::Uniform2f(
                self.rec_uniform_window_size,
                x as f32,
                y as f32
            )};
        }
    }

    //shader and actual gpu things :D
    
    /// loads the shader for shapes return a None of success String if error
    pub fn link_shape(&mut self, vertex: &str, fragment: &str) -> Option<String>{
        //vertex shader loading
        let vertex_source = match fs::read_to_string(vertex){
            Ok(data) => data,
            Err(e) => return Some(e.to_string()),
        };

        let vertex_shader = match sdr::compile_shader(&vertex_source, sdr::ShaderType::Vertex){
            Ok(shader) => shader,
            Err(e) => return Some(e.to_string()),
        };

        //fragment shader loading
        let fragment_source = match fs::read_to_string(fragment){
            Ok(data) => data,
            Err(e) => return Some(e.to_string()),
        };

        let fragment_shader = match sdr::compile_shader(&fragment_source, sdr::ShaderType::Fragment){
            Ok(shader) => shader,
            Err(e) => return Some(e.to_string()),
        };

        self.shape_shader = Some(match sdr::link_shader(vertex_shader, fragment_shader){
            Ok(program) => program,
            Err(e) => return Some(e.to_string())
        });
        
        unsafe{
            gl::DeleteShader(fragment_shader);
            gl::DeleteShader(vertex_shader);
        }

        self.rec_uniform_window_size = unsafe {
            gl::GetUniformLocation(
                self.shape_shader.unwrap_unchecked(),
                c"winsize".as_ptr(),
            )
        };

        self.rec_uniform_color = unsafe {
            gl::GetUniformLocation(
                self.shape_shader.unwrap_unchecked(),
                c"color".as_ptr(),
            )
        };

        None
    }
    
    //rec inner control
    ///create a rectangle object
    pub fn build_rec(&self, w: u32, h: u32, x: u32, y: u32, r: u8, g: u8, b: u8) -> Option<Rec>{
        let mut rec = obj::Rec::new(w, h, x, y, r, g, b);
        rec.create_struct();
        Some(rec)
    }

    ///change how a rec is
    pub fn change_rec(&self, w: u32, h: u32, x: u32, y: u32, r: u8, g: u8, b: u8, rec: &mut Rec){
        rec.change(w, h, x, y, r, g, b);
    }

    //drawing shapes
    ///drawing rec shape
    pub fn draw_rec(&self, object: &Rec){
        if 
            object.vao == 0
            &&
            object.vbo == 0
            &&
            match self.shape_shader{
                Some(_) => true,
                None => false,
            }{
            return;
        }

        unsafe{
            gl::UseProgram(self.shape_shader.unwrap());

            gl::Uniform3ui(
                self.rec_uniform_color,
                object.mr as u32,
                object.mg as u32,
                object.mb as u32
            );

            gl::BindVertexArray(object.vao);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }
    }
    
    //io with rec
    ///mouse hover rec
    pub fn mouse_hover_rec(&self, rec: &Rec) -> bool{
        let (x, y) = self.mouse_cord();
        if rec.mx <= x && rec.mx+rec.mw >=x &&
           rec.my <= y && rec.my+rec.mh >= y{
            return true;
        }
        false
    }

    ///mouse click on rec with any button
    pub fn mouse_click_rec(&self, rec: &Rec, button: u8) -> bool{
        if self.mouse_hover_rec(rec) &&
           self.mouse_button(button){
            return true;
        }
        false
    }
}

///generate the basic vertex shader GLUIE need
///```
///use std::fs;
///
///fn main(){
///    fs::write("vertex.glsl", gluie::basic_vertex()).unwrap();
///}
///```
pub fn basic_vertex() -> String{
    "
#version 330 core

layout(location = 0) in uvec2 cord;
uniform uvec3 color;

uniform vec2 winsize;

out vec3 aColor;

void main(){
    vec2 pos = cord / winsize;
    pos.y = 1 - pos.y;

    gl_Position = vec4(pos * vec2(2, 2) - vec2(1, 1), 0, 1);
    aColor = color / vec3(255, 255, 255);
}
    ".to_string()

}

///Generates the basic fragment shader GLUIE need
///```
///use std::fs;
///
///fn main(){
///    fs::write("fragment.glsl", gluie::basic_fragment()).unwrap();
///}
///```
pub fn basic_fragment() -> String{
    "
#version 330 core

in vec3 aColor;
out vec4 color;
void main(){
    color = vec4(aColor, 1);
}
    ".to_string()
}
