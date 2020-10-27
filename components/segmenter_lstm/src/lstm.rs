use crate::structs;

pub struct Lstm {
    data: structs::PlaceholderData,
}

impl Lstm {
    pub fn new(data: structs::PlaceholderData) -> Self {
        Self { data }
    }

    pub fn get_sample(&self) -> f32 {
        self.data.a
    }
}
