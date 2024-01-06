#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    aon: AON,
    aon_common: AON_COMMON,
    aon_misc: AON_MISC,
    _reserved3: [u8; 0x04],
    bg_sys_top: BG_SYS_TOP,
    dcdc_top_0: DCDC_TOP_0,
    dcdc_top_1: DCDC_TOP_1,
    ldo11soc_and_dctest: LDO11SOC_AND_DCTEST,
    psw_irrcv: PSW_IRRCV,
    dcdc_top_2: DCDC_TOP_2,
    _reserved9: [u8; 0x04],
    psw_misc: PSW_MISC,
    _reserved10: [u8; 0x50],
    rf_top_aon: RF_TOP_AON,
    xtal_cfg: XTAL_CFG,
    tsen: TSEN,
    _reserved13: [u8; 0x74],
    acomp0_ctrl: ACOMP0_CTRL,
    acomp1_ctrl: ACOMP1_CTRL,
    acomp_ctrl: ACOMP_CTRL,
    gpadc_reg_cmd: GPADC_REG_CMD,
    gpadc_reg_config1: GPADC_REG_CONFIG1,
    gpadc_reg_config2: GPADC_REG_CONFIG2,
    _reserved19: [u8; 0x10],
    gpadc_reg_status: GPADC_REG_STATUS,
    gpadc_reg_isr: GPADC_REG_ISR,
    gpadc_reg_result: GPADC_REG_RESULT,
    gpadc_reg_raw_result: GPADC_REG_RAW_RESULT,
    gpadc_reg_define: GPADC_REG_DEFINE,
    hbncore_resv0: HBNCORE_RESV0,
    hbncore_resv1: HBNCORE_RESV1,
}
impl RegisterBlock {
    #[doc = "0x800 - aon."]
    #[inline(always)]
    pub const fn aon(&self) -> &AON {
        &self.aon
    }
    #[doc = "0x804 - aon_common."]
    #[inline(always)]
    pub const fn aon_common(&self) -> &AON_COMMON {
        &self.aon_common
    }
    #[doc = "0x808 - aon_misc."]
    #[inline(always)]
    pub const fn aon_misc(&self) -> &AON_MISC {
        &self.aon_misc
    }
    #[doc = "0x810 - bg_sys_top."]
    #[inline(always)]
    pub const fn bg_sys_top(&self) -> &BG_SYS_TOP {
        &self.bg_sys_top
    }
    #[doc = "0x814 - dcdc_top_0."]
    #[inline(always)]
    pub const fn dcdc_top_0(&self) -> &DCDC_TOP_0 {
        &self.dcdc_top_0
    }
    #[doc = "0x818 - dcdc_top_1."]
    #[inline(always)]
    pub const fn dcdc_top_1(&self) -> &DCDC_TOP_1 {
        &self.dcdc_top_1
    }
    #[doc = "0x81c - ldo11soc_and_dctest."]
    #[inline(always)]
    pub const fn ldo11soc_and_dctest(&self) -> &LDO11SOC_AND_DCTEST {
        &self.ldo11soc_and_dctest
    }
    #[doc = "0x820 - psw_irrcv."]
    #[inline(always)]
    pub const fn psw_irrcv(&self) -> &PSW_IRRCV {
        &self.psw_irrcv
    }
    #[doc = "0x824 - dcdc_top_2."]
    #[inline(always)]
    pub const fn dcdc_top_2(&self) -> &DCDC_TOP_2 {
        &self.dcdc_top_2
    }
    #[doc = "0x82c - psw_misc."]
    #[inline(always)]
    pub const fn psw_misc(&self) -> &PSW_MISC {
        &self.psw_misc
    }
    #[doc = "0x880 - rf_top_aon."]
    #[inline(always)]
    pub const fn rf_top_aon(&self) -> &RF_TOP_AON {
        &self.rf_top_aon
    }
    #[doc = "0x884 - xtal_cfg."]
    #[inline(always)]
    pub const fn xtal_cfg(&self) -> &XTAL_CFG {
        &self.xtal_cfg
    }
    #[doc = "0x888 - tsen."]
    #[inline(always)]
    pub const fn tsen(&self) -> &TSEN {
        &self.tsen
    }
    #[doc = "0x900 - acomp0_ctrl."]
    #[inline(always)]
    pub const fn acomp0_ctrl(&self) -> &ACOMP0_CTRL {
        &self.acomp0_ctrl
    }
    #[doc = "0x904 - acomp1_ctrl."]
    #[inline(always)]
    pub const fn acomp1_ctrl(&self) -> &ACOMP1_CTRL {
        &self.acomp1_ctrl
    }
    #[doc = "0x908 - acomp_ctrl."]
    #[inline(always)]
    pub const fn acomp_ctrl(&self) -> &ACOMP_CTRL {
        &self.acomp_ctrl
    }
    #[doc = "0x90c - gpadc_reg_cmd."]
    #[inline(always)]
    pub const fn gpadc_reg_cmd(&self) -> &GPADC_REG_CMD {
        &self.gpadc_reg_cmd
    }
    #[doc = "0x910 - gpadc_reg_config1."]
    #[inline(always)]
    pub const fn gpadc_reg_config1(&self) -> &GPADC_REG_CONFIG1 {
        &self.gpadc_reg_config1
    }
    #[doc = "0x914 - gpadc_reg_config2."]
    #[inline(always)]
    pub const fn gpadc_reg_config2(&self) -> &GPADC_REG_CONFIG2 {
        &self.gpadc_reg_config2
    }
    #[doc = "0x928 - gpadc_reg_status."]
    #[inline(always)]
    pub const fn gpadc_reg_status(&self) -> &GPADC_REG_STATUS {
        &self.gpadc_reg_status
    }
    #[doc = "0x92c - gpadc_reg_isr."]
    #[inline(always)]
    pub const fn gpadc_reg_isr(&self) -> &GPADC_REG_ISR {
        &self.gpadc_reg_isr
    }
    #[doc = "0x930 - gpadc_reg_result."]
    #[inline(always)]
    pub const fn gpadc_reg_result(&self) -> &GPADC_REG_RESULT {
        &self.gpadc_reg_result
    }
    #[doc = "0x934 - gpadc_reg_raw_result."]
    #[inline(always)]
    pub const fn gpadc_reg_raw_result(&self) -> &GPADC_REG_RAW_RESULT {
        &self.gpadc_reg_raw_result
    }
    #[doc = "0x938 - gpadc_reg_define."]
    #[inline(always)]
    pub const fn gpadc_reg_define(&self) -> &GPADC_REG_DEFINE {
        &self.gpadc_reg_define
    }
    #[doc = "0x93c - hbncore_resv0."]
    #[inline(always)]
    pub const fn hbncore_resv0(&self) -> &HBNCORE_RESV0 {
        &self.hbncore_resv0
    }
    #[doc = "0x940 - hbncore_resv1."]
    #[inline(always)]
    pub const fn hbncore_resv1(&self) -> &HBNCORE_RESV1 {
        &self.hbncore_resv1
    }
}
#[doc = "aon (rw) register accessor: aon.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon`]
module"]
pub type AON = crate::Reg<aon::AON_SPEC>;
#[doc = "aon."]
pub mod aon;
#[doc = "aon_common (rw) register accessor: aon_common.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_common::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_common::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_common`]
module"]
pub type AON_COMMON = crate::Reg<aon_common::AON_COMMON_SPEC>;
#[doc = "aon_common."]
pub mod aon_common;
#[doc = "aon_misc (rw) register accessor: aon_misc.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_misc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_misc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_misc`]
module"]
pub type AON_MISC = crate::Reg<aon_misc::AON_MISC_SPEC>;
#[doc = "aon_misc."]
pub mod aon_misc;
#[doc = "bg_sys_top (rw) register accessor: bg_sys_top.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bg_sys_top::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bg_sys_top::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bg_sys_top`]
module"]
pub type BG_SYS_TOP = crate::Reg<bg_sys_top::BG_SYS_TOP_SPEC>;
#[doc = "bg_sys_top."]
pub mod bg_sys_top;
#[doc = "dcdc_top_0 (rw) register accessor: dcdc_top_0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdc_top_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdc_top_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc_top_0`]
module"]
pub type DCDC_TOP_0 = crate::Reg<dcdc_top_0::DCDC_TOP_0_SPEC>;
#[doc = "dcdc_top_0."]
pub mod dcdc_top_0;
#[doc = "dcdc_top_1 (rw) register accessor: dcdc_top_1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdc_top_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdc_top_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc_top_1`]
module"]
pub type DCDC_TOP_1 = crate::Reg<dcdc_top_1::DCDC_TOP_1_SPEC>;
#[doc = "dcdc_top_1."]
pub mod dcdc_top_1;
#[doc = "ldo11soc_and_dctest (rw) register accessor: ldo11soc_and_dctest.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ldo11soc_and_dctest::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ldo11soc_and_dctest::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ldo11soc_and_dctest`]
module"]
pub type LDO11SOC_AND_DCTEST = crate::Reg<ldo11soc_and_dctest::LDO11SOC_AND_DCTEST_SPEC>;
#[doc = "ldo11soc_and_dctest."]
pub mod ldo11soc_and_dctest;
#[doc = "psw_irrcv (rw) register accessor: psw_irrcv.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psw_irrcv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psw_irrcv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psw_irrcv`]
module"]
pub type PSW_IRRCV = crate::Reg<psw_irrcv::PSW_IRRCV_SPEC>;
#[doc = "psw_irrcv."]
pub mod psw_irrcv;
#[doc = "dcdc_top_2 (rw) register accessor: dcdc_top_2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdc_top_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdc_top_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc_top_2`]
module"]
pub type DCDC_TOP_2 = crate::Reg<dcdc_top_2::DCDC_TOP_2_SPEC>;
#[doc = "dcdc_top_2."]
pub mod dcdc_top_2;
#[doc = "psw_misc (rw) register accessor: psw_misc.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psw_misc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psw_misc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psw_misc`]
module"]
pub type PSW_MISC = crate::Reg<psw_misc::PSW_MISC_SPEC>;
#[doc = "psw_misc."]
pub mod psw_misc;
#[doc = "rf_top_aon (rw) register accessor: rf_top_aon.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rf_top_aon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rf_top_aon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_top_aon`]
module"]
pub type RF_TOP_AON = crate::Reg<rf_top_aon::RF_TOP_AON_SPEC>;
#[doc = "rf_top_aon."]
pub mod rf_top_aon;
#[doc = "xtal_cfg (rw) register accessor: xtal_cfg.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xtal_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtal_cfg`]
module"]
pub type XTAL_CFG = crate::Reg<xtal_cfg::XTAL_CFG_SPEC>;
#[doc = "xtal_cfg."]
pub mod xtal_cfg;
#[doc = "tsen (rw) register accessor: tsen.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsen`]
module"]
pub type TSEN = crate::Reg<tsen::TSEN_SPEC>;
#[doc = "tsen."]
pub mod tsen;
#[doc = "acomp0_ctrl (rw) register accessor: acomp0_ctrl.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acomp0_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acomp0_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acomp0_ctrl`]
module"]
pub type ACOMP0_CTRL = crate::Reg<acomp0_ctrl::ACOMP0_CTRL_SPEC>;
#[doc = "acomp0_ctrl."]
pub mod acomp0_ctrl;
#[doc = "acomp1_ctrl (rw) register accessor: acomp1_ctrl.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acomp1_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acomp1_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acomp1_ctrl`]
module"]
pub type ACOMP1_CTRL = crate::Reg<acomp1_ctrl::ACOMP1_CTRL_SPEC>;
#[doc = "acomp1_ctrl."]
pub mod acomp1_ctrl;
#[doc = "acomp_ctrl (rw) register accessor: acomp_ctrl.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acomp_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acomp_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acomp_ctrl`]
module"]
pub type ACOMP_CTRL = crate::Reg<acomp_ctrl::ACOMP_CTRL_SPEC>;
#[doc = "acomp_ctrl."]
pub mod acomp_ctrl;
#[doc = "gpadc_reg_cmd (rw) register accessor: gpadc_reg_cmd.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpadc_reg_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpadc_reg_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_cmd`]
module"]
pub type GPADC_REG_CMD = crate::Reg<gpadc_reg_cmd::GPADC_REG_CMD_SPEC>;
#[doc = "gpadc_reg_cmd."]
pub mod gpadc_reg_cmd;
#[doc = "gpadc_reg_config1 (rw) register accessor: gpadc_reg_config1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpadc_reg_config1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpadc_reg_config1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_config1`]
module"]
pub type GPADC_REG_CONFIG1 = crate::Reg<gpadc_reg_config1::GPADC_REG_CONFIG1_SPEC>;
#[doc = "gpadc_reg_config1."]
pub mod gpadc_reg_config1;
#[doc = "gpadc_reg_config2 (rw) register accessor: gpadc_reg_config2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpadc_reg_config2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpadc_reg_config2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_config2`]
module"]
pub type GPADC_REG_CONFIG2 = crate::Reg<gpadc_reg_config2::GPADC_REG_CONFIG2_SPEC>;
#[doc = "gpadc_reg_config2."]
pub mod gpadc_reg_config2;
#[doc = "gpadc_reg_status (rw) register accessor: gpadc_reg_status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpadc_reg_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpadc_reg_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_status`]
module"]
pub type GPADC_REG_STATUS = crate::Reg<gpadc_reg_status::GPADC_REG_STATUS_SPEC>;
#[doc = "gpadc_reg_status."]
pub mod gpadc_reg_status;
#[doc = "gpadc_reg_isr (rw) register accessor: gpadc_reg_isr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpadc_reg_isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpadc_reg_isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_isr`]
module"]
pub type GPADC_REG_ISR = crate::Reg<gpadc_reg_isr::GPADC_REG_ISR_SPEC>;
#[doc = "gpadc_reg_isr."]
pub mod gpadc_reg_isr;
#[doc = "gpadc_reg_result (rw) register accessor: gpadc_reg_result.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpadc_reg_result::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpadc_reg_result::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_result`]
module"]
pub type GPADC_REG_RESULT = crate::Reg<gpadc_reg_result::GPADC_REG_RESULT_SPEC>;
#[doc = "gpadc_reg_result."]
pub mod gpadc_reg_result;
#[doc = "gpadc_reg_raw_result (rw) register accessor: gpadc_reg_raw_result.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpadc_reg_raw_result::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpadc_reg_raw_result::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_raw_result`]
module"]
pub type GPADC_REG_RAW_RESULT = crate::Reg<gpadc_reg_raw_result::GPADC_REG_RAW_RESULT_SPEC>;
#[doc = "gpadc_reg_raw_result."]
pub mod gpadc_reg_raw_result;
#[doc = "gpadc_reg_define (rw) register accessor: gpadc_reg_define.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpadc_reg_define::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpadc_reg_define::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_reg_define`]
module"]
pub type GPADC_REG_DEFINE = crate::Reg<gpadc_reg_define::GPADC_REG_DEFINE_SPEC>;
#[doc = "gpadc_reg_define."]
pub mod gpadc_reg_define;
#[doc = "hbncore_resv0 (rw) register accessor: hbncore_resv0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hbncore_resv0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hbncore_resv0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbncore_resv0`]
module"]
pub type HBNCORE_RESV0 = crate::Reg<hbncore_resv0::HBNCORE_RESV0_SPEC>;
#[doc = "hbncore_resv0."]
pub mod hbncore_resv0;
#[doc = "hbncore_resv1 (rw) register accessor: hbncore_resv1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hbncore_resv1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hbncore_resv1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hbncore_resv1`]
module"]
pub type HBNCORE_RESV1 = crate::Reg<hbncore_resv1::HBNCORE_RESV1_SPEC>;
#[doc = "hbncore_resv1."]
pub mod hbncore_resv1;
