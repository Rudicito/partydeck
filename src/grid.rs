use crate::util::Resolution;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GridPosition{
    pub row: u32,
    pub col: u32,
}

#[derive(Clone)]
pub struct Grid {
    pub rows: u32,
    pub cols: u32,
    pub res: Resolution,
}

impl Grid {
    pub fn cells_number(&self) -> u32 {
        self.rows * self.cols
    }
    
    pub fn single_cell_res(&self) -> Resolution {
        Resolution {
            width: self.res.width/self.cols, 
            height: self.res.height/self.rows
        }
    }
    
    pub fn new(player_count: u32, resolution: Resolution) -> Grid {
        let player_count_f = player_count as f32;
        let rows_f = player_count_f.sqrt().ceil();
        let cols_f = (player_count_f / rows_f).ceil();

        let rows = rows_f as u32;
        let cols = cols_f as u32;
        
        Grid { rows, cols, res: resolution }
    }
}