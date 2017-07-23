// https://en.wikipedia.org/wiki/Matrix_(mathematics)
// https://en.wikipedia.org/wiki/Minor_(linear_algebra)
// https://en.wikipedia.org/wiki/Adjugate_matrix
// verify inverse: http://matrix.reshish.com/inverCalculation.php



// TODO FIXME:
// split code & tests out
// put tests in cfg/mod test block




#[allow(unused_imports)]
use std::io::Write;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::iter::Iterator;
#[allow(unused_imports)]
use num::PrimInt;
use num::Integer;
use util::range;
#[macro_use]
#[allow(unused_imports)]
// use modulo::mod_num::{ModuloSignedExt};
use modulo::Mod;



#[allow(dead_code)]
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Matrix {
  rows: usize,
  cols: usize,
  size: usize,
  iter_index: usize,
  entries: Vec<isize>
}



#[allow(dead_code)]
impl Matrix {

  /// new()
  ///
  /// new rows x cols sized matrix with a flat representation of entries
  ///
  /// new(2, 2, vec![1, 2, 3, 4])
  ///
  /// 1 2
  /// 3 4
  ///
  pub fn new(rows: usize, cols: usize, entries: Vec<isize>) -> Self {
    assert_eq!(rows * cols, entries.len());
    Matrix {
      rows: rows,
      cols: cols,
      size: rows * cols,
      iter_index: 1,
      entries: entries
    }
  }
  
  /// accessor: rows()
  ///
  pub fn rows(&self) -> usize {
    self.rows
  }

  /// accessor: cols()
  ///
  pub fn cols(&self) -> usize {
    self.cols
  }

  /// accessor: size()
  ///
  pub fn size(&self) -> usize {
    self.size
  }

  /// accessors: entries()
  ///
  pub fn entries(&self) -> Vec<isize> {
    self.entries.to_owned()
  }

  /// nth_offset()
  ///
  /// point directly into a flat vector
  ///
  pub fn nth_offset(&self, row: usize, col: usize) -> usize {
    (((row-1) * self.cols) + col) - 1
  }

  /// nth()
  ///
  /// 01 02 03 04
  ///
  /// (1,1) = 01 = 0
  /// (1,2) = 02 = 1
  /// (2,1) = 03 = 2
  /// (2,2) = 04 = 3
  ///
  /// nth = (((row-1) * cols) + col) - 1
  ///
  /// (1,1) (((1  -1) * 2) + 1) - 1
  ///       (((0) * 2) + 1) - 1
  ///       1 - 1
  ///       0
  /// (1,2) (((1  -1) * 2) + 2) - 1
  ///       (((0) * 2) + 2) - 1
  ///       2 - 1
  ///       1
  /// (2,1) (((2  -1) * 2) + 1) - 1
  ///       (((1) * 2) + 1) - 1
  ///       3 - 1
  ///       2
  /// (2,2) (((2  -1) * 2) + 2) - 1
  ///       (((1) * 2) + 2) - 1
  ///       4 - 1
  ///       3
  ///
  pub fn nth(&self, row: usize, col: usize) -> isize {
    assert!(row > 0 && col > 0, "row > 0 && col > 0");
    let offset = self.nth_offset(row, col);
    self.entries.get(offset).unwrap().to_owned()
  }

  /// set_nth()
  ///
  /// set_nth(1, 2, v):
  /// 0 0 = 0 v
  /// 0 0   0 0
  ///
  pub fn set_nth(&mut self, row: usize, col: usize, value: isize) -> &Self {
    let offset = self.nth_offset(row, col);
    self.entries[offset] = value;
    self
  }

  /// row()
  ///
  /// 01 02 03 04
  ///
  /// row 1: 01 02
  /// row 2: 03 04
  ///
  pub fn row(&self, row: usize) -> Vec<isize> {
    assert!(row > 0, "row > 0");
    let start = (row-1) * self.cols;
    let end = start + self.cols;
    let mut v = Vec::new();
    v.extend_from_slice(&self.entries[start .. end]);
    v
  }

  /// col()
  ///
  /// 01 02 03 04
  ///
  /// row 1: 01 03
  /// row 2: 02 04
  ///
  pub fn col(&self, col: usize) -> Vec<isize> {
    assert!(col > 0, "col > 0");
    let mut v = Vec::new();
    for i in range::SimpleStepRange((col - 1) as isize, self.size as isize, self.cols as isize) {
      let e = self.entries.get(i as usize).unwrap();
      v.push(e.to_owned());
    }
    v
  }

  /// identity()
  ///
  /// 01 00
  /// 00 01
  ///
  /// 01 00 00 01
  ///
  /// a_ij = 1
  ///
  pub fn identity(&self) -> Self {
    let mut entries: Vec<isize> = Vec::with_capacity(self.size);
    for row in 1 .. (self.rows+1) {
      for col in 1 .. (self.cols+1) {
        if row == col {
          entries.push(1);
        } else {
          entries.push(0);
        }
      }
    }
    Matrix::new(self.rows, self.cols, entries)
  }

