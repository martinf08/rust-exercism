use crate::Part::{Bottle, Pass, Wall, WallEnd};
use crate::Punctuation::{Comma, Dot, Null};

enum Punctuation {
    Comma,
    Dot,
    Null,
}

impl Punctuation {
    fn get(&self) -> String {
        match self {
            Comma => ".",
            Dot => ",",
            Null => "",
        }
        .to_string()
    }
}

enum Part {
    Wall(u32, Punctuation),
    Bottle(u32, Punctuation),
    Pass(u32),
    WallEnd(u32, Punctuation),
}

impl Part {
    fn write(&self, line: &mut String) {
        match self {
            Wall(_n, p) => {
                line.push_str(format!(" on the wall{} ", p.get()).as_str());
            }
            Bottle(n, p) => {
                let mut segment = Box::new(String::new());
                match *n {
                    0 => segment.push_str(format!("no more bottles of beer{}", p.get()).as_str()),
                    m if m == 1 => {
                        let v = Box::new(n.to_string());
                        let string = format!("{} bottle of beer{}", v, p.get());
                        segment.push_str(string.as_str())
                    }
                    _ => (),
                }

                if segment.is_empty() {
                    let v = Box::new(n.to_string());
                    let string = format!("{} bottles of beer{}", v, p.get());
                    segment.push_str(string.as_str())
                }
                line.push_str(segment.as_str());
            }
            Pass(n) => match *n {
                0 => line.push_str("\nGo to the store and buy some more, "),
                1 => line.push_str("\nTake it down and pass it around, "),
                _ => line.push_str("\nTake one down and pass it around, "),
            },
            WallEnd(n, _p) => {
                if *n == 0 {
                    line.push_str("99 bottles of beer on the wall.\n");
                } else {
                    match (*n).saturating_sub(1) {
                        0 => line.push_str("no more bottles of beer on the wall.\n"),
                        1 => line.push_str("1 bottle of beer on the wall.\n"),
                        x if x > 1 => {
                            line.push_str(format!("{} bottles of beer on the wall.\n", x).as_str())
                        }
                        _ => (),
                    }
                }
            }
        }

        if let Some(r) = line.get_mut(0..1) {
            r.make_ascii_uppercase();
        }
    }
}

pub fn verse(n: u32) -> String {
    let mut verse = String::new();
    let verse_flow = [
        Bottle(n, Null),
        Wall(n, Dot),
        Bottle(n, Comma),
        Pass(n),
        WallEnd(n, Dot),
    ];
    verse_flow.iter().for_each(|p| p.write(&mut verse));

    verse
}

fn push(sing: &mut String, verse: String) {
    sing.push_str(verse.as_str());
}

pub fn sing(start: u32, end: u32) -> String {
    let mut sing = String::new();

    (end..=start).rev().for_each(|x| {
        push(&mut sing, verse(x));
        if x != end {
            sing.push('\n')
        }
    });

    sing
}
