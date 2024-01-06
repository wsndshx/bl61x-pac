#[doc = "Register `channel` reader"]
pub type R = crate::R<CHANNEL_SPEC>;
#[doc = "Register `channel` writer"]
pub type W = crate::W<CHANNEL_SPEC>;
#[doc = "Field `positive_signal(0-3)` reader - Enable or disable positive signal for channel"]
pub type POSITIVE_SIGNAL_R = crate::BitReader;
#[doc = "Field `positive_signal(0-3)` writer - Enable or disable positive signal for channel"]
pub type POSITIVE_SIGNAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `positive_idle(0-3)` reader - Idle state of positive signal"]
pub type POSITIVE_IDLE_R = crate::BitReader;
#[doc = "Field `positive_idle(0-3)` writer - Idle state of positive signal"]
pub type POSITIVE_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `negative_signal(0-3)` reader - Enable or disable negative signal for channel"]
pub type NEGATIVE_SIGNAL_R = crate::BitReader;
#[doc = "Field `negative_signal(0-3)` writer - Enable or disable negative signal for channel"]
pub type NEGATIVE_SIGNAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `negative_idle(0-3)` reader - Idle state of negative signal"]
pub type NEGATIVE_IDLE_R = crate::BitReader;
#[doc = "Field `negative_idle(0-3)` writer - Idle state of negative signal"]
pub type NEGATIVE_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `positive_polarity(0-3)` reader - Polarity of positive signal"]
pub type POSITIVE_POLARITY_R = crate::BitReader;
#[doc = "Field `positive_polarity(0-3)` writer - Polarity of positive signal"]
pub type POSITIVE_POLARITY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `negative_polarity(0-3)` reader - Polarity of negative signal"]
pub type NEGATIVE_POLARITY_R = crate::BitReader;
#[doc = "Field `negative_polarity(0-3)` writer - Polarity of negative signal"]
pub type NEGATIVE_POLARITY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `positive_break(0-3)` reader - Break state on positive signal of this channel"]
pub type POSITIVE_BREAK_R = crate::BitReader;
#[doc = "Field `positive_break(0-3)` writer - Break state on positive signal of this channel"]
pub type POSITIVE_BREAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `negative_break(0-3)` reader - Break state on negative signal of this channel"]
pub type NEGATIVE_BREAK_R = crate::BitReader;
#[doc = "Field `negative_break(0-3)` writer - Break state on negative signal of this channel"]
pub type NEGATIVE_BREAK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Enable or disable positive signal for channel"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `positive_signal0` field"]
    #[inline(always)]
    pub fn positive_signal(&self, n: u8) -> POSITIVE_SIGNAL_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        POSITIVE_SIGNAL_R::new(((self.bits >> (n * 4)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Enable or disable positive signal for channel"]
    #[inline(always)]
    pub fn positive_signal_iter(&self) -> impl Iterator<Item = POSITIVE_SIGNAL_R> + '_ {
        (0..4).map(move |n| POSITIVE_SIGNAL_R::new(((self.bits >> (n * 4)) & 1) != 0))
    }
    #[doc = "Bit 0 - Enable or disable positive signal for channel"]
    #[inline(always)]
    pub fn positive_signal0(&self) -> POSITIVE_SIGNAL_R {
        POSITIVE_SIGNAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Enable or disable positive signal for channel"]
    #[inline(always)]
    pub fn positive_signal1(&self) -> POSITIVE_SIGNAL_R {
        POSITIVE_SIGNAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable or disable positive signal for channel"]
    #[inline(always)]
    pub fn positive_signal2(&self) -> POSITIVE_SIGNAL_R {
        POSITIVE_SIGNAL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable or disable positive signal for channel"]
    #[inline(always)]
    pub fn positive_signal3(&self) -> POSITIVE_SIGNAL_R {
        POSITIVE_SIGNAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Idle state of positive signal"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `positive_idle0` field"]
    #[inline(always)]
    pub fn positive_idle(&self, n: u8) -> POSITIVE_IDLE_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        POSITIVE_IDLE_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Idle state of positive signal"]
    #[inline(always)]
    pub fn positive_idle_iter(&self) -> impl Iterator<Item = POSITIVE_IDLE_R> + '_ {
        (0..4).map(move |n| POSITIVE_IDLE_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - Idle state of positive signal"]
    #[inline(always)]
    pub fn positive_idle0(&self) -> POSITIVE_IDLE_R {
        POSITIVE_IDLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Idle state of positive signal"]
    #[inline(always)]
    pub fn positive_idle1(&self) -> POSITIVE_IDLE_R {
        POSITIVE_IDLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - Idle state of positive signal"]
    #[inline(always)]
    pub fn positive_idle2(&self) -> POSITIVE_IDLE_R {
        POSITIVE_IDLE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - Idle state of positive signal"]
    #[inline(always)]
    pub fn positive_idle3(&self) -> POSITIVE_IDLE_R {
        POSITIVE_IDLE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Enable or disable negative signal for channel"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `negative_signal0` field"]
    #[inline(always)]
    pub fn negative_signal(&self, n: u8) -> NEGATIVE_SIGNAL_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        NEGATIVE_SIGNAL_R::new(((self.bits >> (n * 4 + 2)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Enable or disable negative signal for channel"]
    #[inline(always)]
    pub fn negative_signal_iter(&self) -> impl Iterator<Item = NEGATIVE_SIGNAL_R> + '_ {
        (0..4).map(move |n| NEGATIVE_SIGNAL_R::new(((self.bits >> (n * 4 + 2)) & 1) != 0))
    }
    #[doc = "Bit 2 - Enable or disable negative signal for channel"]
    #[inline(always)]
    pub fn negative_signal0(&self) -> NEGATIVE_SIGNAL_R {
        NEGATIVE_SIGNAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable or disable negative signal for channel"]
    #[inline(always)]
    pub fn negative_signal1(&self) -> NEGATIVE_SIGNAL_R {
        NEGATIVE_SIGNAL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable or disable negative signal for channel"]
    #[inline(always)]
    pub fn negative_signal2(&self) -> NEGATIVE_SIGNAL_R {
        NEGATIVE_SIGNAL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable or disable negative signal for channel"]
    #[inline(always)]
    pub fn negative_signal3(&self) -> NEGATIVE_SIGNAL_R {
        NEGATIVE_SIGNAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Idle state of negative signal"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `negative_idle0` field"]
    #[inline(always)]
    pub fn negative_idle(&self, n: u8) -> NEGATIVE_IDLE_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        NEGATIVE_IDLE_R::new(((self.bits >> (n * 4 + 3)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Idle state of negative signal"]
    #[inline(always)]
    pub fn negative_idle_iter(&self) -> impl Iterator<Item = NEGATIVE_IDLE_R> + '_ {
        (0..4).map(move |n| NEGATIVE_IDLE_R::new(((self.bits >> (n * 4 + 3)) & 1) != 0))
    }
    #[doc = "Bit 3 - Idle state of negative signal"]
    #[inline(always)]
    pub fn negative_idle0(&self) -> NEGATIVE_IDLE_R {
        NEGATIVE_IDLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Idle state of negative signal"]
    #[inline(always)]
    pub fn negative_idle1(&self) -> NEGATIVE_IDLE_R {
        NEGATIVE_IDLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - Idle state of negative signal"]
    #[inline(always)]
    pub fn negative_idle2(&self) -> NEGATIVE_IDLE_R {
        NEGATIVE_IDLE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Idle state of negative signal"]
    #[inline(always)]
    pub fn negative_idle3(&self) -> NEGATIVE_IDLE_R {
        NEGATIVE_IDLE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Polarity of positive signal"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `positive_polarity0` field"]
    #[inline(always)]
    pub fn positive_polarity(&self, n: u8) -> POSITIVE_POLARITY_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        POSITIVE_POLARITY_R::new(((self.bits >> (n * 2 + 16)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Polarity of positive signal"]
    #[inline(always)]
    pub fn positive_polarity_iter(&self) -> impl Iterator<Item = POSITIVE_POLARITY_R> + '_ {
        (0..4).map(move |n| POSITIVE_POLARITY_R::new(((self.bits >> (n * 2 + 16)) & 1) != 0))
    }
    #[doc = "Bit 16 - Polarity of positive signal"]
    #[inline(always)]
    pub fn positive_polarity0(&self) -> POSITIVE_POLARITY_R {
        POSITIVE_POLARITY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Polarity of positive signal"]
    #[inline(always)]
    pub fn positive_polarity1(&self) -> POSITIVE_POLARITY_R {
        POSITIVE_POLARITY_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Polarity of positive signal"]
    #[inline(always)]
    pub fn positive_polarity2(&self) -> POSITIVE_POLARITY_R {
        POSITIVE_POLARITY_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Polarity of positive signal"]
    #[inline(always)]
    pub fn positive_polarity3(&self) -> POSITIVE_POLARITY_R {
        POSITIVE_POLARITY_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Polarity of negative signal"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `negative_polarity0` field"]
    #[inline(always)]
    pub fn negative_polarity(&self, n: u8) -> NEGATIVE_POLARITY_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        NEGATIVE_POLARITY_R::new(((self.bits >> (n * 2 + 17)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Polarity of negative signal"]
    #[inline(always)]
    pub fn negative_polarity_iter(&self) -> impl Iterator<Item = NEGATIVE_POLARITY_R> + '_ {
        (0..4).map(move |n| NEGATIVE_POLARITY_R::new(((self.bits >> (n * 2 + 17)) & 1) != 0))
    }
    #[doc = "Bit 17 - Polarity of negative signal"]
    #[inline(always)]
    pub fn negative_polarity0(&self) -> NEGATIVE_POLARITY_R {
        NEGATIVE_POLARITY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Polarity of negative signal"]
    #[inline(always)]
    pub fn negative_polarity1(&self) -> NEGATIVE_POLARITY_R {
        NEGATIVE_POLARITY_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Polarity of negative signal"]
    #[inline(always)]
    pub fn negative_polarity2(&self) -> NEGATIVE_POLARITY_R {
        NEGATIVE_POLARITY_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Polarity of negative signal"]
    #[inline(always)]
    pub fn negative_polarity3(&self) -> NEGATIVE_POLARITY_R {
        NEGATIVE_POLARITY_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Break state on positive signal of this channel"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `positive_break0` field"]
    #[inline(always)]
    pub fn positive_break(&self, n: u8) -> POSITIVE_BREAK_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        POSITIVE_BREAK_R::new(((self.bits >> (n * 2 + 24)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Break state on positive signal of this channel"]
    #[inline(always)]
    pub fn positive_break_iter(&self) -> impl Iterator<Item = POSITIVE_BREAK_R> + '_ {
        (0..4).map(move |n| POSITIVE_BREAK_R::new(((self.bits >> (n * 2 + 24)) & 1) != 0))
    }
    #[doc = "Bit 24 - Break state on positive signal of this channel"]
    #[inline(always)]
    pub fn positive_break0(&self) -> POSITIVE_BREAK_R {
        POSITIVE_BREAK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Break state on positive signal of this channel"]
    #[inline(always)]
    pub fn positive_break1(&self) -> POSITIVE_BREAK_R {
        POSITIVE_BREAK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Break state on positive signal of this channel"]
    #[inline(always)]
    pub fn positive_break2(&self) -> POSITIVE_BREAK_R {
        POSITIVE_BREAK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Break state on positive signal of this channel"]
    #[inline(always)]
    pub fn positive_break3(&self) -> POSITIVE_BREAK_R {
        POSITIVE_BREAK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Break state on negative signal of this channel"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `negative_break0` field"]
    #[inline(always)]
    pub fn negative_break(&self, n: u8) -> NEGATIVE_BREAK_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        NEGATIVE_BREAK_R::new(((self.bits >> (n * 2 + 25)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Break state on negative signal of this channel"]
    #[inline(always)]
    pub fn negative_break_iter(&self) -> impl Iterator<Item = NEGATIVE_BREAK_R> + '_ {
        (0..4).map(move |n| NEGATIVE_BREAK_R::new(((self.bits >> (n * 2 + 25)) & 1) != 0))
    }
    #[doc = "Bit 25 - Break state on negative signal of this channel"]
    #[inline(always)]
    pub fn negative_break0(&self) -> NEGATIVE_BREAK_R {
        NEGATIVE_BREAK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Break state on negative signal of this channel"]
    #[inline(always)]
    pub fn negative_break1(&self) -> NEGATIVE_BREAK_R {
        NEGATIVE_BREAK_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - Break state on negative signal of this channel"]
    #[inline(always)]
    pub fn negative_break2(&self) -> NEGATIVE_BREAK_R {
        NEGATIVE_BREAK_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Break state on negative signal of this channel"]
    #[inline(always)]
    pub fn negative_break3(&self) -> NEGATIVE_BREAK_R {
        NEGATIVE_BREAK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Enable or disable positive signal for channel"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `positive_signal0` field"]
    #[inline(always)]
    #[must_use]
    pub fn positive_signal(&mut self, n: u8) -> POSITIVE_SIGNAL_W<CHANNEL_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        POSITIVE_SIGNAL_W::new(self, n * 4)
    }
    #[doc = "Bit 0 - Enable or disable positive signal for channel"]
    #[inline(always)]
    #[must_use]
    pub fn positive_signal0(&mut self) -> POSITIVE_SIGNAL_W<CHANNEL_SPEC> {
        POSITIVE_SIGNAL_W::new(self, 0)
    }
    #[doc = "Bit 4 - Enable or disable positive signal for channel"]
    #[inline(always)]
    #[must_use]
    pub fn positive_signal1(&mut self) -> POSITIVE_SIGNAL_W<CHANNEL_SPEC> {
        POSITIVE_SIGNAL_W::new(self, 4)
    }
    #[doc = "Bit 8 - Enable or disable positive signal for channel"]
    #[inline(always)]
    #[must_use]
    pub fn positive_signal2(&mut self) -> POSITIVE_SIGNAL_W<CHANNEL_SPEC> {
        POSITIVE_SIGNAL_W::new(self, 8)
    }
    #[doc = "Bit 12 - Enable or disable positive signal for channel"]
    #[inline(always)]
    #[must_use]
    pub fn positive_signal3(&mut self) -> POSITIVE_SIGNAL_W<CHANNEL_SPEC> {
        POSITIVE_SIGNAL_W::new(self, 12)
    }
    #[doc = "Idle state of positive signal"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `positive_idle0` field"]
    #[inline(always)]
    #[must_use]
    pub fn positive_idle(&mut self, n: u8) -> POSITIVE_IDLE_W<CHANNEL_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        POSITIVE_IDLE_W::new(self, n * 4 + 1)
    }
    #[doc = "Bit 1 - Idle state of positive signal"]
    #[inline(always)]
    #[must_use]
    pub fn positive_idle0(&mut self) -> POSITIVE_IDLE_W<CHANNEL_SPEC> {
        POSITIVE_IDLE_W::new(self, 1)
    }
    #[doc = "Bit 5 - Idle state of positive signal"]
    #[inline(always)]
    #[must_use]
    pub fn positive_idle1(&mut self) -> POSITIVE_IDLE_W<CHANNEL_SPEC> {
        POSITIVE_IDLE_W::new(self, 5)
    }
    #[doc = "Bit 9 - Idle state of positive signal"]
    #[inline(always)]
    #[must_use]
    pub fn positive_idle2(&mut self) -> POSITIVE_IDLE_W<CHANNEL_SPEC> {
        POSITIVE_IDLE_W::new(self, 9)
    }
    #[doc = "Bit 13 - Idle state of positive signal"]
    #[inline(always)]
    #[must_use]
    pub fn positive_idle3(&mut self) -> POSITIVE_IDLE_W<CHANNEL_SPEC> {
        POSITIVE_IDLE_W::new(self, 13)
    }
    #[doc = "Enable or disable negative signal for channel"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `negative_signal0` field"]
    #[inline(always)]
    #[must_use]
    pub fn negative_signal(&mut self, n: u8) -> NEGATIVE_SIGNAL_W<CHANNEL_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        NEGATIVE_SIGNAL_W::new(self, n * 4 + 2)
    }
    #[doc = "Bit 2 - Enable or disable negative signal for channel"]
    #[inline(always)]
    #[must_use]
    pub fn negative_signal0(&mut self) -> NEGATIVE_SIGNAL_W<CHANNEL_SPEC> {
        NEGATIVE_SIGNAL_W::new(self, 2)
    }
    #[doc = "Bit 6 - Enable or disable negative signal for channel"]
    #[inline(always)]
    #[must_use]
    pub fn negative_signal1(&mut self) -> NEGATIVE_SIGNAL_W<CHANNEL_SPEC> {
        NEGATIVE_SIGNAL_W::new(self, 6)
    }
    #[doc = "Bit 10 - Enable or disable negative signal for channel"]
    #[inline(always)]
    #[must_use]
    pub fn negative_signal2(&mut self) -> NEGATIVE_SIGNAL_W<CHANNEL_SPEC> {
        NEGATIVE_SIGNAL_W::new(self, 10)
    }
    #[doc = "Bit 14 - Enable or disable negative signal for channel"]
    #[inline(always)]
    #[must_use]
    pub fn negative_signal3(&mut self) -> NEGATIVE_SIGNAL_W<CHANNEL_SPEC> {
        NEGATIVE_SIGNAL_W::new(self, 14)
    }
    #[doc = "Idle state of negative signal"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `negative_idle0` field"]
    #[inline(always)]
    #[must_use]
    pub fn negative_idle(&mut self, n: u8) -> NEGATIVE_IDLE_W<CHANNEL_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        NEGATIVE_IDLE_W::new(self, n * 4 + 3)
    }
    #[doc = "Bit 3 - Idle state of negative signal"]
    #[inline(always)]
    #[must_use]
    pub fn negative_idle0(&mut self) -> NEGATIVE_IDLE_W<CHANNEL_SPEC> {
        NEGATIVE_IDLE_W::new(self, 3)
    }
    #[doc = "Bit 7 - Idle state of negative signal"]
    #[inline(always)]
    #[must_use]
    pub fn negative_idle1(&mut self) -> NEGATIVE_IDLE_W<CHANNEL_SPEC> {
        NEGATIVE_IDLE_W::new(self, 7)
    }
    #[doc = "Bit 11 - Idle state of negative signal"]
    #[inline(always)]
    #[must_use]
    pub fn negative_idle2(&mut self) -> NEGATIVE_IDLE_W<CHANNEL_SPEC> {
        NEGATIVE_IDLE_W::new(self, 11)
    }
    #[doc = "Bit 15 - Idle state of negative signal"]
    #[inline(always)]
    #[must_use]
    pub fn negative_idle3(&mut self) -> NEGATIVE_IDLE_W<CHANNEL_SPEC> {
        NEGATIVE_IDLE_W::new(self, 15)
    }
    #[doc = "Polarity of positive signal"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `positive_polarity0` field"]
    #[inline(always)]
    #[must_use]
    pub fn positive_polarity(&mut self, n: u8) -> POSITIVE_POLARITY_W<CHANNEL_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        POSITIVE_POLARITY_W::new(self, n * 2 + 16)
    }
    #[doc = "Bit 16 - Polarity of positive signal"]
    #[inline(always)]
    #[must_use]
    pub fn positive_polarity0(&mut self) -> POSITIVE_POLARITY_W<CHANNEL_SPEC> {
        POSITIVE_POLARITY_W::new(self, 16)
    }
    #[doc = "Bit 18 - Polarity of positive signal"]
    #[inline(always)]
    #[must_use]
    pub fn positive_polarity1(&mut self) -> POSITIVE_POLARITY_W<CHANNEL_SPEC> {
        POSITIVE_POLARITY_W::new(self, 18)
    }
    #[doc = "Bit 20 - Polarity of positive signal"]
    #[inline(always)]
    #[must_use]
    pub fn positive_polarity2(&mut self) -> POSITIVE_POLARITY_W<CHANNEL_SPEC> {
        POSITIVE_POLARITY_W::new(self, 20)
    }
    #[doc = "Bit 22 - Polarity of positive signal"]
    #[inline(always)]
    #[must_use]
    pub fn positive_polarity3(&mut self) -> POSITIVE_POLARITY_W<CHANNEL_SPEC> {
        POSITIVE_POLARITY_W::new(self, 22)
    }
    #[doc = "Polarity of negative signal"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `negative_polarity0` field"]
    #[inline(always)]
    #[must_use]
    pub fn negative_polarity(&mut self, n: u8) -> NEGATIVE_POLARITY_W<CHANNEL_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        NEGATIVE_POLARITY_W::new(self, n * 2 + 17)
    }
    #[doc = "Bit 17 - Polarity of negative signal"]
    #[inline(always)]
    #[must_use]
    pub fn negative_polarity0(&mut self) -> NEGATIVE_POLARITY_W<CHANNEL_SPEC> {
        NEGATIVE_POLARITY_W::new(self, 17)
    }
    #[doc = "Bit 19 - Polarity of negative signal"]
    #[inline(always)]
    #[must_use]
    pub fn negative_polarity1(&mut self) -> NEGATIVE_POLARITY_W<CHANNEL_SPEC> {
        NEGATIVE_POLARITY_W::new(self, 19)
    }
    #[doc = "Bit 21 - Polarity of negative signal"]
    #[inline(always)]
    #[must_use]
    pub fn negative_polarity2(&mut self) -> NEGATIVE_POLARITY_W<CHANNEL_SPEC> {
        NEGATIVE_POLARITY_W::new(self, 21)
    }
    #[doc = "Bit 23 - Polarity of negative signal"]
    #[inline(always)]
    #[must_use]
    pub fn negative_polarity3(&mut self) -> NEGATIVE_POLARITY_W<CHANNEL_SPEC> {
        NEGATIVE_POLARITY_W::new(self, 23)
    }
    #[doc = "Break state on positive signal of this channel"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `positive_break0` field"]
    #[inline(always)]
    #[must_use]
    pub fn positive_break(&mut self, n: u8) -> POSITIVE_BREAK_W<CHANNEL_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        POSITIVE_BREAK_W::new(self, n * 2 + 24)
    }
    #[doc = "Bit 24 - Break state on positive signal of this channel"]
    #[inline(always)]
    #[must_use]
    pub fn positive_break0(&mut self) -> POSITIVE_BREAK_W<CHANNEL_SPEC> {
        POSITIVE_BREAK_W::new(self, 24)
    }
    #[doc = "Bit 26 - Break state on positive signal of this channel"]
    #[inline(always)]
    #[must_use]
    pub fn positive_break1(&mut self) -> POSITIVE_BREAK_W<CHANNEL_SPEC> {
        POSITIVE_BREAK_W::new(self, 26)
    }
    #[doc = "Bit 28 - Break state on positive signal of this channel"]
    #[inline(always)]
    #[must_use]
    pub fn positive_break2(&mut self) -> POSITIVE_BREAK_W<CHANNEL_SPEC> {
        POSITIVE_BREAK_W::new(self, 28)
    }
    #[doc = "Bit 30 - Break state on positive signal of this channel"]
    #[inline(always)]
    #[must_use]
    pub fn positive_break3(&mut self) -> POSITIVE_BREAK_W<CHANNEL_SPEC> {
        POSITIVE_BREAK_W::new(self, 30)
    }
    #[doc = "Break state on negative signal of this channel"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `negative_break0` field"]
    #[inline(always)]
    #[must_use]
    pub fn negative_break(&mut self, n: u8) -> NEGATIVE_BREAK_W<CHANNEL_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        NEGATIVE_BREAK_W::new(self, n * 2 + 25)
    }
    #[doc = "Bit 25 - Break state on negative signal of this channel"]
    #[inline(always)]
    #[must_use]
    pub fn negative_break0(&mut self) -> NEGATIVE_BREAK_W<CHANNEL_SPEC> {
        NEGATIVE_BREAK_W::new(self, 25)
    }
    #[doc = "Bit 27 - Break state on negative signal of this channel"]
    #[inline(always)]
    #[must_use]
    pub fn negative_break1(&mut self) -> NEGATIVE_BREAK_W<CHANNEL_SPEC> {
        NEGATIVE_BREAK_W::new(self, 27)
    }
    #[doc = "Bit 29 - Break state on negative signal of this channel"]
    #[inline(always)]
    #[must_use]
    pub fn negative_break2(&mut self) -> NEGATIVE_BREAK_W<CHANNEL_SPEC> {
        NEGATIVE_BREAK_W::new(self, 29)
    }
    #[doc = "Bit 31 - Break state on negative signal of this channel"]
    #[inline(always)]
    #[must_use]
    pub fn negative_break3(&mut self) -> NEGATIVE_BREAK_W<CHANNEL_SPEC> {
        NEGATIVE_BREAK_W::new(self, 31)
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
#[doc = "Channel configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`channel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`channel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHANNEL_SPEC;
impl crate::RegisterSpec for CHANNEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`channel::R`](R) reader structure"]
impl crate::Readable for CHANNEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`channel::W`](W) writer structure"]
impl crate::Writable for CHANNEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets channel to value 0"]
impl crate::Resettable for CHANNEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
