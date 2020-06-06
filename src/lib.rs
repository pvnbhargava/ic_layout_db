/// The size of each point in a GDS is 4 bytes, so each coordinate of the rect is an i32
#[derive(Debug)]
struct Rectangle {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

/// TODO: Should we be explicit that the first point must be the lower-left?
/// TODO: Add input validation procedure?
impl Rectangle {
    /// Creates a rectangle from the raw x and y numbers
    /// Currently assumes that the first x,y pair is the lower-left corner and that the second
    /// x,y pair is the upper-right corner
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        return Rectangle {
            x1,
            y1,
            x2,
            y2,
        };
    }

    /// Generates the rectangle from a pair of points.
    pub fn from_point() -> Self {
        unimplemented!()
    }

    /// TODO: Should this really be an i64?
    /// Currently assumes that the first x,y pair is the lower-left corner and that the second
    /// x,y pair is the upper-right corner
    pub fn area(&self) -> i32 {
        self.width() * self.height()
    }

    /// Returns lower-left coord
    pub fn ll(&self) -> (i32, i32){
        (self.x1, self.y1)
    }

    /// Returns upper-right coord
    pub fn ur(&self) -> (i32, i32){
        (self.x2, self.y2)
    }

    pub fn width(&self) -> i32 {
        self.x2 - self.x1
    }

    pub fn height(&self) -> i32 {
        self.y2 - self.y1
    }

    pub fn contains_point(&self) -> bool {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Rectangle};

    #[test]
    fn make_basic_rect() {
        let new_rect = Rectangle::new(
            0,
            0,
            5,
            5,
        );
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn area() {
        let new_rect = Rectangle::new(
            0,
            0,
            5,
            5,
        );
        assert_eq!(new_rect.area(), 25);
    }

    #[test]
    fn width() {
        let new_rect = Rectangle::new(
            0,
            0,
            5,
            5,
        );
        assert_eq!(new_rect.width(), 5);
    }

    #[test]
    fn height() {
        let new_rect = Rectangle::new(
            0,
            0,
            5,
            5,
        );
        assert_eq!(new_rect.height(), 5);
    }

}
