#[doc = "Register `receive_bit_count` reader"]
pub type R = crate::R<RECEIVE_BIT_COUNT_SPEC>;
#[doc = "Register `receive_bit_count` writer"]
pub type W = crate::W<RECEIVE_BIT_COUNT_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RECEIVE_BIT_COUNT_SPEC> {
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
#[doc = "??\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_bit_count::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`receive_bit_count::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RECEIVE_BIT_COUNT_SPEC;
impl crate::RegisterSpec for RECEIVE_BIT_COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`receive_bit_count::R`](R) reader structure"]
impl crate::Readable for RECEIVE_BIT_COUNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`receive_bit_count::W`](W) writer structure"]
impl crate::Writable for RECEIVE_BIT_COUNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets receive_bit_count to value 0"]
impl crate::Resettable for RECEIVE_BIT_COUNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
