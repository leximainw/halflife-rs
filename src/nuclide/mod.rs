mod data;
use data::get_nuclides;

pub struct Nuclide {
    protons: i32,
    neutrons: i32,
    half_life: Option<f64>,
    decay_modes: Vec<Decay>,
}

impl Nuclide {
    #[inline]
    pub fn all() -> Vec<Nuclide> {
        get_nuclides()
    }
}

pub struct Decay {
    decay: DecayMode,
    qty: f64,
}

pub enum DecayMode {
    BetaMinus,
}
