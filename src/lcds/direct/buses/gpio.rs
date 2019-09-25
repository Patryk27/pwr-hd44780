// @todo describe how this bus works

use rppal::gpio::{Level as PinLevel, OutputPin, Pin};

use crate::{lcds::direct, Result, wait_ms, wait_ns, wait_us};

pub struct GpioBus {
    pins: GpioPins,
}

pub struct GpioConfig {
    pub data: [Pin; 4],
    pub rs: Pin,
    pub enable: Pin,
    pub backlight: Option<Pin>,
}

struct GpioPins {
    data: [OutputPin; 4],
    rs: OutputPin,
    enable: OutputPin,
    backlight: Option<OutputPin>,
}

impl GpioPins {
    fn from_config(config: GpioConfig) -> Self {
        let [data0, data1, data2, data3] = config.data;

        Self {
            data: [
                data0.into_output(),
                data1.into_output(),
                data2.into_output(),
                data3.into_output(),
            ],

            rs: config.rs.into_output(),
            enable: config.enable.into_output(),
            backlight: config.backlight.map(Pin::into_output),
        }
    }

    fn write_nibble(&mut self, nibble: u8, as_data: bool) {
        self.enable.set_low();
        self.rs.write(pin_level(as_data));

        self.data[0].write(pin_level(nibble & 0b0001_0000u8 > 0));
        self.data[1].write(pin_level(nibble & 0b0010_0000u8 > 0));
        self.data[2].write(pin_level(nibble & 0b0100_0000u8 > 0));
        self.data[3].write(pin_level(nibble & 0b1000_0000u8 > 0));

        // Wait 1ms to give some time for GPIOs to stabilize and for LCD to notice the change
        wait_ms(1);

        // Pull up the `enable` pin and wait ~450ns (enable pulse must be >450ns)
        self.enable.set_high();
        wait_ns(450);

        // Pull down the `enable` pin and wait ~37us (commands need 37us to settle)
        self.enable.set_low();
        wait_us(37);
    }

    fn write_byte(&mut self, byte: u8, as_data: bool) {
        self.write_nibble(byte << 0, as_data);
        self.write_nibble(byte << 4, as_data);
    }
}

impl GpioBus {
    pub fn new(config: GpioConfig) -> Self {
        let mut pins = GpioPins::from_config(config);

        let commands: [u8; 4] = [
            // Try to put LCD in 8-bit mode 3 times - it's required to properly initialize the LCD
            // when it's dirty (i.e. not restarted)
            0x03,
            0x03,
            0x03,

            // Now we can safely put LCD back to proper 4-bit mode
            0x02,
        ];

        for cmd in &commands {
            pins.write_nibble(cmd << 4, false);
            wait_ms(1);
        }

        Self { pins }
    }
}

impl direct::Bus for GpioBus {
    fn write_command(&mut self, byte: u8) -> Result<()> {
        self.pins.write_byte(byte, false);
        Ok(())
    }

    fn write_data(&mut self, byte: u8) -> Result<()> {
        self.pins.write_byte(byte, true);
        Ok(())
    }

    fn enable_backlight(&mut self, enabled: bool) -> Result<()> {
        if let Some(backlight) = &mut self.pins.backlight {
            backlight.write(pin_level(enabled));
        }

        Ok(())
    }

    fn size(&self) -> direct::BusSize {
        direct::BusSize::FourBit
    }
}

#[inline]
fn pin_level(high: bool) -> PinLevel {
    if high {
        PinLevel::High
    } else {
        PinLevel::Low
    }
}