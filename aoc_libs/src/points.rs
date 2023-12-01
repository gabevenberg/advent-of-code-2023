#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    /// converts a point (representing a point on a 4 quadrant grid with positive xy in the
    /// top-right) into a upoint (representing a point on a 1 quadrant grid with the origin in the
    /// top-left corner). Returns none if the resulting point would have either number negative.
    pub fn to_upoint(self, zero_coord: &UPoint) -> Option<UPoint> {
        Some(UPoint {
            x: zero_coord.x.checked_add_signed(self.x)?,
            y: zero_coord.y.checked_add_signed(-self.y)?,
        })
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl std::ops::Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}

/// an unsigned point in 2d space
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UPoint {
    pub x: usize,
    pub y: usize,
}

impl UPoint {
    /// converts a upoint (representing a point on a 1 quadrant grid with the origin in the
    /// top-left corner) into a point( representing a point on a 4 quadrant grid with positive xy
    /// in the top-right)
    pub fn to_point(self, zero_coord: &UPoint) -> Point {
        Point {
            x: -(zero_coord.x as isize - self.x as isize),
            y: zero_coord.y as isize - self.y as isize,
        }
    }
}

impl std::ops::Add for UPoint {
    type Output = UPoint;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign for UPoint {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl std::ops::Sub for UPoint {
    type Output = UPoint;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::SubAssign for UPoint {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

/// A matrix that allows negative co-oordinates. Will panic if referencing out of bounds, just like
/// a normal 2d array.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FourQuadrantMatrix<const X: usize, const Y: usize, T> {
    matrix: [[T; X]; Y],
    max_point: Point,
    min_point: Point,
    zero_coord: UPoint,
}

impl<const X: usize, const Y: usize, T> FourQuadrantMatrix<{ X }, { Y }, T>
where
    T: Copy,
    T: Default,
{
    /// generates a new FourQuadrantMatrix with a given zero point (the point in the underlying 2d
    /// array considered to be (0,0))
    pub fn new(zero_coord: UPoint) -> FourQuadrantMatrix<{ X }, { Y }, T> {
        FourQuadrantMatrix {
            matrix: [[T::default(); X]; Y],
            max_point: UPoint { x: X - 1, y: 0 }.to_point(&zero_coord),
            min_point: UPoint { x: 0, y: Y - 1 }.to_point(&zero_coord),
            zero_coord,
        }
    }

    pub fn zero_coord(&self) -> UPoint {
        self.zero_coord
    }

    pub fn min_point(&self) -> Point {
        self.min_point
    }

    pub fn max_point(&self) -> Point {
        self.max_point
    }

    /// makes sure a point is in bounds and if not, brings it in bounds.
    pub fn bound_point(&self, point: &mut Point) {
        if point.x > self.max_point.x {
            point.x = self.max_point.x
        }

        if point.y > self.max_point.y {
            point.y = self.max_point.y
        }

        if point.x < self.min_point.x {
            point.x = self.min_point.x
        }

        if point.y < self.min_point.y {
            point.y = self.min_point.y
        }
    }

    /// checks if the point is in bounds.
    pub fn is_in_bounds(&self, point: &Point) -> bool {
        point.x <= self.max_point.x
            && point.y <= self.max_point.y
            && point.x >= self.min_point.x
            && point.y >= self.min_point.y
    }
    /// fills the matrix with the Ts default value.
    pub fn reset_matrix(&mut self) {
        self.matrix = [[T::default(); X]; Y];
    }
}

impl<T, const X: usize, const Y: usize> std::ops::IndexMut<Point> for FourQuadrantMatrix<{ X }, { Y }, T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        let upoint = index
            .to_upoint(&self.zero_coord)
            .expect("would result in negative unsigned coordinate!");
        &mut self.matrix[upoint.y][upoint.x]
    }
}

impl<T, const X: usize, const Y: usize> std::ops::Index<Point> for FourQuadrantMatrix<{ X }, { Y }, T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        let upoint = index
            .to_upoint(&self.zero_coord)
            .expect("would result in negative unsigned coordinate!");
        &self.matrix[upoint.y][upoint.x]
    }
}

impl<T, const X: usize, const Y: usize> From<FourQuadrantMatrix<{ X }, { Y }, T>> for [[T; X]; Y] {
    fn from(value: FourQuadrantMatrix<{ X }, { Y }, T>) -> Self {
        value.matrix
    }
}
