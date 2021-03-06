#[doc = "Reader of register BUS_ERRORS"]
pub type R = crate::R<u32, super::BUS_ERRORS>;
#[doc = "Reader of field `bus_errors`"]
pub type BUS_ERRORS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bus_errors(&self) -> BUS_ERRORS_R {
        BUS_ERRORS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