  /// is_square()
  ///
  pub fn is_square(&self) -> bool {
    self.rows == self.cols
  }

  /// is_invertible()
  ///
  pub fn is_invertible(&self) -> bool {
    self.det() != 0
  }

  /// is_invertible_mod()
  ///
  pub fn is_invertible_mod(&self, mod_m: usize) -> bool {
    self.det().gcd(&(mod_m as isize)) == 1
  }

  /// transpose()
  ///
  /// 1 2 3 = 1 4 = 1 4 2 5 3 6
  /// 4 5 6   2 5
  ///         3 6
  ///
  /// (1,1),(2,1),(1,2),(2,2),(1,3),(2,3)
  ///
  pub fn transpose(&self) -> Matrix {
    let mut entries = Vec::with_capacity(self.size);
    for j in 1 .. self.cols+1 {
      for i in 1 .. self.rows+1 {
        let e = self.nth(i,j);
        entries.push(e.to_owned());
      }
    }
    Matrix::new(self.cols, self.rows, entries)
  }

  /// diagonal()
  ///
  /// 1 2 3 = 1 0 0
  /// 4 5 6   0 5 0
  /// 7 8 9   0 0 9
  ///
  pub fn diagonal(&self) -> Matrix {
    assert!(self.is_square() == true, "must be a square matrix");
    let mut entries: Vec<isize> = Vec::with_capacity(self.size);
    for r in 1 .. self.rows+1 {
      for c in 1 .. self.cols+1 {
        if r == c {
          let e = self.nth(r, c);
          entries.push(e.to_owned())
        } else {
          entries.push(0)
        }
      }
    }
    Matrix::new(self.rows, self.cols, entries)
  }

  /// right_diagonal()
  ///
  pub fn right_diagonal(&self) -> Matrix {
    self.diagonal()
  }

  /// left_diagonal()
  ///
  /// 1,2,3 = 0,0,3
  /// 4,5,6   0,5,0
  /// 7,8,9   7,0,0
  ///
  pub fn left_diagonal(&self) -> Matrix {
    assert!(self.is_square() == true, "must be a square matrix");
    let mut entries: Vec<isize> = Vec::with_capacity(self.size);
    let mut i_r = 1;
    let mut i_c = self.cols;
    for r in 1 .. self.rows+1 {
      for c in 1 .. self.cols+1 {
        if r == i_r && c == i_c {
          let e = self.nth(r, c);
          entries.push(e.to_owned());
          i_r += 1;
          i_c -= 1;
        } else {
          entries.push(0)
        }
      }
    }
    Matrix::new(self.rows, self.cols, entries)
  }

  /// det()
  ///
  /// determinant - TODO: extend to arbitrary sized matrix
  ///
  pub fn det(&self) -> isize {
    assert!(self.is_square(), "must be a square matrix");
    match (self.rows, self.cols) {
      (1,1) => self.det_1x1(),
      (2,2) => self.det_2x2(),
      _     => self.det_NxN()
    }
  }

  /// det_1x1()
  ///
  /// det a = a
  ///
  pub fn det_1x1(&self) -> isize {
    self.nth(1,1)
  }

  /// det_2x2()
  ///
  /// det a b = ad - bc
  ///     c d
  ///
  pub fn det_2x2(&self) -> isize {
    let (a, b, c, d) = (self.nth(1,1), self.nth(1,2), self.nth(2,1), self.nth(2,2));
    a*d - b*c
  }

  /// det_NxN()
  ///
  /// Definition 1.5 of Cryptography Theory & Practice:
  ///
  /// Suppose that A = (a_ij) is an m x m matrix.
  ///  For
  ///   1 <= i <= m,
  ///   1 <= j <= m,
  ///  Define A_ij to be the matrix obtained from A by:
  ///   deleting the ith row
  ///   deleting the jth column
  ///
  ///  The determinant of A, denoted det_A, is:
  ///   the value a_1,1 if m = 1.
  ///   if m > 1, then det_A is computed recursively from the formula:
  ///    det A = SUM(j=1 to m) of ((-1)^i+j)a_i,j(det A_ij),
  ///     where i is any fixed integer between 1 and m.
  ///
  #[allow(non_snake_case)]
  pub fn det_NxN(&self) -> isize {
    (1 .. self.cols+1).map(|j| self.det_NxN_helper(1, j)).sum()
  }

  #[allow(non_snake_case)]
  pub fn det_NxN_helper(&self, i: usize, j: usize) -> isize {
    ((-1).pow((i+j) as u32)) * self.nth(i, j) * self.submatrix(i,j).det()
  }

  /// zeroes()
  ///
  pub fn zeroes(&self) -> Matrix {
    zero_fill_matrix(self.rows, self.cols)
  }

  /// negate()
  ///
  pub fn negate(&self) -> Matrix {
    // let entries: Vec<isize> = self.entries.to_owned().into_iter().map(|x| x * (-1)).collect();
    // Matrix::new(self.rows, self.cols, entries)
    (-1) * self.to_owned()
  }

