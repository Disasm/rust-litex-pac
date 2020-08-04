#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI Control."]
    pub control: CONTROL,
    #[doc = "0x04 - SPI Status."]
    pub status: STATUS,
    #[doc = "0x08 - SPI MOSI data (MSB-first serialization)."]
    pub mosi: MOSI,
    #[doc = "0x0c - SPI MISO data (MSB-first de-serialization)."]
    pub miso: MISO,
    #[doc = "0x10 - SPI Chip Select."]
    pub cs: CS,
    #[doc = "0x14 - SPI loopback mode. Write ``1`` to enable MOSI to MISO internal loopback."]
    pub loopback: LOOPBACK,
    #[doc = "0x18 - SPI Clk Divider."]
    pub clk_divider: CLK_DIVIDER,
}
#[doc = "SPI Control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](control) module"]
pub type CONTROL = crate::Reg<u32, _CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONTROL;
#[doc = "`read()` method returns [control::R](control::R) reader structure"]
impl crate::Readable for CONTROL {}
#[doc = "`write(|w| ..)` method takes [control::W](control::W) writer structure"]
impl crate::Writable for CONTROL {}
#[doc = "SPI Control."]
pub mod control;
#[doc = "SPI Status.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "SPI Status."]
pub mod status;
#[doc = "SPI MOSI data (MSB-first serialization).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mosi](mosi) module"]
pub type MOSI = crate::Reg<u32, _MOSI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MOSI;
#[doc = "`read()` method returns [mosi::R](mosi::R) reader structure"]
impl crate::Readable for MOSI {}
#[doc = "`write(|w| ..)` method takes [mosi::W](mosi::W) writer structure"]
impl crate::Writable for MOSI {}
#[doc = "SPI MOSI data (MSB-first serialization)."]
pub mod mosi;
#[doc = "SPI MISO data (MSB-first de-serialization).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [miso](miso) module"]
pub type MISO = crate::Reg<u32, _MISO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISO;
#[doc = "`read()` method returns [miso::R](miso::R) reader structure"]
impl crate::Readable for MISO {}
#[doc = "SPI MISO data (MSB-first de-serialization)."]
pub mod miso;
#[doc = "SPI Chip Select.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cs](cs) module"]
pub type CS = crate::Reg<u32, _CS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CS;
#[doc = "`read()` method returns [cs::R](cs::R) reader structure"]
impl crate::Readable for CS {}
#[doc = "`write(|w| ..)` method takes [cs::W](cs::W) writer structure"]
impl crate::Writable for CS {}
#[doc = "SPI Chip Select."]
pub mod cs;
#[doc = "SPI loopback mode. Write ``1`` to enable MOSI to MISO internal loopback.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [loopback](loopback) module"]
pub type LOOPBACK = crate::Reg<u32, _LOOPBACK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOOPBACK;
#[doc = "`read()` method returns [loopback::R](loopback::R) reader structure"]
impl crate::Readable for LOOPBACK {}
#[doc = "`write(|w| ..)` method takes [loopback::W](loopback::W) writer structure"]
impl crate::Writable for LOOPBACK {}
#[doc = "SPI loopback mode. Write ``1`` to enable MOSI to MISO internal loopback."]
pub mod loopback;
#[doc = "SPI Clk Divider.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_divider](clk_divider) module"]
pub type CLK_DIVIDER = crate::Reg<u32, _CLK_DIVIDER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_DIVIDER;
#[doc = "`read()` method returns [clk_divider::R](clk_divider::R) reader structure"]
impl crate::Readable for CLK_DIVIDER {}
#[doc = "`write(|w| ..)` method takes [clk_divider::W](clk_divider::W) writer structure"]
impl crate::Writable for CLK_DIVIDER {}
#[doc = "SPI Clk Divider."]
pub mod clk_divider;
