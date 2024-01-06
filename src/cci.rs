#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    cci_cfg: CCI_CFG,
    cci_addr: CCI_ADDR,
    cci_wdata: CCI_WDATA,
    cci_rdata: CCI_RDATA,
    cci_ctl: CCI_CTL,
    _reserved5: [u8; 0x073c],
    audio_pll_cfg0: AUDIO_PLL_CFG0,
    audio_pll_cfg1: AUDIO_PLL_CFG1,
    audio_pll_cfg2: AUDIO_PLL_CFG2,
    audio_pll_cfg3: AUDIO_PLL_CFG3,
    audio_pll_cfg4: AUDIO_PLL_CFG4,
    audio_pll_cfg5: AUDIO_PLL_CFG5,
    audio_pll_cfg6: AUDIO_PLL_CFG6,
    audio_pll_cfg7: AUDIO_PLL_CFG7,
    audio_pll_cfg8: AUDIO_PLL_CFG8,
    audio_pll_cfg9: AUDIO_PLL_CFG9,
    audio_pll_cfg10: AUDIO_PLL_CFG10,
    audio_pll_cfg11: AUDIO_PLL_CFG11,
}
impl RegisterBlock {
    #[doc = "0x00 - cci_cfg."]
    #[inline(always)]
    pub const fn cci_cfg(&self) -> &CCI_CFG {
        &self.cci_cfg
    }
    #[doc = "0x04 - cci_addr."]
    #[inline(always)]
    pub const fn cci_addr(&self) -> &CCI_ADDR {
        &self.cci_addr
    }
    #[doc = "0x08 - cci_wdata."]
    #[inline(always)]
    pub const fn cci_wdata(&self) -> &CCI_WDATA {
        &self.cci_wdata
    }
    #[doc = "0x0c - cci_rdata."]
    #[inline(always)]
    pub const fn cci_rdata(&self) -> &CCI_RDATA {
        &self.cci_rdata
    }
    #[doc = "0x10 - cci_ctl."]
    #[inline(always)]
    pub const fn cci_ctl(&self) -> &CCI_CTL {
        &self.cci_ctl
    }
    #[doc = "0x750 - audio_pll_cfg0."]
    #[inline(always)]
    pub const fn audio_pll_cfg0(&self) -> &AUDIO_PLL_CFG0 {
        &self.audio_pll_cfg0
    }
    #[doc = "0x754 - audio_pll_cfg1."]
    #[inline(always)]
    pub const fn audio_pll_cfg1(&self) -> &AUDIO_PLL_CFG1 {
        &self.audio_pll_cfg1
    }
    #[doc = "0x758 - audio_pll_cfg2."]
    #[inline(always)]
    pub const fn audio_pll_cfg2(&self) -> &AUDIO_PLL_CFG2 {
        &self.audio_pll_cfg2
    }
    #[doc = "0x75c - audio_pll_cfg3."]
    #[inline(always)]
    pub const fn audio_pll_cfg3(&self) -> &AUDIO_PLL_CFG3 {
        &self.audio_pll_cfg3
    }
    #[doc = "0x760 - audio_pll_cfg4."]
    #[inline(always)]
    pub const fn audio_pll_cfg4(&self) -> &AUDIO_PLL_CFG4 {
        &self.audio_pll_cfg4
    }
    #[doc = "0x764 - audio_pll_cfg5."]
    #[inline(always)]
    pub const fn audio_pll_cfg5(&self) -> &AUDIO_PLL_CFG5 {
        &self.audio_pll_cfg5
    }
    #[doc = "0x768 - audio_pll_cfg6."]
    #[inline(always)]
    pub const fn audio_pll_cfg6(&self) -> &AUDIO_PLL_CFG6 {
        &self.audio_pll_cfg6
    }
    #[doc = "0x76c - audio_pll_cfg7."]
    #[inline(always)]
    pub const fn audio_pll_cfg7(&self) -> &AUDIO_PLL_CFG7 {
        &self.audio_pll_cfg7
    }
    #[doc = "0x770 - audio_pll_cfg8."]
    #[inline(always)]
    pub const fn audio_pll_cfg8(&self) -> &AUDIO_PLL_CFG8 {
        &self.audio_pll_cfg8
    }
    #[doc = "0x774 - audio_pll_cfg9."]
    #[inline(always)]
    pub const fn audio_pll_cfg9(&self) -> &AUDIO_PLL_CFG9 {
        &self.audio_pll_cfg9
    }
    #[doc = "0x778 - audio_pll_cfg10."]
    #[inline(always)]
    pub const fn audio_pll_cfg10(&self) -> &AUDIO_PLL_CFG10 {
        &self.audio_pll_cfg10
    }
    #[doc = "0x77c - audio_pll_cfg11."]
    #[inline(always)]
    pub const fn audio_pll_cfg11(&self) -> &AUDIO_PLL_CFG11 {
        &self.audio_pll_cfg11
    }
}
#[doc = "cci_cfg (rw) register accessor: cci_cfg.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci_cfg`]
module"]
pub type CCI_CFG = crate::Reg<cci_cfg::CCI_CFG_SPEC>;
#[doc = "cci_cfg."]
pub mod cci_cfg;
#[doc = "cci_addr (rw) register accessor: cci_addr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci_addr`]
module"]
pub type CCI_ADDR = crate::Reg<cci_addr::CCI_ADDR_SPEC>;
#[doc = "cci_addr."]
pub mod cci_addr;
#[doc = "cci_wdata (rw) register accessor: cci_wdata.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci_wdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci_wdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci_wdata`]
module"]
pub type CCI_WDATA = crate::Reg<cci_wdata::CCI_WDATA_SPEC>;
#[doc = "cci_wdata."]
pub mod cci_wdata;
#[doc = "cci_rdata (rw) register accessor: cci_rdata.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci_rdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci_rdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci_rdata`]
module"]
pub type CCI_RDATA = crate::Reg<cci_rdata::CCI_RDATA_SPEC>;
#[doc = "cci_rdata."]
pub mod cci_rdata;
#[doc = "cci_ctl (rw) register accessor: cci_ctl.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cci_ctl`]
module"]
pub type CCI_CTL = crate::Reg<cci_ctl::CCI_CTL_SPEC>;
#[doc = "cci_ctl."]
pub mod cci_ctl;
#[doc = "audio_pll_cfg0 (rw) register accessor: audio_pll_cfg0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`audio_pll_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`audio_pll_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_pll_cfg0`]
module"]
pub type AUDIO_PLL_CFG0 = crate::Reg<audio_pll_cfg0::AUDIO_PLL_CFG0_SPEC>;
#[doc = "audio_pll_cfg0."]
pub mod audio_pll_cfg0;
#[doc = "audio_pll_cfg1 (rw) register accessor: audio_pll_cfg1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`audio_pll_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`audio_pll_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_pll_cfg1`]
module"]
pub type AUDIO_PLL_CFG1 = crate::Reg<audio_pll_cfg1::AUDIO_PLL_CFG1_SPEC>;
#[doc = "audio_pll_cfg1."]
pub mod audio_pll_cfg1;
#[doc = "audio_pll_cfg2 (rw) register accessor: audio_pll_cfg2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`audio_pll_cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`audio_pll_cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_pll_cfg2`]
module"]
pub type AUDIO_PLL_CFG2 = crate::Reg<audio_pll_cfg2::AUDIO_PLL_CFG2_SPEC>;
#[doc = "audio_pll_cfg2."]
pub mod audio_pll_cfg2;
#[doc = "audio_pll_cfg3 (rw) register accessor: audio_pll_cfg3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`audio_pll_cfg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`audio_pll_cfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_pll_cfg3`]
module"]
pub type AUDIO_PLL_CFG3 = crate::Reg<audio_pll_cfg3::AUDIO_PLL_CFG3_SPEC>;
#[doc = "audio_pll_cfg3."]
pub mod audio_pll_cfg3;
#[doc = "audio_pll_cfg4 (rw) register accessor: audio_pll_cfg4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`audio_pll_cfg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`audio_pll_cfg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_pll_cfg4`]
module"]
pub type AUDIO_PLL_CFG4 = crate::Reg<audio_pll_cfg4::AUDIO_PLL_CFG4_SPEC>;
#[doc = "audio_pll_cfg4."]
pub mod audio_pll_cfg4;
#[doc = "audio_pll_cfg5 (rw) register accessor: audio_pll_cfg5.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`audio_pll_cfg5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`audio_pll_cfg5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_pll_cfg5`]
module"]
pub type AUDIO_PLL_CFG5 = crate::Reg<audio_pll_cfg5::AUDIO_PLL_CFG5_SPEC>;
#[doc = "audio_pll_cfg5."]
pub mod audio_pll_cfg5;
#[doc = "audio_pll_cfg6 (rw) register accessor: audio_pll_cfg6.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`audio_pll_cfg6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`audio_pll_cfg6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_pll_cfg6`]
module"]
pub type AUDIO_PLL_CFG6 = crate::Reg<audio_pll_cfg6::AUDIO_PLL_CFG6_SPEC>;
#[doc = "audio_pll_cfg6."]
pub mod audio_pll_cfg6;
#[doc = "audio_pll_cfg7 (rw) register accessor: audio_pll_cfg7.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`audio_pll_cfg7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`audio_pll_cfg7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_pll_cfg7`]
module"]
pub type AUDIO_PLL_CFG7 = crate::Reg<audio_pll_cfg7::AUDIO_PLL_CFG7_SPEC>;
#[doc = "audio_pll_cfg7."]
pub mod audio_pll_cfg7;
#[doc = "audio_pll_cfg8 (rw) register accessor: audio_pll_cfg8.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`audio_pll_cfg8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`audio_pll_cfg8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_pll_cfg8`]
module"]
pub type AUDIO_PLL_CFG8 = crate::Reg<audio_pll_cfg8::AUDIO_PLL_CFG8_SPEC>;
#[doc = "audio_pll_cfg8."]
pub mod audio_pll_cfg8;
#[doc = "audio_pll_cfg9 (rw) register accessor: audio_pll_cfg9.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`audio_pll_cfg9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`audio_pll_cfg9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_pll_cfg9`]
module"]
pub type AUDIO_PLL_CFG9 = crate::Reg<audio_pll_cfg9::AUDIO_PLL_CFG9_SPEC>;
#[doc = "audio_pll_cfg9."]
pub mod audio_pll_cfg9;
#[doc = "audio_pll_cfg10 (rw) register accessor: audio_pll_cfg10.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`audio_pll_cfg10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`audio_pll_cfg10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_pll_cfg10`]
module"]
pub type AUDIO_PLL_CFG10 = crate::Reg<audio_pll_cfg10::AUDIO_PLL_CFG10_SPEC>;
#[doc = "audio_pll_cfg10."]
pub mod audio_pll_cfg10;
#[doc = "audio_pll_cfg11 (rw) register accessor: audio_pll_cfg11.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`audio_pll_cfg11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`audio_pll_cfg11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@audio_pll_cfg11`]
module"]
pub type AUDIO_PLL_CFG11 = crate::Reg<audio_pll_cfg11::AUDIO_PLL_CFG11_SPEC>;
#[doc = "audio_pll_cfg11."]
pub mod audio_pll_cfg11;
