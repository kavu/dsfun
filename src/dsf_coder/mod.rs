mod coding_loop;
mod decoder;
mod encoder;
mod key;

pub use decoder::decode;
pub use encoder::encode;
pub use key::SimpleKey;

#[derive(Clone, Copy)]
pub struct CoderOptions {
    force: bool,
}

impl CoderOptions {
    pub fn new() -> Self {
        CoderOptions { force: false }
    }

    pub fn get_force(self) -> bool {
      self.force
    }

    pub fn force(mut self, force: bool) -> Self {
      self.force = force;

      self
    }
}
