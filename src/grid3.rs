use std::ops::{Index, IndexMut, Range};
// use std::vec;





use crate::{double_sided_vec::*, double_vec};


#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone)]
pub struct Grid3<C> {
    pub size_x : usize,
    pub size_y : usize,
    pub size_z : usize,
    pub center_x : usize,
    pub center_y : usize,
    pub center_z : usize,
    grid : DoubleSidedVec<DoubleSidedVec<DoubleSidedVec<C>>>,
}

impl<C> Grid3<C> {
    pub fn new(size_x : usize, size_y : usize, size_z : usize, default : C) -> Self
        where C : Clone
    {
        Self {
            size_x,
            size_y,
            size_z,
            center_x : size_x / 2,
            center_y : size_y / 2,
            center_z : size_z / 2,
            grid : double_vec![double_vec![double_vec![default; size_z]; size_y]; size_x],
        }
    }

    pub fn new_center(size_x : usize, size_y : usize, size_z : usize, center_x : usize, center_y : usize, center_z : usize, default : C) -> Self
        where C : Clone
    {
        Self {
            size_x,
            size_y,
            size_z,
            center_x,
            center_y,
            center_z,
            grid : double_vec![double_vec![double_vec![default; size_z; center_z]; size_y; center_y]; size_x; center_x],
        }
    }

    pub fn from_elem(default : C, count : usize) -> Self
        where C : Clone
    {
        Self {
            size_x : count,
            size_y : count,
            size_z : count,
            center_x : count / 2,
            center_y : count / 2,
            center_z : count / 2,
            grid : double_vec![double_vec![double_vec![default; count]; count]; count],
        }
    }

    pub fn set_center_x(&mut self, center : usize) {
        self.center_x = center;
        self.grid.set_center(center);
    }

    pub fn set_center_y(&mut self, center : usize) {
        self.center_z = center;
        self.grid.iter_mut().for_each(|dv| dv.set_center(center));
    }

    pub fn expand_neg_x(&mut self, amount : usize)
        where C : Clone + Default
    {
        for _ in 0..amount {

            self.grid.push_back(double_vec![double_vec![C::default(); self.size_z; self.center_z]; self.size_y; self.center_y]);
        }
        self.size_x += amount;
        self.center_x += amount;
    }

    pub fn expand_pos_x(&mut self, amount : usize)
        where C : Clone + Default
    {
        for _ in 0..amount {
            self.grid.push_front(double_vec![double_vec![C::default(); self.size_z; self.center_z]; self.size_y; self.center_y]);
        }
    }

    pub fn expand_neg_y(&mut self, amount : usize)
        where C : Clone + Default
    {

        for dv in self.grid.iter_mut() {
            for _ in 0..amount {
                dv.push_back(double_vec![C::default(); self.size_z; self.center_z]);
            }
        }
        self.size_y += amount;
        self.center_y += amount;
    }

    pub fn expand_pos_y(&mut self, amount : usize)
        where C : Clone + Default
    {
        for dv in self.grid.iter_mut() {
            for _ in 0..amount {
                dv.push_front(double_vec![C::default(); self.size_z; self.center_z]);
            }
        }
    }

    pub fn expand_neg_z(&mut self, amount : usize)
        where C : Clone + Default
    {

        for dv in self.grid.iter_mut() {
            for dv2 in dv.iter_mut() {
                for _ in 0..amount {
                    dv2.push_back(C::default());
                }
            }
        }
        self.size_z += amount;
        self.center_z += amount;
    }

    pub fn expand_pos_z(&mut self, amount : usize)
        where C : Clone + Default
    {
        for dv in self.grid.iter_mut() {
            for dv2 in dv.iter_mut() {
                for _ in 0..amount {
                    dv2.push_front(C::default());
                }
            }
        }
    }

    pub fn move_cell(&mut self, locations : ((isize, isize, isize), (isize, isize, isize)))
        where C : Clone + Default
    {
        self.grid[locations.1.0][locations.1.1][locations.1.2] = self.grid[locations.0.0][locations.0.1][locations.0.2].clone();
        self.grid[locations.0.0][locations.0.1][locations.0.2] = C::default();
    }

