use std::sync;
use broadsword::runtime;
use crate::pointerchain;

static GAME_BASE: sync::OnceLock<usize> = sync::OnceLock::new();

// TODO: make AOBs
pub const IBO_WORLD_CHR_MAN: usize = 0x3CDCDD8;
pub const OFFSET_MAIN_PLAYER: usize = 0x1E508;
pub const IBO_FLIPPER_UPDATE: usize = 0xE44630;
pub const IBO_WORLDCHRMAN_UPDATE: usize = 0x50C340;
pub const IBO_CHRINS_DESTRUCTOR: usize = 0x3E5480;
pub const IBO_SPECIALEFFECT_APPLY: usize = 0x4F7280;
pub const IBO_SPECIALEFFECT_REMOVE: usize = 0x4F8070;

pub static mut GLOBAL_TIME_MULTIPLIER: f32 = 1.0;

pub fn get_game_base() -> usize {
    *GAME_BASE.get_or_init(|| {
        runtime::get_module_handle("eldenring.exe".to_string()).unwrap()
    })
}

pub fn get_main_player_ins() -> usize {
    pointerchain!(usize, get_game_base() + IBO_WORLD_CHR_MAN, OFFSET_MAIN_PLAYER) as *const usize as usize
}

// Incomplete of course
#[repr(C)]
pub struct SpecialEffect {
    pub vfptr: usize,
    pub unk1: usize,
    pub owner: usize,
}

#[repr(C)]
pub struct SpecialEffectEntry {
    pub unk1: usize,
    pub special_effect_id: u32,
}

#[repr(C)]
pub struct CSFlipperImp {
    pub unk1: [u8; 0x270],
    pub delta_time: f32,
}
