#![doc = "Peripheral access API for LEDS microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(legacy_directory_ownership)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(plugin_as_library)]
#![deny(private_in_public)]
#![deny(safe_extern_statics)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate riscv;
#[cfg(feature = "rt")]
extern crate riscv_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "CTRL"]
pub struct CTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CTRL {}
impl CTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ctrl::RegisterBlock {
        0xe000_0000 as *const _
    }
}
impl Deref for CTRL {
    type Target = ctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CTRL::ptr() }
    }
}
#[doc = "CTRL"]
pub mod ctrl;
#[doc = "UART_PHY"]
pub struct UART_PHY {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART_PHY {}
impl UART_PHY {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart_phy::RegisterBlock {
        0xe000_1000 as *const _
    }
}
impl Deref for UART_PHY {
    type Target = uart_phy::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART_PHY::ptr() }
    }
}
#[doc = "UART_PHY"]
pub mod uart_phy;
#[doc = "UART"]
pub struct UART {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART {}
impl UART {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart::RegisterBlock {
        0xe000_1800 as *const _
    }
}
impl Deref for UART {
    type Target = uart::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART::ptr() }
    }
}
#[doc = "UART"]
pub mod uart;
#[doc = "TIMER0"]
pub struct TIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0 {}
impl TIMER0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        0xe000_2000 as *const _
    }
}
impl Deref for TIMER0 {
    type Target = timer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER0::ptr() }
    }
}
#[doc = "TIMER0"]
pub mod timer0;
#[doc = "SPIFLASH"]
pub struct SPIFLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIFLASH {}
impl SPIFLASH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spiflash::RegisterBlock {
        0xe000_2800 as *const _
    }
}
impl Deref for SPIFLASH {
    type Target = spiflash::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPIFLASH::ptr() }
    }
}
#[doc = "SPIFLASH"]
pub mod spiflash;
#[doc = "OLED_SPI"]
pub struct OLED_SPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OLED_SPI {}
impl OLED_SPI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const oled_spi::RegisterBlock {
        0xe000_3000 as *const _
    }
}
impl Deref for OLED_SPI {
    type Target = oled_spi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OLED_SPI::ptr() }
    }
}
#[doc = "OLED_SPI"]
pub mod oled_spi;
#[doc = "OLED_SPI2"]
pub struct OLED_SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OLED_SPI2 {}
impl OLED_SPI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const oled_spi::RegisterBlock {
        0xe000_3800 as *const _
    }
}
impl Deref for OLED_SPI2 {
    type Target = oled_spi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OLED_SPI2::ptr() }
    }
}
#[doc = "OLED_CTL"]
pub struct OLED_CTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OLED_CTL {}
impl OLED_CTL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const oled_ctl::RegisterBlock {
        0xe000_4000 as *const _
    }
}
impl Deref for OLED_CTL {
    type Target = oled_ctl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OLED_CTL::ptr() }
    }
}
#[doc = "OLED_CTL"]
pub mod oled_ctl;
#[doc = "LEDS"]
pub struct LEDS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LEDS {}
impl LEDS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const leds::RegisterBlock {
        0xe000_4800 as *const _
    }
}
impl Deref for LEDS {
    type Target = leds::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LEDS::ptr() }
    }
}
#[doc = "LEDS"]
pub mod leds;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "CTRL"]
    pub CTRL: CTRL,
    #[doc = "UART_PHY"]
    pub UART_PHY: UART_PHY,
    #[doc = "UART"]
    pub UART: UART,
    #[doc = "TIMER0"]
    pub TIMER0: TIMER0,
    #[doc = "SPIFLASH"]
    pub SPIFLASH: SPIFLASH,
    #[doc = "OLED_SPI"]
    pub OLED_SPI: OLED_SPI,
    #[doc = "OLED_SPI2"]
    pub OLED_SPI2: OLED_SPI2,
    #[doc = "OLED_CTL"]
    pub OLED_CTL: OLED_CTL,
    #[doc = "LEDS"]
    pub LEDS: LEDS,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        riscv::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            CTRL: CTRL {
                _marker: PhantomData,
            },
            UART_PHY: UART_PHY {
                _marker: PhantomData,
            },
            UART: UART {
                _marker: PhantomData,
            },
            TIMER0: TIMER0 {
                _marker: PhantomData,
            },
            SPIFLASH: SPIFLASH {
                _marker: PhantomData,
            },
            OLED_SPI: OLED_SPI {
                _marker: PhantomData,
            },
            OLED_SPI2: OLED_SPI2 {
                _marker: PhantomData,
            },
            OLED_CTL: OLED_CTL {
                _marker: PhantomData,
            },
            LEDS: LEDS {
                _marker: PhantomData,
            },
        }
    }
}
