#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    config: CONFIG,
    interrupt_state: INTERRUPT_STATE,
    _reserved2: [u8; 0x08],
    base_clock: BASE_CLOCK,
    _reserved3: [u8; 0x6c],
    fifo_config_0: FIFO_CONFIG_0,
    fifo_config_1: FIFO_CONFIG_1,
    data_write: DATA_WRITE,
    data_read: DATA_READ,
}
impl RegisterBlock {
    #[doc = "0x00 - Function configuration register"]
    #[inline(always)]
    pub const fn config(&self) -> &CONFIG {
        &self.config
    }
    #[doc = "0x04 - Interrupt enables, masks and states"]
    #[inline(always)]
    pub const fn interrupt_state(&self) -> &INTERRUPT_STATE {
        &self.interrupt_state
    }
    #[doc = "0x10 - Base clock divider"]
    #[inline(always)]
    pub const fn base_clock(&self) -> &BASE_CLOCK {
        &self.base_clock
    }
    #[doc = "0x80 - FIFO configuration register 0"]
    #[inline(always)]
    pub const fn fifo_config_0(&self) -> &FIFO_CONFIG_0 {
        &self.fifo_config_0
    }
    #[doc = "0x84 - FIFO configuration register 1"]
    #[inline(always)]
    pub const fn fifo_config_1(&self) -> &FIFO_CONFIG_1 {
        &self.fifo_config_1
    }
    #[doc = "0x88 - FIFO write data register"]
    #[inline(always)]
    pub const fn data_write(&self) -> &DATA_WRITE {
        &self.data_write
    }
    #[doc = "0x8c - FIFO read data register"]
    #[inline(always)]
    pub const fn data_read(&self) -> &DATA_READ {
        &self.data_read
    }
}
#[doc = "config (rw) register accessor: Function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Function configuration register"]
pub mod config;
#[doc = "interrupt_state (rw) register accessor: Interrupt enables, masks and states\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_state`]
module"]
pub type INTERRUPT_STATE = crate::Reg<interrupt_state::INTERRUPT_STATE_SPEC>;
#[doc = "Interrupt enables, masks and states"]
pub mod interrupt_state;
#[doc = "base_clock (rw) register accessor: Base clock divider\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`base_clock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`base_clock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base_clock`]
module"]
pub type BASE_CLOCK = crate::Reg<base_clock::BASE_CLOCK_SPEC>;
#[doc = "Base clock divider"]
pub mod base_clock;
#[doc = "fifo_config_0 (rw) register accessor: FIFO configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_config_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_config_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_config_0`]
module"]
pub type FIFO_CONFIG_0 = crate::Reg<fifo_config_0::FIFO_CONFIG_0_SPEC>;
#[doc = "FIFO configuration register 0"]
pub mod fifo_config_0;
#[doc = "fifo_config_1 (rw) register accessor: FIFO configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_config_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_config_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_config_1`]
module"]
pub type FIFO_CONFIG_1 = crate::Reg<fifo_config_1::FIFO_CONFIG_1_SPEC>;
#[doc = "FIFO configuration register 1"]
pub mod fifo_config_1;
#[doc = "data_write (w) register accessor: FIFO write data register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_write::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_write`]
module"]
pub type DATA_WRITE = crate::Reg<data_write::DATA_WRITE_SPEC>;
#[doc = "FIFO write data register"]
pub mod data_write;
#[doc = "data_read (r) register accessor: FIFO read data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_read::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_read`]
module"]
pub type DATA_READ = crate::Reg<data_read::DATA_READ_SPEC>;
#[doc = "FIFO read data register"]
pub mod data_read;
