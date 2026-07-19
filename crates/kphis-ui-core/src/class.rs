use dominator::class;
use std::sync::LazyLock;

pub static MONO_PRE_WRAP: LazyLock<String> = LazyLock::new(|| class! {
    .style("white-space","pre-wrap")
    .style("font-family",["Consolas","monospace","serif"])
});

pub const ABSOLUTE_FB: [&str; 4] = ["position-absolute","start-0","bottom-0","w-100"];
pub const ABSOLUTE_RT: [&str; 3] = ["position-absolute","end-0","top-0"];

pub const ACCORDION_BTN_CYANS_P2: [&str; 3] = ["accordion-button","bg-info-subtle","p-2"];
pub const ACCORDION_BTN_COLLAPSED_CYANS_P2: [&str; 4] = ["accordion-button","collapsed","bg-info-subtle","p-2"];
pub const ACCORDION_BTN_GOLD_P2: [&str; 3] = ["accordion-button","bg-warning-subtle","p-2"];
pub const ACCORDION_BTN_COLLAPSED_GOLD_P2: [&str; 4] = ["accordion-button","collapsed","bg-warning-subtle","p-2"];
pub const ACCORDION_COLLAPSE: [&str; 2] = ["accordion-collapse","collapse"];
pub const ACCORDION_COLLAPSE_SHOW: [&str; 3] = ["accordion-collapse","collapse","show"];

pub const ALERT_BLUE: [&str; 2] = ["alert","alert-primary"];
pub const ALERT_CYAN: [&str; 2] = ["alert","alert-info"];
pub const ALERT_GREEN: [&str; 2] = ["alert","alert-success"];
pub const ALERT_GRAY: [&str; 2] = ["alert","alert-secondary"];

pub const ALERT_GRAY_TX: [&str; 5] = ["alert","alert-secondary","row","p-0","ps-2"];
pub const ALERT_BOLD_C: [&str; 5] = ["alert","fw-bold","text-center","mx-1","mb-0"];

pub const ALIGN_MIDDLE_R: [&str; 2] = ["align-middle","ms-1"];

pub const BADGE_BLUE: [&str; 2] = ["badge","bg-primary"];
pub const BADGE_BLUE_L: [&str; 3] = ["badge","bg-primary","me-1"];
pub const BADGE_BLUE_R: [&str; 3] = ["badge","bg-primary","ms-1"];
pub const BADGE_CYAN: [&str; 2] = ["badge","text-bg-info"];
pub const BADGE_CYAN_L: [&str;3] = ["badge","text-bg-info","me-1"];
pub const BADGE_CYAN25_L: [&str; 4] = ["badge","text-bg-info","bg-opacity-25","me-1"];
pub const BADGE_CYAN_R: [&str; 3] = ["badge","text-bg-info","ms-1"];
pub const BADGE_GOLD: [&str; 2] = ["badge","text-bg-warning"];
pub const BADGE_GOLD_L: [&str; 3] = ["badge","text-bg-warning","me-1"];
pub const BADGE_GOLD_R: [&str; 3] = ["badge","text-bg-warning","ms-1"];
pub const BADGE_GOLD_RT: [&str; 4] = ["badge","text-bg-warning","mb-1","ms-1"];
pub const BADGE_GREEN_L: [&str; 3] = ["badge","text-bg-success","me-1"];
pub const BADGE_GREEN_R: [&str; 3] = ["badge","text-bg-success","ms-1"];
pub const BADGE_GREEN_RT: [&str; 4] = ["badge","text-bg-success","mb-1","ms-1"];
pub const BADGE_GRAY: [&str; 2] = ["badge","text-bg-secondary"];
pub const BADGE_GRAY_L: [&str; 3] = ["badge","text-bg-secondary","me-1"];
pub const BADGE_GRAY_R: [&str; 3] = ["badge","text-bg-secondary","ms-1"];
pub const BADGE_GRAY_RT: [&str; 4] = ["badge","text-bg-secondary","mb-1","ms-1"];
pub const BADGE_RED: [&str; 2] = ["badge","text-bg-danger"];
pub const BADGE_RED_L: [&str; 3] = ["badge","text-bg-danger","me-1"];
pub const BADGE_RED75_L: [&str; 4] = ["badge","text-bg-danger","bg-opacity-75","me-1"];
pub const BADGE_RED25_L: [&str; 4] = ["badge","text-bg-danger","bg-opacity-25","me-1"];
pub const BADGE_RED_R: [&str; 3] = ["badge","text-bg-danger","ms-1"];
pub const BADGE_RED_RT: [&str; 4] = ["badge","text-bg-danger","mb-1","ms-1"];

pub const BADGE_TRUNC: [&str; 2] = ["badge","text-truncate"];
pub const BADGE_TRUNC_BLUE: [&str; 3] = ["badge","text-truncate","bg-primary"];
pub const BADGE_TRUNC_CYAN: [&str; 3] = ["badge","text-truncate","text-bg-info"];
pub const BADGE_TRUNC_GOLD: [&str; 3] = ["badge","text-truncate","text-bg-warning"];
pub const BADGE_TRUNC_GREEN: [&str; 3] = ["badge","text-truncate","text-bg-success"];
pub const BADGE_TRUNC_GRAY: [&str; 3] = ["badge","text-truncate","text-bg-secondary"];
pub const BADGE_TRUNC_RED: [&str; 3] = ["badge","text-truncate","text-bg-danger"];

pub const BADGE_WRAP_R_BLUE: [&str; 5] = ["badge","d-inline-flex","flex-wrap","text-bg-primary","ms-1"];
pub const BADGE_WRAP_R_CYAN: [&str; 5] = ["badge","d-inline-flex","flex-wrap","text-bg-info","ms-1"];
pub const BADGE_WRAP_R_GREEN: [&str; 5] = ["badge","d-inline-flex","flex-wrap","text-bg-success","ms-1"];
pub const BADGE_WRAP_R_GOLD: [&str; 5] = ["badge","d-inline-flex","flex-wrap","text-bg-warning","ms-1"];
pub const BADGE_WRAP_R_GRAY: [&str; 5] = ["badge","d-inline-flex","flex-wrap","text-bg-secondary","ms-1"];
pub const BADGE_WRAP_R_RED: [&str; 5] = ["badge","d-inline-flex","flex-wrap","text-bg-danger","ms-1"];

pub const BADGE_WRAP_RT_GRAY: [&str; 6] = ["badge","d-inline-flex","flex-wrap","text-bg-secondary","ms-1","mt-1"];

pub const BADGE_FR_GRAY: [&str; 3] = ["badge","text-bg-secondary","float-end"];
pub const BADGE_L: [&str; 2] = ["badge","me-1"];
pub const BADGE_R: [&str; 2] = ["badge","ms-1"];
pub const BADGE_LB: [&str; 3] = ["badge","mt-1","me-1"];
pub const BADGE_TB_C: [&str; 3] = ["badge","text-black","text-center"];
pub const BADGE_RT_PX0_GRAY: [&str; 5] = ["badge","mb-1","ms-1","px-0","bg-secondary"];
pub const BADGE_RT_PX2: [&str; 4] = ["badge","mb-1","ms-1","px-2"];

pub const BADGE_FIX_RT_BLUE: [&str; 8] = ["badge","rounded-pill","bg-primary","position-absolute","top-0","start-100","translate-middle","z-1"];
pub const BADGE_FIX_RT_RED: [&str; 8] = ["badge","rounded-pill","bg-danger","position-absolute","top-0","start-100","translate-middle","z-1"];
pub const BADGE_FIX_RB_GRAY: [&str; 8] = ["badge","rounded-circle","bg-secondary","position-absolute","top-100","start-100","translate-middle","z-1"];

pub const BADGE_FIX_T_CYAN: [&str; 5] = ["badge","rounded-pill","text-bg-info","position-absolute","z-1"];

pub const BG_CYAN_10: [&str; 2] = ["bg-info","bg-opacity-10"];
pub const BG_GREEN_10: [&str; 2] = ["bg-success","bg-opacity-10"];
pub const BG_GOLD_10: [&str; 2] = ["bg-warning","bg-opacity-10"];

pub const BOLD: [&str; 2] = ["fw-bold","me-1"];
pub const BOLD_L2: [&str; 2] = ["fw-bold","me-2"];
pub const BOLD_R2: [&str; 2] = ["fw-bold","ms-2"];
pub const BOLD_PT1: [&str; 2] = ["fw-bold","pt-1"];
pub const BOLD_FS5_L: [&str; 3] = ["fw-bold","fs-5","me-1"];
pub const BOLD_FS5_R: [&str; 3] = ["fw-bold","fs-5","ms-1"];
pub const BOLD_U_L: [&str; 3] = ["fw-bold","text-decoration-underline","me-1"];
pub const BOLD_C: [&str; 2] = ["fw-bold","text-center"];
pub const BOLD_C_PB2: [&str; 3] = ["fw-bold","text-center","pb-2"];
pub const BOLD_C_PT2: [&str; 3] = ["fw-bold","text-center","pt-2"];
pub const BOLD_C_PY2: [&str; 3] = ["fw-bold","text-center","py-2"];
pub const BOLD_R: [&str; 2] = ["fw-bold","text-end"];
pub const BOLD_M2: [&str; 2] = ["fw-bold","m-2"];
pub const BOLD_X: [&str; 2] = ["fw-bold","mx-2"];
pub const BOLD_Y: [&str; 2] = ["fw-bold","my-2"];
pub const BOLD_T1: [&str; 2] = ["fw-bold","mb-1"];
pub const BOLD_T2: [&str; 2] = ["fw-bold","mb-2"];
pub const BOLD_T3L: [&str; 3] = ["fw-bold","mb-3","me-1"];
pub const BOLD_TB: [&str; 3] = ["fw-bold","text-black","me-1"];
pub const BOLD_D3_C: [&str; 3] = ["fw-bold","text-center","display-3"];
pub const BOLD_BB_C_P1: [&str; 4] = ["fw-bold","text-center","border-bottom","p-1"];
pub const BOLD_BB_T2_PSB2: [&str; 5] = ["fw-bold","border-bottom","mb-2","pb-2","ps-2"];