  /// swap_rows()
  ///
  #[allow(unused_variables)]
  pub fn swap_rows(&mut self, row_a: usize, row_b: usize) -> &Self {
    self
  }

  /// add_rows()
  ///
  #[allow(unused_variables)]
  pub fn add_rows(&mut self, row_a: usize, row_b: usize) -> &Self {
    self
  }

  /// add_row_by()
  ///
  #[allow(unused_variables)]
  pub fn add_row_by(&mut self, row: usize, scalar: isize) -> &Self {
    self
  }

  /// mul_rows()
  ///
  #[allow(unused_variables)]
  pub fn mul_rows(&mut self, row_a: usize, row_b: usize) -> &Self {
    self
  }

  /// mul_row_by()
  ///
  #[allow(unused_variables)]
  pub fn mul_row_by(&mut self, row_a: usize, scalar: isize) -> &Self {
    self
  }

  /// swap_cols()
  ///
  #[allow(unused_variables)]
  pub fn swap_cols(&mut self, col_a: usize, col_b: usize) -> &Self {
    self
  }

  /// add_cols()
  ///
  #[allow(unused_variables)]
  pub fn add_cols(&mut self, col_a: usize, col_b: usize) -> &Self {
    self
  }

  /// add_col_by()
  ///
  #[allow(unused_variables)]
  pub fn add_col_by(&mut self, col: usize, scalar: isize) -> &Self {
    self
  }

  /// mul_cols()
  ///
  #[allow(unused_variables)]
  pub fn mul_cols(&mut self, col_a: usize, col_b: usize) -> &Self {
    self
  }

  /// mul_col_by()
  ///
  #[allow(unused_variables)]
  pub fn mul_col_by(&mut self, col_a: usize, scalar: isize) -> &Self {
    self
  }

  /// submatrix()
  ///
  /// submatrix(3, 2)
  /// 1 2  3  4  = 1 3 4
  /// 5 6  7  8    5 7 8
  /// 9 10 11 12
  pub fn submatrix(&self, row: usize, col: usize) -> Matrix {
    let row_count = if row > 0 { 1 } else { 0 };
    let col_count = if col > 0 { 1 } else { 0 };
    let new_size = (self.rows - row_count) * (self.cols - col_count);
    let mut entries: Vec<isize> = Vec::with_capacity(new_size);
    for i in (1 .. self.rows+1).filter(|r| r.to_owned() != row) {
      for j in (1 .. self.cols+1).filter(|c| c.to_owned() != col) {
        let e = self.nth(i, j);
        entries.push(e);
      }
    }
    Matrix::new(self.rows - row_count, self.cols - col_count, entries)
  }

  /// minor()
  ///
  /// minor of a submatrix
  ///
  ///  1 4 7 -> M_2,3 = det 1 4 _ = det 1 4 = (9 - (-4)) = 13
  ///  3 0 5                _ _ _      -1 9
  /// -1 9 11              -1 9 _
  pub fn minor(&self, row: usize, col: usize) -> isize {
    self.submatrix(row, col).det()
  }

  /// minors()
  ///
  /// 1 3 2 -> minors = -13 2 18
  /// 4 1 3              -4 -2 -1
  /// 2 5 2               7 -5 -11
  ///
  pub fn minors(&self) -> Matrix {
    let mut entries: Vec<isize> = Vec::with_capacity(self.size);
    for i in 1 .. self.rows+1 {
      for j in 1 .. self.cols+1 {
        let minor = self.minor(i, j);
        entries.push(minor);
      }
    }
    Matrix::new(self.rows, self.cols, entries)
  }

  /// cofactor()
  ///
  /// cofactor of a minor of a submatrix
  ///
  ///  1 4 7 -> M_2,3 = det 1 4 _ = det 1 4 = (9 - (-4)) = 13 = C_2,3 = (-1)^(2+3)(13) = -13
  ///  3 0 5                _ _ _      -1 9
  /// -1 9 11              -1 9 _
  pub fn cofactor(&self, row: usize, col: usize) -> isize {
    (-1).pow((row+col) as u32) * self.submatrix(row, col).det()
  }

  /// cofactors()
  ///
  /// 1 3 2 -> cofactors = -13 -2 18
  /// 4 1 3                  4 -2  1
  /// 2 5 2                  7  5 -11
  ///
  pub fn cofactors(&self) -> Matrix {
    let mut entries: Vec<isize> = Vec::with_capacity(self.size);
    for i in 1 .. self.rows+1 {
      for j in 1 .. self.cols+1 {
        let cofactor = self.cofactor(i, j);
        entries.push(cofactor);
      }
    }
    Matrix::new(self.rows, self.cols, entries)
  }

  /// adjugate()
  ///
  /// often denoted A*, B* etc
  ///
  pub fn adjugate(&self) -> Matrix {
    self.cofactors().transpose()
  }

  /// adjoint()
  ///
  pub fn adjoint(&self) -> Matrix {
    self.adjugate()
  }

