#[doc = "Register `rf_top_aon` reader"]
pub type R = crate::R<RF_TOP_AON_SPEC>;
#[doc = "Register `rf_top_aon` writer"]
pub type W = crate::W<RF_TOP_AON_SPEC>;
#[doc = "Field `pu_mbg_aon` reader - "]
pub type PU_MBG_AON_R = crate::BitReader;
#[doc = "Field `pu_mbg_aon` writer - "]
pub type PU_MBG_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_ldo15rf_aon` reader - "]
pub type PU_LDO15RF_AON_R = crate::BitReader;
#[doc = "Field `pu_ldo15rf_aon` writer - "]
pub type PU_LDO15RF_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_sfreg_aon` reader - "]
pub type PU_SFREG_AON_R = crate::BitReader;
#[doc = "Field `pu_sfreg_aon` writer - "]
pub type PU_SFREG_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_xtal_buf_aon` reader - "]
pub type PU_XTAL_BUF_AON_R = crate::BitReader;
#[doc = "Field `pu_xtal_buf_aon` writer - "]
pub type PU_XTAL_BUF_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_xtal_aon` reader - "]
pub type PU_XTAL_AON_R = crate::BitReader;
#[doc = "Field `pu_xtal_aon` writer - "]
pub type PU_XTAL_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo15rf_sstart_sel_aon` reader - "]
pub type LDO15RF_SSTART_SEL_AON_R = crate::BitReader;
#[doc = "Field `ldo15rf_sstart_sel_aon` writer - "]
pub type LDO15RF_SSTART_SEL_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo15rf_sstart_delay_aon` reader - "]
pub type LDO15RF_SSTART_DELAY_AON_R = crate::FieldReader;
#[doc = "Field `ldo15rf_sstart_delay_aon` writer - "]
pub type LDO15RF_SSTART_DELAY_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ldo15rf_pulldown_aon` reader - "]
pub type LDO15RF_PULLDOWN_AON_R = crate::BitReader;
#[doc = "Field `ldo15rf_pulldown_aon` writer - "]
pub type LDO15RF_PULLDOWN_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo15rf_pulldown_sel_aon` reader - "]
pub type LDO15RF_PULLDOWN_SEL_AON_R = crate::BitReader;
#[doc = "Field `ldo15rf_pulldown_sel_aon` writer - "]
pub type LDO15RF_PULLDOWN_SEL_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo15rf_vout_sel_aon` reader - "]
pub type LDO15RF_VOUT_SEL_AON_R = crate::FieldReader;
#[doc = "Field `ldo15rf_vout_sel_aon` writer - "]
pub type LDO15RF_VOUT_SEL_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ldo15rf_cc_aon` reader - "]
pub type LDO15RF_CC_AON_R = crate::FieldReader;
#[doc = "Field `ldo15rf_cc_aon` writer - "]
pub type LDO15RF_CC_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ldo15rf_bypass_aon` reader - "]
pub type LDO15RF_BYPASS_AON_R = crate::BitReader;
#[doc = "Field `ldo15rf_bypass_aon` writer - "]
pub type LDO15RF_BYPASS_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo15rf_vout_trim_aon` reader - "]
pub type LDO15RF_VOUT_TRIM_AON_R = crate::FieldReader;
#[doc = "Field `ldo15rf_vout_trim_aon` writer - "]
pub type LDO15RF_VOUT_TRIM_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_mbg_aon(&self) -> PU_MBG_AON_R {
        PU_MBG_AON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pu_ldo15rf_aon(&self) -> PU_LDO15RF_AON_R {
        PU_LDO15RF_AON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pu_sfreg_aon(&self) -> PU_SFREG_AON_R {
        PU_SFREG_AON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pu_xtal_buf_aon(&self) -> PU_XTAL_BUF_AON_R {
        PU_XTAL_BUF_AON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_xtal_aon(&self) -> PU_XTAL_AON_R {
        PU_XTAL_AON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ldo15rf_sstart_sel_aon(&self) -> LDO15RF_SSTART_SEL_AON_R {
        LDO15RF_SSTART_SEL_AON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn ldo15rf_sstart_delay_aon(&self) -> LDO15RF_SSTART_DELAY_AON_R {
        LDO15RF_SSTART_DELAY_AON_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ldo15rf_pulldown_aon(&self) -> LDO15RF_PULLDOWN_AON_R {
        LDO15RF_PULLDOWN_AON_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ldo15rf_pulldown_sel_aon(&self) -> LDO15RF_PULLDOWN_SEL_AON_R {
        LDO15RF_PULLDOWN_SEL_AON_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn ldo15rf_vout_sel_aon(&self) -> LDO15RF_VOUT_SEL_AON_R {
        LDO15RF_VOUT_SEL_AON_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ldo15rf_cc_aon(&self) -> LDO15RF_CC_AON_R {
        LDO15RF_CC_AON_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ldo15rf_bypass_aon(&self) -> LDO15RF_BYPASS_AON_R {
        LDO15RF_BYPASS_AON_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn ldo15rf_vout_trim_aon(&self) -> LDO15RF_VOUT_TRIM_AON_R {
        LDO15RF_VOUT_TRIM_AON_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pu_mbg_aon(&mut self) -> PU_MBG_AON_W<RF_TOP_AON_SPEC> {
        PU_MBG_AON_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pu_ldo15rf_aon(&mut self) -> PU_LDO15RF_AON_W<RF_TOP_AON_SPEC> {
        PU_LDO15RF_AON_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pu_sfreg_aon(&mut self) -> PU_SFREG_AON_W<RF_TOP_AON_SPEC> {
        PU_SFREG_AON_W::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pu_xtal_buf_aon(&mut self) -> PU_XTAL_BUF_AON_W<RF_TOP_AON_SPEC> {
        PU_XTAL_BUF_AON_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pu_xtal_aon(&mut self) -> PU_XTAL_AON_W<RF_TOP_AON_SPEC> {
        PU_XTAL_AON_W::new(self, 5)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn ldo15rf_sstart_sel_aon(&mut self) -> LDO15RF_SSTART_SEL_AON_W<RF_TOP_AON_SPEC> {
        LDO15RF_SSTART_SEL_AON_W::new(self, 8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    #[must_use]
    pub fn ldo15rf_sstart_delay_aon(&mut self) -> LDO15RF_SSTART_DELAY_AON_W<RF_TOP_AON_SPEC> {
        LDO15RF_SSTART_DELAY_AON_W::new(self, 9)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn ldo15rf_pulldown_aon(&mut self) -> LDO15RF_PULLDOWN_AON_W<RF_TOP_AON_SPEC> {
        LDO15RF_PULLDOWN_AON_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn ldo15rf_pulldown_sel_aon(&mut self) -> LDO15RF_PULLDOWN_SEL_AON_W<RF_TOP_AON_SPEC> {
        LDO15RF_PULLDOWN_SEL_AON_W::new(self, 13)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn ldo15rf_vout_sel_aon(&mut self) -> LDO15RF_VOUT_SEL_AON_W<RF_TOP_AON_SPEC> {
        LDO15RF_VOUT_SEL_AON_W::new(self, 16)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn ldo15rf_cc_aon(&mut self) -> LDO15RF_CC_AON_W<RF_TOP_AON_SPEC> {
        LDO15RF_CC_AON_W::new(self, 24)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn ldo15rf_bypass_aon(&mut self) -> LDO15RF_BYPASS_AON_W<RF_TOP_AON_SPEC> {
        LDO15RF_BYPASS_AON_W::new(self, 27)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn ldo15rf_vout_trim_aon(&mut self) -> LDO15RF_VOUT_TRIM_AON_W<RF_TOP_AON_SPEC> {
        LDO15RF_VOUT_TRIM_AON_W::new(self, 28)
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
#[doc = "rf_top_aon.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rf_top_aon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rf_top_aon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RF_TOP_AON_SPEC;
impl crate::RegisterSpec for RF_TOP_AON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf_top_aon::R`](R) reader structure"]
impl crate::Readable for RF_TOP_AON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rf_top_aon::W`](W) writer structure"]
impl crate::Writable for RF_TOP_AON_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rf_top_aon to value 0"]
impl crate::Resettable for RF_TOP_AON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
