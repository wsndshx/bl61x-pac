#[doc = "Register `pulse_width` reader"]
pub type R = crate::R<PULSE_WIDTH_SPEC>;
#[doc = "Register `pulse_width` writer"]
pub type W = crate::W<PULSE_WIDTH_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<PULSE_WIDTH_SPEC> {
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
#[doc = "Pulse-Width Modulator control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pulse_width::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pulse_width::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PULSE_WIDTH_SPEC;
impl crate::RegisterSpec for PULSE_WIDTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pulse_width::R`](R) reader structure"]
impl crate::Readable for PULSE_WIDTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pulse_width::W`](W) writer structure"]
impl crate::Writable for PULSE_WIDTH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pulse_width to value 0"]
impl crate::Resettable for PULSE_WIDTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
