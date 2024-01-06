#[doc = "Register `period` reader"]
pub type R = crate::R<PERIOD_SPEC>;
#[doc = "Register `period` writer"]
pub type W = crate::W<PERIOD_SPEC>;
#[doc = "Field `interrupt_cycles` reader - If internal counter reaches this cycle count, it interrupts"]
pub type INTERRUPT_CYCLES_R = crate::FieldReader<u16>;
#[doc = "Field `interrupt_cycles` writer - If internal counter reaches this cycle count, it interrupts"]
pub type INTERRUPT_CYCLES_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `repeat_cycles` reader - How many clock cycles a Pulse-Width Modulation cycle includes"]
pub type REPEAT_CYCLES_R = crate::FieldReader<u16>;
#[doc = "Field `repeat_cycles` writer - How many clock cycles a Pulse-Width Modulation cycle includes"]
pub type REPEAT_CYCLES_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - If internal counter reaches this cycle count, it interrupts"]
    #[inline(always)]
    pub fn interrupt_cycles(&self) -> INTERRUPT_CYCLES_R {
        INTERRUPT_CYCLES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - How many clock cycles a Pulse-Width Modulation cycle includes"]
    #[inline(always)]
    pub fn repeat_cycles(&self) -> REPEAT_CYCLES_R {
        REPEAT_CYCLES_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - If internal counter reaches this cycle count, it interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_cycles(&mut self) -> INTERRUPT_CYCLES_W<PERIOD_SPEC> {
        INTERRUPT_CYCLES_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - How many clock cycles a Pulse-Width Modulation cycle includes"]
    #[inline(always)]
    #[must_use]
    pub fn repeat_cycles(&mut self) -> REPEAT_CYCLES_W<PERIOD_SPEC> {
        REPEAT_CYCLES_W::new(self, 16)
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
#[doc = "Pulse clock period register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`period::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`period::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERIOD_SPEC;
impl crate::RegisterSpec for PERIOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`period::R`](R) reader structure"]
impl crate::Readable for PERIOD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`period::W`](W) writer structure"]
impl crate::Writable for PERIOD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets period to value 0"]
impl crate::Resettable for PERIOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
