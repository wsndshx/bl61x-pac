#[doc = "Register `hbncore_resv1` reader"]
pub type R = crate::R<HBNCORE_RESV1_SPEC>;
#[doc = "Register `hbncore_resv1` writer"]
pub type W = crate::W<HBNCORE_RESV1_SPEC>;
#[doc = "Field `hbncore_resv1_data` reader - "]
pub type HBNCORE_RESV1_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `hbncore_resv1_data` writer - "]
pub type HBNCORE_RESV1_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hbncore_resv1_data(&self) -> HBNCORE_RESV1_DATA_R {
        HBNCORE_RESV1_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn hbncore_resv1_data(&mut self) -> HBNCORE_RESV1_DATA_W<HBNCORE_RESV1_SPEC> {
        HBNCORE_RESV1_DATA_W::new(self, 0)
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
#[doc = "hbncore_resv1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hbncore_resv1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hbncore_resv1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HBNCORE_RESV1_SPEC;
impl crate::RegisterSpec for HBNCORE_RESV1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hbncore_resv1::R`](R) reader structure"]
impl crate::Readable for HBNCORE_RESV1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hbncore_resv1::W`](W) writer structure"]
impl crate::Writable for HBNCORE_RESV1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hbncore_resv1 to value 0"]
impl crate::Resettable for HBNCORE_RESV1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
