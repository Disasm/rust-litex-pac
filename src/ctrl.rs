#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Write a ``1`` to this register to reset the SoC."]
    pub reset: RESET,
    #[doc = "0x04 - Use this register as a scratch space to verify that software read/write accesses to the Wishbone/CSR bus are working correctly. The initial reset value of 0x1234578 can be used to verify endianness."]
    pub scratch: SCRATCH,
    #[doc = "0x08 - Total number of Wishbone bus errors (timeouts) since start."]
    pub bus_errors: BUS_ERRORS,
}
#[doc = "Write a ``1`` to this register to reset the SoC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset](reset) module"]
pub type RESET = crate::Reg<u32, _RESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESET;
#[doc = "`read()` method returns [reset::R](reset::R) reader structure"]
impl crate::Readable for RESET {}
#[doc = "`write(|w| ..)` method takes [reset::W](reset::W) writer structure"]
impl crate::Writable for RESET {}
#[doc = "Write a ``1`` to this register to reset the SoC."]
pub mod reset;
#[doc = "Use this register as a scratch space to verify that software read/write accesses to the Wishbone/CSR bus are working correctly. The initial reset value of 0x1234578 can be used to verify endianness.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scratch](scratch) module"]
pub type SCRATCH = crate::Reg<u32, _SCRATCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCRATCH;
#[doc = "`read()` method returns [scratch::R](scratch::R) reader structure"]
impl crate::Readable for SCRATCH {}
#[doc = "`write(|w| ..)` method takes [scratch::W](scratch::W) writer structure"]
impl crate::Writable for SCRATCH {}
#[doc = "Use this register as a scratch space to verify that software read/write accesses to the Wishbone/CSR bus are working correctly. The initial reset value of 0x1234578 can be used to verify endianness."]
pub mod scratch;
#[doc = "Total number of Wishbone bus errors (timeouts) since start.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_errors](bus_errors) module"]
pub type BUS_ERRORS = crate::Reg<u32, _BUS_ERRORS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUS_ERRORS;
#[doc = "`read()` method returns [bus_errors::R](bus_errors::R) reader structure"]
impl crate::Readable for BUS_ERRORS {}
#[doc = "Total number of Wishbone bus errors (timeouts) since start."]
pub mod bus_errors;
