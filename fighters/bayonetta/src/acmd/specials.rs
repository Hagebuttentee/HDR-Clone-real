
use super::*;


#[acmd_script( agent = "bayonetta", scripts = ["game_specialnstarth", "game_specialnstartf", "game_specialairnstarth", "game_specialairnstartf"] , category = ACMD_GAME , low_priority)]
unsafe fn bayonetta_special_n_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 0.5);//van
}

#[acmd_script( agent = "bayonetta", scripts = ["game_specialnchargef", "game_specialnchargeh", "game_specialairnchargef", "game_specialairnchargeh"] , category = ACMD_GAME , low_priority)]
unsafe fn bayonetta_special_n_charge_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE(fighter, 1.65); //van
}

#[acmd_script( agent = "bayonetta", scripts = ["game_specialnendh", "game_specialnendf", "game_specialairnendh", "game_specialairnendf"] , category = ACMD_GAME , low_priority)]
unsafe fn bayonetta_special_n_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 65.0, 25.0);//32 > 26
}

#[acmd_script( agent = "bayonetta", script = "game_specials", category = ACMD_GAME, low_priority )]
unsafe fn bayonetta_special_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 10, 0, 20, 0, false);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        GroundModule::set_correct(boma, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        ATTACK(fighter, 0, 0, Hash40::new("footr"), 8.0, 42, 8, 0, 101, 4.0, 0.0, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("footr"), 8.0, 47, 8, 0, 95, 4.0, 0.0, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        fighter.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_WALL_CHECK);
        ATTACK(fighter, 0, 0, Hash40::new("footr"), 7.5, 50, 8, 0, 83, 4.0, 0.0, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        fighter.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        ATTACK(fighter, 0, 0, Hash40::new("footr"), 7.0, 62, 8, 0, 68, 4.0, 0.0, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_HOLD_END_CHECK);
            fighter.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        } else {AttackModule::clear_all(boma); }
    }
    frame(lua_state, 40.0);
    FT_MOTION_RATE_RANGE(fighter, 40.0, 66.0, 22.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        fighter.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_WALL_CHECK);
    }    
}

#[acmd_script( agent = "bayonetta", script = "game_specialsholdend", category = ACMD_GAME, low_priority )]
unsafe fn bayonetta_special_s_hold_end_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 34.5, 10.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 2, 0, 0, 0, false);
        ATTACK(fighter, 0, 0, Hash40::new("footr"), 7.0, 70, 8, 0, 50, 4.0, 0.5, 0.0, 0.0, Some(-8.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        fighter.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        fighter.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        AttackModule::clear_all(fighter.module_accessor);
        fighter.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_END_SPECIAL_S);
    }
    frame(lua_state, 34.5);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 37.5);
    FT_MOTION_RATE_RANGE(fighter, 37.5, 40.0, 3.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("footr"), 5.0, 98, 66, 0, 60, 4.8, 1.2, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 5.0, 98, 66, 0, 60, 4.3, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("waist"), 5.0, 90, 66, 0, 60, 3.9, 0.0, -0.9, -1.1, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 40.0);
    FT_MOTION_RATE_RANGE(fighter, 40.0, 44.0, 2.0);
    frame(lua_state, 44.0);
    FT_MOTION_RATE_RANGE(fighter, 44.0, 61.0, 15.0); //61 faf
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "bayonetta", script = "game_specialairsu" , category = ACMD_GAME , low_priority)]
unsafe fn bayonetta_special_air_s_u_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 12.0, 6.5);
    if is_excute(fighter) {
        JostleModule::set_status(boma, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 5, 0, 20, 0, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, true, 5);
    }
    frame(lua_state, 12.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        fighter.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 6.0, 65, 27, 0, 71, 4.5, 4.0, 0.0, 0.0, Some(-2.5), Some(0.0), Some(0.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);   
        VarModule::inc_int(boma.object(), vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 6.0, 70, 27, 0, 65, 4.5, 4.0, 0.0, 0.0, Some(-2.5), Some(0.0), Some(0.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 7.0, 45, 100, 70, 0, 4.5, 4.0, 0.0, 0.0, Some(-2.5), Some(0.0), Some(0.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 7.7, 45, 100, 65, 0, 4.5, 4.0, 0.0, 0.0, Some(-2.5), Some(0.0), Some(0.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("kneer"), 7.7, 50, 100, 50, 0, 4.5, 4.0, 0.0, 0.0, Some(-2.5), Some(0.0), Some(0.0), 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        fighter.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        fighter.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
        fighter.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        JostleModule::set_status(boma, true);
        AttackModule::clear_all(boma);
        fighter.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
    }
    frame(lua_state, 33.0); //29
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::bayonetta::instance::IS_HIT) { //filters out bullet hits
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
    frame(lua_state, 36.0); //32
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    frame(lua_state, 43.0); //39
    if is_excute(fighter) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
    }
}

#[acmd_script( agent = "bayonetta", script = "effect_specialairsu", category = ACMD_EFFECT, low_priority )]
unsafe fn bayonetta_special_air_s_u_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.5);
    if is_excute(fighter) {
        EFFECT_FOLLOW_WORK(fighter, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_AFTERBURNER_LINE, Hash40::new("top"), 0, 13.0, 12.1, -21, 0, 0, 1.0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("bayonetta_afterburner_line2"), Hash40::new("top"), 0, 13.8, 13.5, -21, 0, 0, 1.0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND_WORK(fighter, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_AFTERBURNER_LINE, -1);
        EFFECT_DETACH_KIND(fighter, Hash40::new("bayonetta_afterburner_line2"), -1);
    }
}

#[acmd_script( agent = "bayonetta", script = "game_shootinglegl_atkon_specialairsu", category = ACMD_GAME, low_priority )]
unsafe fn bayonetta_shootinglegl_atk_on_special_air_s_u_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        if !fighter.is_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_DISABLE_ROOT_ATTACK) {
            ATTACK(fighter, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_00 as u64, 0, Hash40::new("top"), 0.5, 340, 100, 0, 10, 3.5, 0.0, 4.5, -0.5, Some(0.0), Some(4.5), Some(3.5), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        }
        ATTACK(fighter, *FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01 as u64, 0, Hash40::new("top"), 0.5, 340, 0, 0, 0, 2.5, 0.0, 2.5, 3.0, Some(0.0), Some(-18.0), Some(59.4), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BAYONETTA_HIT_01, *ATTACK_REGION_NONE);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_63_bullet"), 4, true, *BATTLE_OBJECT_ID_INVALID as u32);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x36db1a34c9), FIGHTER_BAYONETTA_SHOOTING_ATTACK_ID_01, 6, 4.05);
    }
}