  /// inverse()
  ///
  /// 11 08 -> inverse = 07/53 -3/53
  /// 03 07              -8/53 11/53
  ///
  pub fn inverse(&self) -> Option<Matrix> {
    if !self.is_invertible() {
      None
    } else {
      let d = self.det();
      let m = self.adjugate();
      Some((1/d) * m)
    }
  }

  /// inverse_unsafe()
  ///
  pub fn inverse_unsafe(&self) -> Matrix {
    self.inverse().unwrap()
  }

  /// inverse_mod()
  ///
  /// 11 08 -> inverse_mod 26 = 07 18
  /// 03 07                     23 11
  ///
  pub fn inverse_mod(&self, mod_m: usize) -> Option<Matrix> {
    if !self.is_invertible_mod(mod_m) {
      None
    } else {
      let adj = self.adjugate();
      // TODO FIXME: Get rid of count()? Warnings say it might not be consumed..
      // let _ = adj.to_owned().map(|x| x.modulo(m.clone())).count();
      let entries: Vec<isize> = adj.entries.into_iter().map(|x| x.modulo(mod_m.clone() as isize)).collect();
      Some(Matrix::new(adj.rows, adj.cols, entries))
    }
  }

  /// inverse_mod_unsafe()
  ///
  pub fn inverse_mod_unsafe(&self, m: usize) -> Matrix {
    self.inverse_mod(m).unwrap()
  }

  /// remove_row()
  ///
  pub fn remove_row(&self, _: usize) -> Matrix {
    Matrix::new(1,1,vec![1])
  }

  /// remove_col()
  ///
  pub fn remove_col(&self, _: usize) -> Matrix {
    Matrix::new(1,1,vec![1])
  }

  /// map()
  ///
  pub fn map<F>(&self, f: F) -> Matrix where F: Fn(isize) -> isize {
    let entries: Vec<isize> = self.to_owned().entries.into_iter().map(f).collect();
    Matrix::new(self.rows, self.cols, entries)
  }

}

/// Matrix Multiplication
///
/// (C_11 C_12) = (A_11 A_12) * (B_11 B_12) = (A_11*B_11+A12*B_21 A_11*B_12+A_12+B_22)
/// (C_21 C_22) = (A_21 A_22)   (B_21 B_22)   (A_21*B_11+A22*B_21 A_21*B_12+A_22+B_22)
///
/// 
/// A = 2|3|4   B = 0|1000
///     1|0|0       1|100
///                 0|10
///
/// C = 2*0+3*1+4*0 | 2*1000+3*100+4*10
///     1*0+0*1+0*0 | 1*1000+0*100+0*10
///
impl Mul for Matrix {
  type Output = Self;
  fn mul(self, rhs:Self) -> Self {
    // if C = AB for an n × m matrix A and an m × p matrix B, then C is an n × p matrix with entries
    // C_ij = SUM from k=1 to m {a_ik * b_kj}
    let (ar, ac, br, bc) = (self.rows, self.cols, rhs.rows, rhs.cols);
    assert_eq!(ac, br);
    let (cr, cc, c_size) = (ar, bc, ar * bc);
    let mut entries: Vec<isize> = Vec::with_capacity(c_size);
    for ci in 1 .. (cr+1) {
      for cj in 1 .. (cc+1) {
        let row = self.row(ci);
        let col = rhs.col(cj);
        let value = row.iter().zip(col.iter()).map(|(x,y)| x*y).sum();
        entries.push(value);
      }
    }
    Matrix::new(cr, cc, entries)
  }
}



/// Matrix Addition
///
/// A = 01|02   B = 05|06
///     03|04       07|08
///
/// C = 01+05 | 02+06
///     03+07 | 04+08
///
impl Add for Matrix {
  type Output = Self;
  fn add(self, rhs:Self) -> Self {
    let (ar, ac, br, bc) = (self.rows, self.cols, rhs.rows, rhs.cols);
    assert_eq!(ar, br);
    assert_eq!(ac, bc);
    let mut entries: Vec<isize> = Vec::with_capacity(self.size);
    for ci in 1 .. (ar+1) {
      for cj in 1 .. (ac+1) {
        let a = self.nth(ci, cj);
        let b = rhs.nth(ci, cj);
        entries.push(a + b);
      }
    }
    Matrix::new(ar, ac, entries)
  }
}



/// Matrix Subtraction
///
/// A = 01|02   B = 05|06
///     03|04       07|08
///
/// C = 01-05 | 02-06
///     03-07 | 04-08
///
impl Sub for Matrix {
  type Output = Self;
  fn sub(self, rhs:Self) -> Self {
    let (ar, ac, br, bc) = (self.rows, self.cols, rhs.rows, rhs.cols);
    assert_eq!(ar, br);
    assert_eq!(ac, bc);
    let mut entries: Vec<isize> = Vec::with_capacity(self.size);
    for ci in 1 .. ar+1 {
      for cj in 1 .. ac+1 {
        let a = self.nth(ci, cj);
        let b = rhs.nth(ci, cj);
        entries.push(a - b);
      }
    }
    Matrix::new(ar, ac, entries)
  }
}



