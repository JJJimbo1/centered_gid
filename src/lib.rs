pub mod double_sided_vec;
pub mod grid2;
pub mod grid3;

pub use double_sided_vec::*;
pub use grid2::*;
pub use grid3::*;

mod tests {
    #[cfg(feature = "index_usize")]
    #[test]
    fn index() {
        use crate::{DoubleSidedVec, Grid2};
        let mut dv = double_vec![42, 14, 51, 034, 1, 6, 12];
        for i in 0..dv.len() {
            dv[i] = 100;
        }
        let ref_dv = double_vec![100, 100, 100, 100, 100, 100, 100];
        assert_eq!(dv, ref_dv);
    }
}