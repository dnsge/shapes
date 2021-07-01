pub struct ScreenBuffer {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl ScreenBuffer {
    pub fn new(width: usize, height: usize) -> ScreenBuffer {
        ScreenBuffer {
            buffer: vec![0; width * height],
            width,
            height,
        }
    }

    pub fn get_coords(&mut self, x: usize, y: usize) -> Option<&mut u32> {
        if x >= self.width || y >= self.height {
            None
        } else {
            self.buffer.get_mut(y * self.width + x)
        }
    }

    pub fn get_coords_i(&mut self, x: isize, y: isize) -> Option<&mut u32> {
        if x < 0 || y < 0 {
            None
        } else {
            self.get_coords(x as usize, y as usize)
        }
    }

    pub fn get_pixel(&mut self, pixel: (usize, usize)) -> Option<&mut u32> {
        self.get_coords(pixel.0, pixel.1)
    }

    pub fn get_pixel_i(&mut self, pixel: (isize, isize)) -> Option<&mut u32> {
        if pixel.0 < 0 || pixel.1 < 0 {
            None
        } else {
            self.get_coords(pixel.0 as usize, pixel.1 as usize)
        }
    }

    pub fn set_pixel(&mut self, pixel: (usize, usize), value: u32) -> bool {
        if let Some(p) = self.get_pixel(pixel) {
            *p = value;
            true
        } else {
            false
        }
    }

    pub fn set_pixel_i(&mut self, pixel: (isize, isize), value: u32) -> bool {
        if pixel.0 < 0 || pixel.1 < 0 {
            false
        } else {
            self.set_pixel((pixel.0 as usize, pixel.1 as usize), value)
        }
    }

    pub fn clear(&mut self, color: u32) {
        self.buffer.fill(color)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn buffer(&self) -> &[u32] {
        &self.buffer
    }

    pub fn outside_screen(&self, p: (isize, isize)) -> bool {
        (p.0 < 0 || p.0 >= (self.width as isize)) // outside x
            || (p.1 < 0 || p.1 >= (self.height as isize)) // outside y
    }

    pub fn inside_screen(&self, p: (isize, isize)) -> bool {
        (0 < p.0 && p.0 < (self.width as isize)) // inside x
            && (0 < p.1 && p.1 < (self.height as isize)) // inside y
    }

    // Naive implementation of checking if triangle is within the screen
    // Simple bounding box check
    pub fn triangle_inside_screen(
        &self,
        p1: (isize, isize),
        p2: (isize, isize),
        p3: (isize, isize),
    ) -> bool {
        let min_x = isize::min(p1.0, isize::min(p2.0, p3.0));
        let max_x = isize::max(p1.0, isize::max(p2.0, p3.0));
        let min_y = isize::min(p1.1, isize::min(p2.1, p3.1));
        let max_y = isize::max(p1.1, isize::max(p2.1, p3.1));

        self.inside_screen((max_x, min_y))
            || self.inside_screen((min_x, min_y))
            || self.inside_screen((max_x, max_y))
            || self.inside_screen((min_x, max_y))
    }
}
