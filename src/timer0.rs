#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Load value when Timer is (re-)enabled. In One-Shot mode, the value written to this register specifies the Timer's duration in clock cycles."]
    pub load: LOAD,
    #[doc = "0x04 - Reload value when Timer reaches ``0``. In Periodic mode, the value written to this register specify the Timer's period in clock cycles."]
    pub reload: RELOAD,
    #[doc = "0x08 - Enable flag of the Timer. Set this flag to ``1`` to enable/start the Timer. Set to ``0`` to disable the Timer."]
    pub en: EN,
    #[doc = "0x0c - Update trigger for the current countdown value. A write to this register latches the current countdown value to ``value`` register."]
    pub update_value: UPDATE_VALUE,
    #[doc = "0x10 - Latched countdown value. This value is updated by writing to ``update_value``."]
    pub value: VALUE,
    #[doc = "0x14 - "]
    pub ev_status: EV_STATUS,
    #[doc = "0x18 - "]
    pub ev_pending: EV_PENDING,
    #[doc = "0x1c - "]
    pub ev_enable: EV_ENABLE,
}
#[doc = "Load value when Timer is (re-)enabled. In One-Shot mode, the value written to this register specifies the Timer's duration in clock cycles.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [load](load) module"]
pub type LOAD = crate::Reg<u32, _LOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOAD;
#[doc = "`read()` method returns [load::R](load::R) reader structure"]
impl crate::Readable for LOAD {}
#[doc = "`write(|w| ..)` method takes [load::W](load::W) writer structure"]
impl crate::Writable for LOAD {}
#[doc = "Load value when Timer is (re-)enabled. In One-Shot mode, the value written to this register specifies the Timer's duration in clock cycles."]
pub mod load;
#[doc = "Reload value when Timer reaches ``0``. In Periodic mode, the value written to this register specify the Timer's period in clock cycles.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reload](reload) module"]
pub type RELOAD = crate::Reg<u32, _RELOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RELOAD;
#[doc = "`read()` method returns [reload::R](reload::R) reader structure"]
impl crate::Readable for RELOAD {}
#[doc = "`write(|w| ..)` method takes [reload::W](reload::W) writer structure"]
impl crate::Writable for RELOAD {}
#[doc = "Reload value when Timer reaches ``0``. In Periodic mode, the value written to this register specify the Timer's period in clock cycles."]
pub mod reload;
#[doc = "Enable flag of the Timer. Set this flag to ``1`` to enable/start the Timer. Set to ``0`` to disable the Timer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en](en) module"]
pub type EN = crate::Reg<u32, _EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EN;
#[doc = "`read()` method returns [en::R](en::R) reader structure"]
impl crate::Readable for EN {}
#[doc = "`write(|w| ..)` method takes [en::W](en::W) writer structure"]
impl crate::Writable for EN {}
#[doc = "Enable flag of the Timer. Set this flag to ``1`` to enable/start the Timer. Set to ``0`` to disable the Timer."]
pub mod en;
#[doc = "Update trigger for the current countdown value. A write to this register latches the current countdown value to ``value`` register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [update_value](update_value) module"]
pub type UPDATE_VALUE = crate::Reg<u32, _UPDATE_VALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPDATE_VALUE;
#[doc = "`read()` method returns [update_value::R](update_value::R) reader structure"]
impl crate::Readable for UPDATE_VALUE {}
#[doc = "`write(|w| ..)` method takes [update_value::W](update_value::W) writer structure"]
impl crate::Writable for UPDATE_VALUE {}
#[doc = "Update trigger for the current countdown value. A write to this register latches the current countdown value to ``value`` register."]
pub mod update_value;
#[doc = "Latched countdown value. This value is updated by writing to ``update_value``.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [value](value) module"]
pub type VALUE = crate::Reg<u32, _VALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VALUE;
#[doc = "`read()` method returns [value::R](value::R) reader structure"]
impl crate::Readable for VALUE {}
#[doc = "Latched countdown value. This value is updated by writing to ``update_value``."]
pub mod value;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ev_status](ev_status) module"]
pub type EV_STATUS = crate::Reg<u32, _EV_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EV_STATUS;
#[doc = "`read()` method returns [ev_status::R](ev_status::R) reader structure"]
impl crate::Readable for EV_STATUS {}
#[doc = "`write(|w| ..)` method takes [ev_status::W](ev_status::W) writer structure"]
impl crate::Writable for EV_STATUS {}
#[doc = ""]
pub mod ev_status;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ev_pending](ev_pending) module"]
pub type EV_PENDING = crate::Reg<u32, _EV_PENDING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EV_PENDING;
#[doc = "`read()` method returns [ev_pending::R](ev_pending::R) reader structure"]
impl crate::Readable for EV_PENDING {}
#[doc = "`write(|w| ..)` method takes [ev_pending::W](ev_pending::W) writer structure"]
impl crate::Writable for EV_PENDING {}
#[doc = ""]
pub mod ev_pending;
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ev_enable](ev_enable) module"]
pub type EV_ENABLE = crate::Reg<u32, _EV_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EV_ENABLE;
#[doc = "`read()` method returns [ev_enable::R](ev_enable::R) reader structure"]
impl crate::Readable for EV_ENABLE {}
#[doc = "`write(|w| ..)` method takes [ev_enable::W](ev_enable::W) writer structure"]
impl crate::Writable for EV_ENABLE {}
#[doc = ""]
pub mod ev_enable;
