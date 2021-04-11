pub struct UartPlugin {
    data: u32,
}

impl spike_rs::Plugin for UartPlugin {
    fn new(args: &str) -> Self {
        let data = u32::from_str_radix(args, 16).unwrap_or(0xABAB);
        println!(
            "Rust MMIO Plugin: Created uart plugin with data: {:x}",
            data
        );
        Self { data }
    }

    fn load(&mut self, addr: u64, buf: &mut [u8]) -> bool {
        let data = self.data.to_le_bytes();
        buf.copy_from_slice(&data);
        println!(
            "Rust MMIO Plugin: Loading {:x?} from: {}. Data: {:x}",
            data, addr, self.data
        );
        true
    }

    fn store(&mut self, addr: u64, buf: &[u8]) -> bool {
        println!(
            "Rust MMIO Plugin: Storing {:x?} into: {}. Data: {:x}",
            buf, addr, self.data
        );
        true
    }
}

spike_rs::register_plugins! {
    "my_plugin" => UartPlugin,
}
