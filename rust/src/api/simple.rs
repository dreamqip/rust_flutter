#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

pub struct Example {
    pub number: i32,
}

impl Example {
    pub fn boo() {
        println!("boo! Example::boo() was called!");
    }

    pub fn answer(&mut self) {
        self.number += 42;
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn get_number(&self) -> i32 {
        self.number
    }
}
