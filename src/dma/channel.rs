#[doc = r"Register block"]
#[repr(C)]
pub struct CHANNEL {
    source: SOURCE,
    destination: DESTINATION,
    linked_list: LINKED_LIST,
    control: CONTROL,
    config: CONFIG,
}
impl CHANNEL {
    #[doc = "0x00 - Source address"]
    #[inline(always)]
    pub const fn source(&self) -> &SOURCE {
        &self.source
    }
    #[doc = "0x04 - Destination address"]
    #[inline(always)]
    pub const fn destination(&self) -> &DESTINATION {
        &self.destination
    }
    #[doc = "0x08 - Linked list buffer base address"]
    #[inline(always)]
    pub const fn linked_list(&self) -> &LINKED_LIST {
        &self.linked_list
    }
    #[doc = "0x0c - Control register"]
    #[inline(always)]
    pub const fn control(&self) -> &CONTROL {
        &self.control
    }
    #[doc = "0x10 - Configuration register"]
    #[inline(always)]
    pub const fn config(&self) -> &CONFIG {
        &self.config
    }
}
#[doc = "source (rw) register accessor: Source address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`source::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`source::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@source`]
module"]
pub type SOURCE = crate::Reg<source::SOURCE_SPEC>;
#[doc = "Source address"]
pub mod source;
#[doc = "destination (rw) register accessor: Destination address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`destination::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`destination::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@destination`]
module"]
pub type DESTINATION = crate::Reg<destination::DESTINATION_SPEC>;
#[doc = "Destination address"]
pub mod destination;
#[doc = "linked_list (rw) register accessor: Linked list buffer base address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`linked_list::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linked_list::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@linked_list`]
module"]
pub type LINKED_LIST = crate::Reg<linked_list::LINKED_LIST_SPEC>;
#[doc = "Linked list buffer base address"]
pub mod linked_list;
#[doc = "control (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Control register"]
pub mod control;
#[doc = "config (rw) register accessor: Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration register"]
pub mod config;
