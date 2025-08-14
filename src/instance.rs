use std::collections::HashMap;
use crate::GUEST_NAMES;
use crate::app::{InstanceLayoutMode, PartyConfig};
use crate::util::{get_screen_resolution, Resolution};
use crate::grid::*;

#[derive(Clone)]
pub struct Instance {
    pub devices: Vec<usize>,
    pub profname: String,
    pub profselection: usize,
    pub width: u32,
    pub height: u32,
    pub gridposition: Option<GridPosition>,
}

#[derive(Clone)]
pub struct InstanceManager {
    pub items: Vec<Instance>,
    pub grid: Option<Grid>,
    pub position_map: HashMap<GridPosition, usize>,
}

impl InstanceManager {
    pub(crate) fn new() -> InstanceManager {
        InstanceManager { items: Vec::new(), grid: None, position_map: HashMap::new() }
    }

    pub(crate) fn clear(&mut self) {
        self.items.clear();
        self.grid = None;
    }

    pub fn get_row(&self, row: u32) -> Vec<&Instance> {
        if let Some(grid) = &self.grid {
            (0..grid.cols)
                .filter_map(|col| self.position_map.get(&GridPosition{ row, col })
                    .map(|&idx| &self.items[idx]))
                .collect()
        } else {
            Vec::new()
        }
    }
}

pub fn set_instance_resolutions(instance_manager: &mut InstanceManager, cfg: &PartyConfig) {
    let (basewidth, baseheight) = get_screen_resolution();
    let playercount = instance_manager.items.len() as u32;

    match cfg.instance_layout_mode {

        InstanceLayoutMode::KWin => {
            if playercount > 4 {
                panic!("Max number of player using KWin layout is 4, was {playercount}");
            }

            let mut i = 0;
            for instance in instance_manager.items.iter_mut() {
                let (mut w, mut h) = match playercount {
                    1 => (basewidth, baseheight),
                    2 => {
                        if cfg.vertical_two_player {
                            (basewidth / 2, baseheight)
                        } else {
                            (basewidth, baseheight / 2)
                        }
                    }
                    _ => (basewidth / 2, baseheight / 2),
                };
                if h < 600 && cfg.gamescope_fix_lowres {
                    let ratio = w as f32 / h as f32;
                    h = 600;
                    w = (h as f32 * ratio) as u32;
                }
                println!("Resolution for instance {}/{playercount}: {w}x{h}", i + 1);
                instance.width = w;
                instance.height = h;
                i += 1;
            }
        }

        InstanceLayoutMode::Sway => {
            instance_manager.grid = Some(Grid::new(playercount, Resolution::new(basewidth, baseheight)));

            let grid = instance_manager.grid.as_ref().unwrap();
            let cell_res = grid.single_cell_res();

            for (i, instance) in instance_manager.items.iter_mut().enumerate() {
                instance.width = cell_res.width;
                instance.height = cell_res.height;
                instance.gridposition = Some(GridPosition {
                    row: i as u32 / grid.cols,
                    col: i as u32 % grid.cols,
                });
                
                instance_manager.position_map.insert(instance.gridposition.unwrap(), i);
            }
        }

        InstanceLayoutMode::Manual => {
            // todo: FIXME
            println!("test")
        }
    }
}

pub fn set_instance_names(instances: &mut Vec<Instance>, profiles: &[String]) {
    let mut guests = GUEST_NAMES.to_vec();

    for instance in instances {
        if instance.profselection == 0 {
            let i = fastrand::usize(..guests.len());
            instance.profname = format!(".{}", guests[i]);
            guests.swap_remove(i);
        } else {
            instance.profname = profiles[instance.profselection].to_owned();
        }
    }
}
