#[doc = "Register `volume_1` reader"]
pub type R = crate::R<VOLUME_1_SPEC>;
#[doc = "Register `volume_1` writer"]
pub type W = crate::W<VOLUME_1_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<VOLUME_1_SPEC> {
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
#[doc = "Volume control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`volume_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`volume_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VOLUME_1_SPEC;
impl crate::RegisterSpec for VOLUME_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`volume_1::R`](R) reader structure"]
impl crate::Readable for VOLUME_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`volume_1::W`](W) writer structure"]
impl crate::Writable for VOLUME_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets volume_1 to value 0"]
impl crate::Resettable for VOLUME_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
