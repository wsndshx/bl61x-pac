#[doc = "Register `tzc_bmx_s0` reader"]
pub type R = crate::R<TZC_BMX_S0_SPEC>;
#[doc = "Register `tzc_bmx_s0` writer"]
pub type W = crate::W<TZC_BMX_S0_SPEC>;
#[doc = "Field `tzc_bmx_dma_tzsid_en` reader - Enable DMA TrustZone Security ID."]
pub type TZC_BMX_DMA_TZSID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_bmx_dma_tzsid_en` writer - Enable DMA TrustZone Security ID."]
pub type TZC_BMX_DMA_TZSID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_bmx_pwr_tzsid_en` reader - Enable Power TrustZone Security ID."]
pub type TZC_BMX_PWR_TZSID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_bmx_pwr_tzsid_en` writer - Enable Power TrustZone Security ID."]
pub type TZC_BMX_PWR_TZSID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_bmx_sdh_tzsid_en` reader - Enable SDH TrustZone Security ID."]
pub type TZC_BMX_SDH_TZSID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_bmx_sdh_tzsid_en` writer - Enable SDH TrustZone Security ID."]
pub type TZC_BMX_SDH_TZSID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_bmx_emac_tzsid_en` reader - Enable EMAC TrustZone Security ID."]
pub type TZC_BMX_EMAC_TZSID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_bmx_emac_tzsid_en` writer - Enable EMAC TrustZone Security ID."]
pub type TZC_BMX_EMAC_TZSID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_bmx_sdu_tzsid_en` reader - Enable SDU TrustZone Security ID."]
pub type TZC_BMX_SDU_TZSID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_bmx_sdu_tzsid_en` writer - Enable SDU TrustZone Security ID."]
pub type TZC_BMX_SDU_TZSID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_bmx_dma_tzsid_lock` reader - Lock DMA TrustZone Security ID."]
pub type TZC_BMX_DMA_TZSID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_bmx_dma_tzsid_lock` writer - Lock DMA TrustZone Security ID."]
pub type TZC_BMX_DMA_TZSID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_bmx_pwr_tzsid_lock` reader - Lock Power TrustZone Security ID."]
pub type TZC_BMX_PWR_TZSID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_bmx_pwr_tzsid_lock` writer - Lock Power TrustZone Security ID."]
pub type TZC_BMX_PWR_TZSID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_bmx_sdh_tzsid_lock` reader - Lock SDH TrustZone Security ID."]
pub type TZC_BMX_SDH_TZSID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_bmx_sdh_tzsid_lock` writer - Lock SDH TrustZone Security ID."]
pub type TZC_BMX_SDH_TZSID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_bmx_emac_tzsid_lock` reader - Lock EMAC TrustZone Security ID."]
pub type TZC_BMX_EMAC_TZSID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_bmx_emac_tzsid_lock` writer - Lock EMAC TrustZone Security ID."]
pub type TZC_BMX_EMAC_TZSID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_bmx_sdu_tzsid_lock` reader - Lock SDU TrustZone Security ID."]
pub type TZC_BMX_SDU_TZSID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_bmx_sdu_tzsid_lock` writer - Lock SDU TrustZone Security ID."]
pub type TZC_BMX_SDU_TZSID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:3 - Enable DMA TrustZone Security ID."]
    #[inline(always)]
    pub fn tzc_bmx_dma_tzsid_en(&self) -> TZC_BMX_DMA_TZSID_EN_R {
        TZC_BMX_DMA_TZSID_EN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Enable Power TrustZone Security ID."]
    #[inline(always)]
    pub fn tzc_bmx_pwr_tzsid_en(&self) -> TZC_BMX_PWR_TZSID_EN_R {
        TZC_BMX_PWR_TZSID_EN_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Enable SDH TrustZone Security ID."]
    #[inline(always)]
    pub fn tzc_bmx_sdh_tzsid_en(&self) -> TZC_BMX_SDH_TZSID_EN_R {
        TZC_BMX_SDH_TZSID_EN_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Enable EMAC TrustZone Security ID."]
    #[inline(always)]
    pub fn tzc_bmx_emac_tzsid_en(&self) -> TZC_BMX_EMAC_TZSID_EN_R {
        TZC_BMX_EMAC_TZSID_EN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Enable SDU TrustZone Security ID."]
    #[inline(always)]
    pub fn tzc_bmx_sdu_tzsid_en(&self) -> TZC_BMX_SDU_TZSID_EN_R {
        TZC_BMX_SDU_TZSID_EN_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 17 - Lock DMA TrustZone Security ID."]
    #[inline(always)]
    pub fn tzc_bmx_dma_tzsid_lock(&self) -> TZC_BMX_DMA_TZSID_LOCK_R {
        TZC_BMX_DMA_TZSID_LOCK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Lock Power TrustZone Security ID."]
    #[inline(always)]
    pub fn tzc_bmx_pwr_tzsid_lock(&self) -> TZC_BMX_PWR_TZSID_LOCK_R {
        TZC_BMX_PWR_TZSID_LOCK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Lock SDH TrustZone Security ID."]
    #[inline(always)]
    pub fn tzc_bmx_sdh_tzsid_lock(&self) -> TZC_BMX_SDH_TZSID_LOCK_R {
        TZC_BMX_SDH_TZSID_LOCK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Lock EMAC TrustZone Security ID."]
    #[inline(always)]
    pub fn tzc_bmx_emac_tzsid_lock(&self) -> TZC_BMX_EMAC_TZSID_LOCK_R {
        TZC_BMX_EMAC_TZSID_LOCK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Lock SDU TrustZone Security ID."]
    #[inline(always)]
    pub fn tzc_bmx_sdu_tzsid_lock(&self) -> TZC_BMX_SDU_TZSID_LOCK_R {
        TZC_BMX_SDU_TZSID_LOCK_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:3 - Enable DMA TrustZone Security ID."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_dma_tzsid_en(&mut self) -> TZC_BMX_DMA_TZSID_EN_W<TZC_BMX_S0_SPEC> {
        TZC_BMX_DMA_TZSID_EN_W::new(self, 2)
    }
    #[doc = "Bits 6:7 - Enable Power TrustZone Security ID."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_pwr_tzsid_en(&mut self) -> TZC_BMX_PWR_TZSID_EN_W<TZC_BMX_S0_SPEC> {
        TZC_BMX_PWR_TZSID_EN_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Enable SDH TrustZone Security ID."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_sdh_tzsid_en(&mut self) -> TZC_BMX_SDH_TZSID_EN_W<TZC_BMX_S0_SPEC> {
        TZC_BMX_SDH_TZSID_EN_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Enable EMAC TrustZone Security ID."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_emac_tzsid_en(&mut self) -> TZC_BMX_EMAC_TZSID_EN_W<TZC_BMX_S0_SPEC> {
        TZC_BMX_EMAC_TZSID_EN_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Enable SDU TrustZone Security ID."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_sdu_tzsid_en(&mut self) -> TZC_BMX_SDU_TZSID_EN_W<TZC_BMX_S0_SPEC> {
        TZC_BMX_SDU_TZSID_EN_W::new(self, 12)
    }
    #[doc = "Bit 17 - Lock DMA TrustZone Security ID."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_dma_tzsid_lock(&mut self) -> TZC_BMX_DMA_TZSID_LOCK_W<TZC_BMX_S0_SPEC> {
        TZC_BMX_DMA_TZSID_LOCK_W::new(self, 17)
    }
    #[doc = "Bit 19 - Lock Power TrustZone Security ID."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_pwr_tzsid_lock(&mut self) -> TZC_BMX_PWR_TZSID_LOCK_W<TZC_BMX_S0_SPEC> {
        TZC_BMX_PWR_TZSID_LOCK_W::new(self, 19)
    }
    #[doc = "Bit 20 - Lock SDH TrustZone Security ID."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_sdh_tzsid_lock(&mut self) -> TZC_BMX_SDH_TZSID_LOCK_W<TZC_BMX_S0_SPEC> {
        TZC_BMX_SDH_TZSID_LOCK_W::new(self, 20)
    }
    #[doc = "Bit 21 - Lock EMAC TrustZone Security ID."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_emac_tzsid_lock(&mut self) -> TZC_BMX_EMAC_TZSID_LOCK_W<TZC_BMX_S0_SPEC> {
        TZC_BMX_EMAC_TZSID_LOCK_W::new(self, 21)
    }
    #[doc = "Bit 22 - Lock SDU TrustZone Security ID."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_bmx_sdu_tzsid_lock(&mut self) -> TZC_BMX_SDU_TZSID_LOCK_W<TZC_BMX_S0_SPEC> {
        TZC_BMX_SDU_TZSID_LOCK_W::new(self, 22)
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
#[doc = "TrustZone Controller BMX Security Register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_bmx_s0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_bmx_s0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_BMX_S0_SPEC;
impl crate::RegisterSpec for TZC_BMX_S0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_bmx_s0::R`](R) reader structure"]
impl crate::Readable for TZC_BMX_S0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tzc_bmx_s0::W`](W) writer structure"]
impl crate::Writable for TZC_BMX_S0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_bmx_s0 to value 0"]
impl crate::Resettable for TZC_BMX_S0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
