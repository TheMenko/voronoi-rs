#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Point {
    pub coord_x: f64,
    pub coord_y: f64,
}

impl Point {
    pub fn new(coord_x: f64,coord_y: f64) -> Point {
        Self {
            coord_x,
            coord_y
        }
    }
    
    pub fn from_vec(tparr: Vec<(f64, f64)>) -> Vec<Point> {
        tparr.iter().map(|k| Point { coord_x: k.0, coord_y: k.1}).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Point;
    
    #[test]
    fn float_points() {
        let nums = vec![(1.,2.), (3.,4.), (5.,6.)];
        let points: Vec<Point> = Point::from_vec(nums);
        assert_eq!(points, vec![Point{ coord_x: 1., coord_y: 2. }, Point{ coord_x: 3., coord_y: 4. }, Point{ coord_x: 5., coord_y: 6. }]);
    }
}