    pub fn swap_cells(&mut self, locations : ((isize, isize, isize), (isize, isize, isize)))
        where C : Clone
    {
        let temp = self.grid[locations.1.0][locations.1.1][locations.1.2].clone();
        self.grid[locations.1.0][locations.1.1][locations.1.2] = self.grid[locations.0.0][locations.0.1][locations.0.2].clone();
        self.grid[locations.0.0][locations.0.1][locations.0.2] = temp;
    }

    pub fn grid(&self) -> &DoubleSidedVec<DoubleSidedVec<DoubleSidedVec<C>>> {
        &self.grid
    }

    ///Do **not** modify the size of the grid directly unless you really know what you're doing.
    pub fn grid_mut(&mut self) -> &DoubleSidedVec<DoubleSidedVec<DoubleSidedVec<C>>> {
        &self.grid
    }

    pub fn get(&self, location : (isize, isize, isize)) -> Option<&C> {
        self.grid.get(location.0).and_then(|f| f.get(location.1).and_then(|f| f.get(location.2)))
    }

    pub fn cells(&self) -> Vec<&C> {
        self.grid.iter().flat_map(|f| f.iter().flat_map(|f| f.iter())).collect::<Vec<&C>>()
    }

    pub fn cells_mut(&mut self) -> Vec<&mut C> {
        self.grid.iter_mut().flat_map(|f| f.iter_mut().flat_map(|f| f.iter_mut())).collect::<Vec<&mut C>>()
    }

    pub fn lens_x(&self) -> (isize, isize) {
        self.grid.lens()
    }

    pub fn range_x(&self) -> Range<isize> {
        self.grid.range()
    }

    pub fn lens_y(&self) -> (isize, isize) {
        if let Some(y) = self.grid.get(0) {
            y.lens()
        } else {
            (0, 0)
        }
    }

    pub fn range_y(&self) -> Range<isize> {
        if let Some(y) = self.grid.get(0) {
            y.range()
        } else {
            0..0
        }
    }

    pub fn lens_z(&self) -> (isize, isize) {
        if let Some(y) = self.grid.get(0) {
            if let Some(z) = y.get(0) {
                z.lens()
            } else {
                (0, 0)
            }
        } else {
            (0, 0)
        }
    }

    pub fn range_z(&self) -> Range<isize> {
        if let Some(y) = self.grid.get(0) {
            if let Some(z) = y.get(0) {
                z.range()
            } else {
                0..0
            }
        } else {
            0..0
        }
    }
}

// impl<C> Index<isize> for CenteredGrid<C> {
//     type Output = DoubleSidedVec<C>;
//     fn index(&self, index: isize) -> &Self::Output {
//         &self.grid[index]
//     }
// }

// impl<C> IndexMut<isize> for CenteredGrid<C> {
//     fn index_mut(&mut self, index: isize) -> &mut Self::Output {
//         &mut self.grid[index]
//     }
// }

impl<C> Index<(isize, isize, isize)> for Grid3<C> {
    type Output = C;
    fn index(&self, index: (isize, isize, isize)) -> &Self::Output {
        &self.grid[index.0][index.1][index.2]
    }
}

impl<C> IndexMut<(isize, isize, isize)> for Grid3<C> {
    fn index_mut(&mut self, index: (isize, isize, isize)) -> &mut Self::Output {
        &mut self.grid[index.0][index.1][index.2]
    }
}

// #[cfg(feature = "index_usize")]
// impl<C> Index<usize> for CenteredGrid<C> {
//     type Output = DoubleSidedVec<C>;
//     fn index(&self, index: usize) -> &Self::Output {
//         &self.grid[index]
//     }
// }

// #[cfg(feature = "index_usize")]
// impl<C> Index<usize> for CenteredGrid<C> {
//     type Output = DoubleSidedVec<C>;
//     fn index(&self, index: usize) -> &Self::Output {
//         &self.grid[index]
//     }
// }

#[cfg(feature = "index_usize")]
impl<C> Index<(usize, usize, usize)> for Grid3<C> {
    type Output = C;
    fn index(&self, index: (usize, usize, usize)) -> &Self::Output {
        &self.grid[index.0][index.1][index.2]
    }
}

#[cfg(feature = "index_usize")]
impl<C> IndexMut<(usize, usize, usize)> for Grid3<C> {
    fn index_mut(&mut self, index: (usize, usize, usize)) -> &mut Self::Output {
        &mut self.grid[index.0][index.1][index.2]
    }
}