#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    clock: CLOCK,
    state: STATE,
    volume_0: VOLUME_0,
    volume_1: VOLUME_1,
    zero_signal: ZERO_SIGNAL,
    config: CONFIG,
    _reserved6: [u8; 0x74],
    fifo_control: FIFO_CONTROL,
    fifo_state: FIFO_STATE,
    fifo_data: FIFO_DATA,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock control register"]
    #[inline(always)]
    pub const fn clock(&self) -> &CLOCK {
        &self.clock
    }
    #[doc = "0x04 - Peripheral state register"]
    #[inline(always)]
    pub const fn state(&self) -> &STATE {
        &self.state
    }
    #[doc = "0x08 - Volume control register 0"]
    #[inline(always)]
    pub const fn volume_0(&self) -> &VOLUME_0 {
        &self.volume_0
    }
    #[doc = "0x0c - Volume control register 1"]
    #[inline(always)]
    pub const fn volume_1(&self) -> &VOLUME_1 {
        &self.volume_1
    }
    #[doc = "0x10 - Zero signal detection"]
    #[inline(always)]
    pub const fn zero_signal(&self) -> &ZERO_SIGNAL {
        &self.zero_signal
    }
    #[doc = "0x14 - Delta-Sigma and mixer control"]
    #[inline(always)]
    pub const fn config(&self) -> &CONFIG {
        &self.config
    }
    #[doc = "0x8c - Controls audio output FIFO"]
    #[inline(always)]
    pub const fn fifo_control(&self) -> &FIFO_CONTROL {
        &self.fifo_control
    }
    #[doc = "0x90 - Gets states of audio output FIFO"]
    #[inline(always)]
    pub const fn fifo_state(&self) -> &FIFO_STATE {
        &self.fifo_state
    }
    #[doc = "0x94 - Writes data into audio output FIFO"]
    #[inline(always)]
    pub const fn fifo_data(&self) -> &FIFO_DATA {
        &self.fifo_data
    }
}
#[doc = "clock (rw) register accessor: Clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock`]
module"]
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
#[doc = "Clock control register"]
pub mod clock;
#[doc = "state (rw) register accessor: Peripheral state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`]
module"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "Peripheral state register"]
pub mod state;
#[doc = "volume_0 (rw) register accessor: Volume control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`volume_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`volume_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@volume_0`]
module"]
pub type VOLUME_0 = crate::Reg<volume_0::VOLUME_0_SPEC>;
#[doc = "Volume control register 0"]
pub mod volume_0;
#[doc = "volume_1 (rw) register accessor: Volume control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`volume_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`volume_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@volume_1`]
module"]
pub type VOLUME_1 = crate::Reg<volume_1::VOLUME_1_SPEC>;
#[doc = "Volume control register 1"]
pub mod volume_1;
#[doc = "zero_signal (rw) register accessor: Zero signal detection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`zero_signal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`zero_signal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@zero_signal`]
module"]
pub type ZERO_SIGNAL = crate::Reg<zero_signal::ZERO_SIGNAL_SPEC>;
#[doc = "Zero signal detection"]
pub mod zero_signal;
#[doc = "config (rw) register accessor: Delta-Sigma and mixer control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Delta-Sigma and mixer control"]
pub mod config;
#[doc = "fifo_control (rw) register accessor: Controls audio output FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_control`]
module"]
pub type FIFO_CONTROL = crate::Reg<fifo_control::FIFO_CONTROL_SPEC>;
#[doc = "Controls audio output FIFO"]
pub mod fifo_control;
#[doc = "fifo_state (rw) register accessor: Gets states of audio output FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_state`]
module"]
pub type FIFO_STATE = crate::Reg<fifo_state::FIFO_STATE_SPEC>;
#[doc = "Gets states of audio output FIFO"]
pub mod fifo_state;
#[doc = "fifo_data (rw) register accessor: Writes data into audio output FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_data`]
module"]
pub type FIFO_DATA = crate::Reg<fifo_data::FIFO_DATA_SPEC>;
#[doc = "Writes data into audio output FIFO"]
pub mod fifo_data;
