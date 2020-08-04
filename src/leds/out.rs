#[doc = "Reader of register OUT"]
pub type R = crate::R<u32, super::OUT>;
#[doc = "Writer for register OUT"]
pub type W = crate::W<u32, super::OUT>;
#[doc = "Register OUT `reset()`'s with value 0"]
impl crate::ResetValue for super::OUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ledr`"]
pub type LEDR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ledr`"]
pub struct LEDR_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ledg`"]
pub type LEDG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ledg`"]
pub struct LEDG_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `hledr1`"]
pub type HLEDR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `hledr1`"]
pub struct HLEDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> HLEDR1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `hledg2`"]
pub type HLEDG2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `hledg2`"]
pub struct HLEDG2_W<'a> {
    w: &'a mut W,
}
impl<'a> HLEDG2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `hledg3`"]
pub type HLEDG3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `hledg3`"]
pub struct HLEDG3_W<'a> {
    w: &'a mut W,
}
impl<'a> HLEDG3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `hledg4`"]
pub type HLEDG4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `hledg4`"]
pub struct HLEDG4_W<'a> {
    w: &'a mut W,
}
impl<'a> HLEDG4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `hledg5`"]
pub type HLEDG5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `hledg5`"]
pub struct HLEDG5_W<'a> {
    w: &'a mut W,
}
impl<'a> HLEDG5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - The Red LED on the main iCEBreaker board."]
    #[inline(always)]
    pub fn ledr(&self) -> LEDR_R {
        LEDR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The Green LED on the main iCEBreaker board."]
    #[inline(always)]
    pub fn ledg(&self) -> LEDG_R {
        LEDG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The center Red LED #1 on the iCEBreaker head."]
    #[inline(always)]
    pub fn hledr1(&self) -> HLEDR1_R {
        HLEDR1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Green LED #2 on the iCEBreaker head."]
    #[inline(always)]
    pub fn hledg2(&self) -> HLEDG2_R {
        HLEDG2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Green LED #3 on the iCEBreaker head."]
    #[inline(always)]
    pub fn hledg3(&self) -> HLEDG3_R {
        HLEDG3_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Green LED #4 on the iCEBreaker head."]
    #[inline(always)]
    pub fn hledg4(&self) -> HLEDG4_R {
        HLEDG4_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Green LED #5 on the iCEBreaker head."]
    #[inline(always)]
    pub fn hledg5(&self) -> HLEDG5_R {
        HLEDG5_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The Red LED on the main iCEBreaker board."]
    #[inline(always)]
    pub fn ledr(&mut self) -> LEDR_W {
        LEDR_W { w: self }
    }
    #[doc = "Bit 1 - The Green LED on the main iCEBreaker board."]
    #[inline(always)]
    pub fn ledg(&mut self) -> LEDG_W {
        LEDG_W { w: self }
    }
    #[doc = "Bit 2 - The center Red LED #1 on the iCEBreaker head."]
    #[inline(always)]
    pub fn hledr1(&mut self) -> HLEDR1_W {
        HLEDR1_W { w: self }
    }
    #[doc = "Bit 3 - Green LED #2 on the iCEBreaker head."]
    #[inline(always)]
    pub fn hledg2(&mut self) -> HLEDG2_W {
        HLEDG2_W { w: self }
    }
    #[doc = "Bit 4 - Green LED #3 on the iCEBreaker head."]
    #[inline(always)]
    pub fn hledg3(&mut self) -> HLEDG3_W {
        HLEDG3_W { w: self }
    }
    #[doc = "Bit 5 - Green LED #4 on the iCEBreaker head."]
    #[inline(always)]
    pub fn hledg4(&mut self) -> HLEDG4_W {
        HLEDG4_W { w: self }
    }
    #[doc = "Bit 6 - Green LED #5 on the iCEBreaker head."]
    #[inline(always)]
    pub fn hledg5(&mut self) -> HLEDG5_W {
        HLEDG5_W { w: self }
    }
}
