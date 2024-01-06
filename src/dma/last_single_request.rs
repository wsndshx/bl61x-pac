#[doc = "Register `last_single_request` reader"]
pub type R = crate::R<LAST_SINGLE_REQUEST_SPEC>;
#[doc = "Register `last_single_request` writer"]
pub type W = crate::W<LAST_SINGLE_REQUEST_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<LAST_SINGLE_REQUEST_SPEC> {
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
#[doc = "??\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`last_single_request::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`last_single_request::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LAST_SINGLE_REQUEST_SPEC;
impl crate::RegisterSpec for LAST_SINGLE_REQUEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`last_single_request::R`](R) reader structure"]
impl crate::Readable for LAST_SINGLE_REQUEST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`last_single_request::W`](W) writer structure"]
impl crate::Writable for LAST_SINGLE_REQUEST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets last_single_request to value 0"]
impl crate::Resettable for LAST_SINGLE_REQUEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