/*
 * borrow impl?
 */
impl<'a> Mul for &'a Matrix {
  type Output = Matrix;
  fn mul(self, rhs: &'a Matrix) -> Matrix {
    // TODO FIXME: lol.
    // makes my life easier for now though..
    self.to_owned() * rhs.to_owned()
  }
}

impl<'a> Add for &'a Matrix {
  type Output = Matrix;
  fn add(self, rhs: &'a Matrix) -> Matrix {
    self.to_owned() + rhs.to_owned()
  }
}

/*
 * Scalar multiplication for a Matrix
 */
impl Mul<isize> for Matrix {
  type Output = Matrix;
  fn mul(self, scalar: isize) -> Matrix {
    let v: Vec<isize> = self.entries.into_iter().map(|x| scalar * x).collect();
    Matrix::new(self.rows, self.cols, v)
  }
}

/*
 * Scalar multiplication for a Matrix
 */
impl<'a> Mul<&'a isize> for Matrix {
  type Output = Matrix;
  fn mul(self, scalar: &isize) -> Matrix {
    let v: Vec<isize> = self.entries.into_iter().map(|x| scalar * x).collect();
    Matrix::new(self.rows, self.cols, v)
  }
}

/*
 * TODO FIXME: Why do I have to do this twice (for both sides)? WTF?
 *
 * Scalar multiplication for a Matrix
 */
impl Mul<Matrix> for isize {
  type Output = Matrix;
  fn mul(self, matrix: Matrix) -> Matrix {
    let scalar = self;
    let v: Vec<isize> = matrix.entries.into_iter().map(|x| scalar * x).collect();
    Matrix::new(matrix.rows, matrix.cols, v)
  }
}



/// Iterator for Matrix
///
/*
impl Iterator for Matrix {
  type Item = usize;

  fn next(&mut self) -> Option<usize> {
    if self.iter_index > self.size {
      self.iter_index = 1;
      None
    } else {
      self.iter_index += 1;
      Some(self.iter_index)
    }
  }
}
*/



/// FromIterator for Matrix
///
/*
impl FromIterator for Matrix {
  type Item = isize;

  fn next(&mut self) -> Option<usize> {
    if self.iter_index > self.size {
      self.iter_index = 1;
      None
    } else {
      self.iter_index += 1;
      Some(self.iter_index)
    }
  }
}
*/



/*
 * Helpers
 */

/// fill_matrix
///
pub fn fill_matrix(rows: usize, cols: usize, value: isize) -> Matrix {
  let mut entries: Vec<isize> = Vec::with_capacity(rows * cols);
  for _ in 1 .. (rows * cols)+1 {
    entries.push(value);
  }
  Matrix::new(rows, cols, entries)
}



/// zero_fill_matrix()
///
#[allow(dead_code)]
pub fn zero_fill_matrix(rows: usize, cols:usize) -> Matrix {
  fill_matrix(rows, cols, 0)
}



/// one_fill_matrix()
///
#[allow(dead_code)]
pub fn one_fill_matrix(rows: usize, cols:usize) -> Matrix {
  fill_matrix(rows, cols, 1)
}



/// identity_matrix()
///
#[allow(dead_code)]
pub fn identity_matrix(rows: usize, cols: usize) -> Matrix {
  zero_fill_matrix(rows, cols).identity()
}



#[test]
fn test_matrix() {
  let mat = Matrix::new(2, 2, vec![
    01, 02,
    03, 04]);
  assert_eq!(mat.rows, 2);
  assert_eq!(mat.cols, 2);
  assert_eq!(mat.size, 4);
}

#[test]
fn test_identity_matrix_1() {
  let mat = Matrix::new(2, 2, vec![
    01, 02,
    03, 04]);
  assert_eq!(mat.rows, 2);
  assert_eq!(mat.cols, 2);
  assert_eq!(mat.size, 4);
  assert_eq!(mat.identity().entries, vec![
    01, 00,
    00, 01]);
}

#[test]
fn test_identity_matrix_2() {
  let mat = Matrix::new(3, 3, vec![
    01, 02, 05,
    03, 04, 06,
    07, 08, 09]);
  assert_eq!(mat.identity().entries, vec![
    01, 00, 00,
    00, 01, 00,
    00, 00, 01]);
}

#[test]
fn test_row_matrix() {
  let mat = Matrix::new(2, 2, vec![
    01, 02,
    03, 04]);
  let row = mat.row(1);
  assert_eq!(row, vec![01, 02]);

  let row = mat.row(2);
  assert_eq!(row, vec![03, 04]);
}

#[test]
fn test_col_matrix() {
  let mat = Matrix::new(2, 2, vec![
    01, 02,
    03, 04]);
  let col = mat.col(1);
  assert_eq!(col, vec![01, 03]);

  let col = mat.col(2);
  assert_eq!(col, vec![02, 04]);
}

