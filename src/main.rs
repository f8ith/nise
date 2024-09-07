#[cfg(feature = "nestest")]
use nise::nes::{bus::NiseBus, cpu::Nise6502, rom::Rom};

fn main() {
    #[cfg(feature = "nestest")]
    {
        let nesdata = std::fs::read("./nestest.nes").expect("Unable to read rom!");
        let rom = Rom::new(&nesdata).unwrap();
        let bus = NiseBus::new(rom);
        let mut nes = Nise6502::new(bus);
        let _ = nes.nestest();
    }
}
