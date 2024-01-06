#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    config: CONFIG,
    interrupt_state: INTERRUPT_STATE,
    bus_busy: BUS_BUSY,
    _reserved3: [u8; 0x04],
    period_control: PERIOD_CONTROL,
    period_interval: PERIOD_INTERVAL,
    ignore_index: IGNORE_INDEX,
    timeout: TIMEOUT,
    _reserved7: [u8; 0x60],
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
    #[doc = "0x08 - Bus busy state indicator"]
    #[inline(always)]
    pub const fn bus_busy(&self) -> &BUS_BUSY {
        &self.bus_busy
    }
    #[doc = "0x10 - Duration of control signals"]
    #[inline(always)]
    pub const fn period_control(&self) -> &PERIOD_CONTROL {
        &self.period_control
    }
    #[doc = "0x14 - Interval bitween frames"]
    #[inline(always)]
    pub const fn period_interval(&self) -> &PERIOD_INTERVAL {
        &self.period_interval
    }
    #[doc = "0x18 - Receive ignore index configuration"]
    #[inline(always)]
    pub const fn ignore_index(&self) -> &IGNORE_INDEX {
        &self.ignore_index
    }
    #[doc = "0x1c - Slave mode transmit timeout values"]
    #[inline(always)]
    pub const fn timeout(&self) -> &TIMEOUT {
        &self.timeout
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
#[doc = "bus_busy (rw) register accessor: Bus busy state indicator\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_busy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_busy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_busy`]
module"]
pub type BUS_BUSY = crate::Reg<bus_busy::BUS_BUSY_SPEC>;
#[doc = "Bus busy state indicator"]
pub mod bus_busy;
#[doc = "period_control (rw) register accessor: Duration of control signals\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`period_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`period_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@period_control`]
module"]
pub type PERIOD_CONTROL = crate::Reg<period_control::PERIOD_CONTROL_SPEC>;
#[doc = "Duration of control signals"]
pub mod period_control;
#[doc = "period_interval (rw) register accessor: Interval bitween frames\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`period_interval::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`period_interval::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@period_interval`]
module"]
pub type PERIOD_INTERVAL = crate::Reg<period_interval::PERIOD_INTERVAL_SPEC>;
#[doc = "Interval bitween frames"]
pub mod period_interval;
#[doc = "ignore_index (rw) register accessor: Receive ignore index configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ignore_index::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ignore_index::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ignore_index`]
module"]
pub type IGNORE_INDEX = crate::Reg<ignore_index::IGNORE_INDEX_SPEC>;
#[doc = "Receive ignore index configuration"]
pub mod ignore_index;
#[doc = "timeout (rw) register accessor: Slave mode transmit timeout values\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeout`]
module"]
pub type TIMEOUT = crate::Reg<timeout::TIMEOUT_SPEC>;
#[doc = "Slave mode transmit timeout values"]
pub mod timeout;
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
#[doc = "data_write (rw) register accessor: FIFO write data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_write::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_write::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_write`]
module"]
pub type DATA_WRITE = crate::Reg<data_write::DATA_WRITE_SPEC>;
#[doc = "FIFO write data register"]
pub mod data_write;
#[doc = "data_read (rw) register accessor: FIFO read data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_read::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_read::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_read`]
module"]
pub type DATA_READ = crate::Reg<data_read::DATA_READ_SPEC>;
#[doc = "FIFO read data register"]
pub mod data_read;
