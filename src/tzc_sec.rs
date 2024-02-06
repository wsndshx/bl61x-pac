#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x40],
    tzc_rom_tzsrg_ctrl: TZC_ROM_TZSRG_CTRL,
    tzc_rom_tzsrg_adr_mask: TZC_ROM_TZSRG_ADR_MASK,
    tzc_rom_tzsrg_r0: TZC_ROM_TZSRG_R0,
    tzc_rom_tzsrg_r1: TZC_ROM_TZSRG_R1,
    tzc_rom_tzsrg_r2: TZC_ROM_TZSRG_R2,
    _reserved5: [u8; 0xac],
    tzc_bmx_tzmid: TZC_BMX_TZMID,
    tzc_bmx_tzmid_lock: TZC_BMX_TZMID_LOCK,
    tzc_bmx_s0: TZC_BMX_S0,
    tzc_bmx_s1: TZC_BMX_S1,
    tzc_bmx_s2: TZC_BMX_S2,
    tzc_bmx_s_lock: TZC_BMX_S_LOCK,
    tzc_bmx_s1a: TZC_BMX_S1A,
    tzc_bmx_s1a_lock: TZC_BMX_S1A_LOCK,
    _reserved13: [u8; 0x20],
    tzc_ocram_tzsrg_ctrl: TZC_OCRAM_TZSRG_CTRL,
    tzc_ocram_tzsrg_adr_mask: TZC_OCRAM_TZSRG_ADR_MASK,
    tzc_ocram_tzsrg_r0: TZC_OCRAM_TZSRG_R0,
    tzc_ocram_tzsrg_r1: TZC_OCRAM_TZSRG_R1,
    tzc_ocram_tzsrg_r2: TZC_OCRAM_TZSRG_R2,
    _reserved18: [u8; 0x2c],
    tzc_wram_tzsrg_ctrl: TZC_WRAM_TZSRG_CTRL,
    tzc_wram_tzsrg_adr_mask: TZC_WRAM_TZSRG_ADR_MASK,
    tzc_wram_tzsrg_r0: TZC_WRAM_TZSRG_R0,
    tzc_wram_tzsrg_r1: TZC_WRAM_TZSRG_R1,
    tzc_wram_tzsrg_r2: TZC_WRAM_TZSRG_R2,
    _reserved23: [u8; 0x04],
    tzc_wifi_dbg: TZC_WIFI_DBG,
    _reserved24: [u8; 0xe4],
    tzc_sf_tzsrg_ctrl: TZC_SF_TZSRG_CTRL,
    tzc_sf_tzsrg_adr_mask: TZC_SF_TZSRG_ADR_MASK,
    tzc_sf_tzsrg_r0: TZC_SF_TZSRG_R0,
    tzc_sf_tzsrg_r1: TZC_SF_TZSRG_R1,
    tzc_sf_tzsrg_r2: TZC_SF_TZSRG_R2,
    tzc_sf_tzsrg_r3: TZC_SF_TZSRG_R3,
    tzc_sf_tzsrg_msb: TZC_SF_TZSRG_MSB,
    _reserved31: [u8; 0x64],
    tzc_mm_bmx_tzmid: TZC_MM_BMX_TZMID,
    tzc_mm_bmx_tzmid_lock: TZC_MM_BMX_TZMID_LOCK,
    _reserved33: [u8; 0x98],
    tzc_psramb_tzsrg_ctrl: TZC_PSRAMB_TZSRG_CTRL,
    tzc_psramb_tzsrg_adr_mask: TZC_PSRAMB_TZSRG_ADR_MASK,
    tzc_psramb_tzsrg_r0: TZC_PSRAMB_TZSRG_R0,
    tzc_psramb_tzsrg_r1: TZC_PSRAMB_TZSRG_R1,
    tzc_psramb_tzsrg_r2: TZC_PSRAMB_TZSRG_R2,
    _reserved38: [u8; 0x0b4c],
    tzc_glb_ctrl_0: TZC_GLB_CTRL_0,
    _reserved39: [u8; 0x04],
    tzc_glb_ctrl_2: TZC_GLB_CTRL_2,
    _reserved40: [u8; 0x34],
    tzc_se_ctrl_0: TZC_SE_CTRL_0,
    tzc_se_ctrl_1: TZC_SE_CTRL_1,
    tzc_se_ctrl_2: TZC_SE_CTRL_2,
}
impl RegisterBlock {
    #[doc = "0x40 - TZC ROM TrustZone Security Region Control"]
    #[inline(always)]
    pub const fn tzc_rom_tzsrg_ctrl(&self) -> &TZC_ROM_TZSRG_CTRL {
        &self.tzc_rom_tzsrg_ctrl
    }
    #[doc = "0x44 - TZC ROM TrustZone Security Region Address Mask"]
    #[inline(always)]
    pub const fn tzc_rom_tzsrg_adr_mask(&self) -> &TZC_ROM_TZSRG_ADR_MASK {
        &self.tzc_rom_tzsrg_adr_mask
    }
    #[doc = "0x48 - TZC ROM TrustZone Security Region 0 Range"]
    #[inline(always)]
    pub const fn tzc_rom_tzsrg_r0(&self) -> &TZC_ROM_TZSRG_R0 {
        &self.tzc_rom_tzsrg_r0
    }
    #[doc = "0x4c - TZC ROM TrustZone Security Region 1 Range"]
    #[inline(always)]
    pub const fn tzc_rom_tzsrg_r1(&self) -> &TZC_ROM_TZSRG_R1 {
        &self.tzc_rom_tzsrg_r1
    }
    #[doc = "0x50 - TZC ROM TrustZone Security Region 2 Range"]
    #[inline(always)]
    pub const fn tzc_rom_tzsrg_r2(&self) -> &TZC_ROM_TZSRG_R2 {
        &self.tzc_rom_tzsrg_r2
    }
    #[doc = "0x100 - TZC Bus Matrix TrustZone Master IDs"]
    #[inline(always)]
    pub const fn tzc_bmx_tzmid(&self) -> &TZC_BMX_TZMID {
        &self.tzc_bmx_tzmid
    }
    #[doc = "0x104 - TZC Bus Matrix TrustZone Master ID Lock"]
    #[inline(always)]
    pub const fn tzc_bmx_tzmid_lock(&self) -> &TZC_BMX_TZMID_LOCK {
        &self.tzc_bmx_tzmid_lock
    }
    #[doc = "0x108 - TrustZone Controller BMX Security Register 0."]
    #[inline(always)]
    pub const fn tzc_bmx_s0(&self) -> &TZC_BMX_S0 {
        &self.tzc_bmx_s0
    }
    #[doc = "0x10c - TrustZone Controller BMX Security Register 1."]
    #[inline(always)]
    pub const fn tzc_bmx_s1(&self) -> &TZC_BMX_S1 {
        &self.tzc_bmx_s1
    }
    #[doc = "0x110 - TrustZone Controller BMX Security Register 2."]
    #[inline(always)]
    pub const fn tzc_bmx_s2(&self) -> &TZC_BMX_S2 {
        &self.tzc_bmx_s2
    }
    #[doc = "0x114 - TrustZone Controller BMX Security Lock Register."]
    #[inline(always)]
    pub const fn tzc_bmx_s_lock(&self) -> &TZC_BMX_S_LOCK {
        &self.tzc_bmx_s_lock
    }
    #[doc = "0x118 - TrustZone Controller BMX Security Register 1A."]
    #[inline(always)]
    pub const fn tzc_bmx_s1a(&self) -> &TZC_BMX_S1A {
        &self.tzc_bmx_s1a
    }
    #[doc = "0x11c - TrustZone Controller BMX Security Lock Register 1A."]
    #[inline(always)]
    pub const fn tzc_bmx_s1a_lock(&self) -> &TZC_BMX_S1A_LOCK {
        &self.tzc_bmx_s1a_lock
    }
    #[doc = "0x140 - TrustZone Controller OCram TrustZone Security Register Group Control."]
    #[inline(always)]
    pub const fn tzc_ocram_tzsrg_ctrl(&self) -> &TZC_OCRAM_TZSRG_CTRL {
        &self.tzc_ocram_tzsrg_ctrl
    }
    #[doc = "0x144 - TrustZone Controller OCram TrustZone Security Register Group Address Mask."]
    #[inline(always)]
    pub const fn tzc_ocram_tzsrg_adr_mask(&self) -> &TZC_OCRAM_TZSRG_ADR_MASK {
        &self.tzc_ocram_tzsrg_adr_mask
    }
    #[doc = "0x148 - TrustZone Controller OCram TrustZone Security Register Group Range 0."]
    #[inline(always)]
    pub const fn tzc_ocram_tzsrg_r0(&self) -> &TZC_OCRAM_TZSRG_R0 {
        &self.tzc_ocram_tzsrg_r0
    }
    #[doc = "0x14c - TrustZone Controller OCram TrustZone Security Register Group Range 1."]
    #[inline(always)]
    pub const fn tzc_ocram_tzsrg_r1(&self) -> &TZC_OCRAM_TZSRG_R1 {
        &self.tzc_ocram_tzsrg_r1
    }
    #[doc = "0x150 - TrustZone Controller OCram TrustZone Security Register Group Range 2."]
    #[inline(always)]
    pub const fn tzc_ocram_tzsrg_r2(&self) -> &TZC_OCRAM_TZSRG_R2 {
        &self.tzc_ocram_tzsrg_r2
    }
    #[doc = "0x180 - TrustZone Controller WRam TrustZone Security Register Group Control."]
    #[inline(always)]
    pub const fn tzc_wram_tzsrg_ctrl(&self) -> &TZC_WRAM_TZSRG_CTRL {
        &self.tzc_wram_tzsrg_ctrl
    }
    #[doc = "0x184 - TrustZone Controller WRam TrustZone Security Register Group Address Mask."]
    #[inline(always)]
    pub const fn tzc_wram_tzsrg_adr_mask(&self) -> &TZC_WRAM_TZSRG_ADR_MASK {
        &self.tzc_wram_tzsrg_adr_mask
    }
    #[doc = "0x188 - TrustZone Controller WRam TrustZone Security Register Group Range 0."]
    #[inline(always)]
    pub const fn tzc_wram_tzsrg_r0(&self) -> &TZC_WRAM_TZSRG_R0 {
        &self.tzc_wram_tzsrg_r0
    }
    #[doc = "0x18c - TrustZone Controller WRam TrustZone Security Register Group Range 1."]
    #[inline(always)]
    pub const fn tzc_wram_tzsrg_r1(&self) -> &TZC_WRAM_TZSRG_R1 {
        &self.tzc_wram_tzsrg_r1
    }
    #[doc = "0x190 - TrustZone Controller WRam TrustZone Security Register Group Range 2."]
    #[inline(always)]
    pub const fn tzc_wram_tzsrg_r2(&self) -> &TZC_WRAM_TZSRG_R2 {
        &self.tzc_wram_tzsrg_r2
    }
    #[doc = "0x198 - TrustZone Controller WiFi Debug."]
    #[inline(always)]
    pub const fn tzc_wifi_dbg(&self) -> &TZC_WIFI_DBG {
        &self.tzc_wifi_dbg
    }
    #[doc = "0x280 - TrustZone Controller SF TrustZone Security Register Group Control."]
    #[inline(always)]
    pub const fn tzc_sf_tzsrg_ctrl(&self) -> &TZC_SF_TZSRG_CTRL {
        &self.tzc_sf_tzsrg_ctrl
    }
    #[doc = "0x284 - TrustZone Controller SF TrustZone Security Register Group Address Mask."]
    #[inline(always)]
    pub const fn tzc_sf_tzsrg_adr_mask(&self) -> &TZC_SF_TZSRG_ADR_MASK {
        &self.tzc_sf_tzsrg_adr_mask
    }
    #[doc = "0x288 - TrustZone Controller SF TrustZone Security Register Group Range 0."]
    #[inline(always)]
    pub const fn tzc_sf_tzsrg_r0(&self) -> &TZC_SF_TZSRG_R0 {
        &self.tzc_sf_tzsrg_r0
    }
    #[doc = "0x28c - TrustZone Controller SF TrustZone Security Register Group Range 1."]
    #[inline(always)]
    pub const fn tzc_sf_tzsrg_r1(&self) -> &TZC_SF_TZSRG_R1 {
        &self.tzc_sf_tzsrg_r1
    }
    #[doc = "0x290 - TrustZone Controller SF TrustZone Security Register Group Range 2."]
    #[inline(always)]
    pub const fn tzc_sf_tzsrg_r2(&self) -> &TZC_SF_TZSRG_R2 {
        &self.tzc_sf_tzsrg_r2
    }
    #[doc = "0x294 - TrustZone Controller SF TrustZone Security Register Group Range 3."]
    #[inline(always)]
    pub const fn tzc_sf_tzsrg_r3(&self) -> &TZC_SF_TZSRG_R3 {
        &self.tzc_sf_tzsrg_r3
    }
    #[doc = "0x298 - TrustZone Controller SF TrustZone Security Register Group MSB."]
    #[inline(always)]
    pub const fn tzc_sf_tzsrg_msb(&self) -> &TZC_SF_TZSRG_MSB {
        &self.tzc_sf_tzsrg_msb
    }
    #[doc = "0x300 - TrustZone Controller Memory-Mapped Bus Matrix TrustZone Master ID."]
    #[inline(always)]
    pub const fn tzc_mm_bmx_tzmid(&self) -> &TZC_MM_BMX_TZMID {
        &self.tzc_mm_bmx_tzmid
    }
    #[doc = "0x304 - TrustZone Controller Memory-Mapped Bus Matrix TrustZone Master ID Lock."]
    #[inline(always)]
    pub const fn tzc_mm_bmx_tzmid_lock(&self) -> &TZC_MM_BMX_TZMID_LOCK {
        &self.tzc_mm_bmx_tzmid_lock
    }
    #[doc = "0x3a0 - TrustZone Controller PsramB TrustZone Security Register Group Control."]
    #[inline(always)]
    pub const fn tzc_psramb_tzsrg_ctrl(&self) -> &TZC_PSRAMB_TZSRG_CTRL {
        &self.tzc_psramb_tzsrg_ctrl
    }
    #[doc = "0x3a4 - TrustZone Controller PsramB TrustZone Security Register Group Address Mask."]
    #[inline(always)]
    pub const fn tzc_psramb_tzsrg_adr_mask(&self) -> &TZC_PSRAMB_TZSRG_ADR_MASK {
        &self.tzc_psramb_tzsrg_adr_mask
    }
    #[doc = "0x3a8 - TrustZone Controller PsramB TrustZone Security Register Group Range 0."]
    #[inline(always)]
    pub const fn tzc_psramb_tzsrg_r0(&self) -> &TZC_PSRAMB_TZSRG_R0 {
        &self.tzc_psramb_tzsrg_r0
    }
    #[doc = "0x3ac - TrustZone Controller PsramB TrustZone Security Register Group Range 1."]
    #[inline(always)]
    pub const fn tzc_psramb_tzsrg_r1(&self) -> &TZC_PSRAMB_TZSRG_R1 {
        &self.tzc_psramb_tzsrg_r1
    }
    #[doc = "0x3b0 - TrustZone Controller PsramB TrustZone Security Register Group Range 2."]
    #[inline(always)]
    pub const fn tzc_psramb_tzsrg_r2(&self) -> &TZC_PSRAMB_TZSRG_R2 {
        &self.tzc_psramb_tzsrg_r2
    }
    #[doc = "0xf00 - TrustZone Controller Global Control 0."]
    #[inline(always)]
    pub const fn tzc_glb_ctrl_0(&self) -> &TZC_GLB_CTRL_0 {
        &self.tzc_glb_ctrl_0
    }
    #[doc = "0xf08 - TrustZone Controller Global Control 2."]
    #[inline(always)]
    pub const fn tzc_glb_ctrl_2(&self) -> &TZC_GLB_CTRL_2 {
        &self.tzc_glb_ctrl_2
    }
    #[doc = "0xf40 - TrustZone Controller Secure Engine Control 0."]
    #[inline(always)]
    pub const fn tzc_se_ctrl_0(&self) -> &TZC_SE_CTRL_0 {
        &self.tzc_se_ctrl_0
    }
    #[doc = "0xf44 - TrustZone Controller Secure Engine Control 1."]
    #[inline(always)]
    pub const fn tzc_se_ctrl_1(&self) -> &TZC_SE_CTRL_1 {
        &self.tzc_se_ctrl_1
    }
    #[doc = "0xf48 - TrustZone Controller Secure Engine Control 2."]
    #[inline(always)]
    pub const fn tzc_se_ctrl_2(&self) -> &TZC_SE_CTRL_2 {
        &self.tzc_se_ctrl_2
    }
}
#[doc = "tzc_rom_tzsrg_ctrl (rw) register accessor: TZC ROM TrustZone Security Region Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_rom_tzsrg_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_rom_tzsrg_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_rom_tzsrg_ctrl`]
module"]
pub type TZC_ROM_TZSRG_CTRL = crate::Reg<tzc_rom_tzsrg_ctrl::TZC_ROM_TZSRG_CTRL_SPEC>;
#[doc = "TZC ROM TrustZone Security Region Control"]
pub mod tzc_rom_tzsrg_ctrl;
#[doc = "tzc_rom_tzsrg_adr_mask (rw) register accessor: TZC ROM TrustZone Security Region Address Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_rom_tzsrg_adr_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_rom_tzsrg_adr_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_rom_tzsrg_adr_mask`]
module"]
pub type TZC_ROM_TZSRG_ADR_MASK = crate::Reg<tzc_rom_tzsrg_adr_mask::TZC_ROM_TZSRG_ADR_MASK_SPEC>;
#[doc = "TZC ROM TrustZone Security Region Address Mask"]
pub mod tzc_rom_tzsrg_adr_mask;
#[doc = "tzc_rom_tzsrg_r0 (rw) register accessor: TZC ROM TrustZone Security Region 0 Range\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_rom_tzsrg_r0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_rom_tzsrg_r0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_rom_tzsrg_r0`]
module"]
pub type TZC_ROM_TZSRG_R0 = crate::Reg<tzc_rom_tzsrg_r0::TZC_ROM_TZSRG_R0_SPEC>;
#[doc = "TZC ROM TrustZone Security Region 0 Range"]
pub mod tzc_rom_tzsrg_r0;
#[doc = "tzc_rom_tzsrg_r1 (rw) register accessor: TZC ROM TrustZone Security Region 1 Range\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_rom_tzsrg_r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_rom_tzsrg_r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_rom_tzsrg_r1`]
module"]
pub type TZC_ROM_TZSRG_R1 = crate::Reg<tzc_rom_tzsrg_r1::TZC_ROM_TZSRG_R1_SPEC>;
#[doc = "TZC ROM TrustZone Security Region 1 Range"]
pub mod tzc_rom_tzsrg_r1;
#[doc = "tzc_rom_tzsrg_r2 (rw) register accessor: TZC ROM TrustZone Security Region 2 Range\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_rom_tzsrg_r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_rom_tzsrg_r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_rom_tzsrg_r2`]
module"]
pub type TZC_ROM_TZSRG_R2 = crate::Reg<tzc_rom_tzsrg_r2::TZC_ROM_TZSRG_R2_SPEC>;
#[doc = "TZC ROM TrustZone Security Region 2 Range"]
pub mod tzc_rom_tzsrg_r2;
#[doc = "tzc_bmx_tzmid (rw) register accessor: TZC Bus Matrix TrustZone Master IDs\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_bmx_tzmid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_bmx_tzmid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_bmx_tzmid`]
module"]
pub type TZC_BMX_TZMID = crate::Reg<tzc_bmx_tzmid::TZC_BMX_TZMID_SPEC>;
#[doc = "TZC Bus Matrix TrustZone Master IDs"]
pub mod tzc_bmx_tzmid;
#[doc = "tzc_bmx_tzmid_lock (rw) register accessor: TZC Bus Matrix TrustZone Master ID Lock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_bmx_tzmid_lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_bmx_tzmid_lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_bmx_tzmid_lock`]
module"]
pub type TZC_BMX_TZMID_LOCK = crate::Reg<tzc_bmx_tzmid_lock::TZC_BMX_TZMID_LOCK_SPEC>;
#[doc = "TZC Bus Matrix TrustZone Master ID Lock"]
pub mod tzc_bmx_tzmid_lock;
#[doc = "tzc_bmx_s0 (rw) register accessor: TrustZone Controller BMX Security Register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_bmx_s0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_bmx_s0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_bmx_s0`]
module"]
pub type TZC_BMX_S0 = crate::Reg<tzc_bmx_s0::TZC_BMX_S0_SPEC>;
#[doc = "TrustZone Controller BMX Security Register 0."]
pub mod tzc_bmx_s0;
#[doc = "tzc_bmx_s1 (rw) register accessor: TrustZone Controller BMX Security Register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_bmx_s1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_bmx_s1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_bmx_s1`]
module"]
pub type TZC_BMX_S1 = crate::Reg<tzc_bmx_s1::TZC_BMX_S1_SPEC>;
#[doc = "TrustZone Controller BMX Security Register 1."]
pub mod tzc_bmx_s1;
#[doc = "tzc_bmx_s2 (rw) register accessor: TrustZone Controller BMX Security Register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_bmx_s2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_bmx_s2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_bmx_s2`]
module"]
pub type TZC_BMX_S2 = crate::Reg<tzc_bmx_s2::TZC_BMX_S2_SPEC>;
#[doc = "TrustZone Controller BMX Security Register 2."]
pub mod tzc_bmx_s2;
#[doc = "tzc_bmx_s_lock (rw) register accessor: TrustZone Controller BMX Security Lock Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_bmx_s_lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_bmx_s_lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_bmx_s_lock`]
module"]
pub type TZC_BMX_S_LOCK = crate::Reg<tzc_bmx_s_lock::TZC_BMX_S_LOCK_SPEC>;
#[doc = "TrustZone Controller BMX Security Lock Register."]
pub mod tzc_bmx_s_lock;
#[doc = "tzc_bmx_s1a (rw) register accessor: TrustZone Controller BMX Security Register 1A.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_bmx_s1a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_bmx_s1a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_bmx_s1a`]
module"]
pub type TZC_BMX_S1A = crate::Reg<tzc_bmx_s1a::TZC_BMX_S1A_SPEC>;
#[doc = "TrustZone Controller BMX Security Register 1A."]
pub mod tzc_bmx_s1a;
#[doc = "tzc_bmx_s1a_lock (rw) register accessor: TrustZone Controller BMX Security Lock Register 1A.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_bmx_s1a_lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_bmx_s1a_lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_bmx_s1a_lock`]
module"]
pub type TZC_BMX_S1A_LOCK = crate::Reg<tzc_bmx_s1a_lock::TZC_BMX_S1A_LOCK_SPEC>;
#[doc = "TrustZone Controller BMX Security Lock Register 1A."]
pub mod tzc_bmx_s1a_lock;
#[doc = "tzc_ocram_tzsrg_ctrl (rw) register accessor: TrustZone Controller OCram TrustZone Security Register Group Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_ocram_tzsrg_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_ocram_tzsrg_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_ocram_tzsrg_ctrl`]
module"]
pub type TZC_OCRAM_TZSRG_CTRL = crate::Reg<tzc_ocram_tzsrg_ctrl::TZC_OCRAM_TZSRG_CTRL_SPEC>;
#[doc = "TrustZone Controller OCram TrustZone Security Register Group Control."]
pub mod tzc_ocram_tzsrg_ctrl;
#[doc = "tzc_ocram_tzsrg_adr_mask (rw) register accessor: TrustZone Controller OCram TrustZone Security Register Group Address Mask.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_ocram_tzsrg_adr_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_ocram_tzsrg_adr_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_ocram_tzsrg_adr_mask`]
module"]
pub type TZC_OCRAM_TZSRG_ADR_MASK =
    crate::Reg<tzc_ocram_tzsrg_adr_mask::TZC_OCRAM_TZSRG_ADR_MASK_SPEC>;
