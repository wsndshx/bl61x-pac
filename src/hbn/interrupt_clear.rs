#[doc = "Register `interrupt_clear` reader"]
pub type R = crate::R<INTERRUPT_CLEAR_SPEC>;
#[doc = "Register `interrupt_clear` writer"]
pub type W = crate::W<INTERRUPT_CLEAR_SPEC>;
#[doc = "Field `irq_clr` reader - "]
pub type IRQ_CLR_R = crate::FieldReader<u32>;
#[doc = "Field `irq_clr` writer - "]
pub type IRQ_CLR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn irq_clr(&self) -> IRQ_CLR_R {
        IRQ_CLR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn irq_clr(&mut self) -> IRQ_CLR_W<INTERRUPT_CLEAR_SPEC> {
        IRQ_CLR_W::new(self, 0)
    }
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
#[doc = "Clear hibernate interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_clear::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_clear::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERRUPT_CLEAR_SPEC;
impl crate::RegisterSpec for INTERRUPT_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_clear::R`](R) reader structure"]
impl crate::Readable for INTERRUPT_CLEAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interrupt_clear::W`](W) writer structure"]
impl crate::Writable for INTERRUPT_CLEAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets interrupt_clear to value 0"]
impl crate::Resettable for INTERRUPT_CLEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