#[acmd_script( agent = "bayonetta", script = "game_specialairsd" , category = ACMD_GAME , low_priority)]
unsafe fn bayonetta_special_air_s_d_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        VarModule::inc_int(boma.object(), vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED);
        fighter.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
        ATTACK(fighter, 0, 0, Hash40::new("legr"), 7.5, 86, 35, 0, 95, 5.0, 2.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 7.5, 86, 35, 0, 95, 5.0, 0.5, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        AttackModule::clear_all(boma);
        fighter.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_AIR_S_FLAG_WALL_CHECK);
        fighter.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_LANDING_FALL_SPECIAL);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        JostleModule::set_status(boma, true);
        fighter.off_flag(*FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "bayonetta", script = "effect_specialairsd", category = ACMD_EFFECT, low_priority )]
unsafe fn bayonetta_special_air_s_d_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_WORK(fighter, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_AFTERBURNER_LINE, Hash40::new("top"), 0, 0, 7, 45, -5, 0, 1.1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.9);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("bayonetta_afterburner_line2"), Hash40::new("top"), 0, 0, 7, 45, -5, 0, 1.1, true);
    }
}

#[acmd_script( agent = "bayonetta", script = "game_specialairsdlanding" , category = ACMD_GAME , low_priority)]
unsafe fn bayonetta_special_air_s_d_landing_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }
}

