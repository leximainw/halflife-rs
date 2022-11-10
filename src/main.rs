mod nuclide;

use nuclide::Nuclide;

fn main() {
    let nuclides = Nuclide::all();
    for nuclide in nuclides {
        println!("{:<16}{}",
            nuclide.name(),
            convert_half_life(nuclide.half_life()),
        );
    }
}

fn convert_half_life(half_life: f64) -> String {
    return if half_life.is_infinite() {
        "stable".to_string()
    } else if half_life < 1e-21 {
        format!("{:.3} ys", half_life * 1e24)
    } else if half_life < 1e-18 {
        format!("{:.3} zs", half_life * 1e21)
    } else if half_life < 1e-15 {
        format!("{:.3} as", half_life * 1e18)
    } else if half_life < 1e-12 {
        format!("{:.3} fs", half_life * 1e15)
    } else if half_life < 1e-9 {
        format!("{:.3} ps", half_life * 1e12)
    } else if half_life < 1e-6 {
        format!("{:.3} ns", half_life * 1e9)
    } else if half_life < 0.001 {
        format!("{:.3} \u{00B5}s", half_life * 1e6)
    } else if half_life < 1.0 {
        format!("{:.3} ms", half_life * 1000.0)
    } else if half_life < 60.0 {
        format!("{:.3} s", half_life)
    } else if half_life < 3600.0 {
        format!("{:.3} min", half_life / 60.0)
    } else if half_life < 86400.0 {
        format!("{:.3} hr", half_life / 3600.0)
    } else if half_life < 2.628e6 {
        format!("{:.3} d", half_life / 86400.0)
    } else if half_life < 3.1536e7 {
        format!("{:.3} mo", half_life / 2.628e6)
    } else {
        format!("{:.3} Y", half_life / 3.1536e7)
    }
}
