pub struct Camera {
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub center: (f32, f32),
    pub center_origin: (f32, f32),
    pub offset: (f32, f32),
}

impl Camera {
    pub fn new(position: (f32, f32), size: (f32, f32)) -> Camera {
        let center = (position.0 + (size.0 / 2.0), position.1 + (size.1 / 2.0));
        let center_origin = center;
        let offset = (0.0, 0.0);
        Camera{ position, size, center, center_origin, offset }
    }

    pub fn update(&mut self) {
        self.position = ((self.center.0) - (self.size.0 / 2.0), (self.position.1) - (self.size.1 / 2.0));
        self.offset = (self.center_origin.0 - self.center.0, self.center_origin.1 - self.center.1);
    }
}