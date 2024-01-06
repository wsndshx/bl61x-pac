#[doc = "Register `PDS_TIME1` reader"]
pub type R = crate::R<PDS_TIME1_SPEC>;
#[doc = "Register `PDS_TIME1` writer"]
pub type W = crate::W<PDS_TIME1_SPEC>;
#[doc = "Field `cr_sleep_duration` reader - "]
pub type CR_SLEEP_DURATION_R = crate::FieldReader<u32>;
#[doc = "Field `cr_sleep_duration` writer - "]
pub type CR_SLEEP_DURATION_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cr_sleep_duration(&self) -> CR_SLEEP_DURATION_R {
        CR_SLEEP_DURATION_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn cr_sleep_duration(&mut self) -> CR_SLEEP_DURATION_W<PDS_TIME1_SPEC> {
        CR_SLEEP_DURATION_W::new(self, 0)
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
#[doc = "PDS_TIME1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pds_time1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pds_time1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDS_TIME1_SPEC;
impl crate::RegisterSpec for PDS_TIME1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_time1::R`](R) reader structure"]
impl crate::Readable for PDS_TIME1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pds_time1::W`](W) writer structure"]
impl crate::Writable for PDS_TIME1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDS_TIME1 to value 0"]
impl crate::Resettable for PDS_TIME1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
