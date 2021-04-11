pub struct UartPlugin {
    data: u64,
}

impl spike_rs::Plugin for UartPlugin {
    fn new(_: &str) -> Self {
        println!("Created uart plugin");
        Self { data: 1337 }
    }

    fn load(&mut self, addr: u64, _: &mut [u8]) -> bool {
        println!("{} Load from {:x}", self.data, addr);
        true
    }

    fn store(&mut self, addr: u64, buf: &[u8]) -> bool {
        println!("{} Stored into {:x}: {:x?}", self.data, addr, buf);
        true
    }
}

spike_rs::register_plugins! {
    "my_plugin" => UartPlugin,
}
