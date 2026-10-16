// generator version: 1.3.0
// generator parameters: --chip esp32 -o unstable-hal -o stack-smashing-protection -o defmt -o neovim -o esp

#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use core::time::Duration;
use defmt::{error, info};
use embedded_graphics::geometry::Point;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::main;
use esp_hal::rtc_cntl::Rtc;
use esp_hal::rtc_cntl::sleep::TimerWakeupSource;
use esp_println as _;
use screen_logger::esp_display::{EspDisplay, EspDisplayGpio};

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    error!("{}", panic_info);
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

struct Bsp {
    display: EspDisplay<'static>,
    rtc: Rtc<'static>,
}

impl Bsp {
    fn setup() -> Bsp {
        let config = esp_hal::Config::default();
        let per = esp_hal::init(config);

        info!("Setup");

        let display_gpio = EspDisplayGpio {
            mosi: per.GPIO19.into(),
            sclk: per.GPIO18.into(),
            spi: per.SPI2.into(),
            cs: Output::new(per.GPIO5, Level::High, OutputConfig::default()),
            dc: Output::new(per.GPIO16, Level::High, OutputConfig::default()),
            rst: Output::new(per.GPIO23, Level::High, OutputConfig::default()),
            bl: Output::new(per.GPIO4, Level::High, OutputConfig::default()),
        };

        Self {
            display: EspDisplay::new(display_gpio),
            rtc: Rtc::new(per.LPWR),
        }
    }
}

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    info!("Starting ESP");
    let Bsp {
        mut display,
        mut rtc,
    } = Bsp::setup();

    // Wakeup after 2 seconds
    let wakeup_source = TimerWakeupSource::new(Duration::from_secs(2).into());

    loop {
        display.render("hello", Point::new(0, 20));
        info!("Going to sleep");
        // Enter deep sleep with the configured wakeup source
        rtc.sleep_deep(&[&wakeup_source]);
    }
}