pub const BOLD_BLUE: [&str; 3] = ["fw-bold","text-primary","me-1"];
pub const BOLD_BLUE_EM: [&str; 2] = ["fw-bold","text-primary-emphasis"];
pub const BOLD_BLUE_EM_L: [&str; 3] = ["fw-bold","text-primary-emphasis","me-1"];
pub const BOLD_GOLD: [&str; 3] = ["fw-bold","text-warning","me-1"];
pub const BOLD_GREEN: [&str; 3] = ["fw-bold","text-success","me-1"];
pub const BOLD_RED: [&str; 2] = ["fw-bold","text-danger"];
pub const BOLD_RED_L: [&str; 3] = ["fw-bold","text-danger","me-1"];
pub const BOLD_RED_R: [&str; 4] = ["fw-bold","text-danger","float-end","me-1"];
// pub const BOLD_WHITE: [&str;3] = ["fw-bold","text-white","me-1"];

pub const BOLD_BG_CYAN: [&str; 2] = ["fw-bold","text-bg-info"];
pub const BOLD_BG_GOLD: [&str; 2] = ["fw-bold","text-bg-warning"];
pub const BOLD_BG_GRAY: [&str; 2] = ["fw-bold","bg-secondary-subtle"];
pub const BOLD_BG_GREEN: [&str; 2] = ["fw-bold","text-bg-success"];

pub const BORDER_LT_BLUE: [&str; 4] = ["border","border-primary","me-1","mb-1"];
// pub const BORDER_LT_GOLD: [&str;4] = ["border","border-warning","me-1","mb-1"];
// pub const BORDER_LT_RED: [&str;4] = ["border","border-danger","me-1","mb-1"];
pub const BORDER3_RED: [&str; 2] = ["border-3","border-danger"];
pub const BORDER_SIDE: [&str; 3] = ["border-start","border-end","px-2"];
pub const BORDER_U_RB: [&str; 6] = ["border","border-top-0","rounded-bottom","p-2","mb-2","ms-3"];
pub const BORDER_U_FLEX: [&str; 6] = ["d-flex","border","border-top-0","rounded-bottom","p-1","mb-1"];

pub const BORDER_LB: [&str; 3] = ["border-start","border-bottom","ps-1"];
pub const BORDER_RB: [&str; 2] = ["border-end","border-bottom"];
pub const BORDER_T_Y: [&str; 3] = ["border-top","pt-1","my-2"];
pub const BORDER_T_T: [&str; 3] = ["border-top","pt-1","mt-2"];
pub const BORDER_T2_Y: [&str; 3] = ["border-top","pt-2","my-2"];
pub const BORDER_ROUND: [&str; 4] = ["border","rounded-3","my-2","p-2"];
pub const BORDER_ROUND_SMALL_BG_GOLD: [&str; 6] = ["border","bg-warning-subtle","rounded","small","my-1","p-2"];
pub const BORDER_SMALL_BG_CYAN: [&str; 5] = ["border","bg-info-subtle","small","p-2","mt-1"];
pub const BORDER_SMALL_BG_GOLD: [&str; 5] = ["border","bg-warning-subtle","small","p-2","mt-1"];
pub const BORDER_SMALL_BG_RED: [&str; 5] = ["border","bg-danger-subtle","small","p-2","mt-1"];

pub const BOX_INLINE_ROUND: [&str; 7] = ["d-inline-block","shadow-sm","border","rounded-3","mb-1","ms-1","px-2"];

pub const BOX_ROUND: [&str; 4] = ["shadow-sm","border","rounded-3","p-2"];
pub const BOX_ROUND_T: [&str; 5] = ["shadow-sm","border","rounded-3","mb-2","p-2"];
pub const BOX_ROUND_BLUES_T: [&str; 6] = ["shadow-sm","border","border-primary-subtle","rounded-3","mb-2","p-2"];
pub const BOX_ROUND_MT1_MX2: [&str; 4] = ["border","rounded","mt-1","mx-2"];
pub const BOX_ROUND_P1_T: [&str; 4] = ["border","rounded","p-1","mb-2"];
pub const BOX_ROUND_SMALL_R: [&str; 6] = ["border","border-bottom-0","rounded-top-3","small","ms-3","px-2"];
pub const BOX_ROUND_SMALL_BOLD_PX2: [&str; 5] = ["border","rounded-3","fw-bold","small","px-2"];
pub const BOX_ROUND_DARKS_BOLD_R_P1: [&str; 6] = ["border","border-dark-subtle","rounded","fw-bold","ms-1","p-1"];
pub const BOX_ROUND_DARKS_BOLD_R_PX3: [&str; 7] = ["border","border-dark-subtle","rounded","fw-bold","ms-1","px-3","py-1"];
pub const BOX_T: [&str; 4] = ["shadow-sm","border","mb-2","p-2"];

pub const BTN_CLOSE_M3: [&str; 2] = ["btn-close","m-3"];

pub const BTN_BLUE: [&str; 2] = ["btn","btn-primary"];
pub const BTN_BLUEO: [&str; 2] = ["btn","btn-outline-primary"];
pub const BTN_CYAN: [&str; 2] = ["btn","btn-info"];
pub const BTN_GOLD: [&str; 2] = ["btn","btn-warning"];
pub const BTN_GRAY: [&str; 2] = ["btn","btn-secondary"];
pub const BTN_GREENO: [&str; 2] = ["btn","btn-outline-success"];
pub const BTN_RED: [&str; 2] = ["btn","btn-danger"];
pub const BTN_RED75: [&str; 3] = ["btn","btn-danger","opacity-75"];
pub const BTN_REDO: [&str; 2] = ["btn","btn-outline-danger"];
pub const BTN_GRAYO_P0: [&str; 3] = ["btn","btn-outline-secondary","p-0"];

pub const BTN_DROP_TGL_CYAN: [&str; 3] = ["btn","btn-info","dropdown-toggle"];

pub const BTN_P0: [&str; 2] = ["btn","p-0"];
pub const BTN_L: [&str; 2] = ["btn","me-1"];
pub const BTN_REL_L: [&str; 3] = ["btn","position-relative","me-1"];
pub const BTN_R: [&str; 2] = ["btn","ms-1"];

pub const BTN_B_GRAY: [&str; 3] = ["btn","btn-secondary","mt-1"];

pub const BTN_L_BLUE: [&str; 3] = ["btn","btn-primary","me-1"];
pub const BTN_L_CYAN: [&str; 3] = ["btn","btn-info","me-1"];
pub const BTN_L_GRAY: [&str; 3] = ["btn","btn-secondary","me-1"];
pub const BTN_L_RED: [&str; 3] = ["btn","btn-danger","me-1"];
pub const BTN_L_REDO: [&str; 3] = ["btn","btn-outline-danger","me-1"];

pub const BTN_R_BLUE: [&str; 3] = ["btn","btn-primary","ms-1"];
pub const BTN_R_CYAN: [&str; 3] = ["btn","btn-info","ms-1"];
pub const BTN_R_GRAY: [&str; 3] = ["btn","btn-secondary","ms-1"];
pub const BTN_R_RED: [&str; 3] = ["btn","btn-danger","ms-1"];
pub const BTN_R_REDO: [&str; 3] = ["btn","btn-outline-danger","ms-1"];

pub const BTN_LT: [&str; 3] = ["btn","mb-1","me-1"];
pub const BTN_LT_BLUE: [&str; 4] = ["btn","btn-primary","mb-1","me-1"];
pub const BTN_LT_CYAN: [&str; 4] = ["btn","btn-info","mb-1","me-1"];
pub const BTN_LT_GRAY: [&str; 4] = ["btn","btn-secondary","mb-1","me-1"];
pub const BTN_LB2: [&str; 3] = ["btn","mt-2","me-2"];
pub const BTN_LB2_BLUE: [&str; 4] = ["btn","btn-primary","mt-2","me-2"];
pub const BTN_LB_GRAY: [&str; 4] = ["btn","btn-secondary","mt-1","me-1"];
pub const BTN_LX_RED: [&str; 3] = ["btn","btn-danger","me-auto"];
pub const BTN_RX_BLUE: [&str; 3] = ["btn","btn-primary","ms-auto"];

pub const BTN_T_W100: [&str; 4] = ["btn","text-center","w-100","mb-1"];
pub const BTN_T_BLUE: [&str; 3] = ["btn","btn-primary","mb-2"];
pub const BTN_T_BLUEO: [&str; 3] = ["btn","btn-outline-primary","mb-1"];
pub const BTN_T_REDO: [&str; 3] = ["btn","btn-outline-danger","mb-1"];

pub const BTN_LG_CTRL_BLUE: [&str; 4] = ["btn","btn-primary","btn-lg","form-control"];
pub const BTN_LG_CTRL_GOLD: [&str; 4] = ["btn","btn-warning","btn-lg","form-control"];

pub const BTN_FL_GRAY: [&str; 3] = ["btn","btn-secondary","float-start"];

pub const BTN_FR_L: [&str; 3] = ["btn","float-end","me-1"];
pub const BTN_FR_LB: [&str; 4] = ["btn","float-end","mt-1","me-1"];
pub const BTN_FR_LT_REDO: [&str; 5] = ["btn","btn-outline-danger","float-end","me-1","mb-1"];
pub const BTN_FR_R: [&str;3] = ["btn","float-end","ms-1"];
pub const BTN_FR_L_BLUE: [&str; 4] = ["btn","btn-primary","float-end","me-1"];
pub const BTN_FR_R_BLUE: [&str; 4] = ["btn","btn-primary","float-end","ms-1"];
pub const BTN_FR_R_GOLD: [&str; 4] = ["btn","btn-warning","float-end","ms-1"];
pub const BTN_FR_GRAY: [&str; 3] = ["btn","btn-secondary","float-end"];
pub const BTN_FR_R_GRAY: [&str; 4] = ["btn","btn-secondary","float-end","ms-1"];
pub const BTN_FR_B_GRAY: [&str; 4] = ["btn","btn-secondary","float-end","mt-1"];

