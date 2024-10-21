// Possible state of light
enum LightState {
    On,
    Off,
}

// Define the struct to represent the light bulb
struct LightBulb {
    state: LightState,
}

// LightBulb methods
impl LightBulb {
    // Switch state between On and Off
    fn toggle(&mut self) {
        match self.state {
            LightState::Off => self.state = LightState::On,
            LightState::On => self.state = LightState::Off,
        }
    }

    // Check the current state of the LightBulf
    fn is_on(&self) -> bool {
        match self.state {
            LightState::Off => false,
            LightState::On => true,
        }
    }
}

fn main() {
    // Create a new light bulb instance
    let mut bulb = LightBulb {
        state: LightState::Off,
    };

    // Check the initial state of the light bulb
    println!("Is the light bulb on? {}", bulb.is_on());

    // Toggle the state of the light bulb
    bulb.toggle();

    // Check the state of the light bulb
    println!("Is the light bulb on? {}", bulb.is_on());
}
