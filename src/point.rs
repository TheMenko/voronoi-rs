#[derive(Debug, PartialEq, Eq)]
pub struct Site<T> where T: Copy {
    pub coord_x: T,
    pub coord_y: T,
}

impl<T> Site<T> where T: Copy {
    pub fn new(coord_x: T,coord_y: T) -> Site<T> {
        Self {
            coord_x,
            coord_y
        }
    }
    
    pub fn from_vec(tparr: Vec<(T, T)>) -> Vec<Site<T>> {
        tparr.iter().map(|k| Site { coord_x: k.0, coord_y: k.1}).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Site;
    
    #[test]
    fn int_points() {
        let nums = vec![(1,2), (3,4), (5,6)];
        let points: Vec<Site<u32>> = Site::from_vec(nums);
        assert_eq!(points, vec![Site{ coord_x: 1, coord_y: 2 }, Site{ coord_x: 3, coord_y: 4 }, Site{ coord_x: 5, coord_y: 6 }]);
    }
    
    #[test]
    fn float_points() {
        let nums = vec![(1.,2.), (3.,4.), (5.,6.)];
        let points: Vec<Site<f64>> = Site::from_vec(nums);
        assert_eq!(points, vec![Site{ coord_x: 1., coord_y: 2. }, Site{ coord_x: 3., coord_y: 4. }, Site{ coord_x: 5., coord_y: 6. }]);
    }
}