#[doc = r"Register block"]
#[repr(C)]
pub struct GROUP {
    config: CONFIG,
    channel: CHANNEL,
    period: PERIOD,
    dead_time: DEAD_TIME,
    threshold: [THRESHOLD; 4],
    interrupt_state: INTERRUPT_STATE,
    interrupt_mask: INTERRUPT_MASK,
    interrupt_clear: INTERRUPT_CLEAR,
    interrupt_enable: INTERRUPT_ENABLE,
}
impl GROUP {
    #[doc = "0x00 - Peripheral group configuration"]
    #[inline(always)]
    pub const fn config(&self) -> &CONFIG {
        &self.config
    }
    #[doc = "0x04 - Channel configuration register"]
    #[inline(always)]
    pub const fn channel(&self) -> &CHANNEL {
        &self.channel
    }
    #[doc = "0x08 - Pulse clock period register"]
    #[inline(always)]
    pub const fn period(&self) -> &PERIOD {
        &self.period
    }
    #[doc = "0x0c - Dead time for each channel"]
    #[inline(always)]
    pub const fn dead_time(&self) -> &DEAD_TIME {
        &self.dead_time
    }
    #[doc = "0x10..0x20 - Channel internal counter threshold"]
    #[inline(always)]
    pub const fn threshold(&self, n: usize) -> &THRESHOLD {
        &self.threshold[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x20 - Channel internal counter threshold"]
    #[inline(always)]
    pub fn threshold_iter(&self) -> impl Iterator<Item = &THRESHOLD> {
        self.threshold.iter()
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
}
#[doc = "config (rw) register accessor: Peripheral group configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Peripheral group configuration"]
pub mod config;
#[doc = "channel (rw) register accessor: Channel configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`channel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`channel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@channel`]
module"]
pub type CHANNEL = crate::Reg<channel::CHANNEL_SPEC>;
#[doc = "Channel configuration register"]
pub mod channel;
#[doc = "period (rw) register accessor: Pulse clock period register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`period::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`period::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@period`]
module"]
pub type PERIOD = crate::Reg<period::PERIOD_SPEC>;
#[doc = "Pulse clock period register"]
pub mod period;
#[doc = "dead_time (rw) register accessor: Dead time for each channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dead_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dead_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dead_time`]
module"]
pub type DEAD_TIME = crate::Reg<dead_time::DEAD_TIME_SPEC>;
#[doc = "Dead time for each channel"]
pub mod dead_time;
#[doc = "threshold (rw) register accessor: Channel internal counter threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`threshold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`threshold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@threshold`]
module"]
pub type THRESHOLD = crate::Reg<threshold::THRESHOLD_SPEC>;
#[doc = "Channel internal counter threshold"]
pub mod threshold;
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
