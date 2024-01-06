#[doc = "Register `interrupt_clear` writer"]
pub type W = crate::W<INTERRUPT_CLEAR_SPEC>;
#[doc = "Intenal counter for channel have exceeded low threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTERRUPT_CLEAR_AW {
    #[doc = "1: Write 1 to clear interrupt state"]
    CLEAR = 1,
}
impl From<INTERRUPT_CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: INTERRUPT_CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `threshold_low(0-3)` writer - Intenal counter for channel have exceeded low threshold"]
pub type THRESHOLD_LOW_W<'a, REG> = crate::BitWriter<'a, REG, INTERRUPT_CLEAR_AW>;
impl<'a, REG> THRESHOLD_LOW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write 1 to clear interrupt state"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_CLEAR_AW::CLEAR)
    }
}
#[doc = "Field `threshold_high(0-3)` writer - Intenal counter for channel have exceeded high threshold"]
pub use THRESHOLD_LOW_W as THRESHOLD_HIGH_W;
#[doc = "Field `interrupt_period` writer - Intenal counter for channel have exceeded interrupt cycle threshold"]
pub use THRESHOLD_LOW_W as INTERRUPT_PERIOD_W;
#[doc = "Field `external_break` writer - External break signal occurred"]
pub use THRESHOLD_LOW_W as EXTERNAL_BREAK_W;
#[doc = "Field `repeat` writer - Peripheral group have completed one repeat cycle"]
pub use THRESHOLD_LOW_W as REPEAT_W;
impl W {
    #[doc = "Intenal counter for channel have exceeded low threshold"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `threshold_low0` field"]
    #[inline(always)]
    #[must_use]
    pub fn threshold_low(&mut self, n: u8) -> THRESHOLD_LOW_W<INTERRUPT_CLEAR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        THRESHOLD_LOW_W::new(self, n * 2)
    }
    #[doc = "Bit 0 - Intenal counter for channel have exceeded low threshold"]
    #[inline(always)]
    #[must_use]
    pub fn threshold_low0(&mut self) -> THRESHOLD_LOW_W<INTERRUPT_CLEAR_SPEC> {
        THRESHOLD_LOW_W::new(self, 0)
    }
    #[doc = "Bit 2 - Intenal counter for channel have exceeded low threshold"]
    #[inline(always)]
    #[must_use]
    pub fn threshold_low1(&mut self) -> THRESHOLD_LOW_W<INTERRUPT_CLEAR_SPEC> {
        THRESHOLD_LOW_W::new(self, 2)
    }
    #[doc = "Bit 4 - Intenal counter for channel have exceeded low threshold"]
    #[inline(always)]
    #[must_use]
    pub fn threshold_low2(&mut self) -> THRESHOLD_LOW_W<INTERRUPT_CLEAR_SPEC> {
        THRESHOLD_LOW_W::new(self, 4)
    }
    #[doc = "Bit 6 - Intenal counter for channel have exceeded low threshold"]
    #[inline(always)]
    #[must_use]
    pub fn threshold_low3(&mut self) -> THRESHOLD_LOW_W<INTERRUPT_CLEAR_SPEC> {
        THRESHOLD_LOW_W::new(self, 6)
    }
    #[doc = "Intenal counter for channel have exceeded high threshold"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `threshold_high0` field"]
    #[inline(always)]
    #[must_use]
    pub fn threshold_high(&mut self, n: u8) -> THRESHOLD_HIGH_W<INTERRUPT_CLEAR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        THRESHOLD_HIGH_W::new(self, n * 2 + 1)
    }
    #[doc = "Bit 1 - Intenal counter for channel have exceeded high threshold"]
    #[inline(always)]
    #[must_use]
    pub fn threshold_high0(&mut self) -> THRESHOLD_HIGH_W<INTERRUPT_CLEAR_SPEC> {
        THRESHOLD_HIGH_W::new(self, 1)
    }
    #[doc = "Bit 3 - Intenal counter for channel have exceeded high threshold"]
    #[inline(always)]
    #[must_use]
    pub fn threshold_high1(&mut self) -> THRESHOLD_HIGH_W<INTERRUPT_CLEAR_SPEC> {
        THRESHOLD_HIGH_W::new(self, 3)
    }
    #[doc = "Bit 5 - Intenal counter for channel have exceeded high threshold"]
    #[inline(always)]
    #[must_use]
    pub fn threshold_high2(&mut self) -> THRESHOLD_HIGH_W<INTERRUPT_CLEAR_SPEC> {
        THRESHOLD_HIGH_W::new(self, 5)
    }
    #[doc = "Bit 7 - Intenal counter for channel have exceeded high threshold"]
    #[inline(always)]
    #[must_use]
    pub fn threshold_high3(&mut self) -> THRESHOLD_HIGH_W<INTERRUPT_CLEAR_SPEC> {
        THRESHOLD_HIGH_W::new(self, 7)
    }
    #[doc = "Bit 8 - Intenal counter for channel have exceeded interrupt cycle threshold"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_period(&mut self) -> INTERRUPT_PERIOD_W<INTERRUPT_CLEAR_SPEC> {
        INTERRUPT_PERIOD_W::new(self, 8)
    }
    #[doc = "Bit 9 - External break signal occurred"]
    #[inline(always)]
    #[must_use]
    pub fn external_break(&mut self) -> EXTERNAL_BREAK_W<INTERRUPT_CLEAR_SPEC> {
        EXTERNAL_BREAK_W::new(self, 9)
    }
    #[doc = "Bit 10 - Peripheral group have completed one repeat cycle"]
    #[inline(always)]
    #[must_use]
    pub fn repeat(&mut self) -> REPEAT_W<INTERRUPT_CLEAR_SPEC> {
        REPEAT_W::new(self, 10)
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
#[doc = "Clear interrupt register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_clear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERRUPT_CLEAR_SPEC;
impl crate::RegisterSpec for INTERRUPT_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`interrupt_clear::W`](W) writer structure"]
impl crate::Writable for INTERRUPT_CLEAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets interrupt_clear to value 0"]
impl crate::Resettable for INTERRUPT_CLEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
