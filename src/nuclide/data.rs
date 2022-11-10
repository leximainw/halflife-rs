use super::{
    Nuclide,
    Decay,
    DecayMode
};

pub fn get_nuclides() -> Vec<Nuclide> {
    macro_rules! nuclide {
        ($protons: literal, $neutrons: literal) => {
            Nuclide{
                protons: $protons,
                neutrons: $neutrons,
                half_life: None,
                decay_modes: vec![],
            }
        };
        ($protons: literal, $neutrons: literal,
            $half_life: expr, $decay_modes: expr
        ) => {
            Nuclide{
                protons: $protons,
                neutrons: $neutrons,
                half_life: Some($half_life),
                decay_modes: $decay_modes,
            }
        };
    }

    vec![
        nuclide!{0, 1, (610.1, 0.7), Decay::beta_minus()},
        nuclide!{1, 0},
        nuclide!{1, 1},
        nuclide!{1, 2, (3.8852352e8, 6.3072e5), Decay::beta_minus()},
        nuclide!{1, 3, (1.39e-22, 1.0e-23), Decay::neutron_emission()},
        nuclide!{1, 4, (8.6e-23, 6e-24), Decay::double_neutron_emission()},
        nuclide!{1, 5, (2.94e-22, 6.7e-23), Decay::neutron_emission()},   // NOTE: true decay mode unknown
    ]
}

#[inline]
pub fn get_element_names() -> &'static [&'static str] {
    &[
        "neutron",
        "hydrogen",
    ]
}

#[inline]
pub fn get_element_symbols() -> &'static [&'static str] {
    &[
        "n",
        "H",
    ]
}
