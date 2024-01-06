#[doc = "Register `interrupt_state` reader"]
pub type R = crate::R<INTERRUPT_STATE_SPEC>;
#[doc = "Field `threshold_low(0-3)` reader - Intenal counter for channel have exceeded low threshold"]
pub type THRESHOLD_LOW_R = crate::BitReader<INTERRUPT_STATE_A>;
#[doc = "Intenal counter for channel have exceeded low threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTERRUPT_STATE_A {
    #[doc = "1: Has interrupt"]
    HAS_INTERRUPT = 1,
    #[doc = "0: No interrupt occurred"]
    NO_INTERRUPT = 0,
}
impl From<INTERRUPT_STATE_A> for bool {
    #[inline(always)]
    fn from(variant: INTERRUPT_STATE_A) -> Self {
        variant as u8 != 0
    }
}
impl THRESHOLD_LOW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INTERRUPT_STATE_A {
        match self.bits {
            true => INTERRUPT_STATE_A::HAS_INTERRUPT,
            false => INTERRUPT_STATE_A::NO_INTERRUPT,
        }
    }
    #[doc = "Has interrupt"]
    #[inline(always)]
    pub fn is_has_interrupt(&self) -> bool {
        *self == INTERRUPT_STATE_A::HAS_INTERRUPT
    }
    #[doc = "No interrupt occurred"]
    #[inline(always)]
    pub fn is_no_interrupt(&self) -> bool {
        *self == INTERRUPT_STATE_A::NO_INTERRUPT
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
#[doc = "Interrupt state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_state::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERRUPT_STATE_SPEC;
impl crate::RegisterSpec for INTERRUPT_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_state::R`](R) reader structure"]
impl crate::Readable for INTERRUPT_STATE_SPEC {}
#[doc = "`reset()` method sets interrupt_state to value 0"]
impl crate::Resettable for INTERRUPT_STATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