pub const BTN_SM: [&str; 2] = ["btn","btn-sm"];
pub const BTN_SM_FULL: [&str; 4] = ["btn","btn-sm","border-0","w-100"];

pub const BTN_SM_BLUE: [&str; 3] = ["btn","btn-sm","btn-primary"];
pub const BTN_SM_BLUEO: [&str; 3] = ["btn","btn-sm","btn-outline-primary"];
pub const BTN_SM_CYAN: [&str; 3] = ["btn","btn-sm","btn-info"];
pub const BTN_SM_GRAY: [&str; 3] = ["btn","btn-sm","btn-secondary"];
pub const BTN_SM_GOLD: [&str;3] = ["btn","btn-sm","btn-warning"];
pub const BTN_SM_RED: [&str; 3] = ["btn","btn-sm","btn-danger"];
pub const BTN_SM_REDO: [&str; 3] = ["btn","btn-sm","btn-outline-danger"];
pub const BTN_SM_WHITEO: [&str; 3] = ["btn","btn-sm","btn-outline-light"];

pub const BTN_SM_L: [&str; 3] = ["btn","btn-sm","me-1"];
pub const BTN_SM_L_BLUE: [&str; 4] = ["btn","btn-sm","btn-primary","me-1"];
pub const BTN_SM_L_BLUEO: [&str; 4] = ["btn","btn-sm","btn-outline-primary","me-1"];
pub const BTN_SM_L_GRAY: [&str; 4] = ["btn","btn-sm","btn-secondary","me-1"];
pub const BTN_SM_L_RED: [&str; 4] = ["btn","btn-sm","btn-danger","me-1"];
pub const BTN_SM_LT: [&str; 4] = ["btn","btn-sm","mb-2","me-1"];
pub const BTN_SM_LT_CYAN: [&str; 5] = ["btn","btn-sm","btn-info","mb-2","me-1"];
pub const BTN_SM_LT_GRAY: [&str; 5] = ["btn","btn-sm","btn-secondary","mb-2","me-1"];
pub const BTN_SM_R_BLUE: [&str; 4] = ["btn","btn-sm","btn-primary","ms-1"];
pub const BTN_SM_R_BLUEO: [&str; 4] = ["btn","btn-sm","btn-outline-primary","ms-1"];
pub const BTN_SM_R_CYAN: [&str; 4] = ["btn","btn-sm","btn-info","ms-1"];
pub const BTN_SM_R_GRAY: [&str; 4] = ["btn","btn-sm","btn-secondary","ms-1"];
pub const BTN_SM_R_RED: [&str; 4] = ["btn","btn-sm","btn-danger","ms-1"];
pub const BTN_SM_R_REDO: [&str; 4] = ["btn","btn-sm","btn-outline-danger","ms-1"];
pub const BTN_SM_RB_BLUE: [&str; 5] = ["btn","btn-sm","btn-primary","mt-2","ms-1"];
pub const BTN_SM_RB_CYAN: [&str; 5] = ["btn","btn-sm","btn-info","mt-2","ms-1"];
pub const BTN_SM_RB_GRAY: [&str; 5] = ["btn","btn-sm","btn-secondary","mt-2","ms-1"];
pub const BTN_SM_RB_GOLD: [&str; 5] = ["btn","btn-sm","btn-warning","mt-2","ms-1"];
pub const BTN_SM_RB_RED: [&str; 5] = ["btn","btn-sm","btn-danger","mt-2","ms-1"];
pub const BTN_SM_RT: [&str; 4] = ["btn","btn-sm","mb-1","ms-1"];
pub const BTN_SM_RT_GRAY: [&str; 5] = ["btn","btn-sm","btn-secondary","mb-2","ms-1"];
pub const BTN_SM_RT_BLUE: [&str; 5] = ["btn","btn-sm","btn-primary","mb-1","ms-1"];
pub const BTN_SM_RT_BLUEO: [&str; 5] = ["btn","btn-sm","btn-outline-primary","mb-1","ms-1"];
pub const BTN_SM_RT_CYAN: [&str; 5] = ["btn","btn-sm","btn-info","mb-1","ms-1"];
pub const BTN_SM_RT_GREENO: [&str; 5] = ["btn","btn-sm","btn-outline-success","mb-1","ms-1"];
pub const BTN_SM_RT_REDO: [&str; 5] = ["btn","btn-sm","btn-outline-danger","mb-1","ms-1"];
pub const BTN_SM_T_GRAY: [&str; 4] = ["btn","btn-sm","btn-secondary","mb-1"];

pub const BTN_SM_FL_BLUEO: [&str; 4] =["btn","btn-sm","btn-outline-primary","float-start"];
pub const BTN_SM_FR_BLUE: [&str; 4] =["btn","btn-sm","btn-primary","float-end"];
pub const BTN_SM_FR_BLUEO: [&str; 4] =["btn","btn-sm","btn-outline-primary","float-end"];
pub const BTN_SM_FR_T_BLUEO: [&str; 5] =["btn","btn-sm","btn-outline-primary","float-end","mt-1"];
pub const BTN_SM_FR_GRAY: [&str; 5] = ["btn","btn-sm","btn-secondary","float-end","ms-1"];
pub const BTN_SM_FR_GRAY_P10: [&str; 7] = ["btn","btn-sm","btn-secondary","float-end","ms-1","py-0","px-1"];
pub const BTN_SM_FR_GREEN_P10: [&str; 7] = ["btn","btn-sm","btn-success","float-end","ms-1","py-0","px-1"];
pub const BTN_SM_FR_RED: [&str; 5] = ["btn","btn-sm","btn-danger","float-end","ms-1"];
pub const BTN_SM_FR_L: [&str; 4] = ["btn","btn-sm","float-end","me-1"];
pub const BTN_SM_FR_R: [&str; 4] = ["btn","btn-sm","float-end","ms-1"];
pub const BTN_SM_FR_RT: [&str; 5] = ["btn","btn-sm","float-end","mt-1","ms-1"];
pub const BTN_SM_FR_RT_CYAN: [&str; 6] =["btn","btn-sm","btn-info","float-end","mt-1","ms-1"];
pub const BTN_SM_FR_RT_BLUEO: [&str; 6] =["btn","btn-sm","btn-outline-primary","float-end","mt-1","ms-1"];

pub const BTN_GROUP_SM: [&str; 2] = ["btn-group","btn-group-sm"];
pub const BTN_GROUP_T: [&str; 2] = ["btn-group","mb-2"];
pub const BTN_GROUP_WHITE_R: [&str; 3] = ["btn-group","bg-white","ms-1"];

pub const CARD: [&str; 2] = ["card","mb-2"];
pub const CARD_P: [&str; 3] = ["card","p-3","mb-2"];
pub const CARD_BBLUE: [&str; 2] = ["card","border-primary"];
pub const CARD_BCYAN: [&str; 2] = ["card","border-info"];
pub const CARD_BCYAN_T: [&str; 3] = ["card","border-info","mb-2"];
pub const CARD_BGREEN: [&str; 3] = ["card","border-success","mb-2"];
pub const CARD_BDARKS: [&str; 3] = ["card","border-dark-subtle","mb-2"];
pub const CARD_BRED: [&str; 3] = ["card","border-danger","mb-2"];
pub const CARD_SHADOW: [&str; 2] = ["card","shadow-sm"];
pub const CARD_CYANS: [&str; 3] = ["card","bg-info-subtle","p-2"];
pub const CARD_TW_T_CYANS: [&str; 5] = ["card","text-white","bg-info-subtle","p-2","mb-1"];
pub const CARD_HEAD: [&str; 2] = ["card-header","fw-bold"];
pub const CARD_HEAD_P2: [&str; 2] = ["card-header","p-2"];
pub const CARD_HEAD_BB0_P2: [&str; 3] = ["card-header","border-bottom-0","p-2"];
pub const CARD_HEAD_CYANS: [&str; 2] = ["card-header","bg-info-subtle"];
pub const CARD_HEAD_BDARKS_CYANS: [&str; 3] =["card-header","border-dark-subtle","bg-info-subtle"];
pub const CARD_HEAD_BDARKS_GREENS: [&str; 3] =["card-header","border-dark-subtle","bg-success-subtle"];
pub const CARD_HEAD_BDARKS_GOLDS: [&str; 3] =["card-header","border-dark-subtle","bg-warning-subtle"];
pub const CARD_HEAD_BDARKS_GRAYS: [&str; 3] =["card-header","border-dark-subtle","bg-secondary-subtle"];
pub const CARD_HEAD_BDARKS_LIGHTS: [&str; 3] =["card-header","border-dark-subtle","bg-light-subtle"];
pub const CARD_HEAD_BDARKS_REDS: [&str; 3] =["card-header","border-dark-subtle","bg-danger-subtle"];
pub const CARD_BODY_FLEX: [&str; 2] = ["card-body","d-flex"];
pub const CARD_BODY_P0: [&str; 2] = ["card-body","p-0"];
pub const CARD_BODY_P1: [&str; 2] = ["card-body","p-1"];
pub const CARD_BODY_P2: [&str; 2] = ["card-body","p-2"];
pub const CARD_BODY_CYANS: [&str; 2] = ["card-body","bg-info-subtle"];
pub const CARD_BODY_LIGHTS: [&str; 2] = ["card-body","bg-light-subtle"];
pub const CARD_FOOT_R: [&str; 2] = ["card-footer","text-end"];
pub const CARD_FOOT_BT0: [&str; 2] = ["card-footer","border-top-0"];
pub const CARD_GROUP_T: [&str; 2] = ["card-group","pb-3"];

pub const CIRCLE_L: [&str; 2] = ["rounded-circle","me-2"];
pub const CIRCLE_L_PX: [&str; 3] = ["rounded-circle","me-1","px-2"];
pub const CONF_P3: [&str; 2] = ["container-fluid","p-3"];
pub const CONF_B: [&str; 2] = ["container-fluid","mt-3"];

