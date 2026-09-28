mod load;
use load as ld;

pub struct Rec{
    pub mw: usize, pub mh: usize,
    pub mx: usize, pub my: usize,
    pub mr: u8, pub mg: u8, pub mb: u8,
    structure: Vec<(usize, usize)>,

    pub vao: u32, pub vbo: u32,
}

impl Rec{
    pub fn new(w: usize, h: usize, x: usize, y: usize, r: u8, g: u8, b: u8) -> Rec{
        Rec {mw: w, mh: h, mx: x, my: y, structure: Vec::new(), mr: r, mg: g, mb: b, vao: 0, vbo: 0}
    }

    pub fn create_struct(&mut self, window_size: (usize, usize)){
        self.structure.clear();
        self.structure.push((self.mx, self.my));
        self.structure.push((self.mx+self.mw, self.my));
        self.structure.push((self.mx, self.my+self.mh));
        self.structure.push((self.mx+self.mw, self.my+self.mh));

        (self.vao, self.vbo) = ld::load(&self.structure, window_size, self.mr, self.mg, self.mb);
    }
}
