use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct ID(usize);

impl ID {
    pub fn new() -> Self {
        Self(1)
    }

    pub fn add(&mut self) -> usize {
        let current = self.0;
        self.0 = self.0.wrapping_add(1);
        current
    }
}

impl Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
