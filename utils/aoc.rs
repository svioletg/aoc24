#![allow(dead_code)]

use std::{collections::HashSet, fmt::{Display, Formatter}, ops};

// Matrices
#[derive(Debug, Clone)]
pub struct Matrix<T> {
    pub rows: Vec<Vec<T>>
}

impl<T> Matrix<T> {
    /// Retrieves a value in this matrix by its [MxPoint].
    pub fn get(&self, pt: MxPoint) -> Option<&T> {
        if pt.0 < 0 || pt.1 < 0 { return None }
        self.rows.get(pt.0 as usize).and_then(|r| r.get(pt.1 as usize))
    }

    /// Retrieves a value in this matrix by a flat index, as if every row of the matrix were concatenated together as one.
    pub fn get_flat(&self, idx: usize) -> Option<&T> {
        let pt = MxPoint((idx / self.rows.len()) as isize, (idx - self.rows[0].len()) as isize);
        self.get(pt)
    }

    /// Sets the value of an [MxPoint] in this matrix to `val`.
    pub fn set(&mut self, pt: MxPoint, val: T) {
        self.rows[pt.0 as usize][pt.1 as usize] = val;
    }

    /// Attempts to set the value of an [MxPoint] in this matrix to `val`, returning [Err] if the point does not exist.
    pub fn try_set(&mut self, pt: MxPoint, val: T) -> Result<(), String> {
        if self.get(pt).is_none() { return Err(format!("Point does not exist: {}", pt)) }
        self.rows[pt.0 as usize][pt.1 as usize] = val;

        Ok(())
    }

    pub fn height(&self) -> usize { self.rows.len() }
    pub fn width(&self) -> usize { self.rows[0].len() }

    /// Walks through this matrix beginning at `start` and increasing by `step`
    /// for as long as the point is within the matrix's bounds.
    ///
    /// # Arguments
    /// * `bidi` - Whether to walk the matrix in both directions from the given starting point.
    pub fn walk_pattern(&self, start: MxPoint, step: MxPoint, bidi: bool) -> HashSet<MxPoint> {
        let mut pos = start;
        let mut points: HashSet<MxPoint> = HashSet::new();

        while self.get(pos).is_some() { points.insert(pos); pos = pos + step; }
        if bidi {
            pos = start;
            while self.get(pos).is_some() { points.insert(pos); pos = pos + (step * MxPoint(-1, -1)); }
        }

        points
    }

    pub fn new() -> Matrix<T> {
        Matrix{rows: vec![]}
    }

    pub fn from_str_mapped<F>(s: &str, mut f: F) -> Matrix<T>
    where F: FnMut(char) -> T {
        let data: Vec<Vec<T>> = s.split('\n')
            .filter(|i| !i.is_empty())
            .map(|i| i.chars().map(&mut f).collect())
            .collect();
        Matrix::from(data)
    }
}

impl Matrix<char> {
    pub fn as_string(&self) -> String {
        let mut s: String = "".to_string();
        for row in self.rows.iter() {
            s.push_str(&row.iter().collect::<String>());
            s.push_str("\n");
        }

        s
    }

    pub fn from_str(s: &str) -> Matrix<char> {
        let data: Vec<Vec<char>> = s.split('\n')
            .filter(|i| !i.is_empty())
            .map(|i| i.chars().collect())
            .collect();
        Matrix::from(data)
    }
}

/// Example:
/// ```
/// use aoc::{Matrix, MxPoint};
/// let v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
/// let mat = Matrix::from(v);
/// assert!(mat.get(MxPoint(0, 0)).is_some());
/// assert!(mat.get(MxPoint(1, 2)).is_some());
/// ```
impl<T> From<Vec<Vec<T>>> for Matrix<T> {
    fn from(data: Vec<Vec<T>>) -> Matrix<T> {
        Matrix{rows: data}
    }
}

