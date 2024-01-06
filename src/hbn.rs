#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    control: CONTROL,
    time_lo: TIME_LO,
    time_hi: TIME_HI,
    rtc_time_lo: RTC_TIME_LO,
    rtc_time_hi: RTC_TIME_HI,
    interrupt_mode: INTERRUPT_MODE,
    interrupt_state: INTERRUPT_STATE,
    interrupt_clear: INTERRUPT_CLEAR,
    hbn_pir_cfg: HBN_PIR_CFG,
    hbn_pir_vth: HBN_PIR_VTH,
    hbn_pir_interval: HBN_PIR_INTERVAL,
    hbn_bor_cfg: HBN_BOR_CFG,
    global: GLOBAL,
    sram: SRAM,
    hbn_pad_ctrl_0: HBN_PAD_CTRL_0,
    hbn_pad_ctrl_1: HBN_PAD_CTRL_1,
    vbat_ldo: VBAT_LDO,
    _reserved17: [u8; 0xbc],
    hbn_rsv0: HBN_RSV0,
    hbn_rsv1: HBN_RSV1,
    hbn_rsv2: HBN_RSV2,
    hbn_rsv3: HBN_RSV3,
    _reserved21: [u8; 0xf0],
    rc32k: RC32K,
    xtal32k: XTAL32K,
    rtc_control_0: RTC_CONTROL_0,
    rtc_control_1: RTC_CONTROL_1,
}
impl RegisterBlock {
    #[doc = "0x00 - Miscellaneous control register"]
    #[inline(always)]
    pub const fn control(&self) -> &CONTROL {
        &self.control
    }
    #[doc = "0x04 - Low bits of hibernate time"]
    #[inline(always)]
    pub const fn time_lo(&self) -> &TIME_LO {
        &self.time_lo
    }
    #[doc = "0x08 - High bits of hibernate time"]
    #[inline(always)]
    pub const fn time_hi(&self) -> &TIME_HI {
        &self.time_hi
    }
    #[doc = "0x0c - Low bits of Real-Time Clock time"]
    #[inline(always)]
    pub const fn rtc_time_lo(&self) -> &RTC_TIME_LO {
        &self.rtc_time_lo
    }
    #[doc = "0x10 - High bits of Real-Time Clock time"]
    #[inline(always)]
    pub const fn rtc_time_hi(&self) -> &RTC_TIME_HI {
        &self.rtc_time_hi
    }
    #[doc = "0x14 - Hibernate interrupt contol"]
    #[inline(always)]
    pub const fn interrupt_mode(&self) -> &INTERRUPT_MODE {
        &self.interrupt_mode
    }
    #[doc = "0x18 - Hibernate interrupt state"]
    #[inline(always)]
    pub const fn interrupt_state(&self) -> &INTERRUPT_STATE {
        &self.interrupt_state
    }
    #[doc = "0x1c - Clear hibernate interrupt"]
    #[inline(always)]
    pub const fn interrupt_clear(&self) -> &INTERRUPT_CLEAR {
        &self.interrupt_clear
    }
    #[doc = "0x20 - HBN_PIR_CFG."]
    #[inline(always)]
    pub const fn hbn_pir_cfg(&self) -> &HBN_PIR_CFG {
        &self.hbn_pir_cfg
    }
    #[doc = "0x24 - HBN_PIR_VTH."]
    #[inline(always)]
    pub const fn hbn_pir_vth(&self) -> &HBN_PIR_VTH {
        &self.hbn_pir_vth
    }
    #[doc = "0x28 - HBN_PIR_INTERVAL."]
    #[inline(always)]
    pub const fn hbn_pir_interval(&self) -> &HBN_PIR_INTERVAL {
        &self.hbn_pir_interval
    }
    #[doc = "0x2c - HBN_BOR_CFG."]
    #[inline(always)]
    pub const fn hbn_bor_cfg(&self) -> &HBN_BOR_CFG {
        &self.hbn_bor_cfg
    }
    #[doc = "0x30 - Global hibernate configuration"]
    #[inline(always)]
    pub const fn global(&self) -> &GLOBAL {
        &self.global
    }
    #[doc = "0x34 - Static Random-Access Memory hibernate control"]
    #[inline(always)]
    pub const fn sram(&self) -> &SRAM {
        &self.sram
    }
    #[doc = "0x38 - HBN_PAD_CTRL_0."]
    #[inline(always)]
    pub const fn hbn_pad_ctrl_0(&self) -> &HBN_PAD_CTRL_0 {
        &self.hbn_pad_ctrl_0
    }
    #[doc = "0x3c - HBN_PAD_CTRL_1."]
    #[inline(always)]
    pub const fn hbn_pad_ctrl_1(&self) -> &HBN_PAD_CTRL_1 {
        &self.hbn_pad_ctrl_1
    }
    #[doc = "0x40 - vbat_ldo."]
    #[inline(always)]
    pub const fn vbat_ldo(&self) -> &VBAT_LDO {
        &self.vbat_ldo
    }
    #[doc = "0x100 - HBN_RSV0."]
    #[inline(always)]
    pub const fn hbn_rsv0(&self) -> &HBN_RSV0 {
        &self.hbn_rsv0
    }
    #[doc = "0x104 - HBN_RSV1."]
    #[inline(always)]
    pub const fn hbn_rsv1(&self) -> &HBN_RSV1 {
        &self.hbn_rsv1
    }
    #[doc = "0x108 - HBN_RSV2."]
    #[inline(always)]
    pub const fn hbn_rsv2(&self) -> &HBN_RSV2 {
        &self.hbn_rsv2
    }
    #[doc = "0x10c - HBN_RSV3."]
    #[inline(always)]
    pub const fn hbn_rsv3(&self) -> &HBN_RSV3 {
        &self.hbn_rsv3
    }
    #[doc = "0x200 - 32-kHz internal RC oscillator control"]
    #[inline(always)]
    pub const fn rc32k(&self) -> &RC32K {
        &self.rc32k
    }
    #[doc = "0x204 - External crystal oscillator control"]
    #[inline(always)]
    pub const fn xtal32k(&self) -> &XTAL32K {
        &self.xtal32k
    }
    #[doc = "0x208 - Real-Time Clock control and reset register 0"]
    #[inline(always)]
    pub const fn rtc_control_0(&self) -> &RTC_CONTROL_0 {
        &self.rtc_control_0
    }
    #[doc = "0x20c - Real-Time Clock control and reset register 1"]
    #[inline(always)]
    pub const fn rtc_control_1(&self) -> &RTC_CONTROL_1 {
        &self.rtc_control_1
    }
}
#[doc = "control (rw) register accessor: Miscellaneous control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Miscellaneous control register"]
pub mod control;
#[doc = "time_lo (rw) register accessor: Low bits of hibernate time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time_lo`]
module"]
pub type TIME_LO = crate::Reg<time_lo::TIME_LO_SPEC>;
#[doc = "Low bits of hibernate time"]
pub mod time_lo;
#[doc = "time_hi (rw) register accessor: High bits of hibernate time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time_hi`]
module"]
pub type TIME_HI = crate::Reg<time_hi::TIME_HI_SPEC>;
#[doc = "High bits of hibernate time"]
pub mod time_hi;
#[doc = "rtc_time_lo (rw) register accessor: Low bits of Real-Time Clock time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_time_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_time_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_time_lo`]
module"]
pub type RTC_TIME_LO = crate::Reg<rtc_time_lo::RTC_TIME_LO_SPEC>;
#[doc = "Low bits of Real-Time Clock time"]
pub mod rtc_time_lo;
#[doc = "rtc_time_hi (rw) register accessor: High bits of Real-Time Clock time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_time_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_time_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_time_hi`]
module"]
pub type RTC_TIME_HI = crate::Reg<rtc_time_hi::RTC_TIME_HI_SPEC>;
#[doc = "High bits of Real-Time Clock time"]
pub mod rtc_time_hi;
#[doc = "interrupt_mode (rw) register accessor: Hibernate interrupt contol\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_mode`]
module"]
pub type INTERRUPT_MODE = crate::Reg<interrupt_mode::INTERRUPT_MODE_SPEC>;
#[doc = "Hibernate interrupt contol"]
pub mod interrupt_mode;
#[doc = "interrupt_state (rw) register accessor: Hibernate interrupt state\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_state`]
module"]
pub type INTERRUPT_STATE = crate::Reg<interrupt_state::INTERRUPT_STATE_SPEC>;
#[doc = "Hibernate interrupt state"]
pub mod interrupt_state;
#[doc = "interrupt_clear (rw) register accessor: Clear hibernate interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_clear`]
module"]
pub type INTERRUPT_CLEAR = crate::Reg<interrupt_clear::INTERRUPT_CLEAR_SPEC>;
#[doc = "Clear hibernate interrupt"]
pub mod interrupt_clear;
#[doc = "global (rw) register accessor: Global hibernate configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`global::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`global::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@global`]
module"]
pub type GLOBAL = crate::Reg<global::GLOBAL_SPEC>;
#[doc = "Global hibernate configuration"]
pub mod global;
#[doc = "sram (rw) register accessor: Static Random-Access Memory hibernate control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram`]
module"]
pub type SRAM = crate::Reg<sram::SRAM_SPEC>;
#[doc = "Static Random-Access Memory hibernate control"]
pub mod sram;
#[doc = "rc32k (rw) register accessor: 32-kHz internal RC oscillator control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc32k::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rc32k::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rc32k`]
module"]
pub type RC32K = crate::Reg<rc32k::RC32K_SPEC>;
#[doc = "32-kHz internal RC oscillator control"]
pub mod rc32k;
#[doc = "xtal32k (rw) register accessor: External crystal oscillator control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xtal32k::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal32k::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtal32k`]
module"]
pub type XTAL32K = crate::Reg<xtal32k::XTAL32K_SPEC>;
#[doc = "External crystal oscillator control"]
pub mod xtal32k;
#[doc = "rtc_control_0 (rw) register accessor: Real-Time Clock control and reset register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_control_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_control_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_control_0`]
module"]
pub type RTC_CONTROL_0 = crate::Reg<rtc_control_0::RTC_CONTROL_0_SPEC>;
#[doc = "Real-Time Clock control and reset register 0"]
pub mod rtc_control_0;
#[doc = "rtc_control_1 (rw) register accessor: Real-Time Clock control and reset register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_control_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_control_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_control_1`]
module"]
pub type RTC_CONTROL_1 = crate::Reg<rtc_control_1::RTC_CONTROL_1_SPEC>;
#[doc = "Real-Time Clock control and reset register 1"]
pub mod rtc_control_1;
#[doc = "HBN_PIR_CFG (rw) register accessor: HBN_PIR_CFG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hbn_pir_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hbn_pir_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbn_pir_cfg`]
module"]
pub type HBN_PIR_CFG = crate::Reg<hbn_pir_cfg::HBN_PIR_CFG_SPEC>;
#[doc = "HBN_PIR_CFG."]
pub mod hbn_pir_cfg;
#[doc = "HBN_PIR_VTH (rw) register accessor: HBN_PIR_VTH.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hbn_pir_vth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hbn_pir_vth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbn_pir_vth`]
module"]
pub type HBN_PIR_VTH = crate::Reg<hbn_pir_vth::HBN_PIR_VTH_SPEC>;
#[doc = "HBN_PIR_VTH."]
pub mod hbn_pir_vth;
#[doc = "HBN_PIR_INTERVAL (rw) register accessor: HBN_PIR_INTERVAL.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hbn_pir_interval::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hbn_pir_interval::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbn_pir_interval`]
module"]
pub type HBN_PIR_INTERVAL = crate::Reg<hbn_pir_interval::HBN_PIR_INTERVAL_SPEC>;
#[doc = "HBN_PIR_INTERVAL."]
pub mod hbn_pir_interval;
#[doc = "HBN_BOR_CFG (rw) register accessor: HBN_BOR_CFG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hbn_bor_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hbn_bor_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbn_bor_cfg`]
module"]
pub type HBN_BOR_CFG = crate::Reg<hbn_bor_cfg::HBN_BOR_CFG_SPEC>;
#[doc = "HBN_BOR_CFG."]
pub mod hbn_bor_cfg;
#[doc = "HBN_PAD_CTRL_0 (rw) register accessor: HBN_PAD_CTRL_0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hbn_pad_ctrl_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hbn_pad_ctrl_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbn_pad_ctrl_0`]
module"]
pub type HBN_PAD_CTRL_0 = crate::Reg<hbn_pad_ctrl_0::HBN_PAD_CTRL_0_SPEC>;
#[doc = "HBN_PAD_CTRL_0."]
pub mod hbn_pad_ctrl_0;
#[doc = "HBN_PAD_CTRL_1 (rw) register accessor: HBN_PAD_CTRL_1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hbn_pad_ctrl_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hbn_pad_ctrl_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbn_pad_ctrl_1`]
module"]
pub type HBN_PAD_CTRL_1 = crate::Reg<hbn_pad_ctrl_1::HBN_PAD_CTRL_1_SPEC>;
#[doc = "HBN_PAD_CTRL_1."]
pub mod hbn_pad_ctrl_1;
#[doc = "vbat_ldo (rw) register accessor: vbat_ldo.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbat_ldo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbat_ldo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbat_ldo`]
module"]
pub type VBAT_LDO = crate::Reg<vbat_ldo::VBAT_LDO_SPEC>;
#[doc = "vbat_ldo."]
pub mod vbat_ldo;
#[doc = "HBN_RSV0 (rw) register accessor: HBN_RSV0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hbn_rsv0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hbn_rsv0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbn_rsv0`]
module"]
pub type HBN_RSV0 = crate::Reg<hbn_rsv0::HBN_RSV0_SPEC>;
#[doc = "HBN_RSV0."]
pub mod hbn_rsv0;
#[doc = "HBN_RSV1 (rw) register accessor: HBN_RSV1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hbn_rsv1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hbn_rsv1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbn_rsv1`]
module"]
pub type HBN_RSV1 = crate::Reg<hbn_rsv1::HBN_RSV1_SPEC>;
#[doc = "HBN_RSV1."]
pub mod hbn_rsv1;
#[doc = "HBN_RSV2 (rw) register accessor: HBN_RSV2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hbn_rsv2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hbn_rsv2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbn_rsv2`]
module"]
pub type HBN_RSV2 = crate::Reg<hbn_rsv2::HBN_RSV2_SPEC>;
#[doc = "HBN_RSV2."]
pub mod hbn_rsv2;
#[doc = "HBN_RSV3 (rw) register accessor: HBN_RSV3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hbn_rsv3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hbn_rsv3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbn_rsv3`]
module"]
pub type HBN_RSV3 = crate::Reg<hbn_rsv3::HBN_RSV3_SPEC>;
#[doc = "HBN_RSV3."]
pub mod hbn_rsv3;