#[acmd_script( agent = "bayonetta", scripts = ["game_specialairhi", "game_specialhi"] , category = ACMD_GAME , low_priority)]
unsafe fn bayonetta_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, false, false, true, 20, 0, 15, 0, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, false, false, true, 20);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, false, false, true, 20);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, false, false, true, 20);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 368, 10, 0, 0, 3.0, 0.0, 4.5, 0.0, Some(0.0), Some(12.5), Some(0.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 368, 10, 0, 0, 3.5, 0.0, 5.5, 5.0, Some(0.0), Some(10.0), Some(5.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("top"), &Vector2f{x: 0.5, y: 33.0}, 11, false);
        AttackModule::set_vec_target_pos(boma, 1, Hash40::new("top"), &Vector2f{x: -0.3, y: 33.0}, 11, false);
        VarModule::inc_int(boma.object(), vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        fighter.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
        fighter.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        AttackModule::clear_all(boma);
        //AttackModule::clear(boma, 1, false);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 368, 10, 0, 0, 4.0, 0.0, 18.2, 0.5, Some(0.0), Some(20.5), Some(0.5), 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.5, 368, 10, 0, 0, 2.4, 0.0, 16.2, 0.5, Some(0.0), Some(23.8), Some(0.5), 1.1, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        AttackModule::set_vec_target_pos(boma, 0, Hash40::new("head"), &Vector2f{x: 0.0, y: 15.5}, 7, false);
        AttackModule::set_vec_target_pos(boma, 1, Hash40::new("head"), &Vector2f{x: 0.0, y: 14.5}, 7, false);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        fighter.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        KineticModule::add_speed_outside(fighter.module_accessor, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &Vector3f::new( 0.0, 4.0, 0.0));
        if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_REUSE) {
            VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 20, 35, 0, 40, 4.5, 0.0, 23.2, 0.0, Some(0.0), Some(18.8), Some(0.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        } else {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 40, 55, 0, 45, 4.5, 0.0, 23.2, 0.0, Some(0.0), Some(18.8), Some(0.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2bfb02b69a), true);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::bayonetta::instance::IS_HIT) {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
    frame(lua_state, 32.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
}

#[acmd_script( agent = "bayonetta", scripts = ["effect_specialairhi", "effect_specialhi"], category = ACMD_EFFECT, low_priority )]
unsafe fn bayonetta_special_hi_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_GROUND_START) {
            FOOT_EFFECT(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
            LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_WORK(fighter, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_WITCHTWIST_WIND, Hash40::new("top"), 0, 25.3, 0, 0, 0, 0, 0.86, true);
        LAST_EFFECT_SET_RATE(fighter, 1.1);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("bayonetta_afterburner_line2"), Hash40::new("top"), 0, 25, 0, -90, 0, 0, 0.82, true);
        if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_HI_FLAG_REUSE) {
            EFFECT_FOLLOW_WORK(fighter, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_WITCHTWIST_SPIRAL, Hash40::new("top"), 0, 25.3, 0, 0, 0, 0, 0.8, true);
            LAST_EFFECT_SET_RATE(fighter, 1.7);
        }
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND_WORK(fighter, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_WITCHTWIST_WIND, false, true);
        EFFECT_OFF_KIND_WORK(fighter, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_EFFECT_KIND_BAYONETTA_WITCHTWIST_SPIRAL, false, true);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("bayonetta_afterburner_line2"), false, true);

    }
}

#[acmd_script( agent = "bayonetta", scripts = ["sound_specialhi", "sound_specialairhi"], category = ACMD_SOUND, low_priority )]
unsafe fn bayonetta_special_hi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_bayonetta_special_h01"));
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_bayonetta_special_h02"));
    }
}

#[acmd_script( agent = "bayonetta", scripts = ["game_speciallw", "game_specialairlw"], category = ACMD_GAME , low_priority)]
unsafe fn bayonetta_special_lw (fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let boma = fighter.boma();
	frame(lua_state, 1.0);
    FT_MOTION_RATE_RANGE(fighter, 1.0, 8.0, 4.0);
	if is_excute(fighter) {
		fighter.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_ENABLE_NEXT_NO_COMP);
	}
    frame(lua_state, 8.0);
    FT_MOTION_RATE_RANGE(fighter, 8.0, 15.0, 4.0); //5
	if is_excute(fighter) {
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2ea0f68425), true);
            sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 1.0);
        }
        fighter.on_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_WITCH_TIME);
        macros::SEARCH(fighter, 0, 0, Hash40::new("top"), 11.5, -2.0, 10.0, 0.0, None, None, None, *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_BODY_HEAD, false);
    }
	frame(lua_state, 15.0);
    FT_MOTION_RATE(fighter, 1.0); //9
	if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2ea0f68425), true);
    }
	frame(lua_state, 21.0);
    FT_MOTION_RATE_RANGE(fighter, 21.0, 31.0, 7.0); //15
	if is_excute(fighter) {
		fighter.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_WITCH_TIME);
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(lua_state, 31.0);//22
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        FT_MOTION_RATE_RANGE(fighter, 31.0, 66.0, 28.0); 
    } else { FT_MOTION_RATE_RANGE(fighter, 31.0, 66.0, 23.0); }
	if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2ea0f68425), false);
    }
	frame(lua_state, 44.0);
	if is_excute(fighter) {
		fighter.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_LW_FLAG_ENABLE_NEXT_NO_COMP);
	}
} //faf 50/45

pub fn install() {
    install_acmd_scripts!(
        bayonetta_special_n_charge_game,
        bayonetta_special_n_end_game,
        bayonetta_special_n_start_game,
        bayonetta_special_s_game,
        bayonetta_special_s_hold_end_game,
        bayonetta_special_air_s_u_game,
        bayonetta_special_air_s_u_effect,
        bayonetta_shootinglegl_atk_on_special_air_s_u_game,
        bayonetta_special_air_s_d_game,
        bayonetta_special_air_s_d_effect,
        bayonetta_special_air_s_d_landing_game,
        bayonetta_special_hi_game,
        bayonetta_special_hi_effect,
        bayonetta_special_hi_sound,
        bayonetta_special_lw
    );
}