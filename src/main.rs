mod nuclide;

use nuclide::Nuclide;

fn main() {
    let nuclides = Nuclide::all();
    for nuclide in nuclides {
        println!("{}", nuclide.name());
    }
}
