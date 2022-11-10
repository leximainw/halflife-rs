mod data;
pub use data::{
    get_nuclides,
    get_element_names,
    get_element_symbols,
};

pub struct Nuclide {
    protons: i32,
    neutrons: i32,
    half_life: Option<f64>,
    decay_modes: Vec<Decay>,
}

impl Nuclide {
    #[inline]
    pub fn atomic_number(&self) -> i32 {
        self.protons
    }

    #[inline]
    pub fn atomic_mass_number(&self) -> i32 {
        self.protons + self.neutrons
    }

    #[inline]
    pub fn name(&self) -> String {
        format!("{}-{}",
            get_element_names()[self.protons as usize],
            self.atomic_mass_number())
    }

    #[inline]
    pub fn all() -> Vec<Nuclide> {
        get_nuclides()
    }
}

pub struct Decay {
    decay: DecayMode,
    qty: f64,
}

impl Decay {
    pub fn beta_minus() -> Vec<Decay> {
        vec![
            Decay{
                decay: DecayMode::BetaMinus,
                qty: 1.0,
            }
        ]
    }
}

pub enum DecayMode {
    BetaMinus,
}
