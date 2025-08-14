// Re-export all utility functions from submodules
mod filesystem;
mod profiles;
mod sys;
mod updates;
mod resolution;
mod sway;

// Re-export functions from profiles
pub use profiles::{
    GUEST_NAMES, create_gamesave, create_profile, remove_guest_profiles, scan_profiles,
};

// Re-export functions from filesystem
pub use filesystem::{SanitizePath, copy_dir_recursive, get_rootpath, get_rootpath_handler};

// Re-export functions from launcher
pub use sys::{get_screen_resolution, kwin_dbus_start_script, kwin_dbus_unload_script, msg, yesno};

// Re-export functions from updates
pub use updates::check_for_partydeck_update;

pub use resolution::Resolution;

pub use sway::{sway_load_script, get_sway_socket};