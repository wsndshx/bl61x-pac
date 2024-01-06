#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    clock: CLOCK,
    interface_0: INTERFACE_0,
    finite_impulse: FINITE_IMPULSE,
    high_pass: HIGH_PASS,
    interface_1: INTERFACE_1,
    _reserved5: [u8; 0x08],
    pulse_width: PULSE_WIDTH,
    _reserved6: [u8; 0x18],
    volume: VOLUME,
    _reserved7: [u8; 0x24],
    analog_0: ANALOG_0,
    analog_1: ANALOG_1,
    command: COMMAND,
    sample_data: SAMPLE_DATA,
    _reserved11: [u8; 0x10],
    fifo_control: FIFO_CONTROL,
    fifo_state: FIFO_STATE,
    fifo_data: FIFO_DATA,
}
impl RegisterBlock {
    #[doc = "0x00 - Peripheral clock control register"]
    #[inline(always)]
    pub const fn clock(&self) -> &CLOCK {
        &self.clock
    }
    #[doc = "0x04 - Interface control register 0"]
    #[inline(always)]
    pub const fn interface_0(&self) -> &INTERFACE_0 {
        &self.interface_0
    }
    #[doc = "0x08 - Finite Impulse Response control"]
    #[inline(always)]
    pub const fn finite_impulse(&self) -> &FINITE_IMPULSE {
        &self.finite_impulse
    }
    #[doc = "0x0c - High-Pass Filter control register"]
    #[inline(always)]
    pub const fn high_pass(&self) -> &HIGH_PASS {
        &self.high_pass
    }
    #[doc = "0x10 - Interface control register 1"]
    #[inline(always)]
    pub const fn interface_1(&self) -> &INTERFACE_1 {
        &self.interface_1
    }
    #[doc = "0x1c - Pulse-Width Modulator control"]
    #[inline(always)]
    pub const fn pulse_width(&self) -> &PULSE_WIDTH {
        &self.pulse_width
    }
    #[doc = "0x38 - Volume control register"]
    #[inline(always)]
    pub const fn volume(&self) -> &VOLUME {
        &self.volume
    }
    #[doc = "0x60 - Analog signal configuration 0"]
    #[inline(always)]
    pub const fn analog_0(&self) -> &ANALOG_0 {
        &self.analog_0
    }
    #[doc = "0x64 - Analog signal configuration 1"]
    #[inline(always)]
    pub const fn analog_1(&self) -> &ANALOG_1 {
        &self.analog_1
    }
    #[doc = "0x68 - Analog-to-Digital Converter commands"]
    #[inline(always)]
    pub const fn command(&self) -> &COMMAND {
        &self.command
    }
    #[doc = "0x6c - Analog-to-Digital Converter sample output"]
    #[inline(always)]
    pub const fn sample_data(&self) -> &SAMPLE_DATA {
        &self.sample_data
    }
    #[doc = "0x80 - Controls audio input FIFO"]
    #[inline(always)]
    pub const fn fifo_control(&self) -> &FIFO_CONTROL {
        &self.fifo_control
    }
    #[doc = "0x84 - Gets states of audio input FIFO"]
    #[inline(always)]
    pub const fn fifo_state(&self) -> &FIFO_STATE {
        &self.fifo_state
    }
    #[doc = "0x88 - Reads data from audio input FIFO"]
    #[inline(always)]
    pub const fn fifo_data(&self) -> &FIFO_DATA {
        &self.fifo_data
    }
}
#[doc = "clock (rw) register accessor: Peripheral clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock`]
module"]
pub type CLOCK = crate::Reg<clock::CLOCK_SPEC>;
#[doc = "Peripheral clock control register"]
pub mod clock;
#[doc = "interface_0 (rw) register accessor: Interface control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interface_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interface_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interface_0`]
module"]
pub type INTERFACE_0 = crate::Reg<interface_0::INTERFACE_0_SPEC>;
#[doc = "Interface control register 0"]
pub mod interface_0;
#[doc = "finite_impulse (rw) register accessor: Finite Impulse Response control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`finite_impulse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`finite_impulse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@finite_impulse`]
module"]
pub type FINITE_IMPULSE = crate::Reg<finite_impulse::FINITE_IMPULSE_SPEC>;
#[doc = "Finite Impulse Response control"]
pub mod finite_impulse;
#[doc = "high_pass (rw) register accessor: High-Pass Filter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`high_pass::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`high_pass::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@high_pass`]
module"]
pub type HIGH_PASS = crate::Reg<high_pass::HIGH_PASS_SPEC>;
#[doc = "High-Pass Filter control register"]
pub mod high_pass;
#[doc = "interface_1 (rw) register accessor: Interface control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interface_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interface_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interface_1`]
module"]
pub type INTERFACE_1 = crate::Reg<interface_1::INTERFACE_1_SPEC>;
#[doc = "Interface control register 1"]
pub mod interface_1;
#[doc = "pulse_width (rw) register accessor: Pulse-Width Modulator control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pulse_width::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pulse_width::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pulse_width`]
module"]
pub type PULSE_WIDTH = crate::Reg<pulse_width::PULSE_WIDTH_SPEC>;
#[doc = "Pulse-Width Modulator control"]
pub mod pulse_width;
#[doc = "volume (rw) register accessor: Volume control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`volume::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`volume::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@volume`]
module"]
pub type VOLUME = crate::Reg<volume::VOLUME_SPEC>;
#[doc = "Volume control register"]
pub mod volume;
#[doc = "analog_0 (rw) register accessor: Analog signal configuration 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_0`]
module"]
pub type ANALOG_0 = crate::Reg<analog_0::ANALOG_0_SPEC>;
#[doc = "Analog signal configuration 0"]
pub mod analog_0;
#[doc = "analog_1 (rw) register accessor: Analog signal configuration 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`analog_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`analog_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@analog_1`]
module"]
pub type ANALOG_1 = crate::Reg<analog_1::ANALOG_1_SPEC>;
#[doc = "Analog signal configuration 1"]
pub mod analog_1;
#[doc = "command (rw) register accessor: Analog-to-Digital Converter commands\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`command::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`command::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@command`]
module"]
pub type COMMAND = crate::Reg<command::COMMAND_SPEC>;
#[doc = "Analog-to-Digital Converter commands"]
pub mod command;
#[doc = "sample_data (rw) register accessor: Analog-to-Digital Converter sample output\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sample_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sample_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sample_data`]
module"]
pub type SAMPLE_DATA = crate::Reg<sample_data::SAMPLE_DATA_SPEC>;
#[doc = "Analog-to-Digital Converter sample output"]
pub mod sample_data;
#[doc = "fifo_control (rw) register accessor: Controls audio input FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_control`]
module"]
pub type FIFO_CONTROL = crate::Reg<fifo_control::FIFO_CONTROL_SPEC>;
#[doc = "Controls audio input FIFO"]
pub mod fifo_control;
#[doc = "fifo_state (rw) register accessor: Gets states of audio input FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_state`]
module"]
pub type FIFO_STATE = crate::Reg<fifo_state::FIFO_STATE_SPEC>;
#[doc = "Gets states of audio input FIFO"]
pub mod fifo_state;
#[doc = "fifo_data (rw) register accessor: Reads data from audio input FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_data`]
module"]
pub type FIFO_DATA = crate::Reg<fifo_data::FIFO_DATA_SPEC>;
#[doc = "Reads data from audio input FIFO"]
pub mod fifo_data;
