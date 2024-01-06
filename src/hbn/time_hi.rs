#[doc = "Register `time_hi` reader"]
pub type R = crate::R<TIME_HI_SPEC>;
#[doc = "Register `time_hi` writer"]
pub type W = crate::W<TIME_HI_SPEC>;
#[doc = "Field `hbn_time_h` reader - "]
pub type HBN_TIME_H_R = crate::FieldReader;
#[doc = "Field `hbn_time_h` writer - "]
pub type HBN_TIME_H_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn hbn_time_h(&self) -> HBN_TIME_H_R {
        HBN_TIME_H_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_time_h(&mut self) -> HBN_TIME_H_W<TIME_HI_SPEC> {
        HBN_TIME_H_W::new(self, 0)
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
#[doc = "High bits of hibernate time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time_hi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time_hi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIME_HI_SPEC;
impl crate::RegisterSpec for TIME_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time_hi::R`](R) reader structure"]
impl crate::Readable for TIME_HI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`time_hi::W`](W) writer structure"]
impl crate::Writable for TIME_HI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets time_hi to value 0"]
impl crate::Resettable for TIME_HI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