pub const COL_PS0: [&str; 2] = ["col","ps-0"];
pub const COL_R: [&str; 2] = ["col","text-end"];
pub const COL_T: [&str; 2] = ["col","mb-2"];
pub const COL12_B: [&str; 2] = ["col-12","mt-1"];
pub const COL12_T: [&str; 2] = ["col-12","mb-2"];
pub const COL12_Y: [&str; 2] = ["col-12","my-2"];
pub const COL12_RRX: [&str; 3] = ["col-12","ms-auto","text-end"];

pub const COL_MD3_P0: [&str; 2] = ["col-md-3","p-0"];
pub const COL_MD4_T: [&str; 2] = ["col-md-4","mb-2"];
pub const COL_MD6_T: [&str; 2] = ["col-md-6","mb-2"];
// pub const COL_MD6_MID: [&str; 2] = ["col-md-6","offset-md-2"];
pub const COL_MD9_B: [&str; 2] = ["col-md-9","mt-1"];
pub const COL_MD12_R: [&str; 2] = ["col-md-12","text-end"];
pub const COL_MD12_RT: [&str; 3] = ["col-md-12","text-end","mb-2"];
pub const COL_MD12_T: [&str; 2] = ["col-md-12","mb-2"];
pub const COL_MD12_PY2: [&str; 2] = ["col-md-12","py-2"];
pub const COL_MD12_C_RED: [&str; 3] = ["col-md-12","text-center","text-danger"];

pub const COL_SM2_BOLD_R: [&str; 3] = ["col-sm-2","fw-bold","text-end"];
pub const COL_SM12_BOLD: [&str; 2] = ["col-sm-12","fw-bold"];
pub const COL_SM12_BOLD_C_FS5_T: [&str; 5] = ["col-sm-12","fw-bold","text-center","fs-5","mb-3"];

pub const COL_SM1_PT1: [&str; 2] = ["col-sm-1","pt-1"];
pub const COL_SM2_PT1: [&str; 2] = ["col-sm-2","pt-1"];
pub const COL_SM3_PX0: [&str; 2] = ["col-sm-3","px-0"];
pub const COL_SM7_PX0: [&str; 2] = ["col-sm-7","px-0"];
pub const COL_SM4_P0: [&str; 2] = ["col-sm-4","p-0"];
pub const COL_SM5_B: [&str; 2] = ["col-sm-5","mt-1"];
pub const COL_SM5_P0S2: [&str; 3] = ["col-sm-5","p-0","ps-2"];
pub const COL_SM6_P0S2: [&str; 3] = ["col-sm-6","p-0","ps-2"];

pub const COL_SM1_OFS5: [&str; 2] = ["col-sm-1","offset-sm-5"];
pub const COL_SM1_OFS2_R: [&str; 3] = ["col-sm-1","offset-sm-2","text-end"];
pub const COL_SM2_OFS2: [&str; 2] = ["col-sm-2","offset-sm-2"];
pub const COL_SM2_OFS3: [&str; 2] = ["col-sm-2","offset-sm-3"];
pub const COL_SM3_OFS2: [&str; 2] = ["col-sm-3","offset-sm-2"];
pub const COL_SM5_OFSM4: [&str; 2] = ["col-sm-5","offset-md-4"];
pub const COL_SM6_OFSM5: [&str; 2] = ["col-sm-6","offset-md-5"];
pub const COL_SM7_OFSM2: [&str; 2] = ["col-sm-7","offset-md-2"];
pub const COL_SM11_OFSM1: [&str; 2] = ["col-sm-11","offset-sm-1"];

pub const COL_SM6_MID: [&str; 2] = ["col-sm-6","offset-sm-3"];

pub const COL_SM1_R: [&str; 2] = ["col-sm-1","text-end"];
pub const COL_SM2_R: [&str; 2] = ["col-sm-2","text-end"];
pub const COL_SM3_R: [&str; 2] = ["col-sm-3","text-end"];
pub const COL_SM3_RB: [&str; 3] = ["col-sm-3","text-end","mt-1"];
pub const COL_SM3_R_P0: [&str; 3] = ["col-sm-3","text-end","p-0"];
pub const COL_SM4_R: [&str; 2] = ["col-sm-4","text-end"];
pub const COL_SM5_R: [&str; 2] = ["col-sm-5","text-end"];
pub const COL_SM5_RB: [&str; 3] = ["col-sm-5","text-end","mt-1"];
pub const COL_SM12_R: [&str; 2] = ["col-sm-12","text-end"];
pub const COL_SM12_C: [&str; 2] = ["col-sm-12","text-center"];

pub const COL_SM4_RE0: [&str; 2] = ["col-sm-4","rounded-end-0"];
pub const COL_SM8_RE0: [&str; 2] = ["col-sm-8","rounded-end-0"];

pub const COL_MD1_Y1: [&str; 2] = ["col-md-1","my-1"];
pub const COL_MD2_Y1: [&str; 2] = ["col-md-2","my-1"];
pub const COL_MD2_OFSM1: [&str; 2] = ["col-md-2","offset-md-1"];
pub const COL_MD2_OFSM1_Y1: [&str; 3] = ["col-md-2","offset-md-1","my-1"];
pub const COL_MD4_OFSM1_Y1: [&str; 3] = ["col-md-4","offset-md-1","my-1"];

pub const COLA_B: [&str; 2] = ["col-auto","mt-1"];
pub const COLA_P: [&str; 2] = ["col-auto","p-1"];
pub const COLA_C: [&str; 2] = ["col-auto","text-center"];
pub const COLA_BOLD_P2: [&str; 3] = ["col-auto","fw-bold","p-2"];
pub const COLA_BOLD_R: [&str; 3] = ["col-auto","fw-bold","text-end"];
pub const COLA_T: [&str; 2] = ["col-auto","mb-2"];
pub const COLA_PX_T_PE0: [&str; 3] = ["col-auto","mb-2","pe-0"];
pub const COLA_PX_T: [&str; 3] = ["col-auto","mb-2","px-1"];
pub const COLA_PY_L: [&str; 3] = ["col-auto","me-2","py-1"];
pub const COLA_PY_RX: [&str; 3] = ["col-auto","ms-auto","py-1"];
pub const COLA_MY1: [&str; 2] = ["col-auto","my-1"];

pub const DROP_HEAD_BOLD: [&str; 4] = ["dropdown-header","fw-bold","fs-6","pb-1"];
pub const DROP_MENU_END: [&str; 2] = ["dropdown-menu","dropdown-menu-end"];
pub const DROP_ITEM_TXT_NOWRAP: [&str; 2] = ["dropdown-item-text","text-nowrap"];

pub const EDITOR_ROUND_T: [&str; 3] = ["editor-pre","rounded","mb-2"];

