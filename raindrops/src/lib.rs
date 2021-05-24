use std::fmt;

#[derive(Debug, Copy, Clone)]
enum Sound {
    Pling = 3,
    Plang = 5,
    Plong = 7,
}

impl fmt::Display for Sound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

static SOUNDS: [Sound; 3] = [Sound::Pling, Sound::Plang, Sound::Plong];

pub fn raindrops(n: u32) -> String {
    let mut result = String::new();

    for s in SOUNDS.iter().cloned() {
        if n % s as u32 == 0 {
            result.push_str(&*s.to_string());
        }
    }

    if result.is_empty() {
        result.push_str(&*n.to_string());
    }

    result
}
