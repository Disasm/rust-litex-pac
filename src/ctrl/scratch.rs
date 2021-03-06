#[doc = "Reader of register SCRATCH"]
pub type R = crate::R<u32, super::SCRATCH>;
#[doc = "Writer for register SCRATCH"]
pub type W = crate::W<u32, super::SCRATCH>;
#[doc = "Register SCRATCH `reset()`'s with value 0x1234_5678"]
impl crate::ResetValue for super::SCRATCH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1234_5678
    }
}
#[doc = "Reader of field `scratch`"]
pub type SCRATCH_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `scratch`"]
pub struct SCRATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRATCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn scratch(&self) -> SCRATCH_R {
        SCRATCH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn scratch(&mut self) -> SCRATCH_W {
        SCRATCH_W { w: self }
    }
}