#[doc = "TrustZone Controller OCram TrustZone Security Register Group Address Mask."]
pub mod tzc_ocram_tzsrg_adr_mask;
#[doc = "tzc_ocram_tzsrg_r0 (rw) register accessor: TrustZone Controller OCram TrustZone Security Register Group Range 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_ocram_tzsrg_r0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_ocram_tzsrg_r0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_ocram_tzsrg_r0`]
module"]
pub type TZC_OCRAM_TZSRG_R0 = crate::Reg<tzc_ocram_tzsrg_r0::TZC_OCRAM_TZSRG_R0_SPEC>;
#[doc = "TrustZone Controller OCram TrustZone Security Register Group Range 0."]
pub mod tzc_ocram_tzsrg_r0;
#[doc = "tzc_ocram_tzsrg_r1 (rw) register accessor: TrustZone Controller OCram TrustZone Security Register Group Range 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_ocram_tzsrg_r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_ocram_tzsrg_r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_ocram_tzsrg_r1`]
module"]
pub type TZC_OCRAM_TZSRG_R1 = crate::Reg<tzc_ocram_tzsrg_r1::TZC_OCRAM_TZSRG_R1_SPEC>;
#[doc = "TrustZone Controller OCram TrustZone Security Register Group Range 1."]
pub mod tzc_ocram_tzsrg_r1;
#[doc = "tzc_ocram_tzsrg_r2 (rw) register accessor: TrustZone Controller OCram TrustZone Security Register Group Range 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_ocram_tzsrg_r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_ocram_tzsrg_r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_ocram_tzsrg_r2`]
module"]
pub type TZC_OCRAM_TZSRG_R2 = crate::Reg<tzc_ocram_tzsrg_r2::TZC_OCRAM_TZSRG_R2_SPEC>;
#[doc = "TrustZone Controller OCram TrustZone Security Register Group Range 2."]
pub mod tzc_ocram_tzsrg_r2;
#[doc = "tzc_wram_tzsrg_ctrl (rw) register accessor: TrustZone Controller WRam TrustZone Security Register Group Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_wram_tzsrg_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_wram_tzsrg_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_wram_tzsrg_ctrl`]
module"]
pub type TZC_WRAM_TZSRG_CTRL = crate::Reg<tzc_wram_tzsrg_ctrl::TZC_WRAM_TZSRG_CTRL_SPEC>;
#[doc = "TrustZone Controller WRam TrustZone Security Register Group Control."]
pub mod tzc_wram_tzsrg_ctrl;
#[doc = "tzc_wram_tzsrg_adr_mask (rw) register accessor: TrustZone Controller WRam TrustZone Security Register Group Address Mask.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_wram_tzsrg_adr_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_wram_tzsrg_adr_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_wram_tzsrg_adr_mask`]
module"]
pub type TZC_WRAM_TZSRG_ADR_MASK =
    crate::Reg<tzc_wram_tzsrg_adr_mask::TZC_WRAM_TZSRG_ADR_MASK_SPEC>;
