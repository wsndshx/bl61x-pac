#[doc = "Register `tzc_se_ctrl_0` reader"]
pub type R = crate::R<TZC_SE_CTRL_0_SPEC>;
#[doc = "Register `tzc_se_ctrl_0` writer"]
pub type W = crate::W<TZC_SE_CTRL_0_SPEC>;
#[doc = "Field `tzc_se_sha_tzsid_en` reader - TZSID enable for SHA in Secure Engine."]
pub type TZC_SE_SHA_TZSID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_se_sha_tzsid_en` writer - TZSID enable for SHA in Secure Engine."]
pub type TZC_SE_SHA_TZSID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_se_aes_tzsid_en` reader - TZSID enable for AES in Secure Engine."]
pub type TZC_SE_AES_TZSID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_se_aes_tzsid_en` writer - TZSID enable for AES in Secure Engine."]
pub type TZC_SE_AES_TZSID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_se_trng_tzsid_en` reader - TZSID enable for TRNG in Secure Engine."]
pub type TZC_SE_TRNG_TZSID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_se_trng_tzsid_en` writer - TZSID enable for TRNG in Secure Engine."]
pub type TZC_SE_TRNG_TZSID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_se_pka_tzsid_en` reader - TZSID enable for PKA in Secure Engine."]
pub type TZC_SE_PKA_TZSID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_se_pka_tzsid_en` writer - TZSID enable for PKA in Secure Engine."]
pub type TZC_SE_PKA_TZSID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_se_cdet_tzsid_en` reader - TZSID enable for code detection in Secure Engine."]
pub type TZC_SE_CDET_TZSID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_se_cdet_tzsid_en` writer - TZSID enable for code detection in Secure Engine."]
pub type TZC_SE_CDET_TZSID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_se_gmac_tzsid_en` reader - TZSID enable for GMAC in Secure Engine."]
pub type TZC_SE_GMAC_TZSID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_se_gmac_tzsid_en` writer - TZSID enable for GMAC in Secure Engine."]
pub type TZC_SE_GMAC_TZSID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_se_tzsid_crmd` reader - TZSID control for CRMD in Secure Engine."]
pub type TZC_SE_TZSID_CRMD_R = crate::BitReader;
#[doc = "Field `tzc_se_tzsid_crmd` writer - TZSID control for CRMD in Secure Engine."]
pub type TZC_SE_TZSID_CRMD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tzc_se_wdt_dly` reader - Watchdog timer delay in Secure Engine."]
pub type TZC_SE_WDT_DLY_R = crate::FieldReader<u16>;
#[doc = "Field `tzc_se_wdt_dly` writer - Watchdog timer delay in Secure Engine."]
pub type TZC_SE_WDT_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - TZSID enable for SHA in Secure Engine."]
    #[inline(always)]
    pub fn tzc_se_sha_tzsid_en(&self) -> TZC_SE_SHA_TZSID_EN_R {
        TZC_SE_SHA_TZSID_EN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - TZSID enable for AES in Secure Engine."]
    #[inline(always)]
    pub fn tzc_se_aes_tzsid_en(&self) -> TZC_SE_AES_TZSID_EN_R {
        TZC_SE_AES_TZSID_EN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - TZSID enable for TRNG in Secure Engine."]
    #[inline(always)]
    pub fn tzc_se_trng_tzsid_en(&self) -> TZC_SE_TRNG_TZSID_EN_R {
        TZC_SE_TRNG_TZSID_EN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - TZSID enable for PKA in Secure Engine."]
    #[inline(always)]
    pub fn tzc_se_pka_tzsid_en(&self) -> TZC_SE_PKA_TZSID_EN_R {
        TZC_SE_PKA_TZSID_EN_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TZSID enable for code detection in Secure Engine."]
    #[inline(always)]
    pub fn tzc_se_cdet_tzsid_en(&self) -> TZC_SE_CDET_TZSID_EN_R {
        TZC_SE_CDET_TZSID_EN_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - TZSID enable for GMAC in Secure Engine."]
    #[inline(always)]
    pub fn tzc_se_gmac_tzsid_en(&self) -> TZC_SE_GMAC_TZSID_EN_R {
        TZC_SE_GMAC_TZSID_EN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - TZSID control for CRMD in Secure Engine."]
    #[inline(always)]
    pub fn tzc_se_tzsid_crmd(&self) -> TZC_SE_TZSID_CRMD_R {
        TZC_SE_TZSID_CRMD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Watchdog timer delay in Secure Engine."]
    #[inline(always)]
    pub fn tzc_se_wdt_dly(&self) -> TZC_SE_WDT_DLY_R {
        TZC_SE_WDT_DLY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - TZSID enable for SHA in Secure Engine."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_se_sha_tzsid_en(&mut self) -> TZC_SE_SHA_TZSID_EN_W<TZC_SE_CTRL_0_SPEC> {
        TZC_SE_SHA_TZSID_EN_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - TZSID enable for AES in Secure Engine."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_se_aes_tzsid_en(&mut self) -> TZC_SE_AES_TZSID_EN_W<TZC_SE_CTRL_0_SPEC> {
        TZC_SE_AES_TZSID_EN_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - TZSID enable for TRNG in Secure Engine."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_se_trng_tzsid_en(&mut self) -> TZC_SE_TRNG_TZSID_EN_W<TZC_SE_CTRL_0_SPEC> {
        TZC_SE_TRNG_TZSID_EN_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - TZSID enable for PKA in Secure Engine."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_se_pka_tzsid_en(&mut self) -> TZC_SE_PKA_TZSID_EN_W<TZC_SE_CTRL_0_SPEC> {
        TZC_SE_PKA_TZSID_EN_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - TZSID enable for code detection in Secure Engine."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_se_cdet_tzsid_en(&mut self) -> TZC_SE_CDET_TZSID_EN_W<TZC_SE_CTRL_0_SPEC> {
        TZC_SE_CDET_TZSID_EN_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - TZSID enable for GMAC in Secure Engine."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_se_gmac_tzsid_en(&mut self) -> TZC_SE_GMAC_TZSID_EN_W<TZC_SE_CTRL_0_SPEC> {
        TZC_SE_GMAC_TZSID_EN_W::new(self, 10)
    }
    #[doc = "Bit 12 - TZSID control for CRMD in Secure Engine."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_se_tzsid_crmd(&mut self) -> TZC_SE_TZSID_CRMD_W<TZC_SE_CTRL_0_SPEC> {
        TZC_SE_TZSID_CRMD_W::new(self, 12)
    }
    #[doc = "Bits 16:31 - Watchdog timer delay in Secure Engine."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_se_wdt_dly(&mut self) -> TZC_SE_WDT_DLY_W<TZC_SE_CTRL_0_SPEC> {
        TZC_SE_WDT_DLY_W::new(self, 16)
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
#[doc = "TrustZone Controller Secure Engine Control 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_se_ctrl_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_se_ctrl_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_SE_CTRL_0_SPEC;
impl crate::RegisterSpec for TZC_SE_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_se_ctrl_0::R`](R) reader structure"]
impl crate::Readable for TZC_SE_CTRL_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tzc_se_ctrl_0::W`](W) writer structure"]
impl crate::Writable for TZC_SE_CTRL_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_se_ctrl_0 to value 0"]
impl crate::Resettable for TZC_SE_CTRL_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
