//load any object into the gpu
pub fn load(structure: &Vec<(u32, u32)>) -> (u32, u32){
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
            (structure.len() * 2 * size_of::<u32>()) as isize,
            structure.as_ptr() as *const _,
            gl::DYNAMIC_DRAW,
        );
        
        //bind vao
        gl::BindVertexArray(vao);
        
        //shape
        gl::VertexAttribIPointer(
            0,
            2,
            gl::UNSIGNED_INT,
            size_of::<u32>() as i32 * 2,
            std::ptr::null(),
        );

        gl::EnableVertexAttribArray(0);
    }

    (vbo, vao)
}

//change any object in the gpu
pub fn change(structure: &Vec<(u32, u32)>, vbo: &mut u32){
    unsafe{
        //vbo update
        gl::BindBuffer(gl::ARRAY_BUFFER, *vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (structure.len() * 2 * size_of::<u32>()) as isize,
            structure.as_ptr() as *const _,
            gl::DYNAMIC_DRAW,
        );
    }
}
