#[doc = "Register `pulse_tolerance` reader"]
pub type R = crate::R<PULSE_TOLERANCE_SPEC>;
#[doc = "Register `pulse_tolerance` writer"]
pub type W = crate::W<PULSE_TOLERANCE_SPEC>;
#[doc = "Field `by_five_five` reader - Pulse width tolerance of auto baudrate detection using codeword 0x55"]
pub type BY_FIVE_FIVE_R = crate::FieldReader;
#[doc = "Field `by_five_five` writer - Pulse width tolerance of auto baudrate detection using codeword 0x55"]
pub type BY_FIVE_FIVE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Pulse width tolerance of auto baudrate detection using codeword 0x55"]
    #[inline(always)]
    pub fn by_five_five(&self) -> BY_FIVE_FIVE_R {
        BY_FIVE_FIVE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Pulse width tolerance of auto baudrate detection using codeword 0x55"]
    #[inline(always)]
    #[must_use]
    pub fn by_five_five(&mut self) -> BY_FIVE_FIVE_W<PULSE_TOLERANCE_SPEC> {
        BY_FIVE_FIVE_W::new(self, 0)
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
#[doc = "Pulse width tolerance for auto baudrate\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pulse_tolerance::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pulse_tolerance::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PULSE_TOLERANCE_SPEC;
impl crate::RegisterSpec for PULSE_TOLERANCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pulse_tolerance::R`](R) reader structure"]
impl crate::Readable for PULSE_TOLERANCE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pulse_tolerance::W`](W) writer structure"]
impl crate::Writable for PULSE_TOLERANCE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pulse_tolerance to value 0x03"]
impl crate::Resettable for PULSE_TOLERANCE_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
