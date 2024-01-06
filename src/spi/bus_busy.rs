#[doc = "Register `bus_busy` reader"]
pub type R = crate::R<BUS_BUSY_SPEC>;
#[doc = "Register `bus_busy` writer"]
pub type W = crate::W<BUS_BUSY_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<BUS_BUSY_SPEC> {
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
#[doc = "Bus busy state indicator\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_busy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_busy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUS_BUSY_SPEC;
impl crate::RegisterSpec for BUS_BUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_busy::R`](R) reader structure"]
impl crate::Readable for BUS_BUSY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bus_busy::W`](W) writer structure"]
impl crate::Writable for BUS_BUSY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets bus_busy to value 0"]
impl crate::Resettable for BUS_BUSY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
