use glfw::{Context, Action, MouseButton};
use std::collections::{HashSet, HashMap};

mod shader;
use shader as sdr;

mod objects;
use objects as obj;

use std::fs;

type KeyBoard = glfw::Key;

pub struct Gluie{
    pub window: glfw::PWindow,
    pub events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    pub glfw_init: glfw::Glfw,
    event_buffer: Vec<glfw::WindowEvent>,

    //shaders
    pub shape_shader: Option<u32>,

    //shapes
    pub recs: HashMap<String, obj::Rec>,

    pub uniform_window_size: i32
}

impl Gluie{
    ///init for everything like window create and init variable and load openGL
    pub fn new(size: (u32, u32), name: &str) -> Gluie{
        let mut glfw_init = glfw::init(glfw::fail_on_errors).unwrap();
        let (mut window, events) = glfw_init.create_window(size.0, size.1, name, glfw::WindowMode::Windowed).unwrap();
        window.make_current();
        window.set_key_polling(true);
        gl::load_with(|symbol| {
            window.get_proc_address(symbol).map_or(std::ptr::null(), |p| p as *const _)
        });

        Gluie {window: window, events: events, event_buffer: Vec::new(), glfw_init: glfw_init, shape_shader: None, recs: HashMap::new(), uniform_window_size: 0}
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
    pub fn window_size(&self) -> (usize, usize){
        let (x, y) = self.window.get_size();
        (x as usize, y as usize)
    }
    
    ///set window's size into an other size
    pub fn set_window_size(&mut self, w: u32, h: u32){
        self.window.set_size(w as i32, h as i32);
    }
    
    ///get mouse's pos
    pub fn mouse_cord(&self) -> (usize, usize){
        let (x, y) = self.window.get_cursor_pos();
        (x as usize, y as usize)
    }

    ///check if a button is pressed support up to 1-8 buttons
    pub fn mouse_button(&self, button: u8) -> bool{
        let button = match button{
            1 => MouseButton::Button1,
            2 => MouseButton::Button1,
            3 => MouseButton::Button1,
            4 => MouseButton::Button1,
            5 => MouseButton::Button1,
            6 => MouseButton::Button1,
            7 => MouseButton::Button1,
            8 => MouseButton::Button1,
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
        let (x, y) = self.window_size();
        unsafe{ gl::Viewport(0, 0, x as i32, y as i32) };

        unsafe{ gl::Uniform2f(
            self.uniform_window_size,
            x as f32,
            y as f32
        )};
    }

    //shader and actual gpu things :D
    
    ///loading shader for drawing shape
    /// `shader is writen by taking the vertex at location(layout=0) as vec2 and location(layout=1) for color as vec3`
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

        self.uniform_window_size = unsafe {
            gl::GetUniformLocation(
                self.shape_shader.unwrap(),
                c"winsize".as_ptr(),
            )
       };

        None
    }

    //creating shapes that i can draw
    ///create the rec shape
    pub fn build_rec(&mut self, w: usize, h: usize, x: usize, y: usize, r: u8, g: u8, b: u8, name: &str){
        let mut rec = obj::Rec::new(w, h, x, y, r, g, b);
        rec.create_struct();
        self.recs.insert(name.to_string(), rec);
    }

    //drawing shapes
    ///drawing rec shape
    pub fn draw_rec(&self, name: &str){
        if 
            match self.recs.get(name){
                Some(data) => data,
                None => return,
            }.vao == 0
            &&
            match self.recs.get(name){
                Some(data) => data,
                None => return,
            }.vbo == 0
            &&
            match self.shape_shader{
                Some(_) => true,
                None => false,
            }{
            return;
        }

        unsafe{
            gl::UseProgram(self.shape_shader.unwrap());
            gl::BindVertexArray(self.recs.get(name).unwrap().vao);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }
    }
    
}
