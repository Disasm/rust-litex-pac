#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `done`"]
pub type DONE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - SPI Xfer done when read as ``1``."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 0x01) != 0)
    }
}
