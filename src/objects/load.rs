//load any object into the gpu
pub fn load_rec(structure: &Vec<(u32, u32)>) -> (u32, u32){
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

pub fn load_image(structure: &Vec<(u32, u32)>) -> (u32, u32, u32){
    //2 buffers
    let mut str_vbo = 0;
    let mut uv_vbo = 0;
    let mut vao = 0;
    let uv: [u32; 8] = [
        0, 0,
        1, 0,
        0, 1,
        1, 1
    ];

    //create 2 buffer and link them
    unsafe{
        //vao creating
        gl::GenVertexArrays(1, &mut vao);
        
        //vbo creating
        gl::GenBuffers(1, &mut str_vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, str_vbo);

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
        
        //uv creating
        gl::GenBuffers(1, &mut uv_vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, uv_vbo);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (uv.len() * size_of::<u32>()) as isize,
            uv.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        //bind vao
        gl::BindVertexArray(vao);

        gl:: VertexAttribIPointer(
            1,
            2,
            gl::UNSIGNED_INT,
            size_of::<u32>() as i32 * 2,
            std::ptr::null()
        );
        gl::EnableVertexAttribArray(1);
    }

    (vao, str_vbo, uv_vbo)
}

//change any object in the gpu
pub fn change_rec(structure: &Vec<(u32, u32)>, vbo: &mut u32){
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
