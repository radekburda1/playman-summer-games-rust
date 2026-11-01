//! Named resource offsets from the original binary tables.
//! These values are addresses, not simulation units. Keep provenance here.
pub mod animation {
    pub const IDLE: usize = 2388;
    pub const DEFAULT_POSE: usize = 2422;
    pub const RACE_ACCELERATE: usize = 2426;
    pub const RACE_RUN: usize = 2448;
    pub const RACE_DIVE: usize = 2470;
    pub const LONG_JUMP_RUN: usize = 2512;
    pub const LONG_JUMP_TAKEOFF: usize = 2534;
    pub const LONG_JUMP_STRETCH: usize = 2554;
    pub const LONG_JUMP_FOUL: usize = 2574;
    pub const RELEASE_FALL: usize = 2592;
    pub const LONG_JUMP_LAND: usize = 2596;
    pub const LONG_JUMP_PREPARE: usize = 2618;
    pub const FALL: usize = 2650;
    pub const LONG_JUMP_RECOVER: usize = 2672;
    pub const JAVELIN_RUN_CYCLE: usize = 2684;
    pub const JAVELIN_PREPARE: usize = 2706;
    pub const JAVELIN_RUN: usize = 2728;
    pub const JAVELIN_APPROACH: usize = 2744;
    pub const JAVELIN_CHARGE: usize = 2754;
    pub const JAVELIN_THROW: usize = 2786;
    pub const JAVELIN_WATCH: usize = 2804;
    pub const HURDLES_ACCELERATE: usize = 2834;
    pub const HURDLES_CLEAR: usize = 2866;
    pub const HURDLES_RECOVER_ONE: usize = 2912;
    pub const HURDLES_RECOVER_TWO: usize = 2958;
    pub const HURDLES_RECOVER_THREE: usize = 3004;
    pub const VAULT_PREPARE: usize = 3068;
    pub const VAULT_RUN: usize = 3080;
    pub const VAULT_SWING: usize = 3116;
    pub const VAULT_RELEASE: usize = 3132;
    pub const VAULT_FALL: usize = 3146;
    pub const VAULT_LAND: usize = 3156;
    pub const CELEBRATION_END: usize = 3366;
    pub const CELEBRATION_IDLE: usize = 3382;
    pub const SATISFIED: usize = 3392;
    pub const DISAPPOINTED: usize = 3402;
    pub const DISAPPOINTED_IDLE: usize = 3430;
    pub const CELEBRATION_START: usize = 3440;
    pub const CELEBRATION_JUMP: usize = 3446;
    pub const FOUL_REACTION: usize = 3472;
    pub const FOUL_IDLE: usize = 3536;
}
pub mod scene {
    pub const HURDLE: usize = 16384;
    pub const THROW_OFFICIAL: usize = 16385;
    pub const STARTER: usize = 16386;
    pub const FOREGROUND_ADVERTISING: usize = 8200;
    pub const TRACK_EQUIPMENT: usize = 8201;
    pub const STADIUM_EQUIPMENT: usize = 8202;
    pub const CROWD: usize = 8204;
    pub const STADIUM_ADVERTISING: usize = 8206;
    pub const NEAR_BUILDINGS: usize = 8207;
    pub const DISTANT_BUILDINGS: usize = 8208;
    pub const SKYLINE: usize = 8209;
    pub const BENCH: usize = 8210;
    pub const LONG_JUMP_EQUIPMENT: usize = 8212;
}
pub mod sprite {
    pub const FINISH_CAMERA: usize = 711;
    pub const FINISH_BELL: usize = 712;
    pub const TIMING_CURSOR: usize = 469;
    pub const JAVELIN_SHADOW: usize = 522;
}
