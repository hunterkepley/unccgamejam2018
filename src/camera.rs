pub struct Camera {
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub center: (f32, f32),
}

impl Camera {
    pub fn new(position: (f32, f32), size: (f32, f32)) -> Camera {
        let center = ((position.0 / 2.0) - (size.0 / 2.0), (position.1 / 2.0) - (size.1 / 2.0));
        Camera{ position, size, center }
    }

    pub fn update(&mut self) {
        self.position = ((self.center.0) - (self.size.0 / 2.0), (self.position.1) - (self.size.1 / 2.0));
    }
}