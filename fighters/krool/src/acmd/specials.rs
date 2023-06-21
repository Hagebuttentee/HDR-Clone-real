use super::*;
use std::convert::TryInto;

#[acmd_script( agent = "krool", script = "game_specialnfire" , category = ACMD_GAME , low_priority)]
unsafe fn krool_special_n_fire_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.8);
    frame(lua_state, 26.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SHOOT_IRONBALL);
        WorkModule::on_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_NO_SHOOT_CANCEL);
    }
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SHOOT_CANCEL);
    }
    frame(lua_state, 70.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SHOOT_CANCEL);
        WorkModule::off_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_NO_SHOOT_CANCEL);
    }
    
}

#[acmd_script( agent = "krool", script = "game_specialairnfire" , category = ACMD_GAME , low_priority)]
unsafe fn krool_special_air_n_fire_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.8);
    frame(lua_state, 26.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SHOOT_IRONBALL);
        WorkModule::on_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_NO_SHOOT_CANCEL);
    }
    frame(lua_state, 37.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SHOOT_CANCEL);
    }
    frame(lua_state, 70.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_SHOOT_CANCEL);
        WorkModule::off_flag(boma, *FIGHTER_KROOL_STATUS_SPECIAL_N_FLAG_NO_SHOOT_CANCEL);
    }
    
}

#[acmd_script( agent = "krool", script = "game_specialnloop" , category = ACMD_GAME , low_priority)]
unsafe fn krool_special_n_loop_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 7.0, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(18.0), *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, 1, Hash40::new("top"), 4.5, 0.0, 9.5, 10.7, Some(0.0), Some(9.5), Some(20.5), *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_A);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 160, 100, 50, 0, 9.0, 0.0, 9.0, 22.0, Some(0.0), Some(9.0), Some(27.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        SEARCH(fighter, 0, 0, Hash40::new("top"), 7.0, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(28.0), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        SEARCH(fighter, 0, 0, Hash40::new("top"), 7.0, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(34.0), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        SEARCH(fighter, 0, 0, Hash40::new("top"), 7.0, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(38.0), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true);
    }
    
}

#[acmd_script( agent = "krool", script = "game_specialairnloop" , category = ACMD_GAME , low_priority)]
unsafe fn krool_special_air_n_loop_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 7.0, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(18.0), *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, 1, Hash40::new("top"), 4.5, 0.0, 9.5, 10.7, Some(0.0), Some(9.5), Some(20.5), *FIGHTER_STATUS_KIND_SWALLOWED, *COLLISION_SITUATION_MASK_A);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 160, 100, 50, 0, 9.0, 0.0, 9.0, 22.0, Some(0.0), Some(9.0), Some(27.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        SEARCH(fighter, 0, 0, Hash40::new("top"), 7.0, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(28.0), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        SEARCH(fighter, 0, 0, Hash40::new("top"), 7.0, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(34.0), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true);
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        SEARCH(fighter, 0, 0, Hash40::new("top"), 7.0, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(38.0), *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true);
    }
    
}

#[acmd_script( agent = "krool", scripts = ["game_specialsthrow", "game_specialairsthrow"] , category = ACMD_GAME , low_priority)]
unsafe fn krool_special_special_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        damage!(fighter, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_REACTION_VALUE, 180);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        damage!(fighter, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("arml"), 0.0, 361, 100, 30, 0, 4.0, 5.0, 0.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
    ArticleModule::generate_article(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_CROWN, false, 0);
    VisibilityModule::set_int64(boma, Hash40::new("crown").hash as i64, Hash40::new("crown_hide").hash as i64);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
    AttackModule::clear_all(boma);
    }   
    frame(lua_state, 64.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
}

#[acmd_script( agent = "krool_backpack", scripts = ["game_fly", "game_flywind"], category = ACMD_GAME, low_priority )]
unsafe fn uspecial_propeller(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("wingl1"), 3.0, 80, 30, 0, 90, 5.0, 2.0, 0.0, 0.0, Some(-2.0), Some(0.0), Some(0.0), 1.0, 1.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
}

#[acmd_script( agent = "krool", scripts = ["game_speciallwhit", "game_specialairlwhit"], category = ACMD_GAME, low_priority )]
unsafe fn krool_special_lw_hit_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 2.0, 5.0);
    frame(lua_state, 2.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.battle_object, vars::krool::status::GUT_CHECK_CHARGED) {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 69, 90, 0, 70, 9.0, 0.0, 10.0, 4.25, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::krool::status::GUT_CHECK_CHARGED) {
            let damage = VarModule::get_float(fighter.battle_object, vars::krool::instance::STORED_DAMAGE) / 5.0;
            ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0 + damage, 50, 85, 0, 50, 10.5, 0.0, 10.75, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        }
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 18.0);
    FT_MOTION_RATE(fighter, 0.7);
    frame(lua_state, 48.0);
    FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "krool", scripts = ["effect_speciallwhit", "effect_specialairlwhit"], category = ACMD_EFFECT, low_priority )]
unsafe fn krool_special_lw_hit_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("krool_counter_success_body"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("krool_counter_success_body_l"), true, true);
        EFFECT(fighter, Hash40::new("krool_counter_success"), Hash40::new("top"), 0, 3.25, 0.75, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        let charged = VarModule::is_flag(fighter.battle_object, vars::krool::status::GUT_CHECK_CHARGED);
        if charged {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_counter_flash"), Hash40::new("top"), 0, 12, 6, 0, 0, 0, 0.9, false);
        }
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(fighter, 0.7);
        }
        if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
            EFFECT_FOLLOW(fighter, Hash40::new("krool_counter_attack_body_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("krool_counter_attack"), Hash40::new("top"), 6, 12, 15, -90, 30, 0, 0.4, false);
            if charged {
                EFFECT_FOLLOW(fighter, Hash40::new("krool_counter_attack2"), Hash40::new("top"), 6, 12, 4, 0, 25, 0, 1, false);
            }
        }
        else {
            EFFECT_FOLLOW(fighter, Hash40::new("krool_counter_attack_body"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("krool_counter_attack"), Hash40::new("top"), -6, 12, 15, -90, -30, 0, 0.4, false);
            if charged {
                EFFECT_FOLLOW(fighter, Hash40::new("krool_counter_attack2"), Hash40::new("top"), -6, 12, 4, 0, -25, 0, 1, false);
            }
        }
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("krool_counter_attack2"), false, true);
    }
}

#[acmd_script( agent = "krool", scripts = ["sound_speciallwhit", "sound_specialairlwhit"], category = ACMD_SOUND, low_priority )]
unsafe fn krool_special_lw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_krool_special_l01"));
        PLAY_SE(fighter, Hash40::new("vc_krool_special_l01"));
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_krool_smash_h01"));
        if VarModule::is_flag(fighter.battle_object, vars::krool::status::GUT_CHECK_CHARGED) {
            PLAY_SE(fighter, Hash40::new("se_krool_special_l05"));
        }
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_krool_special_l03"));
    }
}

pub fn install() {
    install_acmd_scripts!(
        uspecial_propeller,
        krool_special_n_fire_game,
        krool_special_air_n_fire_game,
        krool_special_n_loop_game,
        krool_special_air_n_loop_game,
        krool_special_special_s_game,
        krool_special_lw_hit_game,
        krool_special_lw_hit_effect,
        krool_special_lw_sound,
    );
}