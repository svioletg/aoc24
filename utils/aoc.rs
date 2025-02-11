use std::ops;

#[derive(Debug)]
pub struct MxCoord {
    r: i32,
    c: i32
}

impl MxCoord {
    pub fn from(r: i32, c: i32) -> MxCoord {
        MxCoord{r, c}
    }
}

impl ops::Add for MxCoord {
    type Output = Self;
    fn add(self, other: Self) -> Self { MxCoord{r: self.r + other.r, c: self.c + other.c} }
}
impl ops::Sub for MxCoord {
    type Output = Self;
    fn sub(self, other: Self) -> Self { MxCoord{r: self.r - other.r, c: self.c - other.c} }
}
impl ops::Mul for MxCoord {
    type Output = Self;
    fn mul(self, other: Self) -> Self { MxCoord{r: self.r * other.r, c: self.c * other.c} }
}
impl ops::Div for MxCoord {
    type Output = Self;
    fn div(self, other: Self) -> Self { MxCoord{r: self.r / other.r, c: self.c / other.c} }
}

fn main() {
    let a: MxCoord = MxCoord{r: 0, c: 0};
}

pub fn string_to_matrix(s: &String) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = vec![];
    for line in s.split("\n") {
        matrix.push(line.chars().collect());
    }

    matrix
}
