#[doc = "Reader of register MISO"]
pub type R = crate::R<u32, super::MISO>;
#[doc = "Reader of field `miso`"]
pub type MISO_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn miso(&self) -> MISO_R {
        MISO_R::new((self.bits & 0xff) as u8)
    }
}
