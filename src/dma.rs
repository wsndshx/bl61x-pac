#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    interrupt_state: INTERRUPT_STATE,
    terminate_state: TERMINATE_STATE,
    terminate_clear: TERMINATE_CLEAR,
    error_state: ERROR_STATE,
    error_clear: ERROR_CLEAR,
    terminate_state_raw: TERMINATE_STATE_RAW,
    error_state_raw: ERROR_STATE_RAW,
    channel_state: CHANNEL_STATE,
    burst_request: BURST_REQUEST,
    single_request: SINGLE_REQUEST,
    last_burst_request: LAST_BURST_REQUEST,
    last_single_request: LAST_SINGLE_REQUEST,
    config: CONFIG,
    synchronize: SYNCHRONIZE,
    _reserved14: [u8; 0xc8],
    channel: (),
}
impl RegisterBlock {
    #[doc = "0x00 - ??"]
    #[inline(always)]
    pub const fn interrupt_state(&self) -> &INTERRUPT_STATE {
        &self.interrupt_state
    }
    #[doc = "0x04 - ??"]
    #[inline(always)]
    pub const fn terminate_state(&self) -> &TERMINATE_STATE {
        &self.terminate_state
    }
    #[doc = "0x08 - ??"]
    #[inline(always)]
    pub const fn terminate_clear(&self) -> &TERMINATE_CLEAR {
        &self.terminate_clear
    }
    #[doc = "0x0c - ??"]
    #[inline(always)]
    pub const fn error_state(&self) -> &ERROR_STATE {
        &self.error_state
    }
    #[doc = "0x10 - ??"]
    #[inline(always)]
    pub const fn error_clear(&self) -> &ERROR_CLEAR {
        &self.error_clear
    }
    #[doc = "0x14 - ??"]
    #[inline(always)]
    pub const fn terminate_state_raw(&self) -> &TERMINATE_STATE_RAW {
        &self.terminate_state_raw
    }
    #[doc = "0x18 - ??"]
    #[inline(always)]
    pub const fn error_state_raw(&self) -> &ERROR_STATE_RAW {
        &self.error_state_raw
    }
    #[doc = "0x1c - ??"]
    #[inline(always)]
    pub const fn channel_state(&self) -> &CHANNEL_STATE {
        &self.channel_state
    }
    #[doc = "0x20 - ??"]
    #[inline(always)]
    pub const fn burst_request(&self) -> &BURST_REQUEST {
        &self.burst_request
    }
    #[doc = "0x24 - ??"]
    #[inline(always)]
    pub const fn single_request(&self) -> &SINGLE_REQUEST {
        &self.single_request
    }
    #[doc = "0x28 - ??"]
    #[inline(always)]
    pub const fn last_burst_request(&self) -> &LAST_BURST_REQUEST {
        &self.last_burst_request
    }
    #[doc = "0x2c - ??"]
    #[inline(always)]
    pub const fn last_single_request(&self) -> &LAST_SINGLE_REQUEST {
        &self.last_single_request
    }
    #[doc = "0x30 - ??"]
    #[inline(always)]
    pub const fn config(&self) -> &CONFIG {
        &self.config
    }
    #[doc = "0x34 - ??"]
    #[inline(always)]
    pub const fn synchronize(&self) -> &SYNCHRONIZE {
        &self.synchronize
    }
    #[doc = "0x100..0x150 - Direct Memory Access channel"]
    #[inline(always)]
    pub const fn channel(&self, n: usize) -> &CHANNEL {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(256)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x150 - Direct Memory Access channel"]
    #[inline(always)]
    pub fn channel_iter(&self) -> impl Iterator<Item = &CHANNEL> {
        (0..4).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(256)
                .add(256 * n)
                .cast()
        })
    }
}
#[doc = "interrupt_state (rw) register accessor: ??\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_state`]
module"]
pub type INTERRUPT_STATE = crate::Reg<interrupt_state::INTERRUPT_STATE_SPEC>;
#[doc = "??"]
pub mod interrupt_state;
#[doc = "terminate_state (rw) register accessor: ??\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`terminate_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`terminate_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@terminate_state`]
module"]
pub type TERMINATE_STATE = crate::Reg<terminate_state::TERMINATE_STATE_SPEC>;
#[doc = "??"]
pub mod terminate_state;
#[doc = "terminate_clear (rw) register accessor: ??\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`terminate_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`terminate_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@terminate_clear`]
module"]
pub type TERMINATE_CLEAR = crate::Reg<terminate_clear::TERMINATE_CLEAR_SPEC>;
#[doc = "??"]
pub mod terminate_clear;
#[doc = "error_state (rw) register accessor: ??\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`error_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`error_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@error_state`]
module"]
pub type ERROR_STATE = crate::Reg<error_state::ERROR_STATE_SPEC>;
#[doc = "??"]
pub mod error_state;
#[doc = "error_clear (rw) register accessor: ??\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`error_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`error_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@error_clear`]
module"]
pub type ERROR_CLEAR = crate::Reg<error_clear::ERROR_CLEAR_SPEC>;
#[doc = "??"]
pub mod error_clear;
#[doc = "terminate_state_raw (rw) register accessor: ??\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`terminate_state_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`terminate_state_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@terminate_state_raw`]
module"]
pub type TERMINATE_STATE_RAW = crate::Reg<terminate_state_raw::TERMINATE_STATE_RAW_SPEC>;
#[doc = "??"]
pub mod terminate_state_raw;
#[doc = "error_state_raw (rw) register accessor: ??\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`error_state_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`error_state_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@error_state_raw`]
module"]
pub type ERROR_STATE_RAW = crate::Reg<error_state_raw::ERROR_STATE_RAW_SPEC>;
#[doc = "??"]
pub mod error_state_raw;
#[doc = "channel_state (rw) register accessor: ??\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`channel_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`channel_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@channel_state`]
module"]
pub type CHANNEL_STATE = crate::Reg<channel_state::CHANNEL_STATE_SPEC>;
#[doc = "??"]
pub mod channel_state;
#[doc = "burst_request (rw) register accessor: ??\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`burst_request::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`burst_request::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@burst_request`]
module"]
pub type BURST_REQUEST = crate::Reg<burst_request::BURST_REQUEST_SPEC>;
#[doc = "??"]
pub mod burst_request;
#[doc = "single_request (rw) register accessor: ??\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`single_request::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`single_request::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@single_request`]
module"]
pub type SINGLE_REQUEST = crate::Reg<single_request::SINGLE_REQUEST_SPEC>;
#[doc = "??"]
pub mod single_request;
#[doc = "last_burst_request (rw) register accessor: ??\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`last_burst_request::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`last_burst_request::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@last_burst_request`]
module"]
pub type LAST_BURST_REQUEST = crate::Reg<last_burst_request::LAST_BURST_REQUEST_SPEC>;
#[doc = "??"]
pub mod last_burst_request;
#[doc = "last_single_request (rw) register accessor: ??\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`last_single_request::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`last_single_request::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@last_single_request`]
module"]
pub type LAST_SINGLE_REQUEST = crate::Reg<last_single_request::LAST_SINGLE_REQUEST_SPEC>;
#[doc = "??"]
pub mod last_single_request;
#[doc = "config (rw) register accessor: ??\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "??"]
pub mod config;
#[doc = "synchronize (rw) register accessor: ??\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`synchronize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`synchronize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@synchronize`]
module"]
pub type SYNCHRONIZE = crate::Reg<synchronize::SYNCHRONIZE_SPEC>;
#[doc = "??"]
pub mod synchronize;
#[doc = "Direct Memory Access channel"]
pub use self::channel::CHANNEL;
#[doc = r"Cluster"]
#[doc = "Direct Memory Access channel"]
pub mod channel;
