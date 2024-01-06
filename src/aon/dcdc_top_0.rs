#[doc = "Register `dcdc_top_0` reader"]
pub type R = crate::R<DCDC_TOP_0_SPEC>;
#[doc = "Register `dcdc_top_0` writer"]
pub type W = crate::W<DCDC_TOP_0_SPEC>;
#[doc = "Field `dcdc_vout_sel_aon` reader - "]
pub type DCDC_VOUT_SEL_AON_R = crate::FieldReader;
#[doc = "Field `dcdc_vout_sel_aon` writer - "]
pub type DCDC_VOUT_SEL_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `dcdc_vpfm_aon` reader - "]
pub type DCDC_VPFM_AON_R = crate::FieldReader;
#[doc = "Field `dcdc_vpfm_aon` writer - "]
pub type DCDC_VPFM_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dcdc_osc_2m_mode_aon` reader - "]
pub type DCDC_OSC_2M_MODE_AON_R = crate::BitReader;
#[doc = "Field `dcdc_osc_2m_mode_aon` writer - "]
pub type DCDC_OSC_2M_MODE_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc_osc_freq_trim_aon` reader - "]
pub type DCDC_OSC_FREQ_TRIM_AON_R = crate::FieldReader;
#[doc = "Field `dcdc_osc_freq_trim_aon` writer - "]
pub type DCDC_OSC_FREQ_TRIM_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dcdc_slope_curr_sel_aon` reader - "]
pub type DCDC_SLOPE_CURR_SEL_AON_R = crate::FieldReader;
#[doc = "Field `dcdc_slope_curr_sel_aon` writer - "]
pub type DCDC_SLOPE_CURR_SEL_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `dcdc_en_stop_osc_aon` reader - "]
pub type DCDC_EN_STOP_OSC_AON_R = crate::BitReader;
#[doc = "Field `dcdc_en_stop_osc_aon` writer - "]
pub type DCDC_EN_STOP_OSC_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc_en_slow_osc_aon` reader - "]
pub type DCDC_EN_SLOW_OSC_AON_R = crate::BitReader;
#[doc = "Field `dcdc_en_slow_osc_aon` writer - "]
pub type DCDC_EN_SLOW_OSC_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc_en_osc_inhibit_t2_aon` reader - "]
pub type DCDC_EN_OSC_INHIBIT_T2_AON_R = crate::BitReader;
#[doc = "Field `dcdc_en_osc_inhibit_t2_aon` writer - "]
pub type DCDC_EN_OSC_INHIBIT_T2_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc_sstart_time_aon` reader - "]
pub type DCDC_SSTART_TIME_AON_R = crate::FieldReader;
#[doc = "Field `dcdc_sstart_time_aon` writer - "]
pub type DCDC_SSTART_TIME_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dcdc_dis_aon` reader - "]
pub type DCDC_DIS_AON_R = crate::BitReader;
#[doc = "Field `dcdc_dis_aon` writer - "]
pub type DCDC_DIS_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc_rdy_aon` reader - "]
pub type DCDC_RDY_AON_R = crate::BitReader;
#[doc = "Field `dcdc_rdy_aon` writer - "]
pub type DCDC_RDY_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn dcdc_vout_sel_aon(&self) -> DCDC_VOUT_SEL_AON_R {
        DCDC_VOUT_SEL_AON_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn dcdc_vpfm_aon(&self) -> DCDC_VPFM_AON_R {
        DCDC_VPFM_AON_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dcdc_osc_2m_mode_aon(&self) -> DCDC_OSC_2M_MODE_AON_R {
        DCDC_OSC_2M_MODE_AON_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn dcdc_osc_freq_trim_aon(&self) -> DCDC_OSC_FREQ_TRIM_AON_R {
        DCDC_OSC_FREQ_TRIM_AON_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn dcdc_slope_curr_sel_aon(&self) -> DCDC_SLOPE_CURR_SEL_AON_R {
        DCDC_SLOPE_CURR_SEL_AON_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dcdc_en_stop_osc_aon(&self) -> DCDC_EN_STOP_OSC_AON_R {
        DCDC_EN_STOP_OSC_AON_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn dcdc_en_slow_osc_aon(&self) -> DCDC_EN_SLOW_OSC_AON_R {
        DCDC_EN_SLOW_OSC_AON_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn dcdc_en_osc_inhibit_t2_aon(&self) -> DCDC_EN_OSC_INHIBIT_T2_AON_R {
        DCDC_EN_OSC_INHIBIT_T2_AON_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn dcdc_sstart_time_aon(&self) -> DCDC_SSTART_TIME_AON_R {
        DCDC_SSTART_TIME_AON_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dcdc_dis_aon(&self) -> DCDC_DIS_AON_R {
        DCDC_DIS_AON_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dcdc_rdy_aon(&self) -> DCDC_RDY_AON_R {
        DCDC_RDY_AON_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_vout_sel_aon(&mut self) -> DCDC_VOUT_SEL_AON_W<DCDC_TOP_0_SPEC> {
        DCDC_VOUT_SEL_AON_W::new(self, 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_vpfm_aon(&mut self) -> DCDC_VPFM_AON_W<DCDC_TOP_0_SPEC> {
        DCDC_VPFM_AON_W::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_osc_2m_mode_aon(&mut self) -> DCDC_OSC_2M_MODE_AON_W<DCDC_TOP_0_SPEC> {
        DCDC_OSC_2M_MODE_AON_W::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_osc_freq_trim_aon(&mut self) -> DCDC_OSC_FREQ_TRIM_AON_W<DCDC_TOP_0_SPEC> {
        DCDC_OSC_FREQ_TRIM_AON_W::new(self, 16)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_slope_curr_sel_aon(&mut self) -> DCDC_SLOPE_CURR_SEL_AON_W<DCDC_TOP_0_SPEC> {
        DCDC_SLOPE_CURR_SEL_AON_W::new(self, 20)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_en_stop_osc_aon(&mut self) -> DCDC_EN_STOP_OSC_AON_W<DCDC_TOP_0_SPEC> {
        DCDC_EN_STOP_OSC_AON_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_en_slow_osc_aon(&mut self) -> DCDC_EN_SLOW_OSC_AON_W<DCDC_TOP_0_SPEC> {
        DCDC_EN_SLOW_OSC_AON_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_en_osc_inhibit_t2_aon(&mut self) -> DCDC_EN_OSC_INHIBIT_T2_AON_W<DCDC_TOP_0_SPEC> {
        DCDC_EN_OSC_INHIBIT_T2_AON_W::new(self, 27)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_sstart_time_aon(&mut self) -> DCDC_SSTART_TIME_AON_W<DCDC_TOP_0_SPEC> {
        DCDC_SSTART_TIME_AON_W::new(self, 28)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_dis_aon(&mut self) -> DCDC_DIS_AON_W<DCDC_TOP_0_SPEC> {
        DCDC_DIS_AON_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_rdy_aon(&mut self) -> DCDC_RDY_AON_W<DCDC_TOP_0_SPEC> {
        DCDC_RDY_AON_W::new(self, 31)
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
#[doc = "dcdc_top_0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdc_top_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdc_top_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCDC_TOP_0_SPEC;
impl crate::RegisterSpec for DCDC_TOP_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdc_top_0::R`](R) reader structure"]
impl crate::Readable for DCDC_TOP_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcdc_top_0::W`](W) writer structure"]
impl crate::Writable for DCDC_TOP_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dcdc_top_0 to value 0"]
impl crate::Resettable for DCDC_TOP_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
