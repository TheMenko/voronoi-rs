use crate::point::Point;
use crate::error::PlaneError;

type Result<T> = std::result::Result<T, PlaneError>;

#[derive(Debug, Clone)]
pub struct Plane {
    width: u32,
    height: u32,
    points: Option<Vec<Point>>
}

impl Plane {
    pub fn new(width: u32, height: u32) -> Plane {
        Self {
            width,
            height,
            points: None,
        }
    }
    pub fn with_points(&mut self, points: Vec<Point>) -> Result<&Plane> {
        if points.len() < 2 {
            return Err(PlaneError::TooFewPoints)
        }
        self.points = Some(points);
        Ok(self)
    }
    
    pub fn get_subsets(&self) -> Result<(Vec<Point>, Vec<Point>)> {
        if let Some(point) = self.points.clone() {
            let (left, right): (Vec<Point>, Vec<Point>) = point.into_iter().partition(|p| p.coord_x <= (self.width / 2) as f64);
            
            Ok((left, right))
        } else {
            Err(PlaneError::PointsNotProvided)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Point, Plane};
    
    #[test]
    fn make_plane() -> Result<(), String> {
        let points: Vec<Point> = Point::from_vec(vec![(1.,2.), (3.,4.), (5.,6.)]).unwrap();
        match Plane::new(10, 10).with_points(points) {
            Ok(_) => { Ok(()) }
            Err(e) => { Err(e.to_string()) }
        }
    }
    
    #[test]
    fn test_plane_subsets() -> Result<(), String> {
        let points: Vec<Point> = Point::from_vec(vec![(4.,4.), (3.,3.), (7.,8.)]).unwrap();
        let mut plane: Plane = Plane::new(10, 10);
        let plane = plane.with_points(points)?;
        
        let (left, right) = plane.get_subsets()?;
        
        assert_eq!(left, vec![Point { coord_x: 4., coord_y: 4. }, Point { coord_x: 3., coord_y: 3. }]);
        assert_eq!(right, vec![Point { coord_x: 7., coord_y: 8. }]);
        Ok(())
    }
}