mod load;
use load as lad;

//rectangle shape
pub struct Rec{
    pub mw: u32, pub mh: u32,
    pub mx: u32, pub my: u32,
    pub mr: u8, pub mg: u8, pub mb: u8,
    structure: Vec<(u32, u32)>,

    pub vao: u32, pub vbo: u32,
}

impl Rec{
    pub fn new(w: u32, h: u32, x: u32, y: u32, r: u8, g: u8, b: u8) -> Rec{
        Rec {mw: w, mh: h, mx: x, my: y, structure: Vec::new(), vao: 0, vbo: 0, mr: r, mg: g, mb: b}
    }

    pub fn create_struct(&mut self){
        self.structure.push((self.mx, self.my));
        self.structure.push((self.mx+self.mw, self.my));
        self.structure.push((self.mx, self.my+self.mh));
        self.structure.push((self.mx+self.mw, self.my+self.mh));

        (self.vao, self.vbo) = lad::load_rec(&self.structure);
    }

    pub fn change(&mut self, w: u32, h: u32, x: u32, y: u32, r: u8, g: u8, b: u8){
        (self.mw, self.mh, self.mx, self.my, self.mr, self.mg, self.mb) = (w, h, x, y, r, g, b);

        self.structure.clear();
        self.structure.push((self.mx, self.my));
        self.structure.push((self.mx+self.mw, self.my));
        self.structure.push((self.mx, self.my+self.mh));
        self.structure.push((self.mx+self.mw, self.my+self.mh));

        lad::change_rec(&self.structure, &mut self.vbo)
    }
}

//image object
pub struct Image{
    pub mw: u32, pub mh: u32,
    pub mx: u32, pub my: u32,
    pub image: u32,

    structure: Vec<(u32, u32)>,
    pub vao: u32, pub str_vbo: u32, pub uv_vbo: u32
}

impl Image{
    pub fn new(w: u32, h: u32, x: u32, y: u32, image_data: Vec<u8>, image_width: u32, image_height: u32) -> Image{
        let mut image_texture = 0u32;
        unsafe{
            gl::GenTextures(1, &mut image_texture);
            gl::BindTexture(gl::TEXTURE_2D, image_texture);

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::NEAREST as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::NEAREST as i32,
            );

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                image_width as i32,
                image_height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                image_data.as_ptr() as *const _
            );
        }

        Image {mw: w, mh: h, mx: x, my: y, structure: Vec::new(), image: image_texture, vao: 0, str_vbo: 0, uv_vbo: 0}
    }

    pub fn create_struct(&mut self){
        self.structure.push((self.mx, self.my));
        self.structure.push((self.mx+self.mw, self.my));
        self.structure.push((self.mx, self.my+self.mh));
        self.structure.push((self.mx+self.mw, self.my+self.mh));

        (self.vao, self.str_vbo, self.uv_vbo) = lad::load_image(&self.structure);
    }
}
