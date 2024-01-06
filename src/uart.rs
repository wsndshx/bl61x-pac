#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    transmit_config: TRANSMIT_CONFIG,
    receive_config: RECEIVE_CONFIG,
    bit_period: BIT_PERIOD,
    data_config: DATA_CONFIG,
    transmit_position: TRANSMIT_POSITION,
    receive_position: RECEIVE_POSITION,
    receive_timeout: RECEIVE_TIMEOUT,
    signal_override: SIGNAL_OVERRIDE,
    interrupt_state: INTERRUPT_STATE,
    interrupt_mask: INTERRUPT_MASK,
    interrupt_clear: INTERRUPT_CLEAR,
    interrupt_enable: INTERRUPT_ENABLE,
    bus_state: BUS_STATE,
    auto_baudrate: AUTO_BAUDRATE,
    _reserved14: [u8; 0x10],
    pulse_tolerance: PULSE_TOLERANCE,
    _reserved15: [u8; 0x08],
    rs485_transmit: RS485_TRANSMIT,
    _reserved16: [u8; 0x28],
    fifo_config_0: FIFO_CONFIG_0,
    fifo_config_1: FIFO_CONFIG_1,
    data_write: DATA_WRITE,
    data_read: DATA_READ,
}
impl RegisterBlock {
    #[doc = "0x00 - Transmit configuration register"]
    #[inline(always)]
    pub const fn transmit_config(&self) -> &TRANSMIT_CONFIG {
        &self.transmit_config
    }
    #[doc = "0x04 - Receive configuration register"]
    #[inline(always)]
    pub const fn receive_config(&self) -> &RECEIVE_CONFIG {
        &self.receive_config
    }
    #[doc = "0x08 - Bit period control register"]
    #[inline(always)]
    pub const fn bit_period(&self) -> &BIT_PERIOD {
        &self.bit_period
    }
    #[doc = "0x0c - Data configuration register"]
    #[inline(always)]
    pub const fn data_config(&self) -> &DATA_CONFIG {
        &self.data_config
    }
    #[doc = "0x10 - IR-mode transmit position control"]
    #[inline(always)]
    pub const fn transmit_position(&self) -> &TRANSMIT_POSITION {
        &self.transmit_position
    }
    #[doc = "0x14 - IR-mode receive position control"]
    #[inline(always)]
    pub const fn receive_position(&self) -> &RECEIVE_POSITION {
        &self.receive_position
    }
    #[doc = "0x18 - Receive Time-Out interrupt control"]
    #[inline(always)]
    pub const fn receive_timeout(&self) -> &RECEIVE_TIMEOUT {
        &self.receive_timeout
    }
    #[doc = "0x1c - Manual override of flow control signal"]
    #[inline(always)]
    pub const fn signal_override(&self) -> &SIGNAL_OVERRIDE {
        &self.signal_override
    }
    #[doc = "0x20 - Interrupt state register"]
    #[inline(always)]
    pub const fn interrupt_state(&self) -> &INTERRUPT_STATE {
        &self.interrupt_state
    }
    #[doc = "0x24 - Interrupt mask register"]
    #[inline(always)]
    pub const fn interrupt_mask(&self) -> &INTERRUPT_MASK {
        &self.interrupt_mask
    }
    #[doc = "0x28 - Clear interrupt register"]
    #[inline(always)]
    pub const fn interrupt_clear(&self) -> &INTERRUPT_CLEAR {
        &self.interrupt_clear
    }
    #[doc = "0x2c - Interrupt enable register"]
    #[inline(always)]
    pub const fn interrupt_enable(&self) -> &INTERRUPT_ENABLE {
        &self.interrupt_enable
    }
    #[doc = "0x30 - Bus state register"]
    #[inline(always)]
    pub const fn bus_state(&self) -> &BUS_STATE {
        &self.bus_state
    }
    #[doc = "0x34 - Auto baudrate detection register"]
    #[inline(always)]
    pub const fn auto_baudrate(&self) -> &AUTO_BAUDRATE {
        &self.auto_baudrate
    }
    #[doc = "0x48 - Pulse width tolerance for auto baudrate"]
    #[inline(always)]
    pub const fn pulse_tolerance(&self) -> &PULSE_TOLERANCE {
        &self.pulse_tolerance
    }
    #[doc = "0x54 - RS-485 mode transmit configuration"]
    #[inline(always)]
    pub const fn rs485_transmit(&self) -> &RS485_TRANSMIT {
        &self.rs485_transmit
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
#[doc = "transmit_config (rw) register accessor: Transmit configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transmit_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`transmit_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@transmit_config`]
module"]
pub type TRANSMIT_CONFIG = crate::Reg<transmit_config::TRANSMIT_CONFIG_SPEC>;
#[doc = "Transmit configuration register"]
pub mod transmit_config;
#[doc = "receive_config (rw) register accessor: Receive configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`receive_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@receive_config`]
module"]
pub type RECEIVE_CONFIG = crate::Reg<receive_config::RECEIVE_CONFIG_SPEC>;
#[doc = "Receive configuration register"]
pub mod receive_config;
#[doc = "bit_period (rw) register accessor: Bit period control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bit_period::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bit_period::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bit_period`]
module"]
pub type BIT_PERIOD = crate::Reg<bit_period::BIT_PERIOD_SPEC>;
#[doc = "Bit period control register"]
pub mod bit_period;
#[doc = "data_config (rw) register accessor: Data configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_config`]
module"]
pub type DATA_CONFIG = crate::Reg<data_config::DATA_CONFIG_SPEC>;
#[doc = "Data configuration register"]
pub mod data_config;
#[doc = "transmit_position (rw) register accessor: IR-mode transmit position control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transmit_position::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`transmit_position::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@transmit_position`]
module"]
pub type TRANSMIT_POSITION = crate::Reg<transmit_position::TRANSMIT_POSITION_SPEC>;
#[doc = "IR-mode transmit position control"]
pub mod transmit_position;
#[doc = "receive_position (rw) register accessor: IR-mode receive position control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_position::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`receive_position::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@receive_position`]
module"]
pub type RECEIVE_POSITION = crate::Reg<receive_position::RECEIVE_POSITION_SPEC>;
#[doc = "IR-mode receive position control"]
pub mod receive_position;
#[doc = "receive_timeout (rw) register accessor: Receive Time-Out interrupt control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_timeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`receive_timeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@receive_timeout`]
module"]
pub type RECEIVE_TIMEOUT = crate::Reg<receive_timeout::RECEIVE_TIMEOUT_SPEC>;
#[doc = "Receive Time-Out interrupt control"]
pub mod receive_timeout;
#[doc = "signal_override (rw) register accessor: Manual override of flow control signal\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`signal_override::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`signal_override::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@signal_override`]
module"]
pub type SIGNAL_OVERRIDE = crate::Reg<signal_override::SIGNAL_OVERRIDE_SPEC>;
#[doc = "Manual override of flow control signal"]
pub mod signal_override;
#[doc = "interrupt_state (r) register accessor: Interrupt state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_state::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_state`]
module"]
pub type INTERRUPT_STATE = crate::Reg<interrupt_state::INTERRUPT_STATE_SPEC>;
#[doc = "Interrupt state register"]
pub mod interrupt_state;
#[doc = "interrupt_mask (rw) register accessor: Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_mask`]
module"]
pub type INTERRUPT_MASK = crate::Reg<interrupt_mask::INTERRUPT_MASK_SPEC>;
#[doc = "Interrupt mask register"]
pub mod interrupt_mask;
#[doc = "interrupt_clear (w) register accessor: Clear interrupt register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_clear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_clear`]
module"]
pub type INTERRUPT_CLEAR = crate::Reg<interrupt_clear::INTERRUPT_CLEAR_SPEC>;
#[doc = "Clear interrupt register"]
pub mod interrupt_clear;
#[doc = "interrupt_enable (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_enable`]
module"]
pub type INTERRUPT_ENABLE = crate::Reg<interrupt_enable::INTERRUPT_ENABLE_SPEC>;
#[doc = "Interrupt enable register"]
pub mod interrupt_enable;
#[doc = "bus_state (r) register accessor: Bus state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_state::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_state`]
module"]
pub type BUS_STATE = crate::Reg<bus_state::BUS_STATE_SPEC>;
#[doc = "Bus state register"]
pub mod bus_state;
#[doc = "auto_baudrate (r) register accessor: Auto baudrate detection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auto_baudrate::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@auto_baudrate`]
module"]
pub type AUTO_BAUDRATE = crate::Reg<auto_baudrate::AUTO_BAUDRATE_SPEC>;
#[doc = "Auto baudrate detection register"]
pub mod auto_baudrate;
#[doc = "pulse_tolerance (rw) register accessor: Pulse width tolerance for auto baudrate\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pulse_tolerance::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pulse_tolerance::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pulse_tolerance`]
module"]
pub type PULSE_TOLERANCE = crate::Reg<pulse_tolerance::PULSE_TOLERANCE_SPEC>;
#[doc = "Pulse width tolerance for auto baudrate"]
pub mod pulse_tolerance;
#[doc = "rs485_transmit (rw) register accessor: RS-485 mode transmit configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rs485_transmit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rs485_transmit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rs485_transmit`]
module"]
pub type RS485_TRANSMIT = crate::Reg<rs485_transmit::RS485_TRANSMIT_SPEC>;
#[doc = "RS-485 mode transmit configuration"]
pub mod rs485_transmit;
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