#[doc = "TrustZone Controller WRam TrustZone Security Register Group Address Mask."]
pub mod tzc_wram_tzsrg_adr_mask;
#[doc = "tzc_wram_tzsrg_r0 (rw) register accessor: TrustZone Controller WRam TrustZone Security Register Group Range 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_wram_tzsrg_r0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_wram_tzsrg_r0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_wram_tzsrg_r0`]
module"]
pub type TZC_WRAM_TZSRG_R0 = crate::Reg<tzc_wram_tzsrg_r0::TZC_WRAM_TZSRG_R0_SPEC>;
#[doc = "TrustZone Controller WRam TrustZone Security Register Group Range 0."]
pub mod tzc_wram_tzsrg_r0;
#[doc = "tzc_wram_tzsrg_r1 (rw) register accessor: TrustZone Controller WRam TrustZone Security Register Group Range 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_wram_tzsrg_r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_wram_tzsrg_r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_wram_tzsrg_r1`]
module"]
pub type TZC_WRAM_TZSRG_R1 = crate::Reg<tzc_wram_tzsrg_r1::TZC_WRAM_TZSRG_R1_SPEC>;
#[doc = "TrustZone Controller WRam TrustZone Security Register Group Range 1."]
pub mod tzc_wram_tzsrg_r1;
#[doc = "tzc_wram_tzsrg_r2 (rw) register accessor: TrustZone Controller WRam TrustZone Security Register Group Range 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_wram_tzsrg_r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_wram_tzsrg_r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_wram_tzsrg_r2`]
module"]
pub type TZC_WRAM_TZSRG_R2 = crate::Reg<tzc_wram_tzsrg_r2::TZC_WRAM_TZSRG_R2_SPEC>;
#[doc = "TrustZone Controller WRam TrustZone Security Register Group Range 2."]
pub mod tzc_wram_tzsrg_r2;
#[doc = "tzc_wifi_dbg (rw) register accessor: TrustZone Controller WiFi Debug.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_wifi_dbg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_wifi_dbg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_wifi_dbg`]
module"]
pub type TZC_WIFI_DBG = crate::Reg<tzc_wifi_dbg::TZC_WIFI_DBG_SPEC>;
#[doc = "TrustZone Controller WiFi Debug."]
pub mod tzc_wifi_dbg;
#[doc = "tzc_sf_tzsrg_ctrl (rw) register accessor: TrustZone Controller SF TrustZone Security Register Group Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_sf_tzsrg_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_sf_tzsrg_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_sf_tzsrg_ctrl`]
module"]
pub type TZC_SF_TZSRG_CTRL = crate::Reg<tzc_sf_tzsrg_ctrl::TZC_SF_TZSRG_CTRL_SPEC>;
#[doc = "TrustZone Controller SF TrustZone Security Register Group Control."]
pub mod tzc_sf_tzsrg_ctrl;
#[doc = "tzc_sf_tzsrg_adr_mask (rw) register accessor: TrustZone Controller SF TrustZone Security Register Group Address Mask.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_sf_tzsrg_adr_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_sf_tzsrg_adr_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_sf_tzsrg_adr_mask`]
module"]
pub type TZC_SF_TZSRG_ADR_MASK = crate::Reg<tzc_sf_tzsrg_adr_mask::TZC_SF_TZSRG_ADR_MASK_SPEC>;
#[doc = "TrustZone Controller SF TrustZone Security Register Group Address Mask."]
pub mod tzc_sf_tzsrg_adr_mask;
#[doc = "tzc_sf_tzsrg_r0 (rw) register accessor: TrustZone Controller SF TrustZone Security Register Group Range 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_sf_tzsrg_r0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_sf_tzsrg_r0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_sf_tzsrg_r0`]
module"]
pub type TZC_SF_TZSRG_R0 = crate::Reg<tzc_sf_tzsrg_r0::TZC_SF_TZSRG_R0_SPEC>;
#[doc = "TrustZone Controller SF TrustZone Security Register Group Range 0."]
pub mod tzc_sf_tzsrg_r0;
#[doc = "tzc_sf_tzsrg_r1 (rw) register accessor: TrustZone Controller SF TrustZone Security Register Group Range 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_sf_tzsrg_r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_sf_tzsrg_r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_sf_tzsrg_r1`]
module"]
pub type TZC_SF_TZSRG_R1 = crate::Reg<tzc_sf_tzsrg_r1::TZC_SF_TZSRG_R1_SPEC>;
#[doc = "TrustZone Controller SF TrustZone Security Register Group Range 1."]
pub mod tzc_sf_tzsrg_r1;
#[doc = "tzc_sf_tzsrg_r2 (rw) register accessor: TrustZone Controller SF TrustZone Security Register Group Range 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_sf_tzsrg_r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_sf_tzsrg_r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_sf_tzsrg_r2`]
module"]
pub type TZC_SF_TZSRG_R2 = crate::Reg<tzc_sf_tzsrg_r2::TZC_SF_TZSRG_R2_SPEC>;
#[doc = "TrustZone Controller SF TrustZone Security Register Group Range 2."]
pub mod tzc_sf_tzsrg_r2;
#[doc = "tzc_sf_tzsrg_r3 (rw) register accessor: TrustZone Controller SF TrustZone Security Register Group Range 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_sf_tzsrg_r3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_sf_tzsrg_r3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_sf_tzsrg_r3`]
module"]
pub type TZC_SF_TZSRG_R3 = crate::Reg<tzc_sf_tzsrg_r3::TZC_SF_TZSRG_R3_SPEC>;
#[doc = "TrustZone Controller SF TrustZone Security Register Group Range 3."]
pub mod tzc_sf_tzsrg_r3;
#[doc = "tzc_sf_tzsrg_msb (rw) register accessor: TrustZone Controller SF TrustZone Security Register Group MSB.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_sf_tzsrg_msb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_sf_tzsrg_msb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_sf_tzsrg_msb`]
module"]
pub type TZC_SF_TZSRG_MSB = crate::Reg<tzc_sf_tzsrg_msb::TZC_SF_TZSRG_MSB_SPEC>;
#[doc = "TrustZone Controller SF TrustZone Security Register Group MSB."]
pub mod tzc_sf_tzsrg_msb;
#[doc = "tzc_mm_bmx_tzmid (rw) register accessor: TrustZone Controller Memory-Mapped Bus Matrix TrustZone Master ID.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_mm_bmx_tzmid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_mm_bmx_tzmid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_mm_bmx_tzmid`]
module"]
pub type TZC_MM_BMX_TZMID = crate::Reg<tzc_mm_bmx_tzmid::TZC_MM_BMX_TZMID_SPEC>;
#[doc = "TrustZone Controller Memory-Mapped Bus Matrix TrustZone Master ID."]
pub mod tzc_mm_bmx_tzmid;
#[doc = "tzc_mm_bmx_tzmid_lock (rw) register accessor: TrustZone Controller Memory-Mapped Bus Matrix TrustZone Master ID Lock.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_mm_bmx_tzmid_lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_mm_bmx_tzmid_lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_mm_bmx_tzmid_lock`]
module"]
pub type TZC_MM_BMX_TZMID_LOCK = crate::Reg<tzc_mm_bmx_tzmid_lock::TZC_MM_BMX_TZMID_LOCK_SPEC>;
#[doc = "TrustZone Controller Memory-Mapped Bus Matrix TrustZone Master ID Lock."]
pub mod tzc_mm_bmx_tzmid_lock;
#[doc = "tzc_psramb_tzsrg_ctrl (rw) register accessor: TrustZone Controller PsramB TrustZone Security Register Group Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_psramb_tzsrg_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_psramb_tzsrg_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_psramb_tzsrg_ctrl`]
module"]
pub type TZC_PSRAMB_TZSRG_CTRL = crate::Reg<tzc_psramb_tzsrg_ctrl::TZC_PSRAMB_TZSRG_CTRL_SPEC>;
#[doc = "TrustZone Controller PsramB TrustZone Security Register Group Control."]
pub mod tzc_psramb_tzsrg_ctrl;
#[doc = "tzc_psramb_tzsrg_adr_mask (rw) register accessor: TrustZone Controller PsramB TrustZone Security Register Group Address Mask.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_psramb_tzsrg_adr_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_psramb_tzsrg_adr_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_psramb_tzsrg_adr_mask`]
module"]
pub type TZC_PSRAMB_TZSRG_ADR_MASK =
    crate::Reg<tzc_psramb_tzsrg_adr_mask::TZC_PSRAMB_TZSRG_ADR_MASK_SPEC>;
