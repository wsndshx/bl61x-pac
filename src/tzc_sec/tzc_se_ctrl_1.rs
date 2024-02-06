#[doc = "Register `tzc_se_ctrl_1` reader"]
pub type R = crate::R<TZC_SE_CTRL_1_SPEC>;
#[doc = "Register `tzc_se_ctrl_1` writer"]
pub type W = crate::W<TZC_SE_CTRL_1_SPEC>;
#[doc = "Field `tzc_sf_cr_tzsid_en` reader - TZSID enable for CR in Secure Engine."]
pub type TZC_SF_CR_TZSID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_sf_cr_tzsid_en` writer - TZSID enable for CR in Secure Engine."]
pub type TZC_SF_CR_TZSID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_sf_sec_tzsid_en` reader - TZSID enable for security in Secure Engine."]
pub type TZC_SF_SEC_TZSID_EN_R = crate::FieldReader;
#[doc = "Field `tzc_sf_sec_tzsid_en` writer - TZSID enable for security in Secure Engine."]
pub type TZC_SF_SEC_TZSID_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tzc_sf_tzsid_crmd` reader - TZSID control for CRMD in Secure Engine."]
pub type TZC_SF_TZSID_CRMD_R = crate::BitReader;
#[doc = "Field `tzc_sf_tzsid_crmd` writer - TZSID control for CRMD in Secure Engine."]
pub type TZC_SF_TZSID_CRMD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - TZSID enable for CR in Secure Engine."]
    #[inline(always)]
    pub fn tzc_sf_cr_tzsid_en(&self) -> TZC_SF_CR_TZSID_EN_R {
        TZC_SF_CR_TZSID_EN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - TZSID enable for security in Secure Engine."]
    #[inline(always)]
    pub fn tzc_sf_sec_tzsid_en(&self) -> TZC_SF_SEC_TZSID_EN_R {
        TZC_SF_SEC_TZSID_EN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - TZSID control for CRMD in Secure Engine."]
    #[inline(always)]
    pub fn tzc_sf_tzsid_crmd(&self) -> TZC_SF_TZSID_CRMD_R {
        TZC_SF_TZSID_CRMD_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - TZSID enable for CR in Secure Engine."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_cr_tzsid_en(&mut self) -> TZC_SF_CR_TZSID_EN_W<TZC_SE_CTRL_1_SPEC> {
        TZC_SF_CR_TZSID_EN_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - TZSID enable for security in Secure Engine."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_sec_tzsid_en(&mut self) -> TZC_SF_SEC_TZSID_EN_W<TZC_SE_CTRL_1_SPEC> {
        TZC_SF_SEC_TZSID_EN_W::new(self, 2)
    }
    #[doc = "Bit 4 - TZSID control for CRMD in Secure Engine."]
    #[inline(always)]
    #[must_use]
    pub fn tzc_sf_tzsid_crmd(&mut self) -> TZC_SF_TZSID_CRMD_W<TZC_SE_CTRL_1_SPEC> {
        TZC_SF_TZSID_CRMD_W::new(self, 4)
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
#[doc = "TrustZone Controller Secure Engine Control 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_se_ctrl_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_se_ctrl_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_SE_CTRL_1_SPEC;
impl crate::RegisterSpec for TZC_SE_CTRL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_se_ctrl_1::R`](R) reader structure"]
impl crate::Readable for TZC_SE_CTRL_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tzc_se_ctrl_1::W`](W) writer structure"]
impl crate::Writable for TZC_SE_CTRL_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_se_ctrl_1 to value 0"]
impl crate::Resettable for TZC_SE_CTRL_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
