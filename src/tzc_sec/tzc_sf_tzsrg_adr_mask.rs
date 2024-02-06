#[doc = "Register `tzc_sf_tzsrg_adr_mask` reader"]
pub type R = crate::R<TZC_SF_TZSRG_ADR_MASK_SPEC>;
#[doc = "Register `tzc_sf_tzsrg_adr_mask` writer"]
pub type W = crate::W<TZC_SF_TZSRG_ADR_MASK_SPEC>;
#[doc = "Field `tzc_sf_tzsrg_adr_mask` reader - Address Mask for SF TrustZone Security Register Group."]
pub type TZC_SF_TZSRG_ADR_MASK_R = crate::FieldReader<u32>;
#[doc = "Field `tzc_sf_tzsrg_adr_mask` writer - Address Mask for SF TrustZone Security Register Group."]
pub type TZC_SF_TZSRG_ADR_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `tzc_sf_tzsrg_adr_mask_lock` reader - Lock Address Mask for SF TrustZone Security Register Group."]
pub type TZC_SF_TZSRG_ADR_MASK_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_sf_tzsrg_adr_mask_lock` writer - Lock Address Mask for SF TrustZone Security Register Group."]
pub type TZC_SF_TZSRG_ADR_MASK_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:18 - Address Mask for SF TrustZone Security Register Group."]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_adr_mask(&self) -> TZC_SF_TZSRG_ADR_MASK_R {
        TZC_SF_TZSRG_ADR_MASK_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bit 31 - Lock Address Mask for SF TrustZone Security Register Group."]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_adr_mask_lock(&self) -> TZC_SF_TZSRG_ADR_MASK_LOCK_R {
        TZC_SF_TZSRG_ADR_MASK_LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:18 - Address Mask for SF TrustZone Security Register Group."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_tzsrg_adr_mask(&mut self) -> TZC_SF_TZSRG_ADR_MASK_W<TZC_SF_TZSRG_ADR_MASK_SPEC> {
        TZC_SF_TZSRG_ADR_MASK_W::new(self, 0)
    }
    #[doc = "Bit 31 - Lock Address Mask for SF TrustZone Security Register Group."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_tzsrg_adr_mask_lock(
        &mut self,
    ) -> TZC_SF_TZSRG_ADR_MASK_LOCK_W<TZC_SF_TZSRG_ADR_MASK_SPEC> {
        TZC_SF_TZSRG_ADR_MASK_LOCK_W::new(self, 31)
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
#[doc = "TrustZone Controller SF TrustZone Security Register Group Address Mask.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_sf_tzsrg_adr_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_sf_tzsrg_adr_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_SF_TZSRG_ADR_MASK_SPEC;
impl crate::RegisterSpec for TZC_SF_TZSRG_ADR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_sf_tzsrg_adr_mask::R`](R) reader structure"]
impl crate::Readable for TZC_SF_TZSRG_ADR_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tzc_sf_tzsrg_adr_mask::W`](W) writer structure"]
impl crate::Writable for TZC_SF_TZSRG_ADR_MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_sf_tzsrg_adr_mask to value 0"]
impl crate::Resettable for TZC_SF_TZSRG_ADR_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
