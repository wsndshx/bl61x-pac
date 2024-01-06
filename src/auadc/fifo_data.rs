#[doc = "Register `fifo_data` reader"]
pub type R = crate::R<FIFO_DATA_SPEC>;
#[doc = "Register `fifo_data` writer"]
pub type W = crate::W<FIFO_DATA_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<FIFO_DATA_SPEC> {
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
#[doc = "Reads data from audio input FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO_DATA_SPEC;
impl crate::RegisterSpec for FIFO_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_data::R`](R) reader structure"]
impl crate::Readable for FIFO_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifo_data::W`](W) writer structure"]
impl crate::Writable for FIFO_DATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets fifo_data to value 0"]
impl crate::Resettable for FIFO_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
