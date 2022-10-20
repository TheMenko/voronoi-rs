#![allow(unused_variables)]

use crate::plane::Plane;
use crate::point::Point;
use crate::error::VoronoiError;

use rayon::prelude::*;

type Result<T> = std::result::Result<T, VoronoiError>;

pub struct Voronoi;

impl Voronoi {
    pub fn from_plane(plane: Plane) -> Voronoi {
        Self{}
    }
}

pub struct VoronoiBuilder {
    plane: Plane
}

impl VoronoiBuilder {
    pub fn new(width: u32, height: u32, points: Vec<(f64, f64)>) -> Result<VoronoiBuilder> {
        
        let mut new_plane = Plane::new(width, height);
        match new_plane.with_points(match Point::from_vec(points) {
            Ok(p) => p,
            Err(e) => { return Err(VoronoiError::Error(String::from("failed at creating voronoi instance"),e.into())); }
        }) {
            Ok(_) => (),
            Err(e) => { return Err(VoronoiError::Error(String::from("failed at creating voronoi instance"),e.into())); }
        }
        
        Ok(Self {
            plane: new_plane
        })
    }
    
    fn recursive_calc(&self, points: Vec<Point>) -> Result<Plane> {
        
        Ok(Plane::new(1, 1))
    }
    
    fn calc_for_corners(&self) -> Result<Vec<(u8, Point)>> {
        
        let points: Vec<Point> = match self.plane.points.clone() {
            Some(p) => p,
            None => { return Err(VoronoiError::EmptyPlanePoints) }
        };
        
        let calc_dist_to_corner = |corner: Point, points: &Vec<Point>| -> Point {
            points.iter().reduce(|p,k| if p.euclidean_dist(corner) < k.euclidean_dist(corner) { p } else { k }).unwrap().to_owned()
        };
        
        let corners = self.plane.get_corners();
            
        let mut closest_points: Vec<(u8, Point)> = corners.par_iter().map(|i| (i.1, calc_dist_to_corner(i.0, &points))).collect();
        
        //todo: if all corners are closest to the same point, its safe to draw out/mark sites in the plane.
        closest_points.sort_by(|k,p| k.0.partial_cmp(&p.0).unwrap());
        Ok(closest_points)
        
    }
    
    pub fn build(&mut self) -> Result<Voronoi> {
        
        // sort points
        self.plane.sort_by_apsci();
        // split on two so we can later merge them when voronoi is calculated for both
        let (left, right) = match self.plane.get_subsets_vertical() {
            Ok(lr) => lr,
            Err(e) => { return Err(VoronoiError::Error(String::from("varanoi error: "),e.into())); }
        };
        
        let mut left_plane = self.recursive_calc(left)?;
        let right_plane = self.recursive_calc(right)?;
        
        let voronoi_plane = left_plane.merge(right_plane).unwrap();
        
        Ok(Voronoi::from_plane(voronoi_plane))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calc_edges() -> Result<()> {
        let points = vec![(1., 1.), (7., 3.), (6., 6.), (1., 9.)];
        let vb = VoronoiBuilder::new(10, 10, points.clone())?;
        
        let closest_points = vb.calc_for_corners()?;
        
        let a: Vec<()> = closest_points.iter().map(|p| println!("{:?}", p)).collect();
        
        assert_eq!(closest_points.get(0).unwrap(), &(1u8, Point::from_tpl(points[0])));
        assert_eq!(closest_points.get(1).unwrap(), &(2u8, Point::from_tpl(points[1])));
        assert_eq!(closest_points.get(2).unwrap(), &(3u8, Point::from_tpl(points[2])));
        assert_eq!(closest_points.get(3).unwrap(), &(4u8, Point::from_tpl(points[3])));
        Ok(())
    }  
}