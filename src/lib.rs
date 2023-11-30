#![feature(absolute_path)]

use std::mem;

mod config;
mod game;
mod util;
mod effect;

use broadsword::dll;
use detour::static_detour;

use game::*;
use crate::config::EffectEntryType;

static_detour! {
    // Function that seems to update the delta time between frames. Patching the values seemingly
    // affects all simulation, from VFX timelines to gravity.
    static FLIPPER_UPDATE_HOOK: fn(*mut CSFlipperImp);
    static WORLDCHRMAN_UPDATE_HOOK: fn(usize, usize) -> usize;

    static CHRINS_DESTRUCTOR: fn(usize, usize) -> usize;

    static SPECIALEFFECT_APPLY: fn(*const SpecialEffect, u32, usize, usize, usize, usize, usize, usize) -> usize;
    static SPECIALEFFECT_REMOVE: fn(*const SpecialEffect, *const SpecialEffectEntry, usize, usize) -> usize;
}

#[dll::entrypoint]
pub fn entry(_: usize) -> bool {
    broadsword::logging::init("logs/time_speffects.log");
    apply_hooks();
    return true;
}

fn apply_hooks() {
    unsafe {
        SPECIALEFFECT_APPLY.initialize(
            mem::transmute(get_game_base() + IBO_SPECIALEFFECT_APPLY),
            |special_effect: *const SpecialEffect, special_effect_id: u32, param_3: usize, param_4: usize, param_5: usize, param_6: usize, param_7: usize, param_8: usize| {
                if let Some(effect) = config::get_effect_mapping(special_effect_id) {
                    match effect.effect_type {
                        EffectEntryType::WorldTime { multiplier } => {
                            GLOBAL_TIME_MULTIPLIER = multiplier
                        }
                        EffectEntryType::ChrTime { multiplier } => {
                            effect::add_affected_chrins((*special_effect).owner, multiplier)
                        }
                    }
                }

                SPECIALEFFECT_APPLY.call(special_effect, special_effect_id, param_3, param_4, param_5, param_6, param_7, param_8)
            }
        ).unwrap();
        SPECIALEFFECT_APPLY.enable().unwrap();

        SPECIALEFFECT_REMOVE.initialize(
            mem::transmute(get_game_base() + IBO_SPECIALEFFECT_REMOVE),
            |special_effect: *const SpecialEffect, special_effect_entry: *const SpecialEffectEntry, param_3: usize, param_4: usize| {
                if let Some(effect) = config::get_effect_mapping((*special_effect_entry).special_effect_id) {
                    match effect.effect_type {
                        EffectEntryType::WorldTime { .. } => {
                            GLOBAL_TIME_MULTIPLIER = 1.0
                        }
                        EffectEntryType::ChrTime { .. } => {
                            effect::remove_affected_chrins((*special_effect).owner)
                        }
                    }
                }

                SPECIALEFFECT_REMOVE.call(special_effect, special_effect_entry, param_3, param_4)
            }
        ).unwrap();
        SPECIALEFFECT_REMOVE.enable().unwrap();

        FLIPPER_UPDATE_HOOK.initialize(
            mem::transmute(get_game_base() + IBO_FLIPPER_UPDATE),
            |flipper: *mut CSFlipperImp| {
                FLIPPER_UPDATE_HOOK.call(flipper);

                (*flipper).delta_time = (*flipper).delta_time * GLOBAL_TIME_MULTIPLIER;
            }
        ).unwrap();
        FLIPPER_UPDATE_HOOK.enable().unwrap();

        WORLDCHRMAN_UPDATE_HOOK.initialize(
            mem::transmute(get_game_base() + IBO_WORLDCHRMAN_UPDATE),
            |worldchrman: usize, delta: usize| {
                let result = WORLDCHRMAN_UPDATE_HOOK.call(worldchrman, delta);

                // Update the frame delta for character instances
                for (chrins, mult) in effect::get_affected_chrinses().iter() {
                    let frame_delta = *((chrins + 0xb0) as *const f32);
                    *((chrins + 0xb0) as *mut f32) = frame_delta * mult;
                }

                result
            }
        ).unwrap();
        WORLDCHRMAN_UPDATE_HOOK.enable().unwrap();

        CHRINS_DESTRUCTOR.initialize(
            mem::transmute(get_game_base() + IBO_CHRINS_DESTRUCTOR),
            |chrins: usize, param_2: usize| {
                if get_main_player_ins() == chrins {
                    GLOBAL_TIME_MULTIPLIER = 1.0;
                } else {
                    effect::remove_affected_chrins(chrins);
                }

                CHRINS_DESTRUCTOR.call(chrins, param_2)
            }
        ).unwrap();
        CHRINS_DESTRUCTOR.enable().unwrap();
    }
}
