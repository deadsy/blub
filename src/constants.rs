// constants
//-----------------------------------------------------------------------------

// FullCycle is a full uint32 phase count.
pub const FULL_CYCLE: u64 = 1 << 32;

// PhaseScale scales a phase value to a uint32 phase step value.
pub const PHASE_SCALE: f32 = ((FULL_CYCLE as f32) / TAU) as f32;

//-----------------------------------------------------------------------------

// Pi (3.14159...)
pub const PI: f32 = std::f32::consts::PI;

// Tau (2 * Pi).
pub const TAU: f32 = 2.0 * PI;

//-----------------------------------------------------------------------------
