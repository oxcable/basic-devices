//! Parameter sets for simulating different reverberant environments.
//!
//! The tapped delays and tapped gains specify the start of the reverb, and must
//! be the same length. They represent the initial reflection paths, and are
//! used to sculpt the echoes of the room.
//!
//! The comb delays are used to fill out the reverberations and create a steady
//! decay; they sculpt the color of the room.

use types::Time;


/// A container for Moorer reverberator parameters.
#[derive(Copy, Clone, Debug)]
pub struct Room<'a> {
    pub tapped_delays: &'a[Time],
    pub tapped_gains: &'a[f32],
    pub comb_delays: &'a[Time]
}


/// A generic concert hall.
///
/// This room was modeled by Moorer in his original paper, where he described
/// the Moorer reverbator.
pub static HALL: Room<'static> = Room {
    // For these parameters, see pg. 24 from Moorer paper
    tapped_delays: &[190, 948, 992, 1182, 1191, 1314, 2020, 2139, 2523,
    2589, 2624, 2699, 3118, 3122, 3202, 3268, 3321, 3515],
    tapped_gains: &[0.841, 0.504, 0.491, 0.379, 0.380, 0.346, 0.289, 0.272,
    192.0, 0.193, 0.217, 0.181, 0.180, 0.181, 0.176, 0.142, 0.167, 0.134],
    // For these parameters, see pg. 18 from Moorer
    comb_delays: &[2205, 2470, 2690, 2999, 3175, 3440],
};
