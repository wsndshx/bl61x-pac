#[doc = "Register `audio_pll_cfg5` reader"]
pub type R = crate::R<AUDIO_PLL_CFG5_SPEC>;
#[doc = "Register `audio_pll_cfg5` writer"]
pub type W = crate::W<AUDIO_PLL_CFG5_SPEC>;
#[doc = "Field `aupll_vco_speed` reader - "]
pub type AUPLL_VCO_SPEED_R = crate::FieldReader;
#[doc = "Field `aupll_vco_speed` writer - "]
pub type AUPLL_VCO_SPEED_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn aupll_vco_speed(&self) -> AUPLL_VCO_SPEED_R {
        AUPLL_VCO_SPEED_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn aupll_vco_speed(&mut self) -> AUPLL_VCO_SPEED_W<AUDIO_PLL_CFG5_SPEC> {
        AUPLL_VCO_SPEED_W::new(self, 0)
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
#[doc = "audio_pll_cfg5.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`audio_pll_cfg5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`audio_pll_cfg5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AUDIO_PLL_CFG5_SPEC;
impl crate::RegisterSpec for AUDIO_PLL_CFG5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audio_pll_cfg5::R`](R) reader structure"]
impl crate::Readable for AUDIO_PLL_CFG5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`audio_pll_cfg5::W`](W) writer structure"]
impl crate::Writable for AUDIO_PLL_CFG5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets audio_pll_cfg5 to value 0"]
impl crate::Resettable for AUDIO_PLL_CFG5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
