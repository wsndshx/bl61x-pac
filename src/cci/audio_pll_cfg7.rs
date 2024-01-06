#[doc = "Register `audio_pll_cfg7` reader"]
pub type R = crate::R<AUDIO_PLL_CFG7_SPEC>;
#[doc = "Register `audio_pll_cfg7` writer"]
pub type W = crate::W<AUDIO_PLL_CFG7_SPEC>;
#[doc = "Field `aupll_sdm_order_sel` reader - "]
pub type AUPLL_SDM_ORDER_SEL_R = crate::BitReader;
#[doc = "Field `aupll_sdm_order_sel` writer - "]
pub type AUPLL_SDM_ORDER_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aupll_sdm_sig_dith_sel` reader - "]
pub type AUPLL_SDM_SIG_DITH_SEL_R = crate::FieldReader;
#[doc = "Field `aupll_sdm_sig_dith_sel` writer - "]
pub type AUPLL_SDM_SIG_DITH_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn aupll_sdm_order_sel(&self) -> AUPLL_SDM_ORDER_SEL_R {
        AUPLL_SDM_ORDER_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn aupll_sdm_sig_dith_sel(&self) -> AUPLL_SDM_SIG_DITH_SEL_R {
        AUPLL_SDM_SIG_DITH_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_sdm_order_sel(&mut self) -> AUPLL_SDM_ORDER_SEL_W<AUDIO_PLL_CFG7_SPEC> {
        AUPLL_SDM_ORDER_SEL_W::new(self, 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_sdm_sig_dith_sel(&mut self) -> AUPLL_SDM_SIG_DITH_SEL_W<AUDIO_PLL_CFG7_SPEC> {
        AUPLL_SDM_SIG_DITH_SEL_W::new(self, 16)
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
#[doc = "audio_pll_cfg7.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`audio_pll_cfg7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`audio_pll_cfg7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AUDIO_PLL_CFG7_SPEC;
impl crate::RegisterSpec for AUDIO_PLL_CFG7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audio_pll_cfg7::R`](R) reader structure"]
impl crate::Readable for AUDIO_PLL_CFG7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`audio_pll_cfg7::W`](W) writer structure"]
impl crate::Writable for AUDIO_PLL_CFG7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets audio_pll_cfg7 to value 0"]
impl crate::Resettable for AUDIO_PLL_CFG7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
