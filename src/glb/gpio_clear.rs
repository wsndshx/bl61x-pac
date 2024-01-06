#[doc = "Register `gpio_clear[%s]` reader"]
pub type R = crate::R<GPIO_CLEAR_SPEC>;
#[doc = "Register `gpio_clear[%s]` writer"]
pub type W = crate::W<GPIO_CLEAR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<GPIO_CLEAR_SPEC> {
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
#[doc = "Set pin output value to low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_clear::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_clear::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_CLEAR_SPEC;
impl crate::RegisterSpec for GPIO_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_clear::R`](R) reader structure"]
impl crate::Readable for GPIO_CLEAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_clear::W`](W) writer structure"]
impl crate::Writable for GPIO_CLEAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpio_clear[%s]
to value 0"]
impl crate::Resettable for GPIO_CLEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
