use gluie::Gluie;

fn main(){
    let mut graphic = Gluie::new((800, 600), "first window");
    
    let mut color = 0;
    let mut inc = true;

    graphic.set_window_size(10, 10);
    while !graphic.should_close(){
        graphic.flush_event();

        graphic.set_screen_color(0, color, 0);
        
        if graphic.click_exit(){
            graphic.set_should_close();
        }

        if inc{
            color += 1;
        }else{
            color -= 1;
        }

        if color == 255 {
            inc = false;
        } else if color == 0{
            inc = true;
        }

        
        graphic.show();
        graphic.set_window_size(1+color as u32, 1+color as u32);
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}
