#[doc = "Reader of register RXFULL"]
pub type R = crate::R<u32, super::RXFULL>;
#[doc = "Reader of field `rxfull`"]
pub type RXFULL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new((self.bits & 0x01) != 0)
    }
}
