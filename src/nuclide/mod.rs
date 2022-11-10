mod data;
pub use data::{
    get_nuclides,
    get_element_names,
    get_element_symbols,
};

pub struct Nuclide {
    protons: i32,
    neutrons: i32,
    half_life: Option<(f64, f64)>,
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
    pub fn half_life(&self) -> f64 {
        match self.half_life{
            Some(half_life) => half_life.0,
            None => f64::INFINITY,
        }
    }

    #[inline]
    pub fn half_life_with_error(&self) -> (f64, f64) {
        match self.half_life{
            Some(half_life) => half_life,
            None => (f64::INFINITY, 0.0),
        }
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

macro_rules! shorthand{
    ($name: ident, $mode: ident) => {
        pub fn $name() -> Vec<Decay> {
            vec![
                Decay{
                    decay: DecayMode::$mode,
                    qty: 1.0,
                }
            ]
        }
    };
}

impl Decay {
    shorthand!(beta_minus, BetaMinus);
    shorthand!(double_neutron_emission, DoubleNeutronEmission);
    shorthand!(neutron_emission, NeutronEmission);
}

pub enum DecayMode {
    BetaMinus,
    DoubleNeutronEmission,
    NeutronEmission,
}
