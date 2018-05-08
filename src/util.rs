use rand::{self, Rng};

pub fn rand_f32() -> f32 {
    rand::thread_rng().gen()
}
