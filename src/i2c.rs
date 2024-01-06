#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    config: CONFIG,
    interrupt: INTERRUPT,
    sub_address: SUB_ADDRESS,
    bus_busy: BUS_BUSY,
    period_start: PERIOD_START,
    period_stop: PERIOD_STOP,
    period_data: PERIOD_DATA,
    _reserved7: [u8; 0x64],
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
    #[doc = "0x04 - Interrupt enables, states and masks"]
    #[inline(always)]
    pub const fn interrupt(&self) -> &INTERRUPT {
        &self.interrupt
    }
    #[doc = "0x08 - Register address of slave device"]
    #[inline(always)]
    pub const fn sub_address(&self) -> &SUB_ADDRESS {
        &self.sub_address
    }
    #[doc = "0x0c - Bus busy state indicator"]
    #[inline(always)]
    pub const fn bus_busy(&self) -> &BUS_BUSY {
        &self.bus_busy
    }
    #[doc = "0x10 - Duration of start phase"]
    #[inline(always)]
    pub const fn period_start(&self) -> &PERIOD_START {
        &self.period_start
    }
    #[doc = "0x14 - Duration of stop phase"]
    #[inline(always)]
    pub const fn period_stop(&self) -> &PERIOD_STOP {
        &self.period_stop
    }
    #[doc = "0x18 - Duration of data phase"]
    #[inline(always)]
    pub const fn period_data(&self) -> &PERIOD_DATA {
        &self.period_data
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
#[doc = "interrupt (rw) register accessor: Interrupt enables, states and masks\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt`]
module"]
pub type INTERRUPT = crate::Reg<interrupt::INTERRUPT_SPEC>;
#[doc = "Interrupt enables, states and masks"]
pub mod interrupt;
#[doc = "sub_address (rw) register accessor: Register address of slave device\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sub_address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sub_address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sub_address`]
module"]
pub type SUB_ADDRESS = crate::Reg<sub_address::SUB_ADDRESS_SPEC>;
#[doc = "Register address of slave device"]
pub mod sub_address;
#[doc = "bus_busy (rw) register accessor: Bus busy state indicator\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_busy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_busy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_busy`]
module"]
pub type BUS_BUSY = crate::Reg<bus_busy::BUS_BUSY_SPEC>;
#[doc = "Bus busy state indicator"]
pub mod bus_busy;
#[doc = "period_start (rw) register accessor: Duration of start phase\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`period_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`period_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@period_start`]
module"]
pub type PERIOD_START = crate::Reg<period_start::PERIOD_START_SPEC>;
#[doc = "Duration of start phase"]
pub mod period_start;
#[doc = "period_stop (rw) register accessor: Duration of stop phase\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`period_stop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`period_stop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@period_stop`]
module"]
pub type PERIOD_STOP = crate::Reg<period_stop::PERIOD_STOP_SPEC>;
#[doc = "Duration of stop phase"]
pub mod period_stop;
#[doc = "period_data (rw) register accessor: Duration of data phase\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`period_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`period_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@period_data`]
module"]
pub type PERIOD_DATA = crate::Reg<period_data::PERIOD_DATA_SPEC>;
#[doc = "Duration of data phase"]
pub mod period_data;
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
