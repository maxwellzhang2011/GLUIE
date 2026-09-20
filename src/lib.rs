use glfw::{Context};

pub struct Gluie{
    pub window: glfw::PWindow,
    pub events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    pub glfw_init: glfw::Glfw,
    event_buffer: Vec<glfw::WindowEvent>
}

impl Gluie{
    //init for everything like window create and init variable and load openGL
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

    //undate screen
    pub fn show(&mut self){
        self.window.swap_buffers();
    }

    //close screen set
    pub fn should_close(&self) -> bool{
        self.window.should_close()
    }
    
    //set close screen
    pub fn set_should_close(&mut self){
        self.window.set_should_close(true);
    }

    //if the exit button is clicked
    pub fn click_exit(&self) -> bool{
        for event in &self.event_buffer{
            match event{
                glfw::WindowEvent::Close => return true,
                _ => {},
            }
        }
        false
    }

    //flush events so we can get window input
    pub fn flush_event(&mut self){
        self.event_buffer.clear();
        self.glfw_init.poll_events();
        for (_, event) in glfw::flush_messages(&self.events){
            self.event_buffer.push(event);
        }
    }

    //set a color for the screen
    pub fn set_screen_color(&self, r: u8, g: u8, b: u8){
        unsafe{
            gl::ClearColor((r as f32) / 255.0, (g as f32) / 255.0, (b as f32) / 255.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        };
    }

    pub fn window_size(&self) -> (i32, i32){
        self.window.get_size()
    }

    pub fn set_window_size(&mut self, w: u32, h: u32){
        self.window.set_size(w as i32, h as i32);
    }
}
