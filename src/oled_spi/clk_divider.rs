#[doc = "Reader of register CLK_DIVIDER"]
pub type R = crate::R<u32, super::CLK_DIVIDER>;
#[doc = "Writer for register CLK_DIVIDER"]
pub type W = crate::W<u32, super::CLK_DIVIDER>;
#[doc = "Register CLK_DIVIDER `reset()`'s with value 0x03"]
impl crate::ResetValue for super::CLK_DIVIDER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "Reader of field `clk_divider`"]
pub type CLK_DIVIDER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `clk_divider`"]
pub struct CLK_DIVIDER_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DIVIDER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn clk_divider(&self) -> CLK_DIVIDER_R {
        CLK_DIVIDER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn clk_divider(&mut self) -> CLK_DIVIDER_W {
        CLK_DIVIDER_W { w: self }
    }
}
