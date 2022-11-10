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
        nuclide!{1, 6, (6.52e-22, 5.58e-22), Decay::double_neutron_emission()},   // NOTE: true decay mode unknown
        nuclide!{2, 0, (1e-9, 1e-9), vec![   // NOTE: true half-life and decay ratios unknown
            Decay{
                decay: DecayMode::ProtonEmission,
                qty: 0.9999,
            },
            Decay{
                decay: DecayMode::BetaPlus,
                qty: 0.0001,
            },
        ]},
        nuclide!{2, 1},
        nuclide!{2, 2},
        nuclide!{2, 3, (6.02e-22, 2.2e-23), Decay::neutron_emission()},
        nuclide!{2, 4, (8.0692e-22, 2.4e-25), vec![
            Decay{
                decay: DecayMode::BetaMinus,
                qty: 0.99999722,
            },
            Decay{
                decay: DecayMode::BetaMinusDelayedDeuteron,
                qty: 0.00000278,
            },
        ]},
        nuclide!{2, 5, (2.51e-21, 7e-23), Decay::neutron_emission()},
        nuclide!{2, 6, (1.195e-1, 1.5e-3), vec![
            Decay{
                decay: DecayMode::BetaMinus,
                qty: 0.831,   // NOTE: 83.1(1.0)%
            },
            Decay{
                decay: DecayMode::BetaMinusDelayedNeutron,
                qty: 0.16,   // NOTE: 16(1)%
            },
            Decay{
                decay: DecayMode::BetaMinusDelayedTriton,
                qty: 0.009,   // NOTE: 0.9(1)%
            },
        ]},
        nuclide!{2, 7, (2.5e-21, 2.3e-21), Decay::neutron_emission()},
        nuclide!{2, 8, (2.6e-22, 4.0e-23), Decay::double_neutron_emission()},
    ]
}

#[inline]
pub fn get_element_names() -> &'static [&'static str] {
    &[
        "neutron",
        "hydrogen",
        "helium",
    ]
}

#[inline]
pub fn get_element_symbols() -> &'static [&'static str] {
    &[
        "n",
        "H",
        "He",
    ]
}