pub const FA_ALERT: [&str; 2] = ["fa-solid","fa-triangle-exclamation"];
pub const FA_ALERT_GOLD: [&str; 3] = ["fa-solid","fa-triangle-exclamation","text-warning"];
pub const FA_ALERT_RED: [&str; 3] = ["fa-solid","fa-triangle-exclamation","text-danger"];
pub const FA_AMBULANCE: [&str; 2] = ["fa-solid","fa-truck-medical"];
pub const FA_ARROW_LR: [&str; 2] = ["fa-solid","fa-arrows-left-right-to-line"];
pub const FA_ASTERISK_RED: [&str; 3] = ["fa-solid","fa-asterisk","text-danger"];
pub const FA_BABY: [&str; 2] = ["fa-solid","fa-baby"];
pub const FA_BACKWARD: [&str; 2] = ["fa-solid","fa-backward"];
pub const FA_BED: [&str; 2] = ["fa-solid","fa-bed"];
pub const FA_BOLT_L: [&str; 3] = ["fa-solid","fa-bolt","me-1"];
pub const FA_BOOK_MED: [&str; 2] = ["fa-solid","fa-book-medical"];
pub const FA_BRAIN: [&str; 2] = ["fa-solid","fa-brain"];
pub const FA_BULLHORN: [&str; 2] = ["fa-solid","fa-bullhorn"];
pub const FA_CAMERA: [&str; 2] = ["fa-regular","fa-camera"];
pub const FA_CARD: [&str; 2] = ["fa-regular","fa-address-card"];
pub const FA_CART: [&str; 2] = ["fa-solid","fa-cart-plus"];
pub const FA_CALCULATOR: [&str; 2] = ["fa-solid","fa-calculator"];
pub const FA_CALENDAR: [&str; 2] = ["fa-regular","fa-calendar"];
pub const FA_CALENDARS: [&str; 2] = ["fa-regular","fa-calendar-days"];
pub const FA_CAPSULE: [&str; 2] = ["fa-solid","fa-capsules"];
pub const FA_CHECK: [&str; 2] = ["fa-solid","fa-check"];
pub const FA_CHECK_GREEN: [&str; 3] = ["fa-solid","fa-check","text-success"];
pub const FA_CHECK_GREEN_R: [&str; 4] = ["fa-solid","fa-check","text-success","ms-1"];
pub const FA_CHECK_CIRCLE: [&str; 2] = ["fa-regular","fa-circle-check"];
pub const FA_CHECK_CIRCLE_L: [&str; 3] = ["fa-regular","fa-circle-check","me-1"];
pub const FA_CHECK_CIRCLE_BLUE: [&str; 3] = ["fa-regular","fa-circle-check","text-primary"];
pub const FA_CHECK_CIRCLE_GREEN: [&str; 3] = ["fa-regular","fa-circle-check","text-success"];
pub const FA_CHECK_CIRCLE_GOLD: [&str; 3] = ["fa-regular","fa-circle-check","text-warning"];
pub const FA_CHECK_CIRCLE_RED: [&str; 3] = ["fa-regular","fa-circle-check","text-danger"];
pub const FA_CHILD: [&str; 2] = ["fa-solid","fa-child"];
pub const FA_CIRCLE_CYAN: [&str; 3] = ["fa-solid","fa-circle","text-info"];
pub const FA_CIRCLE_GRAY: [&str; 3] = ["fa-solid","fa-circle","text-secondary"];
pub const FA_CIRCLE_GREEN: [&str; 3] = ["fa-solid","fa-circle","text-success"];
pub const FA_CIRCLE_RED: [&str; 3] = ["fa-solid","fa-circle","text-danger"];
pub const FA_CIRCLE_NOTCH: [&str; 2] = ["fa-solid","fa-circle-notch"];
pub const FA_CLIPBOARD: [&str; 2] = ["fa-regular","fa-clipboard"];
pub const FA_CLIPBOARD_R: [&str; 3] = ["fa-regular","fa-clipboard","ms-1"];
pub const FA_CLOCK: [&str; 2] = ["fa-regular","fa-clock"];
pub const FA_CLOCK_L_ROTATE: [&str; 2] = ["fa-solid","fa-clock-rotate-left"];
pub const FA_CLONE: [&str; 2] = ["fa-regular","fa-clone"];
pub const FA_COG: [&str; 2] = ["fa-solid","fa-gear"];
pub const FA_COG_L: [&str; 3] = ["fa-solid","fa-gear","me-1"];
pub const FA_COL1: [&str; 2] = ["fa-regular","fa-window-maximize"];
pub const FA_COL2: [&str; 2] = ["fa-solid","fa-table-columns"];
pub const FA_COMMENTS: [&str; 2] = ["fa-regular","fa-comments"];
pub const FA_CROSS_RED: [&str; 3] = ["fa-solid","fa-cross","text-danger"];
pub const FA_DISPLAY: [&str; 2] = ["fa-solid","fa-display"];
pub const FA_DOWN: [&str; 2] = ["fa-solid","fa-arrow-down"];
pub const FA_DOWN19: [&str; 2] = ["fa-solid","fa-arrow-down-1-9"];
pub const FA_DOWNLOAD: [&str; 2] = ["fa-solid","fa-download"];
pub const FA_DROPLET: [&str; 2] = ["fa-solid","fa-droplet"];
pub const FA_EDIT: [&str; 2] = ["fa-regular","fa-pen-to-square"];
pub const FA_ENV: [&str; 2] = ["fa-regular","fa-envelope"];
// pub const FA_ENV_OPEN: [&str;2] = ["fa-regular","fa-envelope-open"];
pub const FA_ERASER: [&str; 2] = ["fa-solid","fa-eraser"];
pub const FA_EXT_LINK: [&str; 2] = ["fa-solid","fa-up-right-from-square"];
pub const FA_EXT_LINK_R: [&str; 3] = ["fa-solid","fa-up-right-from-square","ms-1"];
pub const FA_EYE: [&str; 2] = ["fa-regular","fa-eye"];
pub const FA_EYE_SLASH: [&str; 2] = ["fa-regular","fa-eye-slash"];
pub const FA_FACE_ANGRY: [&str; 2] = ["fa-regular","fa-face-angry"];
pub const FA_FILE: [&str; 2] = ["fa-regular","fa-file"];
pub const FA_FILE_L: [&str; 3] = ["fa-regular","fa-file","me-1"];
pub const FA_FILE_R: [&str; 3] = ["fa-regular","fa-file","ms-1"];
pub const FA_FILE_PDF: [&str; 2] = ["fa-regular","fa-file-pdf"];
pub const FA_FILE_PDF_L: [&str; 3] = ["fa-regular","fa-file-pdf","me-1"];
pub const FA_FILE_PDF_R: [&str; 3] = ["fa-regular","fa-file-pdf","ms-1"];
pub const FA_FILE_RX: [&str; 2] = ["fa-solid","fa-file-prescription"];
pub const FA_FILE_MONEY: [&str; 2] = ["fa-solid","fa-file-invoice-dollar"];
pub const FA_FORWARD: [&str; 2] = ["fa-solid","fa-forward"];
pub const FA_FLASK: [&str; 2] = ["fa-solid","fa-flask"];
pub const FA_HAND_MED: [&str; 2] = ["fa-solid","fa-hand-holding-medical"];
pub const FA_HASHTAG: [&str; 2] = ["fa-solid","fa-hashtag"];
pub const FA_HEART: [&str; 2] = ["fa-regular","fa-heart"];
pub const FA_HEARTBEAT: [&str; 2] = ["fa-solid","fa-heart-pulse"];
pub const FA_HOURGLASS: [&str; 2] = ["fa-regular","fa-hourglass-half"];
pub const FA_HOURGLASS_GREEN: [&str; 3] = ["fa-regular","fa-hourglass-half","text-success"];
pub const FA_HOURGLASS_GOLD: [&str; 3] = ["fa-regular","fa-hourglass-half","text-warning"];
pub const FA_HOURGLASS_RED: [&str; 3] = ["fa-regular","fa-hourglass-half","text-danger"];
pub const FA_HOUSE: [&str; 2] = ["fa-regular","fa-house"];
pub const FA_IMAGE: [&str; 2] = ["fa-regular","fa-image"];
pub const FA_INFO: [&str; 2] = ["fa-regular","fa-comment-dots"];
pub const FA_KEYBOARD: [&str; 2] = ["fa-regular","fa-keyboard"];
pub const FA_L_ANGLE: [&str; 2] = ["fa-solid","fa-angle-left"];
pub const FA_L_ANGLES: [&str; 2] = ["fa-solid","fa-angles-left"];
pub const FA_L_ARROW: [&str; 2] = ["fa-solid","fa-arrow-left"];
pub const FA_L_ARROW_CIRCLE: [&str; 2] = ["fa-regular","fa-circle-left"];
pub const FA_L_CARET: [&str; 2] = ["fa-solid","fa-caret-left"];
pub const FA_LINK_GREEN: [&str; 3] = ["fa-solid","fa-link","text-success"];
pub const FA_LIST: [&str; 2] = ["fa-solid","fa-list"];
pub const FA_LIST_CHECK: [&str; 2] = ["fa-solid","fa-list-check"];
pub const FA_MAGIC: [&str; 2] = ["fa-solid","fa-wand-magic-sparkles"];
pub const FA_MARKER: [&str; 2] = ["fa-solid","fa-marker"];
pub const FA_MAX: [&str; 2] = ["fa-solid","fa-expand"];
pub const FA_MEDICINE: [&str; 2] = ["fa-solid","fa-staff-snake"];
pub const FA_MIN: [&str; 2] = ["fa-solid","fa-compress"];
pub const FA_MINUS: [&str; 2] = ["fa-solid","fa-minus"];
pub const FA_MOON: [&str; 2] = ["fa-regular","fa-moon"];
pub const FA_NETWORK: [&str; 2] = ["fa-solid","fa-network-wired"];
pub const FA_NOTE_MED: [&str; 2] = ["fa-solid","fa-notes-medical"];
pub const FA_PASTE: [&str; 2] = ["fa-regular","fa-paste"];
pub const FA_PHONE: [&str; 2] = ["fa-solid","fa-phone"];
pub const FA_PILLS: [&str; 2] = ["fa-solid","fa-pills"];
pub const FA_PILLS_L: [&str; 3] = ["fa-solid","fa-pills","me-1"];
pub const FA_PLAY: [&str; 2] = ["fa-solid","fa-play"];
pub const FA_PLUS: [&str; 2] = ["fa-solid","fa-plus"];
pub const FA_PLUS_L: [&str; 3] = ["fa-solid","fa-plus","me-1"];
pub const FA_PLUS_SQ: [&str; 2] = ["fa-regular","fa-square-plus"];
pub const FA_PREGNANT: [&str; 2] = ["fa-solid","fa-person-pregnant"];
pub const FA_PRINT: [&str; 2] = ["fa-solid","fa-print"];
pub const FA_QUESTION: [&str; 2] = ["fa-regular","fa-circle-question"];
pub const FA_R_ANGLE: [&str; 2] = ["fa-solid","fa-angle-right"];
pub const FA_R_ANGLES: [&str; 2] = ["fa-solid","fa-angles-right"];
pub const FA_R_CARET: [&str; 2] = ["fa-solid","fa-caret-right"];
pub const FA_R_ARROW: [&str; 2] = ["fa-solid","fa-arrow-right"];
pub const FA_R_ARROW_CIRCLE: [&str; 2] = ["fa-regular","fa-circle-right"];
// pub const FA_REDO: [&str;2] = ["fa-solid","fa-arrow-rotate-right"];
pub const FA_REPLY: [&str; 2] = ["fa-solid","fa-reply"];
pub const FA_RX: [&str; 2] = ["fa-solid","fa-prescription"];
pub const FA_RX_L: [&str; 3] = ["fa-solid","fa-prescription","me-1"];
pub const FA_SAVE: [&str; 2] = ["fa-regular","fa-floppy-disk"];
pub const FA_SEARCH: [&str; 2] = ["fa-solid","fa-magnifying-glass"];
pub const FA_SEARCH_PLUS: [&str; 2] = ["fa-solid","fa-magnifying-glass-plus"];
pub const FA_SHARE: [&str; 2] = ["fa-regular","fa-share-from-square"];
pub const FA_SIGN_IN: [&str; 2] = ["fa-solid","fa-arrow-right-to-bracket"];
pub const FA_SIGN_OUT: [&str; 2] = ["fa-solid","fa-arrow-right-from-bracket"];
pub const FA_SKULL: [&str; 2] = ["fa-solid","fa-skull-crossbones"];
pub const FA_SORT: [&str; 2] = ["fa-solid","fa-sort"];
pub const FA_SPELL_CHECK: [&str; 2] = ["fa-solid","fa-spell-check"];
pub const FA_SPIN: [&str; 4] = ["fa-solid","fa-spinner","fa-spin","fa-pulse"];
pub const FA_SPIN_R: [&str; 5] = ["fa-solid","fa-spinner","fa-spin","fa-pulse","ms-1"];
pub const FA_STAR: [&str; 2] = ["fa-regular","fa-star"];
pub const FA_STAR_L: [&str; 3] = ["fa-solid","fa-star","me-1"];
pub const FA_STAR_GOLD: [&str; 3] = ["fa-solid","fa-star","text-warning"];
pub const FA_SUN: [&str; 2] = ["fa-regular","fa-sun"];
pub const FA_SYNC: [&str; 2] = ["fa-solid","fa-arrows-rotate"];
pub const FA_SYRINGE: [&str; 2] = ["fa-solid","fa-syringe"];
pub const FA_TABLE: [&str; 2] = ["fa-solid","fa-table-list"];
pub const FA_TH: [&str; 2] = ["fa-solid","fa-table-cells"];
pub const FA_TH_LG: [&str; 2] = ["fa-solid","fa-table-cells-large"];
pub const FA_TRASH: [&str; 2] = ["fa-regular","fa-trash-can"];
pub const FA_TREE: [&str; 2] = ["fa-solid","fa-folder-tree"];
pub const FA_VOL_UP: [&str; 2] = ["fa-solid","fa-volume-high"];
pub const FA_VOL_MUTE: [&str; 2] = ["fa-solid","fa-volume-xmark"];
pub const FA_UP: [&str; 2] = ["fa-solid","fa-arrow-up"];
pub const FA_UP91: [&str; 2] = ["fa-solid","fa-arrow-up-9-1"];
pub const FA_UNDO: [&str; 2] = ["fa-solid","fa-arrow-rotate-left"];
pub const FA_USER: [&str; 2] = ["fa-solid","fa-user"];
pub const FA_USER_R: [&str; 3] = ["fa-solid","fa-user","ms-1"];
// pub const FA_USER_CIRCLE: [&str;2] = ["fa-regular","fa-circle-user"];
pub const FA_USER_MD_L: [&str; 3] = ["fa-solid","fa-user-doctor","me-1"];
pub const FA_USER_COG: [&str; 2] = ["fa-solid","fa-user-gear"];
pub const FA_USER_CLOCK: [&str; 2] = ["fa-solid","fa-user-clock"];
pub const FA_USER_INJURED: [&str; 2] = ["fa-solid","fa-user-injured"];
pub const FA_USER_LOCK: [&str; 2] = ["fa-solid","fa-user-lock"];
pub const FA_USER_NURSE_L: [&str; 3] = ["fa-solid","fa-user-nurse","me-1"];
pub const FA_USER_SHIELD: [&str; 2] = ["fa-solid","fa-user-shield"];
pub const FA_USER_TIE_L: [&str; 3] = ["fa-solid","fa-user-tie","me-1"];
pub const FA_USERS: [&str; 2] = ["fa-solid","fa-users"];
pub const FA_WHEELCHAIR: [&str; 2] = ["fa-solid","fa-wheelchair"];
pub const FA_WRENCH: [&str; 2] = ["fa-solid","fa-wrench"];
pub const FA_X: [&str; 2] = ["fa-solid","fa-xmark"];
pub const FA_X_RED: [&str; 3] = ["fa-solid","fa-xmark","text-danger"];
pub const FA_X_CIRCLE_RED: [&str; 3] = ["fa-regular","fa-circle-xmark","text-danger"];
pub const FA_XRAY: [&str; 2] = ["fa-solid","fa-x-ray"];

