use crate::point::Point;
use crate::error::PlaneError;

use rayon::prelude::*;

type Result<T> = std::result::Result<T, PlaneError>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Plane {
    pub width: u32,
    pub height: u32,
    pub points: Option<Vec<Point>>
}

impl Plane {
    pub fn new(width: u32, height: u32) -> Plane {
        Self {
            width,
            height,
            points: None,
        }
    }
    pub fn with_points(&mut self, points: Vec<Point>) -> Result<()> {
        if points.len() < 2 {
            return Err(PlaneError::TooFewPoints)
        }
        self.points = Some(points);
        Ok(())
    }
    
    pub fn get_corners(&self) -> [(Point, u8); 4] {
        [
            (Point::new(0., 0.), 1u8), 
            (Point::new(self.width as f64, 0.), 2u8), 
            (Point::new(self.width as f64, self.height as f64), 3u8), 
            (Point::new(0., self.height as f64), 4u8)
        ]
    }
    
    pub fn get_subsets_vertical(&self) -> Result<(Vec<Point>, Vec<Point>)> {
        if let Some(point) = self.points.clone() {
            let (left, right): (Vec<Point>, Vec<Point>) = point.into_iter().partition(|p| p.coord_x <= (self.width / 2) as f64);
            
            Ok((left, right))
        } else {
            Err(PlaneError::PointsNotProvided)
        }
    }
    pub fn get_subsets_horizontal(&self) -> Result<(Vec<Point>, Vec<Point>)> {
        if let Some(point) = self.points.clone() {
            let (left, right): (Vec<Point>, Vec<Point>) = point.into_iter().partition(|p| p.coord_y <= (self.height / 2) as f64);
            
            Ok((left, right))
        } else {
            Err(PlaneError::PointsNotProvided)
        }
    }
    pub fn sort_by_apsci(&mut self) {
        self.points.as_mut().unwrap().sort_by(|a,b| { a.partial_cmp(b).unwrap() });
    }
    pub fn calc_for_corners(&self) -> Result<Vec<(u8, Point)>> {
        
        let points: Vec<Point> = match self.points.clone() {
            Some(p) => p,
            None => { return Err(PlaneError::PointsNotProvided) }
        };
        
        let calc_dist_to_corner = |corner: Point, points: &Vec<Point>| -> Point {
            points.iter().reduce(|p,k| if p.euclidean_dist(corner) < k.euclidean_dist(corner) { p } else { k }).unwrap().to_owned()
        };
        
        let corners = self.get_corners();
            
        let mut closest_points: Vec<(u8, Point)> = corners.par_iter().map(|i| (i.1, calc_dist_to_corner(i.0, &points))).collect();
        
        
        closest_points.sort_by(|k,p| k.0.partial_cmp(&p.0).unwrap());
        Ok(closest_points)
        
    }
    #[allow(unused_variables)]
    pub fn merge(&mut self, other: Plane) -> Result<Plane> {
        Ok(Plane::new(0, 0))
    }
}

#[cfg(test)]
mod tests {
    use crate::error::PlaneError;

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
        plane.with_points(points)?;
        
        let (left, right) = plane.clone().get_subsets_vertical()?;
        let (top, bottom) = plane.get_subsets_horizontal()?;
        
        assert_eq!(left, vec![Point { coord_x: 4., coord_y: 4. }, Point { coord_x: 3., coord_y: 3. }]);
        assert_eq!(right, vec![Point { coord_x: 7., coord_y: 8. }]);
        assert_eq!(top, vec![Point { coord_x: 4., coord_y: 4. }, Point { coord_x: 3., coord_y: 3. }]);
        assert_eq!(bottom, vec![Point { coord_x: 7., coord_y: 8. }]);
        Ok(())
    }
    
    #[test]
    fn test_sort() -> Result<(), String> {
        let points: Vec<Point> = Point::from_vec(vec![(4.,4.), (3.,3.), (7.,8.), (2.,1.), (2.,9.)]).unwrap();
        let mut plane: Plane = Plane::new(10, 10);
        plane.with_points(points)?;
        plane.sort_by_apsci();
        
        let closest = Point { coord_x: 2., coord_y: 1. };
        
        if plane.points.unwrap().into_iter().nth(0).unwrap() == closest {
            Ok(())
        } else {
            Err(PlaneError::SortedIncorrectly.into())
        }
    }
}