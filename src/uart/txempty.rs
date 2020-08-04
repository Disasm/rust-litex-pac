#[doc = "Reader of register TXEMPTY"]
pub type R = crate::R<u32, super::TXEMPTY>;
#[doc = "Reader of field `txempty`"]
pub type TXEMPTY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new((self.bits & 0x01) != 0)
    }
}
