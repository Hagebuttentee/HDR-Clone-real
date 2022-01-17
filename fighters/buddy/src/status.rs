use super::*;
use globals::*;
// status script import
 
pub unsafe fn end_run(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::remove_motion_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, false);
    fighter.status_end_Run();
    0.into()
}

pub fn install() {
    install_status_scripts!(
        end_run
    );
}