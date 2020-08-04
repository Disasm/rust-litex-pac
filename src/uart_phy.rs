#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub tuning_word: TUNING_WORD,
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tuning_word](tuning_word) module"]
pub type TUNING_WORD = crate::Reg<u32, _TUNING_WORD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TUNING_WORD;
#[doc = "`read()` method returns [tuning_word::R](tuning_word::R) reader structure"]
impl crate::Readable for TUNING_WORD {}
#[doc = "`write(|w| ..)` method takes [tuning_word::W](tuning_word::W) writer structure"]
impl crate::Writable for TUNING_WORD {}
#[doc = ""]
pub mod tuning_word;