pub const FLEX_W100: [&str; 2] = ["d-flex","w-100"];
pub const FLEX_FIX_B2: [&str; 3] = ["d-flex","position-absolute","mt-2"];
pub const FLEX_M: [&str; 2] = ["d-flex","m-2"];
pub const FLEX_MLX: [&str; 3] = ["d-flex","m-2","me-auto"];
pub const FLEX_MRX: [&str; 3] = ["d-flex","m-2","ms-auto"];
pub const FLEX_BOLD_C: [&str; 3] = ["d-flex","fw-bold","text-center"];
pub const FLEX_E2: [&str; 2] = ["d-flex","me-2"];
pub const FLEX_B2: [&str; 2] = ["d-flex","mt-2"];
pub const FLEX_T: [&str; 2] = ["d-flex","mb-1"];
pub const FLEX_T2: [&str; 2] = ["d-flex","mb-2"];
pub const FLEX_PY1: [&str; 2] = ["d-flex","py-1"];
pub const FLEX_PY1_RX: [&str; 3] = ["d-flex","py-1","ms-auto"];
pub const FLEX_C: [&str; 2] = ["d-flex","align-items-center"];
pub const FLEX_C_T: [&str; 3] = ["d-flex","align-items-center","mb-2"];
pub const FLEX_JCC: [&str; 3] = ["d-flex","justify-content-center","align-items-center"];
pub const FLEX_JCR: [&str; 2] = ["d-flex","justify-content-end"];
pub const FLEX_JCR_T: [&str; 3] = ["d-flex","justify-content-end","mb-2"];
pub const FLEX_WRAP: [&str; 2] = ["d-flex","flex-wrap"];
pub const FLEX_WRAP_ROW: [&str; 3] = ["d-flex","flex-wrap","row"];
pub const FLEX_WRAP_G3: [&str; 3] = ["d-flex","flex-wrap","gap-3"];
pub const FLEX_WRAP_T: [&str; 3] = ["d-flex","flex-wrap","mb-2"];
pub const FLEX_WRAP_JC_T: [&str; 4] = ["d-flex","flex-wrap","justify-content-center","mb-2"];
pub const FLEX_WRAP_AC: [&str; 3] = ["d-flex","flex-wrap","align-items-center"];
pub const FLEX_WRAP_AC_T: [&str; 4] = ["d-flex","flex-wrap","align-items-center","mb-2"];
pub const FLEX_GROW1: [&str; 2] = ["d-flex","flex-grow-1"];

pub const FLEX_COL: [&str; 2] = ["d-flex","flex-column"];
pub const FLEX_COL_C: [&str; 5] = ["d-flex","flex-column","align-items-center","justify-content-center","vh-100"];
pub const FLEX_COL_VC: [&str; 3] = ["d-flex","flex-column","justify-content-center"];

pub const FLEX_END: [&str; 2] = ["d-flex","float-end"];

pub const FLEX_ITEM_GROW1_L: [&str; 2] = ["flex-grow-1","me-1"];
pub const FLEX_ITEM_GROW1_L2: [&str; 2] = ["flex-grow-1","me-2"];
pub const FLEX_ITEM_GROW1_R2: [&str; 2] = ["flex-grow-1","ms-2"];
pub const FLEX_ITEM_FILL_R: [&str; 2] = ["flex-fill","ms-2"];

pub const FLOAT_L: [&str; 2] = ["float-start","me-2"];
pub const FLOAT_RB1: [&str; 2] = ["float-end","mt-1"];
pub const FLOAT_RB: [&str; 2] = ["float-end","mt-2"];
pub const FLOAT_RR: [&str; 2] = ["float-end","ms-2"];
pub const FLOAT_RRB1: [&str; 3] = ["float-end","ms-1","mt-1"];
pub const FLOAT_RR_Y1: [&str; 3] = ["float-end","ms-2","my-1"];
pub const FLOAT_RT: [&str; 2] = ["float-end","mb-2"];
pub const FLOAT_RL: [&str; 2] = ["float-end","me-2"];

pub const FORM_CTRL_B0: [&str; 3] = ["form-control","border-0","rounded-0"];
pub const FORM_CTRL_BG_GOLD: [&str; 2] = ["form-control","bg-warning-subtle"];
pub const FORM_CTRL_R: [&str; 2] = ["form-control","ms-2"];
pub const FORM_CTRL_T: [&str; 2] = ["form-control","mb-2"];
pub const FORM_CTRL_C: [&str; 2] = ["form-control","text-center"];
pub const FORM_CTRL_RED: [&str; 2] = ["form-control","text-danger"];
pub const FORM_CTRL_LG: [&str; 2] = ["form-control","form-control-lg"];
pub const FORM_CTRL_SM: [&str; 2] = ["form-control","form-control-sm"];
pub const FORM_CTRL_SM_B0: [&str; 4] = ["form-control","form-control-sm","border-0","rounded-0"];
pub const FORM_CTRL_SM_GOLD: [&str; 3] = ["form-control","form-control-sm","bg-warning-subtle"];
pub const FORM_CTRL_SM_T: [&str; 3] = ["form-control","form-control-sm","mb-1"];
pub const FORM_CTRL_SM_WRAP: [&str; 3] = ["form-control","form-control-sm","text-wrap"];
pub const FORM_CTRL_COL_SM2: [&str; 2] = ["form-control","col-sm-2"];
pub const FORM_CTRL_COL_SM4: [&str; 2] = ["form-control","col-sm-4"];
pub const FORM_CTRL_COL_SM8: [&str; 2] = ["form-control","col-sm-8"];
pub const FORM_CTRL_COL_MD12: [&str; 2] = ["form-control","col-md-12"];

// for date picker under "input-group" (already has "form-control" internally)
pub const B0R0: [&str; 2] = ["border-0","rounded-0"];
pub const FORM_CTRL_ONLY_SM_B0: [&str; 3] = ["form-control-sm","border-0","rounded-0"];
pub const FORM_CTRL_ONLY_SM_R0: [&str; 2] = ["form-control-sm","rounded-0"];
pub const FORM_CTRL_ONLY_SM_R0_L: [&str; 2] = ["form-control-sm","rounded-start-0"];
pub const FORM_CTRL_ONLY_SM_R0_R: [&str; 2] = ["form-control-sm","rounded-end-0"];

