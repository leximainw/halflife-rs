use super::{
    Nuclide,
    Decay,
    DecayMode
};

pub fn get_nuclides() -> Vec<Nuclide> {
    vec![
        Nuclide{
            protons: 0,
            neutrons: 1,
            half_life: Some(610.1),
            decay_modes: vec![
                Decay{
                    decay: DecayMode::BetaMinus,
                    qty: 1.0,
                }
            ],
        }
    ]
}