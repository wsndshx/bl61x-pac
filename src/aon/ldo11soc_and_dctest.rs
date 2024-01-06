#[doc = "Register `ldo11soc_and_dctest` reader"]
pub type R = crate::R<LDO11SOC_AND_DCTEST_SPEC>;
#[doc = "Register `ldo11soc_and_dctest` writer"]
pub type W = crate::W<LDO11SOC_AND_DCTEST_SPEC>;
#[doc = "Field `ldo11soc_vout_trim_aon` reader - "]
pub type LDO11SOC_VOUT_TRIM_AON_R = crate::FieldReader;
#[doc = "Field `ldo11soc_vout_trim_aon` writer - "]
pub type LDO11SOC_VOUT_TRIM_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pu_ldo11soc_aon` reader - "]
pub type PU_LDO11SOC_AON_R = crate::BitReader;
#[doc = "Field `pu_ldo11soc_aon` writer - "]
pub type PU_LDO11SOC_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo11soc_sstart_en_aon` reader - "]
pub type LDO11SOC_SSTART_EN_AON_R = crate::BitReader;
#[doc = "Field `ldo11soc_sstart_en_aon` writer - "]
pub type LDO11SOC_SSTART_EN_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo11soc_sstart_delay_aon` reader - "]
pub type LDO11SOC_SSTART_DELAY_AON_R = crate::FieldReader;
#[doc = "Field `ldo11soc_sstart_delay_aon` writer - "]
pub type LDO11SOC_SSTART_DELAY_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ldo11soc_pulldown_aon` reader - "]
pub type LDO11SOC_PULLDOWN_AON_R = crate::BitReader;
#[doc = "Field `ldo11soc_pulldown_aon` writer - "]
pub type LDO11SOC_PULLDOWN_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo11soc_pulldown_sel_aon` reader - "]
pub type LDO11SOC_PULLDOWN_SEL_AON_R = crate::BitReader;
#[doc = "Field `ldo11soc_pulldown_sel_aon` writer - "]
pub type LDO11SOC_PULLDOWN_SEL_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo11soc_vth_sel_aon` reader - "]
pub type LDO11SOC_VTH_SEL_AON_R = crate::FieldReader;
#[doc = "Field `ldo11soc_vth_sel_aon` writer - "]
pub type LDO11SOC_VTH_SEL_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ldo11soc_cc_aon` reader - "]
pub type LDO11SOC_CC_AON_R = crate::FieldReader;
#[doc = "Field `ldo11soc_cc_aon` writer - "]
pub type LDO11SOC_CC_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ldo11soc_rdy_aon` reader - "]
pub type LDO11SOC_RDY_AON_R = crate::BitReader;
#[doc = "Field `ldo11soc_rdy_aon` writer - "]
pub type LDO11SOC_RDY_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo11soc_power_good_aon` reader - "]
pub type LDO11SOC_POWER_GOOD_AON_R = crate::BitReader;
#[doc = "Field `ldo11soc_power_good_aon` writer - "]
pub type LDO11SOC_POWER_GOOD_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pmip_dc_tp_out_en_aon` reader - "]
pub type PMIP_DC_TP_OUT_EN_AON_R = crate::BitReader;
#[doc = "Field `pmip_dc_tp_out_en_aon` writer - "]
pub type PMIP_DC_TP_OUT_EN_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ldo11soc_vout_trim_aon(&self) -> LDO11SOC_VOUT_TRIM_AON_R {
        LDO11SOC_VOUT_TRIM_AON_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pu_ldo11soc_aon(&self) -> PU_LDO11SOC_AON_R {
        PU_LDO11SOC_AON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ldo11soc_sstart_en_aon(&self) -> LDO11SOC_SSTART_EN_AON_R {
        LDO11SOC_SSTART_EN_AON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ldo11soc_sstart_delay_aon(&self) -> LDO11SOC_SSTART_DELAY_AON_R {
        LDO11SOC_SSTART_DELAY_AON_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ldo11soc_pulldown_aon(&self) -> LDO11SOC_PULLDOWN_AON_R {
        LDO11SOC_PULLDOWN_AON_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ldo11soc_pulldown_sel_aon(&self) -> LDO11SOC_PULLDOWN_SEL_AON_R {
        LDO11SOC_PULLDOWN_SEL_AON_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn ldo11soc_vth_sel_aon(&self) -> LDO11SOC_VTH_SEL_AON_R {
        LDO11SOC_VTH_SEL_AON_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ldo11soc_cc_aon(&self) -> LDO11SOC_CC_AON_R {
        LDO11SOC_CC_AON_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ldo11soc_rdy_aon(&self) -> LDO11SOC_RDY_AON_R {
        LDO11SOC_RDY_AON_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn ldo11soc_power_good_aon(&self) -> LDO11SOC_POWER_GOOD_AON_R {
        LDO11SOC_POWER_GOOD_AON_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pmip_dc_tp_out_en_aon(&self) -> PMIP_DC_TP_OUT_EN_AON_R {
        PMIP_DC_TP_OUT_EN_AON_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn ldo11soc_vout_trim_aon(&mut self) -> LDO11SOC_VOUT_TRIM_AON_W<LDO11SOC_AND_DCTEST_SPEC> {
        LDO11SOC_VOUT_TRIM_AON_W::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pu_ldo11soc_aon(&mut self) -> PU_LDO11SOC_AON_W<LDO11SOC_AND_DCTEST_SPEC> {
        PU_LDO11SOC_AON_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn ldo11soc_sstart_en_aon(&mut self) -> LDO11SOC_SSTART_EN_AON_W<LDO11SOC_AND_DCTEST_SPEC> {
        LDO11SOC_SSTART_EN_AON_W::new(self, 5)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn ldo11soc_sstart_delay_aon(
        &mut self,
    ) -> LDO11SOC_SSTART_DELAY_AON_W<LDO11SOC_AND_DCTEST_SPEC> {
        LDO11SOC_SSTART_DELAY_AON_W::new(self, 8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn ldo11soc_pulldown_aon(&mut self) -> LDO11SOC_PULLDOWN_AON_W<LDO11SOC_AND_DCTEST_SPEC> {
        LDO11SOC_PULLDOWN_AON_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn ldo11soc_pulldown_sel_aon(
        &mut self,
    ) -> LDO11SOC_PULLDOWN_SEL_AON_W<LDO11SOC_AND_DCTEST_SPEC> {
        LDO11SOC_PULLDOWN_SEL_AON_W::new(self, 11)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn ldo11soc_vth_sel_aon(&mut self) -> LDO11SOC_VTH_SEL_AON_W<LDO11SOC_AND_DCTEST_SPEC> {
        LDO11SOC_VTH_SEL_AON_W::new(self, 12)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn ldo11soc_cc_aon(&mut self) -> LDO11SOC_CC_AON_W<LDO11SOC_AND_DCTEST_SPEC> {
        LDO11SOC_CC_AON_W::new(self, 24)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn ldo11soc_rdy_aon(&mut self) -> LDO11SOC_RDY_AON_W<LDO11SOC_AND_DCTEST_SPEC> {
        LDO11SOC_RDY_AON_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn ldo11soc_power_good_aon(
        &mut self,
    ) -> LDO11SOC_POWER_GOOD_AON_W<LDO11SOC_AND_DCTEST_SPEC> {
        LDO11SOC_POWER_GOOD_AON_W::new(self, 29)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn pmip_dc_tp_out_en_aon(&mut self) -> PMIP_DC_TP_OUT_EN_AON_W<LDO11SOC_AND_DCTEST_SPEC> {
        PMIP_DC_TP_OUT_EN_AON_W::new(self, 31)
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
#[doc = "ldo11soc_and_dctest.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ldo11soc_and_dctest::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ldo11soc_and_dctest::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LDO11SOC_AND_DCTEST_SPEC;
impl crate::RegisterSpec for LDO11SOC_AND_DCTEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ldo11soc_and_dctest::R`](R) reader structure"]
impl crate::Readable for LDO11SOC_AND_DCTEST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ldo11soc_and_dctest::W`](W) writer structure"]
impl crate::Writable for LDO11SOC_AND_DCTEST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ldo11soc_and_dctest to value 0"]
impl crate::Resettable for LDO11SOC_AND_DCTEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