#[doc = "TrustZone Controller PsramB TrustZone Security Register Group Address Mask."]
pub mod tzc_psramb_tzsrg_adr_mask;
#[doc = "tzc_psramb_tzsrg_r0 (rw) register accessor: TrustZone Controller PsramB TrustZone Security Register Group Range 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_psramb_tzsrg_r0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_psramb_tzsrg_r0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_psramb_tzsrg_r0`]
module"]
pub type TZC_PSRAMB_TZSRG_R0 = crate::Reg<tzc_psramb_tzsrg_r0::TZC_PSRAMB_TZSRG_R0_SPEC>;
#[doc = "TrustZone Controller PsramB TrustZone Security Register Group Range 0."]
pub mod tzc_psramb_tzsrg_r0;
#[doc = "tzc_psramb_tzsrg_r1 (rw) register accessor: TrustZone Controller PsramB TrustZone Security Register Group Range 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_psramb_tzsrg_r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_psramb_tzsrg_r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_psramb_tzsrg_r1`]
module"]
pub type TZC_PSRAMB_TZSRG_R1 = crate::Reg<tzc_psramb_tzsrg_r1::TZC_PSRAMB_TZSRG_R1_SPEC>;
#[doc = "TrustZone Controller PsramB TrustZone Security Register Group Range 1."]
pub mod tzc_psramb_tzsrg_r1;
#[doc = "tzc_psramb_tzsrg_r2 (rw) register accessor: TrustZone Controller PsramB TrustZone Security Register Group Range 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_psramb_tzsrg_r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_psramb_tzsrg_r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_psramb_tzsrg_r2`]
module"]
pub type TZC_PSRAMB_TZSRG_R2 = crate::Reg<tzc_psramb_tzsrg_r2::TZC_PSRAMB_TZSRG_R2_SPEC>;
#[doc = "TrustZone Controller PsramB TrustZone Security Register Group Range 2."]
pub mod tzc_psramb_tzsrg_r2;
#[doc = "tzc_glb_ctrl_0 (rw) register accessor: TrustZone Controller Global Control 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_glb_ctrl_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_glb_ctrl_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_glb_ctrl_0`]
module"]
pub type TZC_GLB_CTRL_0 = crate::Reg<tzc_glb_ctrl_0::TZC_GLB_CTRL_0_SPEC>;
#[doc = "TrustZone Controller Global Control 0."]
pub mod tzc_glb_ctrl_0;
#[doc = "tzc_glb_ctrl_2 (rw) register accessor: TrustZone Controller Global Control 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_glb_ctrl_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_glb_ctrl_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_glb_ctrl_2`]
module"]
pub type TZC_GLB_CTRL_2 = crate::Reg<tzc_glb_ctrl_2::TZC_GLB_CTRL_2_SPEC>;
#[doc = "TrustZone Controller Global Control 2."]
pub mod tzc_glb_ctrl_2;
#[doc = "tzc_se_ctrl_0 (rw) register accessor: TrustZone Controller Secure Engine Control 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_se_ctrl_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_se_ctrl_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_se_ctrl_0`]
module"]
pub type TZC_SE_CTRL_0 = crate::Reg<tzc_se_ctrl_0::TZC_SE_CTRL_0_SPEC>;
#[doc = "TrustZone Controller Secure Engine Control 0."]
pub mod tzc_se_ctrl_0;
#[doc = "tzc_se_ctrl_1 (rw) register accessor: TrustZone Controller Secure Engine Control 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_se_ctrl_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_se_ctrl_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_se_ctrl_1`]
module"]
pub type TZC_SE_CTRL_1 = crate::Reg<tzc_se_ctrl_1::TZC_SE_CTRL_1_SPEC>;
#[doc = "TrustZone Controller Secure Engine Control 1."]
pub mod tzc_se_ctrl_1;
#[doc = "tzc_se_ctrl_2 (rw) register accessor: TrustZone Controller Secure Engine Control 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_se_ctrl_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_se_ctrl_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tzc_se_ctrl_2`]
module"]
pub type TZC_SE_CTRL_2 = crate::Reg<tzc_se_ctrl_2::TZC_SE_CTRL_2_SPEC>;
#[doc = "TrustZone Controller Secure Engine Control 2."]
pub mod tzc_se_ctrl_2;
