#[doc = "Register `dcdc_top_1` reader"]
pub type R = crate::R<DCDC_TOP_1_SPEC>;
#[doc = "Register `dcdc_top_1` writer"]
pub type W = crate::W<DCDC_TOP_1_SPEC>;
#[doc = "Field `dcdc_force_en_cs_zvs_aon` reader - "]
pub type DCDC_FORCE_EN_CS_ZVS_AON_R = crate::BitReader;
#[doc = "Field `dcdc_force_en_cs_zvs_aon` writer - "]
pub type DCDC_FORCE_EN_CS_ZVS_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc_cs_delay_aon` reader - "]
pub type DCDC_CS_DELAY_AON_R = crate::FieldReader;
#[doc = "Field `dcdc_cs_delay_aon` writer - "]
pub type DCDC_CS_DELAY_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dcdc_zvs_td_opt_aon` reader - "]
pub type DCDC_ZVS_TD_OPT_AON_R = crate::FieldReader;
#[doc = "Field `dcdc_zvs_td_opt_aon` writer - "]
pub type DCDC_ZVS_TD_OPT_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dcdc_nonoverlap_td_aon` reader - "]
pub type DCDC_NONOVERLAP_TD_AON_R = crate::FieldReader;
#[doc = "Field `dcdc_nonoverlap_td_aon` writer - "]
pub type DCDC_NONOVERLAP_TD_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `dcdc_rc_sel_aon` reader - "]
pub type DCDC_RC_SEL_AON_R = crate::FieldReader;
#[doc = "Field `dcdc_rc_sel_aon` writer - "]
pub type DCDC_RC_SEL_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dcdc_chf_sel_aon` reader - "]
pub type DCDC_CHF_SEL_AON_R = crate::FieldReader;
#[doc = "Field `dcdc_chf_sel_aon` writer - "]
pub type DCDC_CHF_SEL_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dcdc_cfb_sel_aon` reader - "]
pub type DCDC_CFB_SEL_AON_R = crate::FieldReader;
#[doc = "Field `dcdc_cfb_sel_aon` writer - "]
pub type DCDC_CFB_SEL_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dcdc_en_antiring_aon` reader - "]
pub type DCDC_EN_ANTIRING_AON_R = crate::BitReader;
#[doc = "Field `dcdc_en_antiring_aon` writer - "]
pub type DCDC_EN_ANTIRING_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc_pulldown_aon` reader - "]
pub type DCDC_PULLDOWN_AON_R = crate::BitReader;
#[doc = "Field `dcdc_pulldown_aon` writer - "]
pub type DCDC_PULLDOWN_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dcdc_force_en_cs_zvs_aon(&self) -> DCDC_FORCE_EN_CS_ZVS_AON_R {
        DCDC_FORCE_EN_CS_ZVS_AON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn dcdc_cs_delay_aon(&self) -> DCDC_CS_DELAY_AON_R {
        DCDC_CS_DELAY_AON_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn dcdc_zvs_td_opt_aon(&self) -> DCDC_ZVS_TD_OPT_AON_R {
        DCDC_ZVS_TD_OPT_AON_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn dcdc_nonoverlap_td_aon(&self) -> DCDC_NONOVERLAP_TD_AON_R {
        DCDC_NONOVERLAP_TD_AON_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn dcdc_rc_sel_aon(&self) -> DCDC_RC_SEL_AON_R {
        DCDC_RC_SEL_AON_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn dcdc_chf_sel_aon(&self) -> DCDC_CHF_SEL_AON_R {
        DCDC_CHF_SEL_AON_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn dcdc_cfb_sel_aon(&self) -> DCDC_CFB_SEL_AON_R {
        DCDC_CFB_SEL_AON_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dcdc_en_antiring_aon(&self) -> DCDC_EN_ANTIRING_AON_R {
        DCDC_EN_ANTIRING_AON_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dcdc_pulldown_aon(&self) -> DCDC_PULLDOWN_AON_R {
        DCDC_PULLDOWN_AON_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_force_en_cs_zvs_aon(&mut self) -> DCDC_FORCE_EN_CS_ZVS_AON_W<DCDC_TOP_1_SPEC> {
        DCDC_FORCE_EN_CS_ZVS_AON_W::new(self, 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_cs_delay_aon(&mut self) -> DCDC_CS_DELAY_AON_W<DCDC_TOP_1_SPEC> {
        DCDC_CS_DELAY_AON_W::new(self, 1)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_zvs_td_opt_aon(&mut self) -> DCDC_ZVS_TD_OPT_AON_W<DCDC_TOP_1_SPEC> {
        DCDC_ZVS_TD_OPT_AON_W::new(self, 4)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_nonoverlap_td_aon(&mut self) -> DCDC_NONOVERLAP_TD_AON_W<DCDC_TOP_1_SPEC> {
        DCDC_NONOVERLAP_TD_AON_W::new(self, 8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_rc_sel_aon(&mut self) -> DCDC_RC_SEL_AON_W<DCDC_TOP_1_SPEC> {
        DCDC_RC_SEL_AON_W::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_chf_sel_aon(&mut self) -> DCDC_CHF_SEL_AON_W<DCDC_TOP_1_SPEC> {
        DCDC_CHF_SEL_AON_W::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_cfb_sel_aon(&mut self) -> DCDC_CFB_SEL_AON_W<DCDC_TOP_1_SPEC> {
        DCDC_CFB_SEL_AON_W::new(self, 24)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_en_antiring_aon(&mut self) -> DCDC_EN_ANTIRING_AON_W<DCDC_TOP_1_SPEC> {
        DCDC_EN_ANTIRING_AON_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_pulldown_aon(&mut self) -> DCDC_PULLDOWN_AON_W<DCDC_TOP_1_SPEC> {
        DCDC_PULLDOWN_AON_W::new(self, 29)
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
#[doc = "dcdc_top_1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdc_top_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdc_top_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCDC_TOP_1_SPEC;
impl crate::RegisterSpec for DCDC_TOP_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdc_top_1::R`](R) reader structure"]
impl crate::Readable for DCDC_TOP_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcdc_top_1::W`](W) writer structure"]
impl crate::Writable for DCDC_TOP_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dcdc_top_1 to value 0"]
impl crate::Resettable for DCDC_TOP_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
