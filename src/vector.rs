use std::ops::{Mul, Div, Add, Sub, AddAssign, SubAssign, DivAssign, MulAssign};

pub(crate) struct Vector;

// pub type VecDimValue = Sized + Copy + Clone + AddAssign + SubAssign + MulAssign + DivAssign;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct VecDim<const NDIM: usize>
where [f64; NDIM]: Sized
{
    coords: [f64; NDIM]
}

impl Vector {
    pub fn new<const N: usize>(coords: [f64; N]) -> VecDim<N> {
        VecDim { coords }
    }
}

impl<const N: usize> Mul for VecDim<N> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut vec = self.clone();
        for i in 0..N {
            vec.coords[i] *= rhs.coords[i];
        }
        vec
    }
}

impl<const N: usize> Div for VecDim<N> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let mut vec = self.clone();
        for i in 0..N {
            vec.coords[i] /= rhs.coords[i];
        }
        vec
    }
}

impl<const N: usize> Add for VecDim<N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut vec = self.clone();
        for i in 0..N {
            vec.coords[i] += rhs.coords[i];
        }
        vec
    }
}

impl<const N: usize> Sub for VecDim<N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut vec = self.clone();
        for i in 0..N {
            vec.coords[i] -= rhs.coords[i];
        }
        vec
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_new_ops() {
        let vec1 = Vector::new([5.0, 4.0, 3.0, 2.0, 1.0]);
        let vec2 = Vector::new([1.0, 2.0, 3.0, 4.0, 5.0]);

        assert_eq!((vec1.clone() + vec2.clone()).coords, [6.0, 6.0, 6.0, 6.0, 6.0]);
        assert_eq!((vec1.clone() - vec2.clone()).coords, [4.0, 2.0, 0.0, -2.0, -4.0]);
        assert_eq!((vec1.clone() * vec2.clone()).coords, [5.0, 8.0, 9.0, 8.0, 5.0]);
        assert_eq!((vec1.clone() / vec2.clone()).coords, [5.0, 2.0, 1.0, 0.5, 0.2]);
    }

    #[test]
    fn vec_of_vec() {
        let points = [Vector::new([5.0, 4.0]); 10];
        let sum_points = points.iter().fold(Vector::new([0.0, 0.0]), |acc, v| acc + v.to_owned()).coords;
        assert_eq!(sum_points, [50.0, 40.0]);
    }

    #[test]
    fn vec_eq() {
        let vec1 = Vector::new([5.0, 4.0, 3.0]);
        let vec2 = Vector::new([5.0, 4.0, 3.0]);
        assert!(vec1 == vec2);
    }

    #[test]
    fn vec_ne() {
        let vec1 = Vector::new([5.0, 4.0, 3.0]);
        let vec2 = Vector::new([5.0, 2.0, 3.0]);
        assert!(vec1 != vec2);
    }
}