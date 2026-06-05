use std::sync::OnceLock;

use iced::widget::image::Handle;
use include_dir::include_dir;

pub static FACTION_ICON_DIR: include_dir::Dir<'_> = include_dir!("images/factions");
pub static GENERAL_DIR: include_dir::Dir<'_> = include_dir!("images/general");
pub static OPEN_WORLDS_DIR: include_dir::Dir<'_> = include_dir!("images/open_worlds");

pub fn get_poe_image() -> Handle {
    static HANDLE: OnceLock<Handle> = OnceLock::new();
    HANDLE
        .get_or_init(|| {
            let bytes = OPEN_WORLDS_DIR
                .get_file("poe.png")
                .expect("poe.png not found")
                .contents();
            Handle::from_bytes(bytes.to_vec())
        })
        .clone()
}

pub fn get_orbvallis_image() -> Handle {
    static HANDLE: OnceLock<Handle> = OnceLock::new();
    HANDLE
        .get_or_init(|| {
            let bytes = OPEN_WORLDS_DIR
                .get_file("orbvallis.png")
                .expect("orbvallis.png not found")
                .contents();
            Handle::from_bytes(bytes.to_vec())
        })
        .clone()
}

pub fn get_cambiondrift_image() -> Handle {
    static HANDLE: OnceLock<Handle> = OnceLock::new();
    HANDLE
        .get_or_init(|| {
            let bytes = OPEN_WORLDS_DIR
                .get_file("cambiondrift.jpg")
                .expect("cambiondrift.jpg not found")
                .contents();
            Handle::from_bytes(bytes.to_vec())
        })
        .clone()
}

