#![no_std]
#![no_main]

use ch32_hal as hal;
use ch32_hal::gpio::Input;
use hal::delay::Delay;
use hal::gpio::{Level, Output, Pull};
use hal::i2c;
use hal::i2c::I2c;
use hal::peripherals;
use hal::time::khz;

const COMMANDS: [[u8; 7]; 11] = [
    [0x51, 0x84, 0x03, 0x10, 0x00, 0x00, 0xa8],
    [0x51, 0x84, 0x03, 0x10, 0x00, 0x0a, 0xa2],
    [0x51, 0x84, 0x03, 0x10, 0x00, 0x14, 0xbc],
    [0x51, 0x84, 0x03, 0x10, 0x00, 0x1e, 0xb6],
    [0x51, 0x84, 0x03, 0x10, 0x00, 0x28, 0x80],
    [0x51, 0x84, 0x03, 0x10, 0x00, 0x32, 0x9a],
    [0x51, 0x84, 0x03, 0x10, 0x00, 0x3c, 0x94],
    [0x51, 0x84, 0x03, 0x10, 0x00, 0x46, 0xee],
    [0x51, 0x84, 0x03, 0x10, 0x00, 0x50, 0xf8],
    [0x51, 0x84, 0x03, 0x10, 0x00, 0x5a, 0xf2],
    [0x51, 0x84, 0x03, 0x10, 0x00, 0x64, 0xcc],
];

// SDI print requires the debugger to be connected.
// Set to false to run without debugger.
const DEBUG: bool = false;

macro_rules! debug_print {
    ($($arg:tt)*) => {
        if DEBUG {
            hal::println!($($arg)*);
        }
    }
}

struct Displays {
    i2c: peripherals::I2C1,
    scl_a: peripherals::PC2,
    sda_a: peripherals::PC1,
    scl_b: peripherals::PC5,
    sda_b: peripherals::PC6,
}

impl Displays {
    fn set(&mut self, command: &[u8; 7]) {
        let mut i2c_config = i2c::Config::default();
        i2c_config.timeout = embassy_time::Duration::from_millis(100);

        {
            let mut i2c = I2c::new_blocking(
                &mut self.i2c,
                &mut self.scl_a,
                &mut self.sda_a,
                khz(100),
                i2c_config,
            );
            i2c.blocking_write(0x37, command)
                .unwrap_or_else(|_| debug_print!("Write failed A"));
        }
        {
            let mut i2c = I2c::new_blocking(
                &mut self.i2c,
                &mut self.scl_b,
                &mut self.sda_b,
                khz(100),
                i2c_config,
            );
            i2c.blocking_write(0x37, command)
                .unwrap_or_else(|_| debug_print!("Write failed B"));
        }
    }
}

#[qingke_rt::entry]
fn main() -> ! {
    if DEBUG {
        hal::debug::SDIPrint::enable();
    }

    let mut config = hal::Config::default();
    
    config.rcc = hal::rcc::Config::SYSCLK_FREQ_48MHZ_HSI;
    
    let peripherals = hal::init(config);
    
    let mut delay = Delay;

    let mut i2c_config = i2c::Config::default();

    i2c_config.timeout = embassy_time::Duration::from_millis(100);

    let plus_button = Input::new(peripherals.PD0, Pull::Up);
    let minus_button = Input::new(peripherals.PC0, Pull::Up);

    let mut displays = Displays {
        i2c: peripherals.I2C1,
        scl_a: peripherals.PC2,
        sda_a: peripherals.PC1,
        scl_b: peripherals.PC5,
        sda_b: peripherals.PC6,
    };
    
    let mut brightness: i8 = 5;
    let mut change: i8 = 0;

    debug_print!("Start");
    loop {
        let new_change = (plus_button.is_high() as i8) - (minus_button.is_high() as i8);

        if new_change == 0 {
            change = 0;
            continue;
        }

        // Detect press and hold
        if change == new_change {
            change = new_change * 10;
        }
        else {
            change = new_change;
        }

        let new_brightness = (brightness + change).clamp(0, 10);
    
        if brightness != new_brightness {
            brightness = new_brightness;
            displays.set(&COMMANDS[brightness  as usize]);
        }

        // Crude debounce and press/hold delay
        delay.delay_ms(300);
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let _ = debug_print!("\n{}", info);
    loop {}
}