/// Example:
/// ```
/// use aoc::{Matrix, MxPoint};
/// let arr = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
/// let mat = Matrix::from(arr);
/// assert!(mat.get(MxPoint(0, 0)).is_some());
/// assert!(mat.get(MxPoint(1, 2)).is_some());
/// ```
impl<T: Clone, const R: usize, const C: usize> From<[[T; C]; R]> for Matrix<T> {
    fn from(data: [[T; C]; R]) -> Matrix<T> {
        Matrix{rows: data.iter().map(|row| row.to_vec()).collect()}
    }
}

// Points

/// Represents a point within a [Matrix], as a tuple of `(row, column)`.
///
/// Actual matrix points cannot be negative, however [MxPoint]s values are signed to allow using
/// points for math operations on other points, e.g.
/// ```
/// use aoc::MxPoint;
/// assert_eq!(MxPoint(3, 5) + MxPoint(-2, -1), MxPoint(1, 4));
/// ```
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

pub trait MxPointDirections {
    const UP:    MxPoint = MxPoint(-1, 0);
    const RIGHT: MxPoint = MxPoint(0, 1);
    const DOWN:  MxPoint = MxPoint(1, 0);
    const LEFT:  MxPoint = MxPoint(0, -1);

    /// Returns a cardinal direction corresponding to the given [char].
    /// Expects one of `^`, `>`, `v`, or `<`. Other values will return [None].
    fn char_cardinal(ch: &char) -> Option<MxPoint>;

    /// Returns cardinal directions N, E, S, W as [MxPoint]s.
    fn cardinals4() -> [MxPoint; 4];
    /// Returns cardinal directions N, NE, E, SE, S, SW, W, NW as [MxPoint]s.
    fn cardinals8() -> [MxPoint; 8];

    /// Returns this point moved upwards `n` times.
    fn to_above(self, n: isize) -> MxPoint;
    /// Returns this point moved right `n` times.
    fn to_right(self, n: isize) -> MxPoint;
    /// Returns this point moved upwards `n` times.
    fn to_below(self, n: isize) -> MxPoint;
    /// Returns this point moved upwards `n` times.
    fn to_left(self, n: isize)  -> MxPoint;

    /// Returns a point turned 90 degrees clockwise.
    /// Only works will cardinal directions, i.e. [MxPoint::UP], [MxPoint::RIGHT], [MxPoint::DOWN], and [MxPoint::LEFT].
    fn turn_90deg(&self) -> MxPoint;
}

impl MxPointDirections for MxPoint {
    fn char_cardinal(ch: &char) -> Option<MxPoint> {
        match ch {
            '^' => Some(MxPoint::UP),
            '>' => Some(MxPoint::RIGHT),
            'v' => Some(MxPoint::DOWN),
            '<' => Some(MxPoint::LEFT),
            _ => None
        }
    }

    fn cardinals4() -> [MxPoint; 4] { [MxPoint::UP, MxPoint::RIGHT, MxPoint::DOWN, MxPoint::LEFT] }
    fn cardinals8() -> [MxPoint; 8] {
        [
            MxPoint::UP, MxPoint::UP.to_right(1),
            MxPoint::RIGHT, MxPoint::RIGHT.to_below(1),
            MxPoint::DOWN, MxPoint::DOWN.to_left(1),
            MxPoint::LEFT, MxPoint::LEFT.to_above(1)
        ]
    }

    fn to_above(self, n: isize) -> MxPoint { self + (MxPoint::UP    * MxPoint(n, n)) }
    fn to_right(self, n: isize) -> MxPoint { self + (MxPoint::RIGHT * MxPoint(n, n)) }
    fn to_below(self, n: isize) -> MxPoint { self + (MxPoint::DOWN  * MxPoint(n, n)) }
    fn to_left(self, n: isize)  -> MxPoint { self + (MxPoint::LEFT  * MxPoint(n, n)) }

    fn turn_90deg(&self) -> MxPoint {
        match self {
            MxPoint(-1, 0) => MxPoint(0, 1),
            MxPoint(0, 1)  => MxPoint(1, 0),
            MxPoint(1, 0)  => MxPoint(0, -1),
            MxPoint(0, -1) => MxPoint(-1, 0),
            _ => panic!("MxPoint::turn_90deg got an invalid input: {self}")
        }
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
