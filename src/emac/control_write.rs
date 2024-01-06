#[doc = "Register `control_write` reader"]
pub type R = crate::R<CONTROL_WRITE_SPEC>;
#[doc = "Register `control_write` writer"]
pub type W = crate::W<CONTROL_WRITE_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CONTROL_WRITE_SPEC> {
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
#[doc = "Write data to MII physcial layer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control_write::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control_write::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONTROL_WRITE_SPEC;
impl crate::RegisterSpec for CONTROL_WRITE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control_write::R`](R) reader structure"]
impl crate::Readable for CONTROL_WRITE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`control_write::W`](W) writer structure"]
impl crate::Writable for CONTROL_WRITE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets control_write to value 0"]
impl crate::Resettable for CONTROL_WRITE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
