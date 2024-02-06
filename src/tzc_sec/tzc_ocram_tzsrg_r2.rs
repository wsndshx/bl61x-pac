#[doc = "Register `tzc_ocram_tzsrg_r2` reader"]
pub type R = crate::R<TZC_OCRAM_TZSRG_R2_SPEC>;
#[doc = "Register `tzc_ocram_tzsrg_r2` writer"]
pub type W = crate::W<TZC_OCRAM_TZSRG_R2_SPEC>;
#[doc = "Field `tzc_ocram_tzsrg_r2_end` reader - End address for TrustZone Range 2."]
pub type TZC_OCRAM_TZSRG_R2_END_R = crate::FieldReader<u16>;
#[doc = "Field `tzc_ocram_tzsrg_r2_end` writer - End address for TrustZone Range 2."]
pub type TZC_OCRAM_TZSRG_R2_END_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `tzc_ocram_tzsrg_r2_start` reader - Start address for TrustZone Range 2."]
pub type TZC_OCRAM_TZSRG_R2_START_R = crate::FieldReader<u16>;
#[doc = "Field `tzc_ocram_tzsrg_r2_start` writer - Start address for TrustZone Range 2."]
pub type TZC_OCRAM_TZSRG_R2_START_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - End address for TrustZone Range 2."]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r2_end(&self) -> TZC_OCRAM_TZSRG_R2_END_R {
        TZC_OCRAM_TZSRG_R2_END_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Start address for TrustZone Range 2."]
    #[inline(always)]
    pub fn tzc_ocram_tzsrg_r2_start(&self) -> TZC_OCRAM_TZSRG_R2_START_R {
        TZC_OCRAM_TZSRG_R2_START_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - End address for TrustZone Range 2."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_ocram_tzsrg_r2_end(&mut self) -> TZC_OCRAM_TZSRG_R2_END_W<TZC_OCRAM_TZSRG_R2_SPEC> {
        TZC_OCRAM_TZSRG_R2_END_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Start address for TrustZone Range 2."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_ocram_tzsrg_r2_start(
        &mut self,
    ) -> TZC_OCRAM_TZSRG_R2_START_W<TZC_OCRAM_TZSRG_R2_SPEC> {
        TZC_OCRAM_TZSRG_R2_START_W::new(self, 16)
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
#[doc = "TrustZone Controller OCram TrustZone Security Register Group Range 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_ocram_tzsrg_r2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_ocram_tzsrg_r2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_OCRAM_TZSRG_R2_SPEC;
impl crate::RegisterSpec for TZC_OCRAM_TZSRG_R2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_ocram_tzsrg_r2::R`](R) reader structure"]
impl crate::Readable for TZC_OCRAM_TZSRG_R2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tzc_ocram_tzsrg_r2::W`](W) writer structure"]
impl crate::Writable for TZC_OCRAM_TZSRG_R2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_ocram_tzsrg_r2 to value 0"]
impl crate::Resettable for TZC_OCRAM_TZSRG_R2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
