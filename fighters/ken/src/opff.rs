// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    magic_series(fighter, boma, id, cat, status_kind, situation_kind, motion_kind, stick_x, stick_y, facing, frame);
    special_fadc_super_cancels(boma);
    target_combos(boma);
    kamabaraigeri(boma, frame);
    rotate_forward_bair(boma);
}

// symbol-based call for the shotos' common opff
extern "Rust" {
    fn shotos_common(fighter: &mut smash::lua2cpp::L2CFighterCommon);
}

#[utils::macros::opff(FIGHTER_KIND_KEN)]
pub fn ken_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		ken_frame(fighter);
        shotos_common(fighter);
    }
}

pub unsafe fn ken_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

/// start_frame: frame to start interpolating the body rotation
/// bend_frame: frame to interpolate to the intended angle amount until
/// return_frame: frame to start interpolating back to regular angle
/// straight_frame: frame the body should be at the regular angle again
unsafe fn forward_bair_rotation(boma: &mut BattleObjectModuleAccessor, start_frame: f32, bend_frame: f32, return_frame: f32, straight_frame: f32) {
    let frame = MotionModule::frame(boma);
    let end_frame = MotionModule::end_frame(boma);
    let max_rotation = 180.0;
    let mut rotation = Vector3f{x: 0.0, y: 0.0, z: 0.0};
        
    if frame >= start_frame && frame < return_frame {
        // this has to be called every frame, or you snap back to the normal joint angle
        // interpolate to the respective body rotation angle
        let calc_body_rotate = max_rotation * ((frame - start_frame) / (bend_frame - start_frame));
        let body_rotation = calc_body_rotate.clamp(0.0, max_rotation);
        rotation = Vector3f{x: 0.0, y: body_rotation, z: 0.0};
        // println!("current body rotation: {}", body_rotation);
        ModelModule::set_joint_rotate(boma, Hash40::new("rot"), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    } else if frame >= return_frame && frame < straight_frame {
        // linear interpolate back to normal
        /*
        let calc_body_rotate = max_rotation *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
        let body_rotation = calc_body_rotate.clamp(0.0, max_rotation);
        */
        let calc_body_rotate = 180.0 *((frame - return_frame) / (straight_frame - return_frame)) + 180.0;
        let body_rotation = calc_body_rotate.clamp(180.0, 360.0);
        rotation = Vector3f{x: 0.0, y: body_rotation, z: 0.0};
        ModelModule::set_joint_rotate(boma, Hash40::new("rot"), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    }
}

unsafe fn rotate_forward_bair(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_motion(Hash40::new("attack_air_b")){
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK) {
            forward_bair_rotation(boma, 5.0, 7.5, 10.0, 30.0);
        }
    }
    else if boma.is_motion(Hash40::new("landing_air_b")){
        if VarModule::is_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK) {
            forward_bair_rotation(boma, 0.0, 0.1, 0.2, 10.0);
        }
    }
}

// start_frame: frame to start interpolating the leg rotation
// bend_frame: frame to interpolate to the intended angle amount until
// return_frame: frame to start interpolating back to regular angle
// straight_frame: frame the leg should be at the regular angle again
unsafe fn fsmash_leg_rotate(boma: &mut BattleObjectModuleAccessor, start_frame: f32, bend_frame: f32, return_frame: f32, straight_frame: f32) {
    let frame = MotionModule::frame(boma);
    let end_frame = MotionModule::end_frame(boma);
    let max_y_rotation = 20.0;
    let max_z_rotation = 75.0;
    let mut rotation = Vector3f{x: 0.0, y: 0.0, z: 0.0};
        
    if frame >= start_frame && frame < return_frame {
        // this has to be called every frame, or you snap back to the normal joint angle
        // interpolate to the respective leg bend angle
        let calc_y_rotate = max_y_rotation * (frame / (bend_frame - start_frame));
        let y_rotation = calc_y_rotate.clamp(0.0, max_y_rotation);
        let calc_z_rotate = max_z_rotation * (frame / (bend_frame - start_frame));
        let z_rotation = calc_z_rotate.clamp(0.0, max_z_rotation);
        rotation = Vector3f{x: 0.0, y: y_rotation, z: z_rotation};
        ModelModule::set_joint_rotate(boma, Hash40::new("kneer"), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8})
    } else if frame >= return_frame && frame < straight_frame {
        // linear interpolate back to normal
        let calc_y_rotate = max_y_rotation *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
        let y_rotation = calc_y_rotate.clamp(0.0, max_y_rotation);
        let calc_z_rotate = max_z_rotation *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
        let z_rotation = calc_z_rotate.clamp(0.0, max_z_rotation);
        rotation = Vector3f{x: 0.0, y: y_rotation, z: z_rotation};
        ModelModule::set_joint_rotate(boma, Hash40::new("kneer"), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8})
    }
}

