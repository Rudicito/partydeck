use crate::app::PartyConfig;
use crate::util::get_screen_resolution;
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
        let ratio = resolution.width as f32 / resolution.height as f32;

        let player_count_f = player_count as f32;
        let cols_f = (player_count_f * ratio).sqrt().ceil();
        let rows_f = (player_count_f / cols_f).ceil();

        let cols = cols_f as u32;
        let rows = rows_f as u32;
        
        Grid { rows, cols, res: resolution }
    }
}