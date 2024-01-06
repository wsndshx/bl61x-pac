#[doc = "Register `dcdc_top_2` reader"]
pub type R = crate::R<DCDC_TOP_2_SPEC>;
#[doc = "Register `dcdc_top_2` writer"]
pub type W = crate::W<DCDC_TOP_2_SPEC>;
#[doc = "Field `dcdc_vout_trim_aon` reader - "]
pub type DCDC_VOUT_TRIM_AON_R = crate::FieldReader;
#[doc = "Field `dcdc_vout_trim_aon` writer - "]
pub type DCDC_VOUT_TRIM_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dcdc_osc_ss_rstn_aon` reader - "]
pub type DCDC_OSC_SS_RSTN_AON_R = crate::BitReader;
#[doc = "Field `dcdc_osc_ss_rstn_aon` writer - "]
pub type DCDC_OSC_SS_RSTN_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc_osc_ss_en_aon` reader - "]
pub type DCDC_OSC_SS_EN_AON_R = crate::BitReader;
#[doc = "Field `dcdc_osc_ss_en_aon` writer - "]
pub type DCDC_OSC_SS_EN_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc_osc_ss_period_aon` reader - "]
pub type DCDC_OSC_SS_PERIOD_AON_R = crate::FieldReader;
#[doc = "Field `dcdc_osc_ss_period_aon` writer - "]
pub type DCDC_OSC_SS_PERIOD_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dcdc_osc_ss_fdev_aon` reader - "]
pub type DCDC_OSC_SS_FDEV_AON_R = crate::FieldReader;
#[doc = "Field `dcdc_osc_ss_fdev_aon` writer - "]
pub type DCDC_OSC_SS_FDEV_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `dcdc_comp_gm_sel_aon` reader - "]
pub type DCDC_COMP_GM_SEL_AON_R = crate::FieldReader;
#[doc = "Field `dcdc_comp_gm_sel_aon` writer - "]
pub type DCDC_COMP_GM_SEL_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dcdc_isense_trim_aon` reader - "]
pub type DCDC_ISENSE_TRIM_AON_R = crate::FieldReader;
#[doc = "Field `dcdc_isense_trim_aon` writer - "]
pub type DCDC_ISENSE_TRIM_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dcdc_vc_clamp_vth_aon` reader - "]
pub type DCDC_VC_CLAMP_VTH_AON_R = crate::FieldReader;
#[doc = "Field `dcdc_vc_clamp_vth_aon` writer - "]
pub type DCDC_VC_CLAMP_VTH_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dcdc_ocp_vth_aon` reader - "]
pub type DCDC_OCP_VTH_AON_R = crate::FieldReader;
#[doc = "Field `dcdc_ocp_vth_aon` writer - "]
pub type DCDC_OCP_VTH_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dcdc_ocp_rst_aon` reader - "]
pub type DCDC_OCP_RST_AON_R = crate::BitReader;
#[doc = "Field `dcdc_ocp_rst_aon` writer - "]
pub type DCDC_OCP_RST_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc_ocp_out_aon` reader - "]
pub type DCDC_OCP_OUT_AON_R = crate::BitReader;
#[doc = "Field `dcdc_ocp_out_aon` writer - "]
pub type DCDC_OCP_OUT_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcdc_drv_sr_aon` reader - "]
pub type DCDC_DRV_SR_AON_R = crate::FieldReader;
#[doc = "Field `dcdc_drv_sr_aon` writer - "]
pub type DCDC_DRV_SR_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dcdc_vout_trim_aon(&self) -> DCDC_VOUT_TRIM_AON_R {
        DCDC_VOUT_TRIM_AON_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dcdc_osc_ss_rstn_aon(&self) -> DCDC_OSC_SS_RSTN_AON_R {
        DCDC_OSC_SS_RSTN_AON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dcdc_osc_ss_en_aon(&self) -> DCDC_OSC_SS_EN_AON_R {
        DCDC_OSC_SS_EN_AON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn dcdc_osc_ss_period_aon(&self) -> DCDC_OSC_SS_PERIOD_AON_R {
        DCDC_OSC_SS_PERIOD_AON_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn dcdc_osc_ss_fdev_aon(&self) -> DCDC_OSC_SS_FDEV_AON_R {
        DCDC_OSC_SS_FDEV_AON_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn dcdc_comp_gm_sel_aon(&self) -> DCDC_COMP_GM_SEL_AON_R {
        DCDC_COMP_GM_SEL_AON_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn dcdc_isense_trim_aon(&self) -> DCDC_ISENSE_TRIM_AON_R {
        DCDC_ISENSE_TRIM_AON_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn dcdc_vc_clamp_vth_aon(&self) -> DCDC_VC_CLAMP_VTH_AON_R {
        DCDC_VC_CLAMP_VTH_AON_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn dcdc_ocp_vth_aon(&self) -> DCDC_OCP_VTH_AON_R {
        DCDC_OCP_VTH_AON_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn dcdc_ocp_rst_aon(&self) -> DCDC_OCP_RST_AON_R {
        DCDC_OCP_RST_AON_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dcdc_ocp_out_aon(&self) -> DCDC_OCP_OUT_AON_R {
        DCDC_OCP_OUT_AON_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn dcdc_drv_sr_aon(&self) -> DCDC_DRV_SR_AON_R {
        DCDC_DRV_SR_AON_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_vout_trim_aon(&mut self) -> DCDC_VOUT_TRIM_AON_W<DCDC_TOP_2_SPEC> {
        DCDC_VOUT_TRIM_AON_W::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_osc_ss_rstn_aon(&mut self) -> DCDC_OSC_SS_RSTN_AON_W<DCDC_TOP_2_SPEC> {
        DCDC_OSC_SS_RSTN_AON_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_osc_ss_en_aon(&mut self) -> DCDC_OSC_SS_EN_AON_W<DCDC_TOP_2_SPEC> {
        DCDC_OSC_SS_EN_AON_W::new(self, 5)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_osc_ss_period_aon(&mut self) -> DCDC_OSC_SS_PERIOD_AON_W<DCDC_TOP_2_SPEC> {
        DCDC_OSC_SS_PERIOD_AON_W::new(self, 8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_osc_ss_fdev_aon(&mut self) -> DCDC_OSC_SS_FDEV_AON_W<DCDC_TOP_2_SPEC> {
        DCDC_OSC_SS_FDEV_AON_W::new(self, 10)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_comp_gm_sel_aon(&mut self) -> DCDC_COMP_GM_SEL_AON_W<DCDC_TOP_2_SPEC> {
        DCDC_COMP_GM_SEL_AON_W::new(self, 12)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_isense_trim_aon(&mut self) -> DCDC_ISENSE_TRIM_AON_W<DCDC_TOP_2_SPEC> {
        DCDC_ISENSE_TRIM_AON_W::new(self, 16)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_vc_clamp_vth_aon(&mut self) -> DCDC_VC_CLAMP_VTH_AON_W<DCDC_TOP_2_SPEC> {
        DCDC_VC_CLAMP_VTH_AON_W::new(self, 20)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_ocp_vth_aon(&mut self) -> DCDC_OCP_VTH_AON_W<DCDC_TOP_2_SPEC> {
        DCDC_OCP_VTH_AON_W::new(self, 24)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_ocp_rst_aon(&mut self) -> DCDC_OCP_RST_AON_W<DCDC_TOP_2_SPEC> {
        DCDC_OCP_RST_AON_W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_ocp_out_aon(&mut self) -> DCDC_OCP_OUT_AON_W<DCDC_TOP_2_SPEC> {
        DCDC_OCP_OUT_AON_W::new(self, 28)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_drv_sr_aon(&mut self) -> DCDC_DRV_SR_AON_W<DCDC_TOP_2_SPEC> {
        DCDC_DRV_SR_AON_W::new(self, 29)
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
#[doc = "dcdc_top_2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdc_top_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdc_top_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCDC_TOP_2_SPEC;
impl crate::RegisterSpec for DCDC_TOP_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdc_top_2::R`](R) reader structure"]
impl crate::Readable for DCDC_TOP_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcdc_top_2::W`](W) writer structure"]
impl crate::Writable for DCDC_TOP_2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dcdc_top_2 to value 0"]
impl crate::Resettable for DCDC_TOP_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