#[test]
fn test_nth_matrix() {
  let mat = Matrix::new(2, 2, vec![
    01, 02,
    03, 04]);
  assert_eq!(mat.nth(1,1), 1);
  assert_eq!(mat.nth(1,2), 2);
  assert_eq!(mat.nth(2,1), 3);
  assert_eq!(mat.nth(2,2), 4);
}

#[test]
fn test_set_nth_matrix() {
  let mut m = Matrix::new(2, 2, vec![
    0,0,
    0,0]);
  assert_eq!(m.set_nth(1, 2, 1).entries, vec![0,1,0,0]);
}

#[test]
fn test_matrix_addition() {
  let ma = Matrix::new(2, 2, vec![
    1, 2,
    3, 4]);
  let mb = Matrix::new(2, 2, vec![
    5, 6,
    7, 8]);
  let mc = ma + mb;
  assert_eq!(mc.entries, vec![6, 8, 10, 12]);
}

#[test]
fn test_matrix_associative_addition() {
  let ma = Matrix::new(2, 2, vec![
    1, 2,
    3, 4]);
  let mb = Matrix::new(2, 2, vec![
    5, 6,
    7, 8]);
  let mc = Matrix::new(2, 2, vec![
    1, 1,
    1, 1]);
  assert_eq!((&ma + &(&mb + &mc)).entries, vec![7, 9, 11, 13]);
  assert_eq!((&(&ma + &mb) + &mc).entries, vec![7, 9, 11, 13]);
}

#[test]
fn test_matrix_commutative_addition() {
  let ma = Matrix::new(2, 2, vec![
    1, 2,
    3, 4]);
  let mb = Matrix::new(2, 2, vec![
    5, 6,
    7, 8]);
  assert_eq!((&ma + &mb).entries, vec![6, 8, 10, 12]);
  assert_eq!((&mb + &ma).entries, vec![6, 8, 10, 12]);
  assert_eq!(&ma + &mb, &mb + &ma);
}

#[test]
fn test_matrix_additive_identity() {
  let m = Matrix::new(2, 2, vec![
    1,2,
    3,4]);
  let zero_m = zero_fill_matrix(2, 2);
  assert_eq!(m.to_owned() + zero_m, m);
  assert_eq!(m.to_owned() + m.zeroes(), m);
}

#[test]
fn test_matrix_additive_inverse() {
  let m = Matrix::new(2, 2, vec![
    1,2,
    3,4]);
  let inv_m = m.negate();
  let zero_m = zero_fill_matrix(2, 2);
  assert_eq!(m.to_owned() + inv_m, zero_m);
}

#[test]
fn test_matrix_subtraction() {
  let ma = Matrix::new(2, 2, vec![
    1, 2,
    3, 4]);
  let mb = Matrix::new(2, 2, vec![
    5, 6,
    7, 8]);
  let mc = ma - mb;
  assert_eq!(mc.entries, vec![-4, -4, -4, -4]);
}

#[test]
fn test_matrix_multiplication() {
  let ma = Matrix::new(2, 3, vec![
    2, 3, 4,
    1, 0, 0]);
  let mb = Matrix::new(3, 2, vec![
    0, 1000,
    1, 100,
    0, 10]);
  let mc = ma * mb;
  assert_eq!(mc.entries, vec![3, 2340, 0, 1000]);
}

#[test]
fn test_matrix_multiplication_of_identity() {
  let ma = Matrix::new(3, 3, vec![
    2, 3, 4,
    9, 0, 2,
    1, 1, 0]);
  assert_eq!(ma.identity() * ma.to_owned(), ma);
}

#[test]
fn test_matrix_multiplication_associativity_borrow_impl() {
  let ma = Matrix::new(2, 3, vec![
    2, 3, 4,
    1, 0, 0]);
  let mb = Matrix::new(3, 2, vec![
    0, 1000,
    1, 100,
    0, 10]);
  let mc = Matrix::new(2, 3, vec![
    5, 6, 7,
    1, 0, 0]);
  // 2x3 * 3x2 * 2x3
  // (2x3 * 3x2) = (2x2 * 2x3) = 2x3
  // 2x3 * (3x2 * 2x3) = 2x3 * 3x3 = 2x3
  assert_eq!(&(&ma*&mb)*&mc, &ma*&(&mb*&mc));
}

#[test]
fn test_matrix_scalar_multiplication() {
  let ma = Matrix::new(2, 2, vec![1, 1, 1, 1]);
  assert_eq!((ma*2).entries, vec![2, 2, 2, 2]);

  let ma = Matrix::new(2, 2, vec![1, 1, 1, 1]);
  assert_eq!((2*ma).entries, vec![2, 2, 2, 2]);
}

