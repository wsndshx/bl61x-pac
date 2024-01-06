#[doc = "Register `rtc_time_hi` reader"]
pub type R = crate::R<RTC_TIME_HI_SPEC>;
#[doc = "Register `rtc_time_hi` writer"]
pub type W = crate::W<RTC_TIME_HI_SPEC>;
#[doc = "Field `rtc_time_latch_h` reader - "]
pub type RTC_TIME_LATCH_H_R = crate::FieldReader;
#[doc = "Field `rtc_time_latch_h` writer - "]
pub type RTC_TIME_LATCH_H_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `rtc_time_latch` reader - "]
pub type RTC_TIME_LATCH_R = crate::BitReader;
#[doc = "Field `rtc_time_latch` writer - "]
pub type RTC_TIME_LATCH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rtc_time_latch_h(&self) -> RTC_TIME_LATCH_H_R {
        RTC_TIME_LATCH_H_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_time_latch(&self) -> RTC_TIME_LATCH_R {
        RTC_TIME_LATCH_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_time_latch_h(&mut self) -> RTC_TIME_LATCH_H_W<RTC_TIME_HI_SPEC> {
        RTC_TIME_LATCH_H_W::new(self, 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_time_latch(&mut self) -> RTC_TIME_LATCH_W<RTC_TIME_HI_SPEC> {
        RTC_TIME_LATCH_W::new(self, 31)
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
#[doc = "High bits of Real-Time Clock time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_time_hi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_time_hi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_TIME_HI_SPEC;
impl crate::RegisterSpec for RTC_TIME_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_time_hi::R`](R) reader structure"]
impl crate::Readable for RTC_TIME_HI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_time_hi::W`](W) writer structure"]
impl crate::Writable for RTC_TIME_HI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rtc_time_hi to value 0"]
impl crate::Resettable for RTC_TIME_HI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
