#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pds_ctl: PDS_CTL,
    pds_time1: PDS_TIME1,
    _reserved2: [u8; 0x04],
    pds_int: PDS_INT,
    pds_ctl2: PDS_CTL2,
    pds_ctl3: PDS_CTL3,
    pds_ctl4: PDS_CTL4,
    pds_stat: PDS_STAT,
    pds_ram1: PDS_RAM1,
    pds_ctl5: PDS_CTL5,
    pds_ram2: PDS_RAM2,
    _reserved10: [u8; 0x04],
    pds_gpio_i_set: PDS_GPIO_I_SET,
    pds_gpio_pd_set: PDS_GPIO_PD_SET,
    _reserved12: [u8; 0x08],
    pds_gpio_int: PDS_GPIO_INT,
    pds_gpio_stat: PDS_GPIO_STAT,
    pds_ram3: PDS_RAM3,
    pds_ram4: PDS_RAM4,
    _reserved16: [u8; 0xc4],
    cpu_core_cfg1: CPU_CORE_CFG1,
    _reserved17: [u8; 0x30],
    cpu_core_cfg14: CPU_CORE_CFG14,
    _reserved18: [u8; 0x01b4],
    rc32m_ctrl0: RC32M_CTRL0,
    rc32m_ctrl1: RC32M_CTRL1,
    rc32m_ctrl2: RC32M_CTRL2,
    _reserved21: [u8; 0xf4],
    pu_rst_clkpll: PU_RST_CLKPLL,
    _reserved22: [u8; 0xfc],
    usb_ctl: USB_CTL,
    usb_phy_ctrl: USB_PHY_CTRL,
}
impl RegisterBlock {
    #[doc = "0x00 - PDS_CTL."]
    #[inline(always)]
    pub const fn pds_ctl(&self) -> &PDS_CTL {
        &self.pds_ctl
    }
    #[doc = "0x04 - PDS_TIME1."]
    #[inline(always)]
    pub const fn pds_time1(&self) -> &PDS_TIME1 {
        &self.pds_time1
    }
    #[doc = "0x0c - PDS_INT."]
    #[inline(always)]
    pub const fn pds_int(&self) -> &PDS_INT {
        &self.pds_int
    }
    #[doc = "0x10 - PDS_CTL2."]
    #[inline(always)]
    pub const fn pds_ctl2(&self) -> &PDS_CTL2 {
        &self.pds_ctl2
    }
    #[doc = "0x14 - PDS_CTL3."]
    #[inline(always)]
    pub const fn pds_ctl3(&self) -> &PDS_CTL3 {
        &self.pds_ctl3
    }
    #[doc = "0x18 - PDS_CTL4."]
    #[inline(always)]
    pub const fn pds_ctl4(&self) -> &PDS_CTL4 {
        &self.pds_ctl4
    }
    #[doc = "0x1c - pds_stat."]
    #[inline(always)]
    pub const fn pds_stat(&self) -> &PDS_STAT {
        &self.pds_stat
    }
    #[doc = "0x20 - pds_ram1."]
    #[inline(always)]
    pub const fn pds_ram1(&self) -> &PDS_RAM1 {
        &self.pds_ram1
    }
    #[doc = "0x24 - PDS_CTL5."]
    #[inline(always)]
    pub const fn pds_ctl5(&self) -> &PDS_CTL5 {
        &self.pds_ctl5
    }
    #[doc = "0x28 - PDS_RAM2."]
    #[inline(always)]
    pub const fn pds_ram2(&self) -> &PDS_RAM2 {
        &self.pds_ram2
    }
    #[doc = "0x30 - pds_gpio_i_set."]
    #[inline(always)]
    pub const fn pds_gpio_i_set(&self) -> &PDS_GPIO_I_SET {
        &self.pds_gpio_i_set
    }
    #[doc = "0x34 - pds_gpio_pd_set."]
    #[inline(always)]
    pub const fn pds_gpio_pd_set(&self) -> &PDS_GPIO_PD_SET {
        &self.pds_gpio_pd_set
    }
    #[doc = "0x40 - pds_gpio_int."]
    #[inline(always)]
    pub const fn pds_gpio_int(&self) -> &PDS_GPIO_INT {
        &self.pds_gpio_int
    }
    #[doc = "0x44 - pds_gpio_stat."]
    #[inline(always)]
    pub const fn pds_gpio_stat(&self) -> &PDS_GPIO_STAT {
        &self.pds_gpio_stat
    }
    #[doc = "0x48 - PDS_RAM3."]
    #[inline(always)]
    pub const fn pds_ram3(&self) -> &PDS_RAM3 {
        &self.pds_ram3
    }
    #[doc = "0x4c - PDS_RAM4."]
    #[inline(always)]
    pub const fn pds_ram4(&self) -> &PDS_RAM4 {
        &self.pds_ram4
    }
    #[doc = "0x114 - cpu_core_cfg1."]
    #[inline(always)]
    pub const fn cpu_core_cfg1(&self) -> &CPU_CORE_CFG1 {
        &self.cpu_core_cfg1
    }
    #[doc = "0x148 - cpu_core_cfg14."]
    #[inline(always)]
    pub const fn cpu_core_cfg14(&self) -> &CPU_CORE_CFG14 {
        &self.cpu_core_cfg14
    }
    #[doc = "0x300 - rc32m_ctrl0."]
    #[inline(always)]
    pub const fn rc32m_ctrl0(&self) -> &RC32M_CTRL0 {
        &self.rc32m_ctrl0
    }
    #[doc = "0x304 - rc32m_ctrl1."]
    #[inline(always)]
    pub const fn rc32m_ctrl1(&self) -> &RC32M_CTRL1 {
        &self.rc32m_ctrl1
    }
    #[doc = "0x308 - rc32m_ctrl2."]
    #[inline(always)]
    pub const fn rc32m_ctrl2(&self) -> &RC32M_CTRL2 {
        &self.rc32m_ctrl2
    }
    #[doc = "0x400 - pu_rst_clkpll."]
    #[inline(always)]
    pub const fn pu_rst_clkpll(&self) -> &PU_RST_CLKPLL {
        &self.pu_rst_clkpll
    }
    #[doc = "0x500 - usb_ctl."]
    #[inline(always)]
    pub const fn usb_ctl(&self) -> &USB_CTL {
        &self.usb_ctl
    }
    #[doc = "0x504 - usb_phy_ctrl."]
    #[inline(always)]
    pub const fn usb_phy_ctrl(&self) -> &USB_PHY_CTRL {
        &self.usb_phy_ctrl
    }
}
#[doc = "PDS_CTL (rw) register accessor: PDS_CTL.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pds_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pds_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_ctl`]
module"]
pub type PDS_CTL = crate::Reg<pds_ctl::PDS_CTL_SPEC>;
#[doc = "PDS_CTL."]
pub mod pds_ctl;
#[doc = "PDS_TIME1 (rw) register accessor: PDS_TIME1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pds_time1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pds_time1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_time1`]
module"]
pub type PDS_TIME1 = crate::Reg<pds_time1::PDS_TIME1_SPEC>;
#[doc = "PDS_TIME1."]
pub mod pds_time1;
#[doc = "PDS_INT (rw) register accessor: PDS_INT.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pds_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pds_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_int`]
module"]
pub type PDS_INT = crate::Reg<pds_int::PDS_INT_SPEC>;
#[doc = "PDS_INT."]
pub mod pds_int;
#[doc = "PDS_CTL2 (rw) register accessor: PDS_CTL2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pds_ctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pds_ctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_ctl2`]
module"]
pub type PDS_CTL2 = crate::Reg<pds_ctl2::PDS_CTL2_SPEC>;
#[doc = "PDS_CTL2."]
pub mod pds_ctl2;
#[doc = "PDS_CTL3 (rw) register accessor: PDS_CTL3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pds_ctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pds_ctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_ctl3`]
module"]
pub type PDS_CTL3 = crate::Reg<pds_ctl3::PDS_CTL3_SPEC>;
#[doc = "PDS_CTL3."]
pub mod pds_ctl3;
#[doc = "PDS_CTL4 (rw) register accessor: PDS_CTL4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pds_ctl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pds_ctl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_ctl4`]
module"]
pub type PDS_CTL4 = crate::Reg<pds_ctl4::PDS_CTL4_SPEC>;
#[doc = "PDS_CTL4."]
pub mod pds_ctl4;
#[doc = "pds_stat (rw) register accessor: pds_stat.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pds_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pds_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_stat`]
module"]
pub type PDS_STAT = crate::Reg<pds_stat::PDS_STAT_SPEC>;
#[doc = "pds_stat."]
pub mod pds_stat;
#[doc = "pds_ram1 (rw) register accessor: pds_ram1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pds_ram1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pds_ram1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_ram1`]
module"]
pub type PDS_RAM1 = crate::Reg<pds_ram1::PDS_RAM1_SPEC>;
#[doc = "pds_ram1."]
pub mod pds_ram1;
#[doc = "PDS_CTL5 (rw) register accessor: PDS_CTL5.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pds_ctl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pds_ctl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_ctl5`]
module"]
pub type PDS_CTL5 = crate::Reg<pds_ctl5::PDS_CTL5_SPEC>;
#[doc = "PDS_CTL5."]
pub mod pds_ctl5;
#[doc = "PDS_RAM2 (rw) register accessor: PDS_RAM2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pds_ram2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pds_ram2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_ram2`]
module"]
pub type PDS_RAM2 = crate::Reg<pds_ram2::PDS_RAM2_SPEC>;
#[doc = "PDS_RAM2."]
pub mod pds_ram2;
#[doc = "pds_gpio_i_set (rw) register accessor: pds_gpio_i_set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pds_gpio_i_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pds_gpio_i_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_gpio_i_set`]
module"]
pub type PDS_GPIO_I_SET = crate::Reg<pds_gpio_i_set::PDS_GPIO_I_SET_SPEC>;
#[doc = "pds_gpio_i_set."]
pub mod pds_gpio_i_set;
#[doc = "pds_gpio_pd_set (rw) register accessor: pds_gpio_pd_set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pds_gpio_pd_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pds_gpio_pd_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_gpio_pd_set`]
module"]
pub type PDS_GPIO_PD_SET = crate::Reg<pds_gpio_pd_set::PDS_GPIO_PD_SET_SPEC>;
#[doc = "pds_gpio_pd_set."]
pub mod pds_gpio_pd_set;
#[doc = "pds_gpio_int (rw) register accessor: pds_gpio_int.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pds_gpio_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pds_gpio_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_gpio_int`]
module"]
pub type PDS_GPIO_INT = crate::Reg<pds_gpio_int::PDS_GPIO_INT_SPEC>;
#[doc = "pds_gpio_int."]
pub mod pds_gpio_int;
#[doc = "pds_gpio_stat (rw) register accessor: pds_gpio_stat.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pds_gpio_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pds_gpio_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_gpio_stat`]
module"]
pub type PDS_GPIO_STAT = crate::Reg<pds_gpio_stat::PDS_GPIO_STAT_SPEC>;
#[doc = "pds_gpio_stat."]
pub mod pds_gpio_stat;
#[doc = "PDS_RAM3 (rw) register accessor: PDS_RAM3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pds_ram3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pds_ram3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_ram3`]
module"]
pub type PDS_RAM3 = crate::Reg<pds_ram3::PDS_RAM3_SPEC>;
#[doc = "PDS_RAM3."]
pub mod pds_ram3;
#[doc = "PDS_RAM4 (rw) register accessor: PDS_RAM4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pds_ram4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pds_ram4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pds_ram4`]
module"]
pub type PDS_RAM4 = crate::Reg<pds_ram4::PDS_RAM4_SPEC>;
#[doc = "PDS_RAM4."]
pub mod pds_ram4;
#[doc = "cpu_core_cfg1 (rw) register accessor: cpu_core_cfg1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_core_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_core_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_core_cfg1`]
module"]
pub type CPU_CORE_CFG1 = crate::Reg<cpu_core_cfg1::CPU_CORE_CFG1_SPEC>;
#[doc = "cpu_core_cfg1."]
pub mod cpu_core_cfg1;
#[doc = "cpu_core_cfg14 (rw) register accessor: cpu_core_cfg14.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_core_cfg14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_core_cfg14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_core_cfg14`]
module"]
pub type CPU_CORE_CFG14 = crate::Reg<cpu_core_cfg14::CPU_CORE_CFG14_SPEC>;
#[doc = "cpu_core_cfg14."]
pub mod cpu_core_cfg14;
#[doc = "rc32m_ctrl0 (rw) register accessor: rc32m_ctrl0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc32m_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rc32m_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rc32m_ctrl0`]
module"]
pub type RC32M_CTRL0 = crate::Reg<rc32m_ctrl0::RC32M_CTRL0_SPEC>;
#[doc = "rc32m_ctrl0."]
pub mod rc32m_ctrl0;
#[doc = "rc32m_ctrl1 (rw) register accessor: rc32m_ctrl1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc32m_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rc32m_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rc32m_ctrl1`]
module"]
pub type RC32M_CTRL1 = crate::Reg<rc32m_ctrl1::RC32M_CTRL1_SPEC>;
#[doc = "rc32m_ctrl1."]
pub mod rc32m_ctrl1;
#[doc = "rc32m_ctrl2 (rw) register accessor: rc32m_ctrl2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc32m_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rc32m_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rc32m_ctrl2`]
module"]
pub type RC32M_CTRL2 = crate::Reg<rc32m_ctrl2::RC32M_CTRL2_SPEC>;
#[doc = "rc32m_ctrl2."]
pub mod rc32m_ctrl2;
#[doc = "pu_rst_clkpll (rw) register accessor: pu_rst_clkpll.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pu_rst_clkpll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pu_rst_clkpll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pu_rst_clkpll`]
module"]
pub type PU_RST_CLKPLL = crate::Reg<pu_rst_clkpll::PU_RST_CLKPLL_SPEC>;
#[doc = "pu_rst_clkpll."]
pub mod pu_rst_clkpll;
#[doc = "usb_ctl (rw) register accessor: usb_ctl.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_ctl`]
module"]
pub type USB_CTL = crate::Reg<usb_ctl::USB_CTL_SPEC>;
#[doc = "usb_ctl."]
pub mod usb_ctl;
#[doc = "usb_phy_ctrl (rw) register accessor: usb_phy_ctrl.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_phy_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_phy_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_phy_ctrl`]
module"]
pub type USB_PHY_CTRL = crate::Reg<usb_phy_ctrl::USB_PHY_CTRL_SPEC>;
#[doc = "usb_phy_ctrl."]
pub mod usb_phy_ctrl;
