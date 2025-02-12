#![allow(unused)]

use std::{fmt::{Display, Formatter, Result}, ops};

// Matrices
pub type Matrix<T> = Vec<Vec<T>>;

pub fn string_to_matrix(s: &String) -> Matrix<char> {
    let mut matrix: Matrix<char> = vec![];
    for line in s.split("\n") {
        matrix.push(line.chars().collect());
    }

    matrix
}

pub fn matrix_map<T, F, N>(m: &Matrix<T>, mapfn: F) -> Matrix<N>
    where F: Fn(&T) -> N {
    let mut mapped: Matrix<N> = vec![];
    for row in m.iter() {
        mapped.push(row.iter().map(|i| mapfn(i)).collect());
    }

    mapped
}

pub fn matget<T>(m: &Matrix<T>, pt: MxPoint) -> Option<&T> {
    if pt.in_matrix(m) { Some(&m[pt.0 as usize][pt.1 as usize]) }
    else { None }
}

pub fn matset<T>(m: &mut Matrix<T>, pt: MxPoint, val: T) {
    if !pt.in_matrix(m) { panic!("Point does not exist in matrix: {pt}"); }
    m[pt.0 as usize][pt.1 as usize] = val;
}

pub fn mat_adjacent<T>(m: &Matrix<T>, pt: MxPoint, relative: bool, corners: bool) -> Vec<MxPoint> {
    let adj_pts: Vec<MxPoint> = if corners {
        vec![
        (-1, -1), (-1, 0), (-1, 1),
         (0, -1),  (0, 0),  (0, 1),
         (1, -1),  (1, 0),  (1, 1)
        ].iter().map(|&i| MxPoint(i.0, i.1)).collect()
    } else {
        vec![
                    (-1, 0),
            (0, -1), (0, 0), (0, 1),
                     (1, 0)
        ].iter().map(|&i| MxPoint(i.0, i.1)).collect()
    };

    if relative { return adj_pts; }

    adj_pts.iter().map(|&i| pt + i).collect()
}

// Points
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct MxPoint(pub isize, pub isize);

impl From<(isize, isize)> for MxPoint {
    fn from(tup: (isize, isize)) -> Self { MxPoint(tup.0, tup.1) }
}

impl From<(usize, usize)> for MxPoint {
    fn from(tup: (usize, usize)) -> Self {
        if tup.0 > isize::max_value() as usize || tup.1 > isize::max_value() as usize {
            panic!("One of the provided tuple values is too large for conversion to `isize`: {}, {}", tup.0, tup.1);
        }

        MxPoint(tup.0 as isize, tup.1 as isize)
    }
}

impl Into<(isize, isize)> for MxPoint {
    fn into(self) -> (isize, isize) { (self.0, self.1) }
}

impl Into<(usize, usize)> for MxPoint {
    fn into(self) -> (usize, usize) { (self.0 as usize, self.1 as usize) }
}

impl Display for MxPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

pub trait MxPointMethods {
    fn up()    -> MxPoint;
    fn right() -> MxPoint;
    fn down()  -> MxPoint;
    fn left()  -> MxPoint;

    fn to_above(self, n: isize) -> MxPoint;
    fn to_right(self, n: isize) -> MxPoint;
    fn to_below(self, n: isize) -> MxPoint;
    fn to_left(self, n: isize)  -> MxPoint;

    fn turn_90deg(&self) -> MxPoint;
    fn in_matrix<T>(&self, m: &Matrix<T>) -> bool;
}

impl MxPointMethods for MxPoint {
    fn up()    -> MxPoint { MxPoint(-1, 0) }
    fn right() -> MxPoint { MxPoint(0, 1) }
    fn down()  -> MxPoint { MxPoint(1, 0) }
    fn left()  -> MxPoint { MxPoint(0, -1) }

    fn to_above(self, n: isize) -> MxPoint { self + (MxPoint::up()    * MxPoint(n, n)) }
    fn to_right(self, n: isize) -> MxPoint { self + (MxPoint::right() * MxPoint(n, n)) }
    fn to_below(self, n: isize) -> MxPoint { self + (MxPoint::down()  * MxPoint(n, n)) }
    fn to_left(self, n: isize)  -> MxPoint { self + (MxPoint::left()  * MxPoint(n, n)) }

    fn turn_90deg(&self) -> MxPoint {
        match self {
            MxPoint(-1, 0) => MxPoint(0, 1),
            MxPoint(0, 1)  => MxPoint(1, 0),
            MxPoint(1, 0)  => MxPoint(0, -1),
            MxPoint(0, -1) => MxPoint(-1, 0),
            _ => panic!("MxPoint::turn_90deg got an invalid input: {self}")
        }
    }

    fn in_matrix<T>(&self, m: &Matrix<T>) -> bool {
        let mat_height: usize = m.len();
        let mat_width: usize = m[0].len();

        // Check if negative before later casting to usize
        if self.0 < 0 || self.1 < 0 { return false; }

        (0..mat_height).contains(&(self.0 as usize)) && (0..mat_width).contains(&(self.0 as usize))
    }
}

impl ops::Add for MxPoint {
    type Output = Self;
    fn add(self, other: Self) -> Self { MxPoint(self.0 + other.0, self.1 + other.1) }
}
impl ops::Sub for MxPoint {
    type Output = Self;
    fn sub(self, other: Self) -> Self { MxPoint(self.0 - other.0, self.1 - other.1) }
}
impl ops::Mul for MxPoint {
    type Output = Self;
    fn mul(self, other: Self) -> Self { MxPoint(self.0 * other.0, self.1 * other.1) }
}
impl ops::Div for MxPoint {
    type Output = Self;
    fn div(self, other: Self) -> Self { MxPoint(self.0 / other.0, self.1 / other.1) }
}
