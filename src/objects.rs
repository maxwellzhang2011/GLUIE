mod load;
use load as lad;

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

        (self.vao, self.vbo) = lad::load(&self.structure);
    }

    pub fn change(&mut self, w: u32, h: u32, x: u32, y: u32, r: u8, g: u8, b: u8){
        (self.mw, self.mh, self.mx, self.my, self.mr, self.mg, self.mb) = (w, h, x, y, r, g, b);

        self.structure.clear();
        self.structure.push((self.mx, self.my));
        self.structure.push((self.mx+self.mw, self.my));
        self.structure.push((self.mx, self.my+self.mh));
        self.structure.push((self.mx+self.mw, self.my+self.mh));

        lad::change(&self.structure, &mut self.vbo)
    }
}