pub const FORM_CHK_R: [&str; 2] = ["form-check","ms-3"];
pub const FORM_CHK_T: [&str; 2] = ["form-check","mb-2"];
pub const FORM_CHK_PT: [&str; 2] = ["form-check","pt-2"];
pub const FORM_CHK_COL_SM1: [&str; 2] = ["form-check","col-sm-1"];
pub const FORM_CHK_COL_SM2: [&str; 2] = ["form-check","col-sm-2"];
pub const FORM_CHK_COL_SM3: [&str; 2] = ["form-check","col-sm-3"];
pub const FORM_CHK_COL_SM4: [&str; 2] = ["form-check","col-sm-4"];
pub const FORM_CHK_COL_SM5: [&str; 2] = ["form-check","col-sm-5"];
pub const FORM_CHK_COL_SM6: [&str; 2] = ["form-check","col-sm-6"];
pub const FORM_CHK_COL_SM12: [&str; 2] = ["form-check","col-sm-12"];
pub const FORM_CHK_COL_SM1_R: [&str; 3] = ["form-check","col-sm-1","ms-3"];
pub const FORM_CHK_COL_SM2_R: [&str; 3] = ["form-check","col-sm-2","ms-3"];
pub const FORM_CHK_COL_SM3_R: [&str; 3] = ["form-check","col-sm-3","ms-3"];
pub const FORM_CHK_COL_SM1_OFS1: [&str; 3] = ["form-check","col-sm-1","offset-sm-1"];
pub const FORM_CHK_COL_SM1_OFS2: [&str; 3] = ["form-check","col-sm-1","offset-sm-2"];
pub const FORM_CHK_COL_SM1_OFS3: [&str; 3] = ["form-check","col-sm-1","offset-sm-3"];
pub const FORM_CHK_COL_SM1_OFS4: [&str; 3] = ["form-check","col-sm-1","offset-sm-4"];
pub const FORM_CHK_COL_SM1_OFS5: [&str; 3] = ["form-check","col-sm-1","offset-sm-5"];
pub const FORM_CHK_COL_SM2_OFS1: [&str; 3] = ["form-check","col-sm-2","offset-sm-1"];
pub const FORM_CHK_COL_SM2_OFS2: [&str; 3] = ["form-check","col-sm-2","offset-sm-2"];
pub const FORM_CHK_COL_SM2_OFS3: [&str; 3] = ["form-check","col-sm-2","offset-sm-3"];
pub const FORM_CHK_COL_SM3_OFS1: [&str; 3] = ["form-check","col-sm-3","offset-sm-1"];
pub const FORM_CHK_INL: [&str; 2] = ["form-check","form-check-inline"];
pub const FORM_CHK_SW: [&str; 2] = ["form-check","form-switch"];
pub const FORM_CHK_SW_PT1: [&str; 3] = ["form-check","form-switch","pt-1"];
pub const FORM_CHK_LBL_R: [&str; 2] = ["form-check-label","ms-1"];
pub const FORM_CHK_LBL_NOWRAP: [&str; 2] = ["form-check-label","text-nowrap"];
pub const FORM_CHK_LBL_BOLD: [&str; 2] = ["form-check-label","fw-bold"];
pub const FORM_ICHK_SW: [&str; 2] = ["form-inline-check","form-switch"];

pub const FORM_LBL_BOLD: [&str; 2] = ["form-label","fw-bold"];
pub const FORM_COL_LBL_AUTO: [&str; 2] = ["col-form-label","col-auto"];
pub const FORM_COL_LBL_SM3_PT0: [&str; 3] = ["col-form-label","col-sm-3","pt-0"];
pub const FORM_COL_LBL_SM3_R: [&str; 3] = ["col-form-label","col-sm-3","text-end"];

pub const FORM_FLOAT_T: [&str; 2] = ["form-floating","mb-2"];

pub const FORM_SELECT_R0: [&str; 2] = ["form-select","rounded-0"];
pub const FORM_SELECT_X1: [&str; 2] = ["form-select","mx-1"];
pub const FORM_SELECT_X: [&str; 2] = ["form-select","mx-2"];
pub const FORM_SELECT_MONO: [&str; 2] = ["form-select","text-monospace"];
pub const FORM_SELECT_SM: [&str; 2] = ["form-select","form-select-sm"];
pub const FORM_SELECT_SM_CYAN: [&str; 3] = ["form-select","form-select-sm","bg-info-subtle"];
pub const FORM_SELECT_SM_R0: [&str; 3] = ["form-select","form-select-sm","rounded-0"];
pub const FORM_SELECT_SM_MONO: [&str; 3] = ["form-select","form-select-sm","text-monospace"];

pub const FORM_TEXT_R: [&str; 2] = ["form-text","text-end"];

pub const FULL: [&str; 2] = ["w-100","h-100"];
pub const FULL_P2: [&str; 3] = ["w-100","h-100","p-2"];

pub const INPUT_GROUP: [&str; 2] = ["input-group","flex-nowrap"];

pub const INPUT_GROUP_L: [&str; 3] = ["input-group","flex-nowrap","me-2"];
pub const INPUT_GROUP_T: [&str; 3] = ["input-group","flex-nowrap","mb-2"];
pub const INPUT_GROUP_TR: [&str; 4] = ["input-group","flex-nowrap","mb-1","ms-1"];
pub const INPUT_GROUP_JC: [&str; 3] = ["input-group","flex-nowrap","justify-content-center"];
pub const INPUT_GROUP_TEXT_BG_CYAN: [&str; 2] = ["input-group-text","bg-info"];
pub const INPUT_GROUP_TEXT_BG_CYANS: [&str; 2] = ["input-group-text","bg-info-subtle"];
pub const INPUT_GROUP_TEXT_BG_GOLD: [&str; 2] = ["input-group-text","text-bg-warning"];
pub const INPUT_GROUP_TEXT_BG_GOLDS: [&str; 2] = ["input-group-text","bg-warning-subtle"];
pub const INPUT_GROUP_TEXT_BG_GRAY: [&str; 2] = ["input-group-text","text-bg-secondary"];
pub const INPUT_GROUP_TEXT_BG_GREENS: [&str; 2] = ["input-group-text","bg-success-subtle"];
pub const INPUT_GROUP_TEXT_BG_REDS: [&str; 2] = ["input-group-text","bg-danger-subtle"];
pub const INPUT_GROUP_TEXT_PX1: [&str; 2] = ["input-group-text","px-1"];
pub const INPUT_GROUP_TEXT_PX1_BG_CYAN: [&str; 3] = ["input-group-text","bg-info","px-1"];
pub const INPUT_GROUP_TEXT_PX1_BG_GOLD: [&str; 3] = ["input-group-text","text-bg-warning","px-1"];
pub const INPUT_GROUP_TEXT_PX1_BG_GRAY: [&str; 3] = ["input-group-text","text-bg-secondary","px-1"];

pub const INPUT_GROUP_TEXT_T: [&str; 2] = ["input-group-text","mb-1"];
pub const INPUT_GROUP_TEXT_BOLD: [&str; 2] = ["input-group-text","fw-bold"];
pub const INPUT_GROUP_TEXT_SQ_BS0: [&str; 3] = ["input-group-text","rounded-0","border-start-0"];

pub const INPUT_GROUP_SM: [&str; 3] = ["input-group","input-group-sm","flex-nowrap"];
pub const INPUT_GROUP_SM_B2: [&str; 4] = ["input-group","input-group-sm","flex-nowrap","mt-2"];
pub const INPUT_GROUP_SM_B2_PX2: [&str; 5] = ["input-group","input-group-sm","flex-nowrap","mt-2","px-2"];
pub const INPUT_GROUP_SM_MC: [&str; 4] = ["input-group","input-group-sm","ms-auto","me-auto"];
pub const INPUT_GROUP_SM_T: [&str; 4] = ["input-group","input-group-sm","flex-nowrap","mb-1"];

pub const ITEMS_LEFT_B0: [&str; 2] = ["align-items-start","pb-0"];

pub const LIST_GROUP_FLUSH_OVFA: [&str; 4] = ["list-group","list-group-flush","overflow-auto","mb-1"];
pub const LIST_GROUP_ITEM_BOLD_C: [&str; 3] = ["list-group-item","fw-bold","text-center"];

pub const M0_P0: [&str; 2] = ["m-0","p-0"];
pub const M0_PY0: [&str; 2] = ["m-0","py-0"];
pub const M_LT: [&str; 2] = ["mb-3","me-2"];
pub const M_R2B1: [&str; 2] = ["ms-2","mt-1"];
pub const M_Y31: [&str; 2] = ["mt-3","mb-1"];
pub const M_YC: [&str; 2] = ["mt-auto","mb-auto"];
pub const P_L0: [&str; 2] = ["p-3","ps-0"];
pub const PY_L: [&str; 2] = ["py-1","me-2"];
pub const PY_RX: [&str; 2] = ["py-1","ms-auto"];

pub const MODAL_BODY_P0: [&str; 2] = ["modal-body","p-0"];
pub const MODAL_BODY_P2: [&str; 2] = ["modal-body","p-2"];
pub const MODAL_CONTENT_X: [&str; 2] = ["modal-content","mx-3"];
// "modal-dialog-scrollable",
pub const MODAL_DIALOG_C: [&str; 2] = ["modal-dialog","modal-dialog-centered"];
pub const MODAL_DIALOG_LG: [&str; 2] = ["modal-dialog","modal-lg"];
pub const MODAL_DIALOG_LG_FULL: [&str; 3] = ["modal-dialog","modal-lg","modal-fullscreen-lg-down"];
pub const MODAL_DIALOG_SM_C: [&str; 3] = ["modal-dialog","modal-sm","modal-dialog-centered"];
pub const MODAL_DIALOG_XL: [&str; 2] = ["modal-dialog","modal-xl"];
pub const MODAL_DIALOG_XL_FULL: [&str; 3] = ["modal-dialog","modal-xl","modal-fullscreen-xl-down"];

pub const MODAL_DIALOG_FULL: [&str; 2] = ["modal-dialog","modal-fullscreen"];
pub const MODAL_DIALOG_FULL_C: [&str; 4] =["modal-dialog","mw-100","w-100","modal-dialog-centered"];
pub const MODAL_FOOTER_C_P0: [&str; 3] = ["modal-footer","justify-content-center","p-0"];

pub const NAV_BAR_BLUE: [&str; 6] = ["navbar","navbar-expand-sm","navbar-dark","text-light","bg-primary","py-0"];
pub const NAV_BAR_BRAND_R: [&str; 2] = ["navbar-brand","ms-2"];
pub const NAV_BAR_TGL: [&str; 2] = ["navbar-toggler","d-lg-none"];
pub const NAV_BAR_COLLAPSE: [&str; 2] = ["navbar-collapse","collapse"];
pub const NAV_BAR_NAV_LX: [&str; 3] = ["navbar-nav","mt-lg-0","me-auto"];

