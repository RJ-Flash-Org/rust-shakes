//! Vision kit (MVP: grayscale image buffer).

use foundry::Result;

/// A simple grayscale image buffer.
#[derive(Debug, Clone)]
pub struct GrayImage {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
}

impl GrayImage {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0u8; width * height],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: u8) -> Result<()> {
        if x >= self.width || y >= self.height {
            return Err(foundry::Error::InvalidArgument("out of bounds".into()));
        }
        self.data[y * self.width + x] = value;
        Ok(())
    }

    pub fn get(&self, x: usize, y: usize) -> Result<u8> {
        if x >= self.width || y >= self.height {
            return Err(foundry::Error::InvalidArgument("out of bounds".into()));
        }
        Ok(self.data[y * self.width + x])
    }
}
