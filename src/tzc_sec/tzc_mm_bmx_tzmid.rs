#[doc = "Register `tzc_mm_bmx_tzmid` reader"]
pub type R = crate::R<TZC_MM_BMX_TZMID_SPEC>;
#[doc = "Register `tzc_mm_bmx_tzmid` writer"]
pub type W = crate::W<TZC_MM_BMX_TZMID_SPEC>;
#[doc = "Field `tzc_codec_tzmid` reader - TrustZone Master ID for the Codec."]
pub type TZC_CODEC_TZMID_R = crate::BitReader;
#[doc = "Field `tzc_codec_tzmid` writer - TrustZone Master ID for the Codec."]
pub type TZC_CODEC_TZMID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_codec_tzmid_sel` reader - TrustZone Master ID Select for the Codec."]
pub type TZC_CODEC_TZMID_SEL_R = crate::BitReader;
#[doc = "Field `tzc_codec_tzmid_sel` writer - TrustZone Master ID Select for the Codec."]
pub type TZC_CODEC_TZMID_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - TrustZone Master ID for the Codec."]
    #[inline(always)]
    pub fn tzc_codec_tzmid(&self) -> TZC_CODEC_TZMID_R {
        TZC_CODEC_TZMID_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 18 - TrustZone Master ID Select for the Codec."]
    #[inline(always)]
    pub fn tzc_codec_tzmid_sel(&self) -> TZC_CODEC_TZMID_SEL_R {
        TZC_CODEC_TZMID_SEL_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - TrustZone Master ID for the Codec."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_codec_tzmid(&mut self) -> TZC_CODEC_TZMID_W<TZC_MM_BMX_TZMID_SPEC> {
        TZC_CODEC_TZMID_W::new(self, 2)
    }
    #[doc = "Bit 18 - TrustZone Master ID Select for the Codec."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_codec_tzmid_sel(&mut self) -> TZC_CODEC_TZMID_SEL_W<TZC_MM_BMX_TZMID_SPEC> {
        TZC_CODEC_TZMID_SEL_W::new(self, 18)
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
#[doc = "TrustZone Controller Memory-Mapped Bus Matrix TrustZone Master ID.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_mm_bmx_tzmid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_mm_bmx_tzmid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_MM_BMX_TZMID_SPEC;
impl crate::RegisterSpec for TZC_MM_BMX_TZMID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_mm_bmx_tzmid::R`](R) reader structure"]
impl crate::Readable for TZC_MM_BMX_TZMID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tzc_mm_bmx_tzmid::W`](W) writer structure"]
impl crate::Writable for TZC_MM_BMX_TZMID_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_mm_bmx_tzmid to value 0"]
impl crate::Resettable for TZC_MM_BMX_TZMID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