#[test]
fn test_matrix_multiplication_associativity() {
  let ma = Matrix::new(2, 3, vec![
    2, 3, 4,
    1, 0, 0]);
  let mb = Matrix::new(3, 2, vec![
    0, 1000,
    1, 100,
    0, 10]);
  let mc = Matrix::new(2, 3, vec![
    5, 6, 7,
    1, 0, 0]);
  // 2x3 * 3x2 * 2x3
  // (2x3 * 3x2) = (2x2 * 2x3) = 2x3
  // 2x3 * (3x2 * 2x3) = 2x3 * 3x3 = 2x3
  assert_eq!((ma.to_owned()*mb.to_owned())*mc.to_owned(), ma*(mb*mc));
}

#[test]
fn test_matrix_left_distributivity() {
  /*
  let ma = Matrix::new(2, 3, vec![
    2, 3, 4,
    1, 0, 0]);
  let mb = Matrix::new(3, 2, vec![
    0, 1000, 1,
    1, 100, 2]);
  let mc = Matrix::new(2, 3, vec![
    5, 6, 7,
    1, 0, 0]);
  assert_eq!(&mc*&(&ma+&mb), (&mc*&ma) + (&mc*&mb));
  */
}

#[test]
fn test_matrix_is_square() {
  let ma = Matrix::new(2, 3, vec![
    2, 3, 4,
    1, 0, 0]);
  assert_eq!(ma.is_square(), false);

  let mb = Matrix::new(2, 2, vec![0, 1, 2, 3]);
  assert_eq!(mb.is_square(), true);
}

#[test]
fn test_matrix_is_invertible() {
  let m = Matrix::new(3, 3, vec![
    1,3,2,
    4,1,3,
    2,5,2]);
  assert_eq!(m.is_invertible(), true);
}

#[test]
fn test_transpose_matrix() {
  /// 1 2 3 = 1 4
  /// 4 5 6   2 5
  ///         3 6
  let ma = Matrix::new(2, 3, vec![
    1, 2, 3,
    4, 5, 6]);
  let mb = Matrix::new(3, 2, vec![
    1, 4,
    2, 5,
    3, 6]);
  assert_eq!(ma.to_owned().transpose(), mb);
  assert_eq!(mb.to_owned().transpose(), ma);
}

#[test]
fn test_diagonal_matrix() {
  let m = Matrix::new(3, 3, vec![
    1,2,3,
    4,5,6,
    7,8,9]);
  assert_eq!(m.diagonal().entries, vec![
    1,0,0,
    0,5,0,
    0,0,9]);
  assert_eq!(m.diagonal().entries, m.right_diagonal().entries);
  assert_eq!(m.diagonal(), m.diagonal());
}

#[test]
fn test_left_diagonal_matrix() {
  let m = Matrix::new(3, 3, vec![
    1,2,3,
    4,5,6,
    7,8,9]);
  assert_eq!(m.left_diagonal().entries, vec![
    0,0,3,
    0,5,0,
    7,0,0]);
  assert_eq!(m.left_diagonal().entries, m.left_diagonal().entries);
  assert_eq!(m.left_diagonal(), m.left_diagonal());
}

#[test]
fn test_det_matrix() {
  let m_1x1 = Matrix::new(1, 1, vec![1]);
  assert_eq!(m_1x1.det(), 1);

  let m_2x2 = Matrix::new(2, 2, vec![
    11,8,
     3,7]);
  assert_eq!(m_2x2.det(), 53);

  let m_3x3 = Matrix::new(3, 3, vec![
    1,3,2,
    4,1,3,
    2,5,2]);
  assert_eq!(m_3x3.det(), 17);

  let m_3x3 = Matrix::new(3, 3, vec![
    6,  1, 1, 
    4, -2, 5,
    2,  8, 7]);
  assert_eq!(m_3x3.det(), -306);

  let m_3x3 = Matrix::new(3, 3, vec![
    10,05,12,
    03,14,21,
    08,09,11]);
  assert_eq!(m_3x3.det().modulo(26), 7);

  let m_4x4 = Matrix::new(4, 4, vec![
    3,2,0,1,
    4,0,1,2,
    3,0,2,1,
    9,2,3,1]);
  assert_eq!(m_4x4.det(), 24);
}

#[test]
fn test_det_matrix_multiplication_rule() {
  let m_2x2 = Matrix::new(3, 3, vec![
    11,8,9,
     3,7,5,
     2,5,8]);

  let m_3x2 = Matrix::new(3, 3, vec![
    1,3,6,
    4,1,1,
    2,5,7]);

  assert_eq!(m_3x2.det()*m_2x2.det(), (m_3x2*m_2x2).det());
}

#[test]
fn test_det_of_identity_matrix() {
  let m = Matrix::new(3, 3, vec![
    1,3,6,
    4,1,1,
    2,5,7]);
  assert_eq!(m.identity().det(), 1);
}

#[test]
fn test_fill_matrix() {
  assert_eq!(fill_matrix(2, 2, 0), Matrix::new(2, 2, vec![0,0,0,0]));
  assert_eq!(zero_fill_matrix(2, 2), Matrix::new(2, 2, vec![0,0,0,0]));
  assert_eq!(one_fill_matrix(2, 2), Matrix::new(2, 2, vec![1,1,1,1]));
}

