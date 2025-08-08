#[derive(Clone, Debug)]
pub struct Grid {
    width: usize,
    height: usize,
    data: Vec<u32>,
}

impl Grid {
    pub(crate) fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0; width * height],
        }
    }

    pub fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn get(&self, x: usize, y: usize) -> u32 {
        self.data[self.index(x, y)]
    }

    pub fn set(&mut self, x: usize, y: usize, value: u32) {
        let idx = self.index(x, y);
        self.data[idx] = value;
    }
}