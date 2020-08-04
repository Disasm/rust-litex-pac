#[doc = "Reader of register TUNING_WORD"]
pub type R = crate::R<u32, super::TUNING_WORD>;
#[doc = "Writer for register TUNING_WORD"]
pub type W = crate::W<u32, super::TUNING_WORD>;
#[doc = "Register TUNING_WORD `reset()`'s with value 0x0167_8303"]
impl crate::ResetValue for super::TUNING_WORD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0167_8303
    }
}
#[doc = "Reader of field `tuning_word`"]
pub type TUNING_WORD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `tuning_word`"]
pub struct TUNING_WORD_W<'a> {
    w: &'a mut W,
}
impl<'a> TUNING_WORD_W<'a> {
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
    pub fn tuning_word(&self) -> TUNING_WORD_R {
        TUNING_WORD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tuning_word(&mut self) -> TUNING_WORD_W {
        TUNING_WORD_W { w: self }
    }
}