#[test]
fn test_submatrix() {
  let m = Matrix::new(3, 4, vec![
    1,2, 3, 4,
    5,6, 7, 8,
    9,10,11,12]);
  let sub = Matrix::new(2, 3, vec![
    1,3,4,
    5,7,8]);
  assert_eq!(m.submatrix(3, 2), sub);
}

#[test]
fn test_minor_matrix() {
  let m = Matrix::new(3, 3, vec![
    1, 4, 7,
    3, 0, 5,
   -1, 9, 11]);
  assert_eq!(m.minor(2, 3), 13);
}

#[test]
fn test_cofactor_matrix() {
  let m = Matrix::new(3, 3, vec![
    1, 4, 7,
    3, 0, 5,
   -1, 9, 11]);
  assert_eq!(m.cofactor(2, 3), -13);
}

#[test]
fn test_minors_matrix() {
  let m = Matrix::new(3, 3, vec![
    1,3,2,
    4,1,3,
    2,5,2]);
  assert_eq!(m.minors().entries, vec![
    -13,2,18,
    -4,-2,-1,
    7,-5,-11]);
}

#[test]
fn test_cofactors_matrix() {
  let m = Matrix::new(3, 3, vec![
    1,3,2,
    4,1,3,
    2,5,2]);
  assert_eq!(m.cofactors().entries, vec![
    -13,-2,18,
    4,-2,1,
    7,5,-11]);
}

#[test]
fn test_adjugate_matrix() {
  let m = Matrix::new(3, 3, vec![
    1,3,2,
    4,1,3,
    2,5,2]);
  assert_eq!(m.adjugate().entries, vec![
    -13,4,7,
    -2,-2,5,
    18,1,-11]);
}

#[test]
fn test_adjugate_matrix_law_1() {
  let m = Matrix::new(3, 3, vec![
    1,3,2,
    4,1,3,
    2,5,2]);
  assert_eq!(&m * &m.adjugate(), m.det() * m.identity());
}

#[test]
fn test_adjugate_matrix_law_identity() {
  let m = Matrix::new(3, 3, vec![
    1,3,2,
    4,1,3,
    2,5,2]);
  assert_eq!(m.identity().adjugate(), m.identity());
}

#[test]
fn test_adjugate_matrix_law_multiplication() {
  let ma = Matrix::new(3, 3, vec![
    9,1,5,
    4,6,4,
    1,2,3]);
  let mb = Matrix::new(3, 3, vec![
    1,3,2,
    4,1,3,
    2,5,2]);
  // adj(AB) = adj(B)*adj(A) -- NOT adj(A)*adj(B)
  assert_eq!((&ma * &mb).adjugate(), mb.adjugate() * ma.adjugate());
}

#[test]
fn test_inverse_matrix() {
  let m = Matrix::new(2, 2, vec![
    11,08,
    03,07]);
  assert_eq!(m.minors().entries, vec![
    07,03,
    08,11]);
  assert_eq!(m.cofactors().entries, vec![
    07,-3,
    -8,11]);
  assert_eq!(m.adjugate().entries, vec![
    07,-8,
    -3,11]);
  assert_eq!(m.inverse_unsafe().entries, vec![
    07/53,-8/53,
    -3/53,11/53]);
}

#[test]
fn test_inverse_mod_matrix() {
  let m = Matrix::new(2, 2, vec![
    11,08,
    03,07]);
  assert_eq!(m.minors().entries, vec![
    07,03,
    08,11]);
  assert_eq!(m.cofactors().entries, vec![
    07,-3,
    -8,11]);
  assert_eq!(m.adjugate().entries, vec![
    07,-8,
    -3,11]);
  assert_eq!(m.inverse_mod_unsafe(26).entries, vec![
    07,18,
    23,11]);
}

#[test]
fn test_inverse_mod_matrix_identity_inverse_law() {
  let m = Matrix::new(2, 2, vec![
    11,08,
    03,07]);
  let m_inv = Matrix::new(2, 2, vec![
    7,18,
    23,11]);
  assert_eq!(&m * &m_inv, Matrix::new(2, 2, vec![
    261,286,
    182,131]));

  let v: Vec<isize> = (&m * &m_inv).entries.into_iter().map(|x| x.modulo(26)).collect();
  assert_eq!(v, identity_matrix(2, 2).entries);

  assert_eq!((&m * &m_inv).map(|x| x.modulo(26)), identity_matrix(2, 2));
}

#[test]
fn test_matrix_misc_pow() {
  assert_eq!((-1).pow(1), -1);
  assert_eq!((-1).pow(2), 1);
  assert_eq!((-1).pow(3), -1);
  assert_eq!((-1).pow(4), 1);

  let i: isize = 2;
  assert_eq!((-1).pow(i as u32), 1);
}

#[test]
fn test_matrix_misc_isize_coerce() {
  let i: isize = 1;
  let j: isize = 1;
  assert_eq!(i as u32, 1);
  assert_eq!((i + j) as u32, 2);
}