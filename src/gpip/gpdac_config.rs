#[doc = "Register `gpdac_config` reader"]
pub type R = crate::R<GPDAC_CONFIG_SPEC>;
#[doc = "Register `gpdac_config` writer"]
pub type W = crate::W<GPDAC_CONFIG_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<GPDAC_CONFIG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
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
#[doc = "Generic Digital-to-Analog Converter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpdac_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpdac_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPDAC_CONFIG_SPEC;
impl crate::RegisterSpec for GPDAC_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpdac_config::R`](R) reader structure"]
impl crate::Readable for GPDAC_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpdac_config::W`](W) writer structure"]
impl crate::Writable for GPDAC_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpdac_config to value 0"]
impl crate::Resettable for GPDAC_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
