#[doc = "Reader of register RELOAD"]
pub type R = crate::R<u32, super::RELOAD>;
#[doc = "Writer for register RELOAD"]
pub type W = crate::W<u32, super::RELOAD>;
#[doc = "Register RELOAD `reset()`'s with value 0"]
impl crate::ResetValue for super::RELOAD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `reload`"]
pub type RELOAD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `reload`"]
pub struct RELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOAD_W<'a> {
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
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W {
        RELOAD_W { w: self }
    }
}
