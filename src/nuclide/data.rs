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
            $half_life: literal, $decay_modes: expr
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
        nuclide!{0, 1},
    ]
}

#[inline]
pub fn get_element_names() -> &'static [&'static str] {
    &[
        "neutron",
    ]
}

#[inline]
pub fn get_element_symbols() -> &'static [&'static str] {
    &[
        "n",
    ]
}
