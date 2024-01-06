#[doc = "Register `audio_pll_cfg6` reader"]
pub type R = crate::R<AUDIO_PLL_CFG6_SPEC>;
#[doc = "Register `audio_pll_cfg6` writer"]
pub type W = crate::W<AUDIO_PLL_CFG6_SPEC>;
#[doc = "Field `aupll_sdmin` reader - "]
pub type AUPLL_SDMIN_R = crate::FieldReader<u32>;
#[doc = "Field `aupll_sdmin` writer - "]
pub type AUPLL_SDMIN_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `aupll_sdm_bypass` reader - "]
pub type AUPLL_SDM_BYPASS_R = crate::BitReader;
#[doc = "Field `aupll_sdm_bypass` writer - "]
pub type AUPLL_SDM_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn aupll_sdmin(&self) -> AUPLL_SDMIN_R {
        AUPLL_SDMIN_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn aupll_sdm_bypass(&self) -> AUPLL_SDM_BYPASS_R {
        AUPLL_SDM_BYPASS_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_sdmin(&mut self) -> AUPLL_SDMIN_W<AUDIO_PLL_CFG6_SPEC> {
        AUPLL_SDMIN_W::new(self, 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_sdm_bypass(&mut self) -> AUPLL_SDM_BYPASS_W<AUDIO_PLL_CFG6_SPEC> {
        AUPLL_SDM_BYPASS_W::new(self, 24)
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
#[doc = "audio_pll_cfg6.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`audio_pll_cfg6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`audio_pll_cfg6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AUDIO_PLL_CFG6_SPEC;
impl crate::RegisterSpec for AUDIO_PLL_CFG6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audio_pll_cfg6::R`](R) reader structure"]
impl crate::Readable for AUDIO_PLL_CFG6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`audio_pll_cfg6::W`](W) writer structure"]
impl crate::Writable for AUDIO_PLL_CFG6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets audio_pll_cfg6 to value 0"]
impl crate::Resettable for AUDIO_PLL_CFG6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
