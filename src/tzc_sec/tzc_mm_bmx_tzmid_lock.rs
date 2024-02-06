#[doc = "Register `tzc_mm_bmx_tzmid_lock` reader"]
pub type R = crate::R<TZC_MM_BMX_TZMID_LOCK_SPEC>;
#[doc = "Register `tzc_mm_bmx_tzmid_lock` writer"]
pub type W = crate::W<TZC_MM_BMX_TZMID_LOCK_SPEC>;
#[doc = "Field `tzc_codec_tzmid_lock` reader - Lock TrustZone Master ID for the Codec."]
pub type TZC_CODEC_TZMID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_codec_tzmid_lock` writer - Lock TrustZone Master ID for the Codec."]
pub type TZC_CODEC_TZMID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Lock TrustZone Master ID for the Codec."]
    #[inline(always)]
    pub fn tzc_codec_tzmid_lock(&self) -> TZC_CODEC_TZMID_LOCK_R {
        TZC_CODEC_TZMID_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Lock TrustZone Master ID for the Codec."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_codec_tzmid_lock(&mut self) -> TZC_CODEC_TZMID_LOCK_W<TZC_MM_BMX_TZMID_LOCK_SPEC> {
        TZC_CODEC_TZMID_LOCK_W::new(self, 2)
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
#[doc = "TrustZone Controller Memory-Mapped Bus Matrix TrustZone Master ID Lock.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_mm_bmx_tzmid_lock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_mm_bmx_tzmid_lock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_MM_BMX_TZMID_LOCK_SPEC;
impl crate::RegisterSpec for TZC_MM_BMX_TZMID_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_mm_bmx_tzmid_lock::R`](R) reader structure"]
impl crate::Readable for TZC_MM_BMX_TZMID_LOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tzc_mm_bmx_tzmid_lock::W`](W) writer structure"]
impl crate::Writable for TZC_MM_BMX_TZMID_LOCK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_mm_bmx_tzmid_lock to value 0"]
impl crate::Resettable for TZC_MM_BMX_TZMID_LOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
