use glfw::{Context, Action, MouseButton};
use std::collections::HashSet;

type KeyBoard = glfw::Key;

pub struct Gluie{
    pub window: glfw::PWindow,
    pub events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    pub glfw_init: glfw::Glfw,
    event_buffer: Vec<glfw::WindowEvent>,
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

        Gluie {window: window, events: events, event_buffer: Vec::new(), glfw_init: glfw_init}
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
                    return;
                }
                _ => {}
            }
        }
        *x = 0.0;
        *y = 0.0;
    }

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
}
