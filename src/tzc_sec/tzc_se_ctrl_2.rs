#[doc = "Register `tzc_se_ctrl_2` reader"]
pub type R = crate::R<TZC_SE_CTRL_2_SPEC>;
#[doc = "Register `tzc_se_ctrl_2` writer"]
pub type W = crate::W<TZC_SE_CTRL_2_SPEC>;
#[doc = "Field `tzc_se_sha_tzsid_lock` reader - Lock for TZSID enable in SHA of Secure Engine."]
pub type TZC_SE_SHA_TZSID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_se_sha_tzsid_lock` writer - Lock for TZSID enable in SHA of Secure Engine."]
pub type TZC_SE_SHA_TZSID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_se_aes_tzsid_lock` reader - Lock for TZSID enable in AES of Secure Engine."]
pub type TZC_SE_AES_TZSID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_se_aes_tzsid_lock` writer - Lock for TZSID enable in AES of Secure Engine."]
pub type TZC_SE_AES_TZSID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_se_trng_tzsid_lock` reader - Lock for TZSID enable in TRNG of Secure Engine."]
pub type TZC_SE_TRNG_TZSID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_se_trng_tzsid_lock` writer - Lock for TZSID enable in TRNG of Secure Engine."]
pub type TZC_SE_TRNG_TZSID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_se_pka_tzsid_lock` reader - Lock for TZSID enable in PKA of Secure Engine."]
pub type TZC_SE_PKA_TZSID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_se_pka_tzsid_lock` writer - Lock for TZSID enable in PKA of Secure Engine."]
pub type TZC_SE_PKA_TZSID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_se_cdet_tzsid_lock` reader - Lock for TZSID enable in code detection of Secure Engine."]
pub type TZC_SE_CDET_TZSID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_se_cdet_tzsid_lock` writer - Lock for TZSID enable in code detection of Secure Engine."]
pub type TZC_SE_CDET_TZSID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_se_gmac_tzsid_lock` reader - Lock for TZSID enable in GMAC of Secure Engine."]
pub type TZC_SE_GMAC_TZSID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_se_gmac_tzsid_lock` writer - Lock for TZSID enable in GMAC of Secure Engine."]
pub type TZC_SE_GMAC_TZSID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_se_tzsid_crmd_lock` reader - Lock for TZSID control in CRMD of Secure Engine."]
pub type TZC_SE_TZSID_CRMD_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_se_tzsid_crmd_lock` writer - Lock for TZSID control in CRMD of Secure Engine."]
pub type TZC_SE_TZSID_CRMD_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sf_cr_tzsid_lock` reader - Lock for TZSID enable in CR of Secure Engine."]
pub type TZC_SF_CR_TZSID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_sf_cr_tzsid_lock` writer - Lock for TZSID enable in CR of Secure Engine."]
pub type TZC_SF_CR_TZSID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sf_sec_tzsid_lock` reader - Lock for TZSID enable in security of Secure Engine."]
pub type TZC_SF_SEC_TZSID_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_sf_sec_tzsid_lock` writer - Lock for TZSID enable in security of Secure Engine."]
pub type TZC_SF_SEC_TZSID_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_sf_tzsid_crmd_lock` reader - Lock for TZSID control in CRMD of Secure Engine."]
pub type TZC_SF_TZSID_CRMD_LOCK_R = crate::BitReader;
#[doc = "Field `tzc_sf_tzsid_crmd_lock` writer - Lock for TZSID control in CRMD of Secure Engine."]
pub type TZC_SF_TZSID_CRMD_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Lock for TZSID enable in SHA of Secure Engine."]
    #[inline(always)]
    pub fn tzc_se_sha_tzsid_lock(&self) -> TZC_SE_SHA_TZSID_LOCK_R {
        TZC_SE_SHA_TZSID_LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lock for TZSID enable in AES of Secure Engine."]
    #[inline(always)]
    pub fn tzc_se_aes_tzsid_lock(&self) -> TZC_SE_AES_TZSID_LOCK_R {
        TZC_SE_AES_TZSID_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Lock for TZSID enable in TRNG of Secure Engine."]
    #[inline(always)]
    pub fn tzc_se_trng_tzsid_lock(&self) -> TZC_SE_TRNG_TZSID_LOCK_R {
        TZC_SE_TRNG_TZSID_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Lock for TZSID enable in PKA of Secure Engine."]
    #[inline(always)]
    pub fn tzc_se_pka_tzsid_lock(&self) -> TZC_SE_PKA_TZSID_LOCK_R {
        TZC_SE_PKA_TZSID_LOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Lock for TZSID enable in code detection of Secure Engine."]
    #[inline(always)]
    pub fn tzc_se_cdet_tzsid_lock(&self) -> TZC_SE_CDET_TZSID_LOCK_R {
        TZC_SE_CDET_TZSID_LOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Lock for TZSID enable in GMAC of Secure Engine."]
    #[inline(always)]
    pub fn tzc_se_gmac_tzsid_lock(&self) -> TZC_SE_GMAC_TZSID_LOCK_R {
        TZC_SE_GMAC_TZSID_LOCK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Lock for TZSID control in CRMD of Secure Engine."]
    #[inline(always)]
    pub fn tzc_se_tzsid_crmd_lock(&self) -> TZC_SE_TZSID_CRMD_LOCK_R {
        TZC_SE_TZSID_CRMD_LOCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - Lock for TZSID enable in CR of Secure Engine."]
    #[inline(always)]
    pub fn tzc_sf_cr_tzsid_lock(&self) -> TZC_SF_CR_TZSID_LOCK_R {
        TZC_SF_CR_TZSID_LOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Lock for TZSID enable in security of Secure Engine."]
    #[inline(always)]
    pub fn tzc_sf_sec_tzsid_lock(&self) -> TZC_SF_SEC_TZSID_LOCK_R {
        TZC_SF_SEC_TZSID_LOCK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Lock for TZSID control in CRMD of Secure Engine."]
    #[inline(always)]
    pub fn tzc_sf_tzsid_crmd_lock(&self) -> TZC_SF_TZSID_CRMD_LOCK_R {
        TZC_SF_TZSID_CRMD_LOCK_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock for TZSID enable in SHA of Secure Engine."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_se_sha_tzsid_lock(&mut self) -> TZC_SE_SHA_TZSID_LOCK_W<TZC_SE_CTRL_2_SPEC> {
        TZC_SE_SHA_TZSID_LOCK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Lock for TZSID enable in AES of Secure Engine."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_se_aes_tzsid_lock(&mut self) -> TZC_SE_AES_TZSID_LOCK_W<TZC_SE_CTRL_2_SPEC> {
        TZC_SE_AES_TZSID_LOCK_W::new(self, 1)
    }
    #[doc = "Bit 2 - Lock for TZSID enable in TRNG of Secure Engine."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_se_trng_tzsid_lock(&mut self) -> TZC_SE_TRNG_TZSID_LOCK_W<TZC_SE_CTRL_2_SPEC> {
        TZC_SE_TRNG_TZSID_LOCK_W::new(self, 2)
    }
    #[doc = "Bit 3 - Lock for TZSID enable in PKA of Secure Engine."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_se_pka_tzsid_lock(&mut self) -> TZC_SE_PKA_TZSID_LOCK_W<TZC_SE_CTRL_2_SPEC> {
        TZC_SE_PKA_TZSID_LOCK_W::new(self, 3)
    }
    #[doc = "Bit 4 - Lock for TZSID enable in code detection of Secure Engine."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_se_cdet_tzsid_lock(&mut self) -> TZC_SE_CDET_TZSID_LOCK_W<TZC_SE_CTRL_2_SPEC> {
        TZC_SE_CDET_TZSID_LOCK_W::new(self, 4)
    }
    #[doc = "Bit 5 - Lock for TZSID enable in GMAC of Secure Engine."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_se_gmac_tzsid_lock(&mut self) -> TZC_SE_GMAC_TZSID_LOCK_W<TZC_SE_CTRL_2_SPEC> {
        TZC_SE_GMAC_TZSID_LOCK_W::new(self, 5)
    }
    #[doc = "Bit 6 - Lock for TZSID control in CRMD of Secure Engine."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_se_tzsid_crmd_lock(&mut self) -> TZC_SE_TZSID_CRMD_LOCK_W<TZC_SE_CTRL_2_SPEC> {
        TZC_SE_TZSID_CRMD_LOCK_W::new(self, 6)
    }
    #[doc = "Bit 16 - Lock for TZSID enable in CR of Secure Engine."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_cr_tzsid_lock(&mut self) -> TZC_SF_CR_TZSID_LOCK_W<TZC_SE_CTRL_2_SPEC> {
        TZC_SF_CR_TZSID_LOCK_W::new(self, 16)
    }
    #[doc = "Bit 17 - Lock for TZSID enable in security of Secure Engine."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_sec_tzsid_lock(&mut self) -> TZC_SF_SEC_TZSID_LOCK_W<TZC_SE_CTRL_2_SPEC> {
        TZC_SF_SEC_TZSID_LOCK_W::new(self, 17)
    }
    #[doc = "Bit 18 - Lock for TZSID control in CRMD of Secure Engine."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_tzsid_crmd_lock(&mut self) -> TZC_SF_TZSID_CRMD_LOCK_W<TZC_SE_CTRL_2_SPEC> {
        TZC_SF_TZSID_CRMD_LOCK_W::new(self, 18)
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
#[doc = "TrustZone Controller Secure Engine Control 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_se_ctrl_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_se_ctrl_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_SE_CTRL_2_SPEC;
impl crate::RegisterSpec for TZC_SE_CTRL_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_se_ctrl_2::R`](R) reader structure"]
impl crate::Readable for TZC_SE_CTRL_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tzc_se_ctrl_2::W`](W) writer structure"]
impl crate::Writable for TZC_SE_CTRL_2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_se_ctrl_2 to value 0"]
impl crate::Resettable for TZC_SE_CTRL_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
