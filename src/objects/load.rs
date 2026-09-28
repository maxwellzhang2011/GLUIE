pub fn load(structure: &Vec<(usize, usize)>, window_size: (usize, usize), r: u8, g: u8, b: u8) -> (u32, u32){
    //restructure it
    let vert: Vec<f32> = structure.iter().flat_map(|(x, y)| 
        [
            (*x as f32) / (window_size.0 as f32) * 2.0 - 1.0, 
            -((*y as f32) / (window_size.1 as f32) * 2.0 - 1.0),
            (r as f32) / 255f32,
            (g as f32) / 255f32,
            (b as f32) / 255f32,
        ]).collect();
    
    //2 buffers
    let mut vbo = 0;
    let mut vao = 0;

    //create 2 buffer and link them
    unsafe{
        //vao creating
        gl::GenVertexArrays(1, &mut vao);
        
        //vbo creating
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vert.len() * size_of::<f32>()) as isize,
            vert.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );
        
        //bind vao
        gl::BindVertexArray(vao);
        
        //shape
        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            5 * size_of::<f32>() as i32,
            std::ptr::null(),
        );

        gl::EnableVertexAttribArray(0);
        
        //color
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            5 * size_of::<f32>() as i32,
            (2*size_of::<f32>()) as *const _,
        );

        gl::EnableVertexAttribArray(1);
    }

    (vbo, vao)
}
