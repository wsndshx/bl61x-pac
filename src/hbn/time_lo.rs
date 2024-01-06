#[doc = "Register `time_lo` reader"]
pub type R = crate::R<TIME_LO_SPEC>;
#[doc = "Register `time_lo` writer"]
pub type W = crate::W<TIME_LO_SPEC>;
#[doc = "Field `hbn_time_l` reader - "]
pub type HBN_TIME_L_R = crate::FieldReader<u32>;
#[doc = "Field `hbn_time_l` writer - "]
pub type HBN_TIME_L_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hbn_time_l(&self) -> HBN_TIME_L_R {
        HBN_TIME_L_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_time_l(&mut self) -> HBN_TIME_L_W<TIME_LO_SPEC> {
        HBN_TIME_L_W::new(self, 0)
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
#[doc = "Low bits of hibernate time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time_lo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time_lo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIME_LO_SPEC;
impl crate::RegisterSpec for TIME_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time_lo::R`](R) reader structure"]
impl crate::Readable for TIME_LO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`time_lo::W`](W) writer structure"]
impl crate::Writable for TIME_LO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets time_lo to value 0"]
impl crate::Resettable for TIME_LO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