// Kamabaraigeri: A special move from USFIV where Ken hits you with a roundhouse knee before extending his leg into a kick
// Activated here via canceling into fsmash through magic series
// Can be canceled into the axe kick like his other command kicks via holding the attack button
unsafe fn kamabaraigeri(boma: &mut BattleObjectModuleAccessor, frame: f32) {
    if boma.is_motion(Hash40::new("attack_s4_s")){
        if VarModule::is_flag(boma.object(), vars::shotos::status::SHOULD_COMBOS_SCALE) {
            fsmash_leg_rotate(boma, 9.0, 12.0, 14.0, 16.0);
        }
        if frame >= (MotionModule::end_frame(boma) - 1.0) {
            // Fix getting stuck in the anim due to not setting the charge flag
            StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_WAIT, false);
        }
    }
}

unsafe fn special_fadc_super_cancels(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP
    ]) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD){
            VarModule::on_flag(boma.object(), vars::shotos::instance::IS_ENABLE_FADC);
        }
        if VarModule::is_flag(boma.object(), vars::shotos::instance::IS_ENABLE_FADC){
            if boma.is_cat_flag(Cat1::SpecialLw){
                if MeterModule::drain(boma.object(), 2){
                    StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
                }
            }
            if boma.is_cat_flag(Cat4::SpecialSCommand | Cat4::SpecialHiCommand){
                if MeterModule::drain(boma.object(), 10) {
                    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL);
                    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_IS_DISCRETION_FINAL_USED);
                    StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_FINAL, true);
                } 
            }
        }
    }
    else{
        VarModule::off_flag(boma.object(), vars::shotos::instance::IS_ENABLE_FADC);
    }
}

// Target combos:
// 1: Prox jab into far heavy jab
// 2: Prox ftilt into light ftilt
unsafe fn target_combos(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_motion_one_of(&[Hash40::new("attack_hi3_w")]){
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD){
            if boma.is_cat_flag(Cat1::AttackN) 
            && !boma.is_cat_flag(Cat1::AttackLw3)
            && !boma.is_cat_flag(Cat1::AttackS3)
            && !boma.is_cat_flag(Cat1::AttackHi3) {
                WorkModule::off_flag(boma, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
                WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK);
                boma.change_status_req(*FIGHTER_STATUS_KIND_ATTACK, false);
            }
        }
    }
}

unsafe fn magic_flag_reset(boma: &mut BattleObjectModuleAccessor) {
    if !(boma.is_motion_one_of(&[
        Hash40::new("attack_12"),
        Hash40::new("attack_s3_s_w"),
        Hash40::new("attack_s3_s_s"),
        Hash40::new("attack_near_w"),
        Hash40::new("attack_hi3_w"),
        Hash40::new("attack_hi3_s"),
        Hash40::new("attack_lw3_w"),
        Hash40::new("attack_lw3_s"),
        Hash40::new("attack_s4"),
        Hash40::new("attack_s4_hold"),
        Hash40::new("attack_hi4"),
        Hash40::new("attack_hi4_hold"),
        Hash40::new("attack_lw4"),
        Hash40::new("attack_lw4_hold")
    ]) || boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_ATTACK,
        *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_ATTACK_TURN,
        *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1,
        *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND2
    ])) {
        VarModule::off_flag(boma.object(), vars::shotos::instance::IS_MAGIC_SERIES_CANCEL);
    }
}

unsafe fn magic_series(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    
    magic_flag_reset(boma);

    if !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) /*&& VarModule::is_flag(boma.object(), vars::shotos::status::IS_ENABLE_MAGIC_SERIES_CANCEL)*/ {
        return;
    }

    // dash cancels
    // if boma.is_motion_one_of(&[
    //     Hash40::new("attack_s3_s_w"),
    // ]) {
    //     boma.check_dash_cancel();
    //     return;
    // }

    // jump cancels
    // if boma.is_status_one_of(&[
    //     *FIGHTER_STATUS_KIND_ATTACK_HI4,
    // ]) {
    //     boma.check_jump_cancel(false);
    //     return;
    // }
    
}