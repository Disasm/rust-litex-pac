#[doc = "Reader of register MOSI"]
pub type R = crate::R<u32, super::MOSI>;
#[doc = "Writer for register MOSI"]
pub type W = crate::W<u32, super::MOSI>;
#[doc = "Register MOSI `reset()`'s with value 0"]
impl crate::ResetValue for super::MOSI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `mosi`"]
pub type MOSI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `mosi`"]
pub struct MOSI_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn mosi(&self) -> MOSI_R {
        MOSI_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn mosi(&mut self) -> MOSI_W {
        MOSI_W { w: self }
    }
}
