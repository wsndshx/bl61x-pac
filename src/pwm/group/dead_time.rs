#[doc = "Register `dead_time` reader"]
pub type R = crate::R<DEAD_TIME_SPEC>;
#[doc = "Register `dead_time` writer"]
pub type W = crate::W<DEAD_TIME_SPEC>;
#[doc = "Field `channel(0-3)` reader - Dead time for each channel in cycles"]
pub type CHANNEL_R = crate::FieldReader;
#[doc = "Field `channel(0-3)` writer - Dead time for each channel in cycles"]
pub type CHANNEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Dead time for each channel in cycles"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `channel0` field"]
    #[inline(always)]
    pub fn channel(&self, n: u8) -> CHANNEL_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CHANNEL_R::new(((self.bits >> (n * 8)) & 0xff) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Dead time for each channel in cycles"]
    #[inline(always)]
    pub fn channel_iter(&self) -> impl Iterator<Item = CHANNEL_R> + '_ {
        (0..4).map(move |n| CHANNEL_R::new(((self.bits >> (n * 8)) & 0xff) as u8))
    }
    #[doc = "Bits 0:7 - Dead time for each channel in cycles"]
    #[inline(always)]
    pub fn channel0(&self) -> CHANNEL_R {
        CHANNEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Dead time for each channel in cycles"]
    #[inline(always)]
    pub fn channel1(&self) -> CHANNEL_R {
        CHANNEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Dead time for each channel in cycles"]
    #[inline(always)]
    pub fn channel2(&self) -> CHANNEL_R {
        CHANNEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Dead time for each channel in cycles"]
    #[inline(always)]
    pub fn channel3(&self) -> CHANNEL_R {
        CHANNEL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Dead time for each channel in cycles"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `channel0` field"]
    #[inline(always)]
    #[must_use]
    pub fn channel(&mut self, n: u8) -> CHANNEL_W<DEAD_TIME_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CHANNEL_W::new(self, n * 8)
    }
    #[doc = "Bits 0:7 - Dead time for each channel in cycles"]
    #[inline(always)]
    #[must_use]
    pub fn channel0(&mut self) -> CHANNEL_W<DEAD_TIME_SPEC> {
        CHANNEL_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Dead time for each channel in cycles"]
    #[inline(always)]
    #[must_use]
    pub fn channel1(&mut self) -> CHANNEL_W<DEAD_TIME_SPEC> {
        CHANNEL_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Dead time for each channel in cycles"]
    #[inline(always)]
    #[must_use]
    pub fn channel2(&mut self) -> CHANNEL_W<DEAD_TIME_SPEC> {
        CHANNEL_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Dead time for each channel in cycles"]
    #[inline(always)]
    #[must_use]
    pub fn channel3(&mut self) -> CHANNEL_W<DEAD_TIME_SPEC> {
        CHANNEL_W::new(self, 24)
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
#[doc = "Dead time for each channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dead_time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dead_time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEAD_TIME_SPEC;
impl crate::RegisterSpec for DEAD_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dead_time::R`](R) reader structure"]
impl crate::Readable for DEAD_TIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dead_time::W`](W) writer structure"]
impl crate::Writable for DEAD_TIME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dead_time to value 0"]
impl crate::Resettable for DEAD_TIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
