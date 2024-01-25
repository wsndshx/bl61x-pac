#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    mcu_bus_cfg0: MCU_BUS_CFG0,
    mcu_bus_cfg1: MCU_BUS_CFG1,
    _reserved2: [u8; 0x0c],
    mcu_e907_rtc: MCU_E907_RTC,
    _reserved3: [u8; 0xe8],
    mcu_cfg1: MCU_CFG1,
    _reserved4: [u8; 0x0c],
    mcu1_log1: MCU1_LOG1,
    mcu1_log2: MCU1_LOG2,
    mcu1_log3: MCU1_LOG3,
    mcu1_log4: MCU1_LOG4,
    mcu1_log5: MCU1_LOG5,
    _reserved9: [u8; 0xdc],
    cpu_mbist: CPU_MBIST,
    _reserved10: [u8; 0x04],
    irom1_misr_dataout_0: IROM1_MISR_DATAOUT_0,
    irom1_misr_dataout_1: IROM1_MISR_DATAOUT_1,
}
impl RegisterBlock {
    #[doc = "0x00 - MCU Bus Configuration 0 Register. Controls MCU bus configuration settings."]
    #[inline(always)]
    pub const fn mcu_bus_cfg0(&self) -> &MCU_BUS_CFG0 {
        &self.mcu_bus_cfg0
    }
    #[doc = "0x04 - MCU Bus Configuration 1 Register. Controls MCU bus configuration settings."]
    #[inline(always)]
    pub const fn mcu_bus_cfg1(&self) -> &MCU_BUS_CFG1 {
        &self.mcu_bus_cfg1
    }
    #[doc = "0x14 - MCU E907 RTC Register. Controls the E907 RTC settings."]
    #[inline(always)]
    pub const fn mcu_e907_rtc(&self) -> &MCU_E907_RTC {
        &self.mcu_e907_rtc
    }
    #[doc = "0x100 - MCU Configuration 1 Register. Controls miscellaneous MCU settings."]
    #[inline(always)]
    pub const fn mcu_cfg1(&self) -> &MCU_CFG1 {
        &self.mcu_cfg1
    }
    #[doc = "0x110 - MCU1 Log 1 Register. Stores information about the last MCU1 exception."]
    #[inline(always)]
    pub const fn mcu1_log1(&self) -> &MCU1_LOG1 {
        &self.mcu1_log1
    }
    #[doc = "0x114 - MCU1 Log 2 Register. Stores information about the last MCU1 interrupt."]
    #[inline(always)]
    pub const fn mcu1_log2(&self) -> &MCU1_LOG2 {
        &self.mcu1_log2
    }
    #[doc = "0x118 - MCU1 Log 3 Register. Stores information about the last MCU1 machine status."]
    #[inline(always)]
    pub const fn mcu1_log3(&self) -> &MCU1_LOG3 {
        &self.mcu1_log3
    }
    #[doc = "0x11c - MCU1 Log 4 Register. Stores information about the last MCU1 program counter."]
    #[inline(always)]
    pub const fn mcu1_log4(&self) -> &MCU1_LOG4 {
        &self.mcu1_log4
    }
    #[doc = "0x120 - MCU1 Log 5 Register. Stores information about the last MCU1 lockup and halt status."]
    #[inline(always)]
    pub const fn mcu1_log5(&self) -> &MCU1_LOG5 {
        &self.mcu1_log5
    }
    #[doc = "0x200 - CPU MBIST Register. Controls the CPU memory built-in self-test (MBIST)."]
    #[inline(always)]
    pub const fn cpu_mbist(&self) -> &CPU_MBIST {
        &self.cpu_mbist
    }
    #[doc = "0x208 - IROM1 MISR Data Out 0 Register. Stores the MISR data output for IROM1."]
    #[inline(always)]
    pub const fn irom1_misr_dataout_0(&self) -> &IROM1_MISR_DATAOUT_0 {
        &self.irom1_misr_dataout_0
    }
    #[doc = "0x20c - IROM1 MISR Data Out 1 Register. Stores the MISR data output for IROM1."]
    #[inline(always)]
    pub const fn irom1_misr_dataout_1(&self) -> &IROM1_MISR_DATAOUT_1 {
        &self.irom1_misr_dataout_1
    }
}
#[doc = "mcu_bus_cfg0 (rw) register accessor: MCU Bus Configuration 0 Register. Controls MCU bus configuration settings.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcu_bus_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcu_bus_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcu_bus_cfg0`]
module"]
pub type MCU_BUS_CFG0 = crate::Reg<mcu_bus_cfg0::MCU_BUS_CFG0_SPEC>;
#[doc = "MCU Bus Configuration 0 Register. Controls MCU bus configuration settings."]
pub mod mcu_bus_cfg0;
#[doc = "mcu_bus_cfg1 (rw) register accessor: MCU Bus Configuration 1 Register. Controls MCU bus configuration settings.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcu_bus_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcu_bus_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcu_bus_cfg1`]
module"]
pub type MCU_BUS_CFG1 = crate::Reg<mcu_bus_cfg1::MCU_BUS_CFG1_SPEC>;
#[doc = "MCU Bus Configuration 1 Register. Controls MCU bus configuration settings."]
pub mod mcu_bus_cfg1;
#[doc = "mcu_e907_rtc (rw) register accessor: MCU E907 RTC Register. Controls the E907 RTC settings.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcu_e907_rtc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcu_e907_rtc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcu_e907_rtc`]
module"]
pub type MCU_E907_RTC = crate::Reg<mcu_e907_rtc::MCU_E907_RTC_SPEC>;
#[doc = "MCU E907 RTC Register. Controls the E907 RTC settings."]
pub mod mcu_e907_rtc;
#[doc = "mcu_cfg1 (rw) register accessor: MCU Configuration 1 Register. Controls miscellaneous MCU settings.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcu_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcu_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcu_cfg1`]
module"]
pub type MCU_CFG1 = crate::Reg<mcu_cfg1::MCU_CFG1_SPEC>;
#[doc = "MCU Configuration 1 Register. Controls miscellaneous MCU settings."]
pub mod mcu_cfg1;
#[doc = "mcu1_log1 (rw) register accessor: MCU1 Log 1 Register. Stores information about the last MCU1 exception.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcu1_log1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcu1_log1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcu1_log1`]
module"]
pub type MCU1_LOG1 = crate::Reg<mcu1_log1::MCU1_LOG1_SPEC>;
#[doc = "MCU1 Log 1 Register. Stores information about the last MCU1 exception."]
pub mod mcu1_log1;
#[doc = "mcu1_log2 (rw) register accessor: MCU1 Log 2 Register. Stores information about the last MCU1 interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcu1_log2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcu1_log2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcu1_log2`]
module"]
pub type MCU1_LOG2 = crate::Reg<mcu1_log2::MCU1_LOG2_SPEC>;
#[doc = "MCU1 Log 2 Register. Stores information about the last MCU1 interrupt."]
pub mod mcu1_log2;
#[doc = "mcu1_log3 (rw) register accessor: MCU1 Log 3 Register. Stores information about the last MCU1 machine status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcu1_log3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcu1_log3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcu1_log3`]
module"]
pub type MCU1_LOG3 = crate::Reg<mcu1_log3::MCU1_LOG3_SPEC>;
#[doc = "MCU1 Log 3 Register. Stores information about the last MCU1 machine status."]
pub mod mcu1_log3;
#[doc = "mcu1_log4 (rw) register accessor: MCU1 Log 4 Register. Stores information about the last MCU1 program counter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcu1_log4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcu1_log4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcu1_log4`]
module"]
pub type MCU1_LOG4 = crate::Reg<mcu1_log4::MCU1_LOG4_SPEC>;
#[doc = "MCU1 Log 4 Register. Stores information about the last MCU1 program counter."]
pub mod mcu1_log4;
#[doc = "mcu1_log5 (rw) register accessor: MCU1 Log 5 Register. Stores information about the last MCU1 lockup and halt status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcu1_log5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcu1_log5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcu1_log5`]
module"]
pub type MCU1_LOG5 = crate::Reg<mcu1_log5::MCU1_LOG5_SPEC>;
#[doc = "MCU1 Log 5 Register. Stores information about the last MCU1 lockup and halt status."]
pub mod mcu1_log5;
#[doc = "cpu_mbist (rw) register accessor: CPU MBIST Register. Controls the CPU memory built-in self-test (MBIST).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_mbist::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_mbist::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_mbist`]
module"]
pub type CPU_MBIST = crate::Reg<cpu_mbist::CPU_MBIST_SPEC>;
#[doc = "CPU MBIST Register. Controls the CPU memory built-in self-test (MBIST)."]
pub mod cpu_mbist;
#[doc = "irom1_misr_dataout_0 (rw) register accessor: IROM1 MISR Data Out 0 Register. Stores the MISR data output for IROM1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irom1_misr_dataout_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irom1_misr_dataout_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irom1_misr_dataout_0`]
module"]
pub type IROM1_MISR_DATAOUT_0 = crate::Reg<irom1_misr_dataout_0::IROM1_MISR_DATAOUT_0_SPEC>;
#[doc = "IROM1 MISR Data Out 0 Register. Stores the MISR data output for IROM1."]
pub mod irom1_misr_dataout_0;
#[doc = "irom1_misr_dataout_1 (rw) register accessor: IROM1 MISR Data Out 1 Register. Stores the MISR data output for IROM1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irom1_misr_dataout_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irom1_misr_dataout_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irom1_misr_dataout_1`]
module"]
pub type IROM1_MISR_DATAOUT_1 = crate::Reg<irom1_misr_dataout_1::IROM1_MISR_DATAOUT_1_SPEC>;
#[doc = "IROM1 MISR Data Out 1 Register. Stores the MISR data output for IROM1."]
pub mod irom1_misr_dataout_1;
