#[doc = "Register `mcu_cfg1` reader"]
pub type R = crate::R<MCU_CFG1_SPEC>;
#[doc = "Register `mcu_cfg1` writer"]
pub type W = crate::W<MCU_CFG1_SPEC>;
#[doc = "Field `reg_mcu1_dfs_req` reader - MCU1 dynamic frequency scaling (DFS) request. Writing 1 to this bit requests a DFS transition."]
pub type REG_MCU1_DFS_REQ_R = crate::BitReader;
#[doc = "Field `reg_mcu1_dfs_req` writer - MCU1 dynamic frequency scaling (DFS) request. Writing 1 to this bit requests a DFS transition."]
pub type REG_MCU1_DFS_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sts_mcu1_dfs_ack` reader - MCU1 DFS acknowledge. This bit is set when a DFS transition is acknowledged."]
pub type STS_MCU1_DFS_ACK_R = crate::BitReader;
#[doc = "Field `sts_mcu1_dfs_ack` writer - MCU1 DFS acknowledge. This bit is set when a DFS transition is acknowledged."]
pub type STS_MCU1_DFS_ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mcu1_srst_en` reader - MCU1 soft reset enable. This field controls the soft reset enable for MCU1."]
pub type REG_MCU1_SRST_EN_R = crate::FieldReader;
#[doc = "Field `reg_mcu1_srst_en` writer - MCU1 soft reset enable. This field controls the soft reset enable for MCU1."]
pub type REG_MCU1_SRST_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sts_mcu1_lpmd_b` reader - MCU1 low power mode B status. This field indicates the current low power mode B state of MCU1."]
pub type STS_MCU1_LPMD_B_R = crate::FieldReader;
#[doc = "Field `sts_mcu1_lpmd_b` writer - MCU1 low power mode B status. This field indicates the current low power mode B state of MCU1."]
pub type STS_MCU1_LPMD_B_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MCU1_WFI_FORCE` reader - MCU1 WFI force. Writing 1 to this bit forces MCU1 into WFI."]
pub type MCU1_WFI_FORCE_R = crate::BitReader;
#[doc = "Field `MCU1_WFI_FORCE` writer - MCU1 WFI force. Writing 1 to this bit forces MCU1 into WFI."]
pub type MCU1_WFI_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mcu1_ndm_rstn_en` reader - MCU1 NDM reset enable. Writing 1 to this bit enables the NDM reset for MCU1."]
pub type MCU1_NDM_RSTN_EN_R = crate::BitReader;
#[doc = "Field `mcu1_ndm_rstn_en` writer - MCU1 NDM reset enable. Writing 1 to this bit enables the NDM reset for MCU1."]
pub type MCU1_NDM_RSTN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mcu1_hart_rstn_en` reader - MCU1 hart reset enable. Writing 1 to this bit enables the hart reset for MCU1."]
pub type MCU1_HART_RSTN_EN_R = crate::BitReader;
#[doc = "Field `mcu1_hart_rstn_en` writer - MCU1 hart reset enable. Writing 1 to this bit enables the hart reset for MCU1."]
pub type MCU1_HART_RSTN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MCU1 dynamic frequency scaling (DFS) request. Writing 1 to this bit requests a DFS transition."]
    #[inline(always)]
    pub fn reg_mcu1_dfs_req(&self) -> REG_MCU1_DFS_REQ_R {
        REG_MCU1_DFS_REQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - MCU1 DFS acknowledge. This bit is set when a DFS transition is acknowledged."]
    #[inline(always)]
    pub fn sts_mcu1_dfs_ack(&self) -> STS_MCU1_DFS_ACK_R {
        STS_MCU1_DFS_ACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - MCU1 soft reset enable. This field controls the soft reset enable for MCU1."]
    #[inline(always)]
    pub fn reg_mcu1_srst_en(&self) -> REG_MCU1_SRST_EN_R {
        REG_MCU1_SRST_EN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 10:11 - MCU1 low power mode B status. This field indicates the current low power mode B state of MCU1."]
    #[inline(always)]
    pub fn sts_mcu1_lpmd_b(&self) -> STS_MCU1_LPMD_B_R {
        STS_MCU1_LPMD_B_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 16 - MCU1 WFI force. Writing 1 to this bit forces MCU1 into WFI."]
    #[inline(always)]
    pub fn mcu1_wfi_force(&self) -> MCU1_WFI_FORCE_R {
        MCU1_WFI_FORCE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 28 - MCU1 NDM reset enable. Writing 1 to this bit enables the NDM reset for MCU1."]
    #[inline(always)]
    pub fn mcu1_ndm_rstn_en(&self) -> MCU1_NDM_RSTN_EN_R {
        MCU1_NDM_RSTN_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - MCU1 hart reset enable. Writing 1 to this bit enables the hart reset for MCU1."]
    #[inline(always)]
    pub fn mcu1_hart_rstn_en(&self) -> MCU1_HART_RSTN_EN_R {
        MCU1_HART_RSTN_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MCU1 dynamic frequency scaling (DFS) request. Writing 1 to this bit requests a DFS transition."]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu1_dfs_req(&mut self) -> REG_MCU1_DFS_REQ_W<MCU_CFG1_SPEC> {
        REG_MCU1_DFS_REQ_W::new(self, 0)
    }
    #[doc = "Bit 2 - MCU1 DFS acknowledge. This bit is set when a DFS transition is acknowledged."]
    #[inline(always)]
    #[must_use]
    pub fn sts_mcu1_dfs_ack(&mut self) -> STS_MCU1_DFS_ACK_W<MCU_CFG1_SPEC> {
        STS_MCU1_DFS_ACK_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - MCU1 soft reset enable. This field controls the soft reset enable for MCU1."]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu1_srst_en(&mut self) -> REG_MCU1_SRST_EN_W<MCU_CFG1_SPEC> {
        REG_MCU1_SRST_EN_W::new(self, 4)
    }
    #[doc = "Bits 10:11 - MCU1 low power mode B status. This field indicates the current low power mode B state of MCU1."]
    #[inline(always)]
    #[must_use]
    pub fn sts_mcu1_lpmd_b(&mut self) -> STS_MCU1_LPMD_B_W<MCU_CFG1_SPEC> {
        STS_MCU1_LPMD_B_W::new(self, 10)
    }
    #[doc = "Bit 16 - MCU1 WFI force. Writing 1 to this bit forces MCU1 into WFI."]
    #[inline(always)]
    #[must_use]
    pub fn mcu1_wfi_force(&mut self) -> MCU1_WFI_FORCE_W<MCU_CFG1_SPEC> {
        MCU1_WFI_FORCE_W::new(self, 16)
    }
    #[doc = "Bit 28 - MCU1 NDM reset enable. Writing 1 to this bit enables the NDM reset for MCU1."]
    #[inline(always)]
    #[must_use]
    pub fn mcu1_ndm_rstn_en(&mut self) -> MCU1_NDM_RSTN_EN_W<MCU_CFG1_SPEC> {
        MCU1_NDM_RSTN_EN_W::new(self, 28)
    }
    #[doc = "Bit 29 - MCU1 hart reset enable. Writing 1 to this bit enables the hart reset for MCU1."]
    #[inline(always)]
    #[must_use]
    pub fn mcu1_hart_rstn_en(&mut self) -> MCU1_HART_RSTN_EN_W<MCU_CFG1_SPEC> {
        MCU1_HART_RSTN_EN_W::new(self, 29)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MCU Configuration 1 Register. Controls miscellaneous MCU settings.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcu_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcu_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCU_CFG1_SPEC;
impl crate::RegisterSpec for MCU_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcu_cfg1::R`](R) reader structure"]
impl crate::Readable for MCU_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcu_cfg1::W`](W) writer structure"]
impl crate::Writable for MCU_CFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mcu_cfg1 to value 0"]
impl crate::Resettable for MCU_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
