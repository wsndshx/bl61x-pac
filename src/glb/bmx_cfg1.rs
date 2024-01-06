#[doc = "Register `bmx_cfg1` reader"]
pub type R = crate::R<BMX_CFG1_SPEC>;
#[doc = "Register `bmx_cfg1` writer"]
pub type W = crate::W<BMX_CFG1_SPEC>;
#[doc = "Field `reg_bmx_berr_int_en` reader - "]
pub type REG_BMX_BERR_INT_EN_R = crate::BitReader;
#[doc = "Field `reg_bmx_berr_int_en` writer - "]
pub type REG_BMX_BERR_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mcu_berr_int_en` reader - "]
pub type REG_MCU_BERR_INT_EN_R = crate::BitReader;
#[doc = "Field `reg_mcu_berr_int_en` writer - "]
pub type REG_MCU_BERR_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bmx_qos_cpu` reader - "]
pub type REG_BMX_QOS_CPU_R = crate::BitReader;
#[doc = "Field `reg_bmx_qos_cpu` writer - "]
pub type REG_BMX_QOS_CPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bmx_qos_sdu` reader - "]
pub type REG_BMX_QOS_SDU_R = crate::BitReader;
#[doc = "Field `reg_bmx_qos_sdu` writer - "]
pub type REG_BMX_QOS_SDU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bmx_qos_sec0` reader - "]
pub type REG_BMX_QOS_SEC0_R = crate::BitReader;
#[doc = "Field `reg_bmx_qos_sec0` writer - "]
pub type REG_BMX_QOS_SEC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bmx_qos_sec1` reader - "]
pub type REG_BMX_QOS_SEC1_R = crate::BitReader;
#[doc = "Field `reg_bmx_qos_sec1` writer - "]
pub type REG_BMX_QOS_SEC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bmx_qos_sec2` reader - "]
pub type REG_BMX_QOS_SEC2_R = crate::BitReader;
#[doc = "Field `reg_bmx_qos_sec2` writer - "]
pub type REG_BMX_QOS_SEC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bmx_qos_dma` reader - "]
pub type REG_BMX_QOS_DMA_R = crate::BitReader;
#[doc = "Field `reg_bmx_qos_dma` writer - "]
pub type REG_BMX_QOS_DMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bmx_qos_cci` reader - "]
pub type REG_BMX_QOS_CCI_R = crate::BitReader;
#[doc = "Field `reg_bmx_qos_cci` writer - "]
pub type REG_BMX_QOS_CCI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bmx_qos_pldma` reader - "]
pub type REG_BMX_QOS_PLDMA_R = crate::BitReader;
#[doc = "Field `reg_bmx_qos_pldma` writer - "]
pub type REG_BMX_QOS_PLDMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bmx_qos_blem` reader - "]
pub type REG_BMX_QOS_BLEM_R = crate::BitReader;
#[doc = "Field `reg_bmx_qos_blem` writer - "]
pub type REG_BMX_QOS_BLEM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bmx_qos_emacA` reader - "]
pub type REG_BMX_QOS_EMAC_A_R = crate::BitReader;
#[doc = "Field `reg_bmx_qos_emacA` writer - "]
pub type REG_BMX_QOS_EMAC_A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bmx_qos_sdhm` reader - "]
pub type REG_BMX_QOS_SDHM_R = crate::BitReader;
#[doc = "Field `reg_bmx_qos_sdhm` writer - "]
pub type REG_BMX_QOS_SDHM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bmx_dbg_sel` reader - "]
pub type BMX_DBG_SEL_R = crate::FieldReader;
#[doc = "Field `bmx_dbg_sel` writer - "]
pub type BMX_DBG_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_bmx_berr_int_en(&self) -> REG_BMX_BERR_INT_EN_R {
        REG_BMX_BERR_INT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_mcu_berr_int_en(&self) -> REG_MCU_BERR_INT_EN_R {
        REG_MCU_BERR_INT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_bmx_qos_cpu(&self) -> REG_BMX_QOS_CPU_R {
        REG_BMX_QOS_CPU_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_bmx_qos_sdu(&self) -> REG_BMX_QOS_SDU_R {
        REG_BMX_QOS_SDU_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn reg_bmx_qos_sec0(&self) -> REG_BMX_QOS_SEC0_R {
        REG_BMX_QOS_SEC0_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reg_bmx_qos_sec1(&self) -> REG_BMX_QOS_SEC1_R {
        REG_BMX_QOS_SEC1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_bmx_qos_sec2(&self) -> REG_BMX_QOS_SEC2_R {
        REG_BMX_QOS_SEC2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_bmx_qos_dma(&self) -> REG_BMX_QOS_DMA_R {
        REG_BMX_QOS_DMA_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn reg_bmx_qos_cci(&self) -> REG_BMX_QOS_CCI_R {
        REG_BMX_QOS_CCI_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn reg_bmx_qos_pldma(&self) -> REG_BMX_QOS_PLDMA_R {
        REG_BMX_QOS_PLDMA_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn reg_bmx_qos_blem(&self) -> REG_BMX_QOS_BLEM_R {
        REG_BMX_QOS_BLEM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn reg_bmx_qos_emac_a(&self) -> REG_BMX_QOS_EMAC_A_R {
        REG_BMX_QOS_EMAC_A_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn reg_bmx_qos_sdhm(&self) -> REG_BMX_QOS_SDHM_R {
        REG_BMX_QOS_SDHM_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn bmx_dbg_sel(&self) -> BMX_DBG_SEL_R {
        BMX_DBG_SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_berr_int_en(&mut self) -> REG_BMX_BERR_INT_EN_W<BMX_CFG1_SPEC> {
        REG_BMX_BERR_INT_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu_berr_int_en(&mut self) -> REG_MCU_BERR_INT_EN_W<BMX_CFG1_SPEC> {
        REG_MCU_BERR_INT_EN_W::new(self, 1)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_qos_cpu(&mut self) -> REG_BMX_QOS_CPU_W<BMX_CFG1_SPEC> {
        REG_BMX_QOS_CPU_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_qos_sdu(&mut self) -> REG_BMX_QOS_SDU_W<BMX_CFG1_SPEC> {
        REG_BMX_QOS_SDU_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_qos_sec0(&mut self) -> REG_BMX_QOS_SEC0_W<BMX_CFG1_SPEC> {
        REG_BMX_QOS_SEC0_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_qos_sec1(&mut self) -> REG_BMX_QOS_SEC1_W<BMX_CFG1_SPEC> {
        REG_BMX_QOS_SEC1_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_qos_sec2(&mut self) -> REG_BMX_QOS_SEC2_W<BMX_CFG1_SPEC> {
        REG_BMX_QOS_SEC2_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_qos_dma(&mut self) -> REG_BMX_QOS_DMA_W<BMX_CFG1_SPEC> {
        REG_BMX_QOS_DMA_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_qos_cci(&mut self) -> REG_BMX_QOS_CCI_W<BMX_CFG1_SPEC> {
        REG_BMX_QOS_CCI_W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_qos_pldma(&mut self) -> REG_BMX_QOS_PLDMA_W<BMX_CFG1_SPEC> {
        REG_BMX_QOS_PLDMA_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_qos_blem(&mut self) -> REG_BMX_QOS_BLEM_W<BMX_CFG1_SPEC> {
        REG_BMX_QOS_BLEM_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_qos_emac_a(&mut self) -> REG_BMX_QOS_EMAC_A_W<BMX_CFG1_SPEC> {
        REG_BMX_QOS_EMAC_A_W::new(self, 25)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_qos_sdhm(&mut self) -> REG_BMX_QOS_SDHM_W<BMX_CFG1_SPEC> {
        REG_BMX_QOS_SDHM_W::new(self, 27)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn bmx_dbg_sel(&mut self) -> BMX_DBG_SEL_W<BMX_CFG1_SPEC> {
        BMX_DBG_SEL_W::new(self, 28)
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
#[doc = "bmx_cfg1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmx_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmx_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BMX_CFG1_SPEC;
impl crate::RegisterSpec for BMX_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmx_cfg1::R`](R) reader structure"]
impl crate::Readable for BMX_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bmx_cfg1::W`](W) writer structure"]
impl crate::Writable for BMX_CFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets bmx_cfg1 to value 0"]
impl crate::Resettable for BMX_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
