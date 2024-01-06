#[doc = "Register `interrupt_enable` reader"]
pub type R = crate::R<INTERRUPT_ENABLE_SPEC>;
#[doc = "Register `interrupt_enable` writer"]
pub type W = crate::W<INTERRUPT_ENABLE_SPEC>;
#[doc = "Field `threshold_low(0-3)` reader - Intenal counter for channel have exceeded low threshold"]
pub type THRESHOLD_LOW_R = crate::BitReader<INTERRUPT_ENABLE_A>;
#[doc = "Intenal counter for channel have exceeded low threshold\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTERRUPT_ENABLE_A {
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
}
impl From<INTERRUPT_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: INTERRUPT_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl THRESHOLD_LOW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INTERRUPT_ENABLE_A {
        match self.bits {
            true => INTERRUPT_ENABLE_A::ENABLE,
            false => INTERRUPT_ENABLE_A::DISABLE,
        }
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == INTERRUPT_ENABLE_A::ENABLE
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == INTERRUPT_ENABLE_A::DISABLE
    }
}
#[doc = "Field `threshold_low(0-3)` writer - Intenal counter for channel have exceeded low threshold"]
pub type THRESHOLD_LOW_W<'a, REG> = crate::BitWriter<'a, REG, INTERRUPT_ENABLE_A>;
impl<'a, REG> THRESHOLD_LOW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_ENABLE_A::ENABLE)
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_ENABLE_A::DISABLE)
    }
}
#[doc = "Field `threshold_high(0-3)` reader - Intenal counter for channel have exceeded high threshold"]
pub use THRESHOLD_LOW_R as THRESHOLD_HIGH_R;
#[doc = "Field `interrupt_period` reader - Intenal counter for channel have exceeded interrupt cycle threshold"]
pub use THRESHOLD_LOW_R as INTERRUPT_PERIOD_R;
#[doc = "Field `external_break` reader - External break signal occurred"]
pub use THRESHOLD_LOW_R as EXTERNAL_BREAK_R;
#[doc = "Field `repeat` reader - Peripheral group have completed one repeat cycle"]
pub use THRESHOLD_LOW_R as REPEAT_R;
#[doc = "Field `threshold_high(0-3)` writer - Intenal counter for channel have exceeded high threshold"]
pub use THRESHOLD_LOW_W as THRESHOLD_HIGH_W;
#[doc = "Field `interrupt_period` writer - Intenal counter for channel have exceeded interrupt cycle threshold"]
pub use THRESHOLD_LOW_W as INTERRUPT_PERIOD_W;
#[doc = "Field `external_break` writer - External break signal occurred"]
pub use THRESHOLD_LOW_W as EXTERNAL_BREAK_W;
#[doc = "Field `repeat` writer - Peripheral group have completed one repeat cycle"]
pub use THRESHOLD_LOW_W as REPEAT_W;
impl R {
    #[doc = "Intenal counter for channel have exceeded low threshold"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `threshold_low0` field"]
    #[inline(always)]
    pub fn threshold_low(&self, n: u8) -> THRESHOLD_LOW_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        THRESHOLD_LOW_R::new(((self.bits >> (n * 2)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Intenal counter for channel have exceeded low threshold"]
    #[inline(always)]
    pub fn threshold_low_iter(&self) -> impl Iterator<Item = THRESHOLD_LOW_R> + '_ {
        (0..4).map(move |n| THRESHOLD_LOW_R::new(((self.bits >> (n * 2)) & 1) != 0))
    }
    #[doc = "Bit 0 - Intenal counter for channel have exceeded low threshold"]
    #[inline(always)]
    pub fn threshold_low0(&self) -> THRESHOLD_LOW_R {
        THRESHOLD_LOW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Intenal counter for channel have exceeded low threshold"]
    #[inline(always)]
    pub fn threshold_low1(&self) -> THRESHOLD_LOW_R {
        THRESHOLD_LOW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Intenal counter for channel have exceeded low threshold"]
    #[inline(always)]
    pub fn threshold_low2(&self) -> THRESHOLD_LOW_R {
        THRESHOLD_LOW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Intenal counter for channel have exceeded low threshold"]
    #[inline(always)]
    pub fn threshold_low3(&self) -> THRESHOLD_LOW_R {
        THRESHOLD_LOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Intenal counter for channel have exceeded high threshold"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `threshold_high0` field"]
    #[inline(always)]
    pub fn threshold_high(&self, n: u8) -> THRESHOLD_HIGH_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        THRESHOLD_HIGH_R::new(((self.bits >> (n * 2 + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Intenal counter for channel have exceeded high threshold"]
    #[inline(always)]
    pub fn threshold_high_iter(&self) -> impl Iterator<Item = THRESHOLD_HIGH_R> + '_ {
        (0..4).map(move |n| THRESHOLD_HIGH_R::new(((self.bits >> (n * 2 + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - Intenal counter for channel have exceeded high threshold"]
    #[inline(always)]
    pub fn threshold_high0(&self) -> THRESHOLD_HIGH_R {
        THRESHOLD_HIGH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Intenal counter for channel have exceeded high threshold"]
    #[inline(always)]
    pub fn threshold_high1(&self) -> THRESHOLD_HIGH_R {
        THRESHOLD_HIGH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Intenal counter for channel have exceeded high threshold"]
    #[inline(always)]
    pub fn threshold_high2(&self) -> THRESHOLD_HIGH_R {
        THRESHOLD_HIGH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Intenal counter for channel have exceeded high threshold"]
    #[inline(always)]
    pub fn threshold_high3(&self) -> THRESHOLD_HIGH_R {
        THRESHOLD_HIGH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Intenal counter for channel have exceeded interrupt cycle threshold"]
    #[inline(always)]
    pub fn interrupt_period(&self) -> INTERRUPT_PERIOD_R {
        INTERRUPT_PERIOD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - External break signal occurred"]
    #[inline(always)]
    pub fn external_break(&self) -> EXTERNAL_BREAK_R {
        EXTERNAL_BREAK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Peripheral group have completed one repeat cycle"]
    #[inline(always)]
    pub fn repeat(&self) -> REPEAT_R {
        REPEAT_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Intenal counter for channel have exceeded low threshold"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `threshold_low0` field"]
    #[inline(always)]
    #[must_use]
    pub fn threshold_low(&mut self, n: u8) -> THRESHOLD_LOW_W<INTERRUPT_ENABLE_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        THRESHOLD_LOW_W::new(self, n * 2)
    }
    #[doc = "Bit 0 - Intenal counter for channel have exceeded low threshold"]
    #[inline(always)]
    #[must_use]
    pub fn threshold_low0(&mut self) -> THRESHOLD_LOW_W<INTERRUPT_ENABLE_SPEC> {
        THRESHOLD_LOW_W::new(self, 0)
    }
    #[doc = "Bit 2 - Intenal counter for channel have exceeded low threshold"]
    #[inline(always)]
    #[must_use]
    pub fn threshold_low1(&mut self) -> THRESHOLD_LOW_W<INTERRUPT_ENABLE_SPEC> {
        THRESHOLD_LOW_W::new(self, 2)
    }
    #[doc = "Bit 4 - Intenal counter for channel have exceeded low threshold"]
    #[inline(always)]
    #[must_use]
    pub fn threshold_low2(&mut self) -> THRESHOLD_LOW_W<INTERRUPT_ENABLE_SPEC> {
        THRESHOLD_LOW_W::new(self, 4)
    }
    #[doc = "Bit 6 - Intenal counter for channel have exceeded low threshold"]
    #[inline(always)]
    #[must_use]
    pub fn threshold_low3(&mut self) -> THRESHOLD_LOW_W<INTERRUPT_ENABLE_SPEC> {
        THRESHOLD_LOW_W::new(self, 6)
    }
    #[doc = "Intenal counter for channel have exceeded high threshold"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `threshold_high0` field"]
    #[inline(always)]
    #[must_use]
    pub fn threshold_high(&mut self, n: u8) -> THRESHOLD_HIGH_W<INTERRUPT_ENABLE_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        THRESHOLD_HIGH_W::new(self, n * 2 + 1)
    }
    #[doc = "Bit 1 - Intenal counter for channel have exceeded high threshold"]
    #[inline(always)]
    #[must_use]
    pub fn threshold_high0(&mut self) -> THRESHOLD_HIGH_W<INTERRUPT_ENABLE_SPEC> {
        THRESHOLD_HIGH_W::new(self, 1)
    }
    #[doc = "Bit 3 - Intenal counter for channel have exceeded high threshold"]
    #[inline(always)]
    #[must_use]
    pub fn threshold_high1(&mut self) -> THRESHOLD_HIGH_W<INTERRUPT_ENABLE_SPEC> {
        THRESHOLD_HIGH_W::new(self, 3)
    }
    #[doc = "Bit 5 - Intenal counter for channel have exceeded high threshold"]
    #[inline(always)]
    #[must_use]
    pub fn threshold_high2(&mut self) -> THRESHOLD_HIGH_W<INTERRUPT_ENABLE_SPEC> {
        THRESHOLD_HIGH_W::new(self, 5)
    }
    #[doc = "Bit 7 - Intenal counter for channel have exceeded high threshold"]
    #[inline(always)]
    #[must_use]
    pub fn threshold_high3(&mut self) -> THRESHOLD_HIGH_W<INTERRUPT_ENABLE_SPEC> {
        THRESHOLD_HIGH_W::new(self, 7)
    }
    #[doc = "Bit 8 - Intenal counter for channel have exceeded interrupt cycle threshold"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_period(&mut self) -> INTERRUPT_PERIOD_W<INTERRUPT_ENABLE_SPEC> {
        INTERRUPT_PERIOD_W::new(self, 8)
    }
    #[doc = "Bit 9 - External break signal occurred"]
    #[inline(always)]
    #[must_use]
    pub fn external_break(&mut self) -> EXTERNAL_BREAK_W<INTERRUPT_ENABLE_SPEC> {
        EXTERNAL_BREAK_W::new(self, 9)
    }
    #[doc = "Bit 10 - Peripheral group have completed one repeat cycle"]
    #[inline(always)]
    #[must_use]
    pub fn repeat(&mut self) -> REPEAT_W<INTERRUPT_ENABLE_SPEC> {
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
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERRUPT_ENABLE_SPEC;
impl crate::RegisterSpec for INTERRUPT_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_enable::R`](R) reader structure"]
impl crate::Readable for INTERRUPT_ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interrupt_enable::W`](W) writer structure"]
impl crate::Writable for INTERRUPT_ENABLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets interrupt_enable to value 0x7f"]
impl crate::Resettable for INTERRUPT_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f;
}