pub const NAV_TABS_T: [&str; 3] = ["nav","nav-tabs","mb-2"];
pub const NAV_PILLS_T: [&str; 3] = ["nav","nav-pills","mb-2"];
pub const NAV_PILLS_M3: [&str; 3] = ["nav","nav-pills","m-3"];
pub const NAV_PILLS_COL_T: [&str; 4] = ["nav","nav-pills","flex-column","mb-2"];
pub const NAV_PILLS_R: [&str; 3] = ["nav","nav-pills","float-end"];
pub const NAV_ITEM_PY: [&str; 2] = ["nav-item","py-1"];
pub const NAV_ITEM_DROP: [&str; 2] = ["nav-item","dropdown"];
pub const NAV_ITEM_DROP_PY: [&str; 3] = ["nav-item","dropdown","py-1"];
pub const NAV_ITEM_LINK: [&str; 2] = ["nav-item","nav-link"];
pub const NAV_ITEM_LINK_P2: [&str; 3] = ["nav-item","nav-link","p-2"];
pub const NAV_ITEM_LINK_R: [&str; 3] = ["nav-item","nav-link","ms-auto"];
pub const NAV_ITEM_LINK_ACTIVE: [&str; 3] = ["nav-item","nav-link","active"];
pub const NAV_ITEM_LINK_ACTIVE_P2: [&str; 4] = ["nav-item","nav-link","active","p-2"];
pub const NAV_LINK_ACTIVE: [&str; 2] = ["nav-link","active"];
pub const NAV_LINK_DROP_TGL: [&str; 2] = ["nav-link","dropdown-toggle"];
pub const NAV_LINK_DROP_TGL_TW_PY: [&str; 4] =["nav-link","dropdown-toggle","text-white","py-1"];

pub const NOWRAP_R: [&str; 2] = ["text-nowrap","text-end"];
pub const NOWRAP_C: [&str; 2] = ["text-nowrap","text-center"];

pub const OVFA_CYANS: [&str; 2] = ["overflow-auto","bg-info-subtle"];
pub const OVFA_T: [&str; 2] = ["overflow-auto","mb-2"];

pub const RELATIVE_L: [&str; 2] = ["position-relative","me-2"];

pub const RESP_LG_SM: [&str; 3] = ["d-none","d-lg-inline","d-sm-none"];
pub const RESP_XL_MD: [&str; 3] = ["d-none","d-xl-inline","d-md-none"];

pub const ROUND_WHITE: [&str; 2] = ["rounded","bg-white"];
pub const ROUND_BOLD: [&str; 2] = ["rounded","fw-bold"];

pub const ROW: [&str; 2] = ["row","my-1"];
pub const ROW_MY2: [&str; 2] = ["row","my-2"];
pub const ROW_NOWRAP: [&str; 2] = ["row","flex-nowrap"];
pub const ROW_AUTO_G2: [&str; 3] = ["row","row-cols-auto","g-2"];
pub const ROW_AUTO_LG_G2_CT: [&str; 5] = ["row","row-cols-lg-auto","g-2","align-items-center","mb-2"];
pub const ROW_AUTO_SM_G2_CT: [&str; 5] = ["row","row-cols-sm-auto","g-2","align-items-center","mb-2"];
pub const ROW_AUTO_SM_G2_JCT: [&str; 5] = ["row","row-cols-sm-auto","g-2","justify-content-center","mb-2"];
pub const ROW_AUTO_SM_G2_JRB: [&str; 5] = ["row","row-cols-sm-auto","g-2","justify-content-end","mt-2"];
pub const ROW_B: [&str; 2] = ["row","mt-2"];
pub const ROW_M: [&str; 2] = ["row","m-2"];
pub const ROW_T: [&str; 2] = ["row","mb-2"];
pub const ROW_GT: [&str; 3] = ["row","g-2","mb-2"];
pub const ROW_COL_MD12: [&str; 2] = ["row","col-md-12"];
pub const ROW_COL_RESP_G2: [&str; 3] = ["row","row-cols-1","g-2"];
pub const ROW_COL_RESP2_MD_G2: [&str; 4] = ["row","row-cols-1","row-cols-md-2","g-2"];
pub const ROW_COL_RESP2_XL_G2: [&str; 4] = ["row","row-cols-1","row-cols-xl-2","g-2"];
pub const ROW_COL_RESP3_XL_G2: [&str; 4] = ["row","row-cols-1","row-cols-xl-3","g-2"];
pub const ROW_COL_RESP3_G2: [&str; 5] = ["row","row-cols-1","row-cols-lg-2","row-cols-xl-3","g-2"];
pub const ROW_COL_RESP4_G2: [&str; 6] = ["row","row-cols-1","row-cols-md-2","row-cols-xl-3","row-cols-xxl-4","g-2"];
pub const ROW_COL5_G2: [&str; 3] = ["row","row-cols-5","g-2"];
pub const ROW_TC: [&str; 2] = ["row","tab-content"];
pub const ROW_ITEMS_C: [&str; 2] = ["row","align-items-center"];

pub const SMALL_L: [&str; 2] = ["small","me-1"];
pub const SMALL_R2: [&str; 2] = ["small","ms-2"];
// pub const SMALL_L: [&str; 2] = ["small","text-start"];
pub const SMALL_R: [&str; 2] = ["small","text-end"];
pub const SMALL_RED: [&str; 2] = ["small","text-danger"];
pub const SMALL_TRUNC: [&str; 2] = ["small","text-truncate"];
pub const SMALL_FLOAT_R: [&str; 4] = ["small","float-end","lh-lg","ms-1"];
pub const SMALL_BOLD: [&str; 2] = ["small","fw-bold"];
pub const SMALL_BOLD_C: [&str; 4] = ["small","fw-bold","text-center","lh-1"];
pub const SMALL_WRAP_BOLD_RED: [&str; 4] = ["small","text-wrap","fw-bold","text-danger"];

pub const SPIN_SM_BLUE: [&str; 3] = ["spinner-border","spinner-border-sm","text-primary"];
pub const SPIN_SM_GLOW_RED: [&str; 3] = ["spinner-grow","spinner-grow-sm","text-danger"];

pub const TABLE: [&str; 3] = ["table","table-bordered","table-hover"];
pub const TABLE_1R: [&str; 2] = ["table","table-bordered"];
pub const TABLE_STRIP: [&str; 4] = ["table","table-striped","table-bordered","table-hover"];
pub const TABLE_SM: [&str; 4] = ["table","table-sm","table-bordered","table-hover"];
pub const TABLE_SM_NB: [&str; 3] = ["table","table-sm","table-hover"];
pub const TABLE_SM_STRIP: [&str; 5] = ["table","table-sm","table-striped","table-bordered","table-hover"];

pub const TABLE_BG_CYAN: [&str; 2] = ["table-info","text-nowrap"];

pub const TAB_FADE_SHOW_ACTIVE: [&str; 4] = ["tab-pane","fade","show","active"];
pub const TAB_ACTIVE: [&str; 2] = ["tab-pane","active"];
pub const TAB_FADE: [&str; 2] = ["tab-pane","fade"];

pub const TXT_BLUE_EM: [&str; 2] = ["text-primary-emphasis","me-1"];
pub const TXT_BLUE_R: [&str; 2] = ["text-primary","ms-1"];

pub const TXT_BG_BLUE_RT: [&str; 3] = ["text-bg-primary","me-1","mb-1"];

pub const TXT_CB: [&str; 2] = ["text-center","mt-2"];
pub const TXT_CT: [&str; 2] = ["text-center","mb-2"];
pub const TXT_C_P0: [&str; 2] = ["text-center","p-0"];
pub const TXT_C_P1: [&str; 2] = ["text-center","p-1"];
pub const TXT_C_P2: [&str; 2] = ["text-center","p-2"];
pub const TXT_C_BX_RED: [&str; 3] = ["text-center","d-block","text-danger"];
pub const TXT_C_BLUE: [&str; 2] = ["text-center","text-primary"];
pub const TXT_C_GRAY: [&str; 2] = ["text-center","text-secondary"];
pub const TXT_C_GRAYS: [&str; 2] = ["text-center","bg-secondary-subtle"];
pub const TXT_C_GREEN: [&str; 2] = ["text-center","text-success"];
pub const TXT_C_RED: [&str; 2] = ["text-center","text-danger"];
pub const TXT_C_TOP: [&str; 2] = ["text-center","align-top"];
pub const TXT_C_MIDDLE: [&str; 2] = ["text-center","align-middle"];
pub const TXT_C_WHITE: [&str; 4] = ["text-center","text-white","overflow-hidden","p-1"];

pub const TXT_RED_L: [&str; 2] = ["text-danger","me-1"];
pub const TXT_BOLD_RED_FS5_L: [&str; 4] = ["text-danger","fw-bold","fs-5","me-1"];

pub const TXT_RRX: [&str; 2] = ["text-end","ms-auto"];
pub const TXT_R_B: [&str; 2] = ["text-end","mt-1"];
pub const TXT_R_B2: [&str; 2] = ["text-end","mt-2"];
pub const TXT_R_P: [&str; 2] = ["text-end","p-2"];
pub const TXT_R_PE: [&str; 2] = ["text-end","pe-3"];
pub const TXT_R_PY: [&str; 2] = ["text-end","py-2"];
pub const TXT_NOWRAP_L: [&str; 2] = ["text-nowrap","me-1"];

pub const TXT_U_L: [&str; 2] = ["text-decoration-underline","me-1"];

pub const TXT_WHITE_RED: [&str; 2] = ["text-white","bg-danger"];

pub const TRUNC_SM: [&str; 2] = ["text-truncate","text-sm"];
pub const TRUNC_BOLD: [&str; 2] = ["text-truncate","fw-bold"];

pub const USER_SELECT: [&str; 2] = ["user-select","-webkit-user-select"];

pub const W100_L: [&str; 2] = ["w-100","me-2"];
