
use crate::error::PointError;

type Result<T> = std::result::Result<T, PointError>;

#[derive(PartialEq, Debug, Clone, Copy, PartialOrd)]
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
    
    pub fn from_vec(tparr: Vec<(f64, f64)>) -> Result<Vec<Point>> {
        if tparr.len() == 0 {
            return Err(PointError::Empty)
        }
        Ok(tparr.iter().map(|k| Point { coord_x: k.0, coord_y: k.1}).collect())
    }
    
    pub fn from_tpl(tpl: (f64, f64)) -> Point {
        Point::new(tpl.0, tpl.1)
    }
    
    pub fn euclidean_dist(&self, other: Point) -> f64 {
        f64::sqrt(f64::powi(other.coord_x - self.coord_x, 2) + f64::powi(other.coord_y - self.coord_y, 2))
    }
}

#[cfg(test)]
mod tests {
    use super::Point;
    
    #[test]
    fn float_points() -> Result<(), String> {
        let nums = vec![(1.,2.), (3.,4.), (5.,6.)];
        let points: Vec<Point> = Point::from_vec(nums)?;
        assert_eq!(points, vec![Point { coord_x: 1., coord_y: 2. }, Point { coord_x: 3., coord_y: 4. }, Point { coord_x: 5., coord_y: 6. }]);
        Ok(())
    }

    
    #[test]
    fn check_eucl_dist() {
        let point_a = Point { coord_x: 3., coord_y: 4. };
        let point_b = Point { coord_x: 4., coord_y: 3. };
        
        assert_eq!(format!("{:.2}", point_a.euclidean_dist(point_b)), "1.41".to_string());
    }
    
}