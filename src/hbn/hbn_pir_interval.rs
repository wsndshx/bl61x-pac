#[doc = "Register `HBN_PIR_INTERVAL` reader"]
pub type R = crate::R<HBN_PIR_INTERVAL_SPEC>;
#[doc = "Register `HBN_PIR_INTERVAL` writer"]
pub type W = crate::W<HBN_PIR_INTERVAL_SPEC>;
#[doc = "Field `pir_interval` reader - "]
pub type PIR_INTERVAL_R = crate::FieldReader<u16>;
#[doc = "Field `pir_interval` writer - "]
pub type PIR_INTERVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn pir_interval(&self) -> PIR_INTERVAL_R {
        PIR_INTERVAL_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn pir_interval(&mut self) -> PIR_INTERVAL_W<HBN_PIR_INTERVAL_SPEC> {
        PIR_INTERVAL_W::new(self, 0)
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
#[doc = "HBN_PIR_INTERVAL.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hbn_pir_interval::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hbn_pir_interval::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HBN_PIR_INTERVAL_SPEC;
impl crate::RegisterSpec for HBN_PIR_INTERVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hbn_pir_interval::R`](R) reader structure"]
impl crate::Readable for HBN_PIR_INTERVAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hbn_pir_interval::W`](W) writer structure"]
impl crate::Writable for HBN_PIR_INTERVAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HBN_PIR_INTERVAL to value 0"]
impl crate::Resettable for HBN_PIR_INTERVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
