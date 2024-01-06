#[doc = "Register `transmit_control` reader"]
pub type R = crate::R<TRANSMIT_CONTROL_SPEC>;
#[doc = "Register `transmit_control` writer"]
pub type W = crate::W<TRANSMIT_CONTROL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TRANSMIT_CONTROL_SPEC> {
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
#[doc = "Transmit control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transmit_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`transmit_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRANSMIT_CONTROL_SPEC;
impl crate::RegisterSpec for TRANSMIT_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`transmit_control::R`](R) reader structure"]
impl crate::Readable for TRANSMIT_CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`transmit_control::W`](W) writer structure"]
impl crate::Writable for TRANSMIT_CONTROL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets transmit_control to value 0"]
impl crate::Resettable for TRANSMIT_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
