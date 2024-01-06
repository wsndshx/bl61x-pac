#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    transmit_config: TRANSMIT_CONFIG,
    transmit_interrupt: TRANSMIT_INTERRUPT,
    transmit_data: [TRANSMIT_DATA; 2],
    transmit_width: TRANSMIT_WIDTH,
    _reserved4: [u8; 0x6c],
    receive_config: RECEIVE_CONFIG,
    receive_interrupt: RECEIVE_INTERRUPT,
    receive_width: RECEIVE_WIDTH,
    _reserved7: [u8; 0x04],
    receive_bit_count: RECEIVE_BIT_COUNT,
    receive_data: [RECEIVE_DATA; 2],
}
impl RegisterBlock {
    #[doc = "0x00 - ??"]
    #[inline(always)]
    pub const fn transmit_config(&self) -> &TRANSMIT_CONFIG {
        &self.transmit_config
    }
    #[doc = "0x04 - ??"]
    #[inline(always)]
    pub const fn transmit_interrupt(&self) -> &TRANSMIT_INTERRUPT {
        &self.transmit_interrupt
    }
    #[doc = "0x08..0x10 - ??"]
    #[inline(always)]
    pub const fn transmit_data(&self, n: usize) -> &TRANSMIT_DATA {
        &self.transmit_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x10 - ??"]
    #[inline(always)]
    pub fn transmit_data_iter(&self) -> impl Iterator<Item = &TRANSMIT_DATA> {
        self.transmit_data.iter()
    }
    #[doc = "0x10 - ??"]
    #[inline(always)]
    pub const fn transmit_width(&self) -> &TRANSMIT_WIDTH {
        &self.transmit_width
    }
    #[doc = "0x80 - ??"]
    #[inline(always)]
    pub const fn receive_config(&self) -> &RECEIVE_CONFIG {
        &self.receive_config
    }
    #[doc = "0x84 - ??"]
    #[inline(always)]
    pub const fn receive_interrupt(&self) -> &RECEIVE_INTERRUPT {
        &self.receive_interrupt
    }
    #[doc = "0x88 - ??"]
    #[inline(always)]
    pub const fn receive_width(&self) -> &RECEIVE_WIDTH {
        &self.receive_width
    }
    #[doc = "0x90 - ??"]
    #[inline(always)]
    pub const fn receive_bit_count(&self) -> &RECEIVE_BIT_COUNT {
        &self.receive_bit_count
    }
    #[doc = "0x94..0x9c - ??"]
    #[inline(always)]
    pub const fn receive_data(&self, n: usize) -> &RECEIVE_DATA {
        &self.receive_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x94..0x9c - ??"]
    #[inline(always)]
    pub fn receive_data_iter(&self) -> impl Iterator<Item = &RECEIVE_DATA> {
        self.receive_data.iter()
    }
}
#[doc = "transmit_config (rw) register accessor: ??\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transmit_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`transmit_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@transmit_config`]
module"]
pub type TRANSMIT_CONFIG = crate::Reg<transmit_config::TRANSMIT_CONFIG_SPEC>;
#[doc = "??"]
pub mod transmit_config;
#[doc = "transmit_interrupt (rw) register accessor: ??\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transmit_interrupt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`transmit_interrupt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@transmit_interrupt`]
module"]
pub type TRANSMIT_INTERRUPT = crate::Reg<transmit_interrupt::TRANSMIT_INTERRUPT_SPEC>;
#[doc = "??"]
pub mod transmit_interrupt;
#[doc = "transmit_data (rw) register accessor: ??\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transmit_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`transmit_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@transmit_data`]
module"]
pub type TRANSMIT_DATA = crate::Reg<transmit_data::TRANSMIT_DATA_SPEC>;
#[doc = "??"]
pub mod transmit_data;
#[doc = "transmit_width (rw) register accessor: ??\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transmit_width::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`transmit_width::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@transmit_width`]
module"]
pub type TRANSMIT_WIDTH = crate::Reg<transmit_width::TRANSMIT_WIDTH_SPEC>;
#[doc = "??"]
pub mod transmit_width;
#[doc = "receive_config (rw) register accessor: ??\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`receive_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@receive_config`]
module"]
pub type RECEIVE_CONFIG = crate::Reg<receive_config::RECEIVE_CONFIG_SPEC>;
#[doc = "??"]
pub mod receive_config;
#[doc = "receive_interrupt (rw) register accessor: ??\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_interrupt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`receive_interrupt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@receive_interrupt`]
module"]
pub type RECEIVE_INTERRUPT = crate::Reg<receive_interrupt::RECEIVE_INTERRUPT_SPEC>;
#[doc = "??"]
pub mod receive_interrupt;
#[doc = "receive_width (rw) register accessor: ??\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_width::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`receive_width::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@receive_width`]
module"]
pub type RECEIVE_WIDTH = crate::Reg<receive_width::RECEIVE_WIDTH_SPEC>;
#[doc = "??"]
pub mod receive_width;
#[doc = "receive_bit_count (rw) register accessor: ??\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_bit_count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`receive_bit_count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@receive_bit_count`]
module"]
pub type RECEIVE_BIT_COUNT = crate::Reg<receive_bit_count::RECEIVE_BIT_COUNT_SPEC>;
#[doc = "??"]
pub mod receive_bit_count;
#[doc = "receive_data (rw) register accessor: ??\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`receive_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@receive_data`]
module"]
pub type RECEIVE_DATA = crate::Reg<receive_data::RECEIVE_DATA_SPEC>;
#[doc = "??"]
pub mod receive_data;
