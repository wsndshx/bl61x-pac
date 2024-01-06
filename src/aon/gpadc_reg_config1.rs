#[doc = "Register `gpadc_reg_config1` reader"]
pub type R = crate::R<GPADC_REG_CONFIG1_SPEC>;
#[doc = "Register `gpadc_reg_config1` writer"]
pub type W = crate::W<GPADC_REG_CONFIG1_SPEC>;
#[doc = "Field `gpadc_cal_os_en` reader - "]
pub type GPADC_CAL_OS_EN_R = crate::BitReader;
#[doc = "Field `gpadc_cal_os_en` writer - "]
pub type GPADC_CAL_OS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_cont_conv_en` reader - "]
pub type GPADC_CONT_CONV_EN_R = crate::BitReader;
#[doc = "Field `gpadc_cont_conv_en` writer - "]
pub type GPADC_CONT_CONV_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_res_sel` reader - "]
pub type GPADC_RES_SEL_R = crate::FieldReader;
#[doc = "Field `gpadc_res_sel` writer - "]
pub type GPADC_RES_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gpadc_vcm_sel_en` reader - "]
pub type GPADC_VCM_SEL_EN_R = crate::BitReader;
#[doc = "Field `gpadc_vcm_sel_en` writer - "]
pub type GPADC_VCM_SEL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_vcm_hyst_sel` reader - "]
pub type GPADC_VCM_HYST_SEL_R = crate::BitReader;
#[doc = "Field `gpadc_vcm_hyst_sel` writer - "]
pub type GPADC_VCM_HYST_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_lowv_det_en` reader - "]
pub type GPADC_LOWV_DET_EN_R = crate::BitReader;
#[doc = "Field `gpadc_lowv_det_en` writer - "]
pub type GPADC_LOWV_DET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_pwm_trg_en` reader - "]
pub type GPADC_PWM_TRG_EN_R = crate::BitReader;
#[doc = "Field `gpadc_pwm_trg_en` writer - "]
pub type GPADC_PWM_TRG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_clk_ana_dly` reader - "]
pub type GPADC_CLK_ANA_DLY_R = crate::FieldReader;
#[doc = "Field `gpadc_clk_ana_dly` writer - "]
pub type GPADC_CLK_ANA_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `gpadc_clk_ana_dly_en` reader - "]
pub type GPADC_CLK_ANA_DLY_EN_R = crate::BitReader;
#[doc = "Field `gpadc_clk_ana_dly_en` writer - "]
pub type GPADC_CLK_ANA_DLY_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_clk_ana_inv` reader - "]
pub type GPADC_CLK_ANA_INV_R = crate::BitReader;
#[doc = "Field `gpadc_clk_ana_inv` writer - "]
pub type GPADC_CLK_ANA_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_clk_div_ratio` reader - "]
pub type GPADC_CLK_DIV_RATIO_R = crate::FieldReader;
#[doc = "Field `gpadc_clk_div_ratio` writer - "]
pub type GPADC_CLK_DIV_RATIO_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gpadc_scan_length` reader - "]
pub type GPADC_SCAN_LENGTH_R = crate::FieldReader;
#[doc = "Field `gpadc_scan_length` writer - "]
pub type GPADC_SCAN_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `gpadc_scan_en` reader - "]
pub type GPADC_SCAN_EN_R = crate::BitReader;
#[doc = "Field `gpadc_scan_en` writer - "]
pub type GPADC_SCAN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_dither_en` reader - "]
pub type GPADC_DITHER_EN_R = crate::BitReader;
#[doc = "Field `gpadc_dither_en` writer - "]
pub type GPADC_DITHER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_v11_sel` reader - "]
pub type GPADC_V11_SEL_R = crate::FieldReader;
#[doc = "Field `gpadc_v11_sel` writer - "]
pub type GPADC_V11_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gpadc_v18_sel` reader - "]
pub type GPADC_V18_SEL_R = crate::FieldReader;
#[doc = "Field `gpadc_v18_sel` writer - "]
pub type GPADC_V18_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_cal_os_en(&self) -> GPADC_CAL_OS_EN_R {
        GPADC_CAL_OS_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpadc_cont_conv_en(&self) -> GPADC_CONT_CONV_EN_R {
        GPADC_CONT_CONV_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn gpadc_res_sel(&self) -> GPADC_RES_SEL_R {
        GPADC_RES_SEL_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_vcm_sel_en(&self) -> GPADC_VCM_SEL_EN_R {
        GPADC_VCM_SEL_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpadc_vcm_hyst_sel(&self) -> GPADC_VCM_HYST_SEL_R {
        GPADC_VCM_HYST_SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpadc_lowv_det_en(&self) -> GPADC_LOWV_DET_EN_R {
        GPADC_LOWV_DET_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn gpadc_pwm_trg_en(&self) -> GPADC_PWM_TRG_EN_R {
        GPADC_PWM_TRG_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn gpadc_clk_ana_dly(&self) -> GPADC_CLK_ANA_DLY_R {
        GPADC_CLK_ANA_DLY_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gpadc_clk_ana_dly_en(&self) -> GPADC_CLK_ANA_DLY_EN_R {
        GPADC_CLK_ANA_DLY_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpadc_clk_ana_inv(&self) -> GPADC_CLK_ANA_INV_R {
        GPADC_CLK_ANA_INV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn gpadc_clk_div_ratio(&self) -> GPADC_CLK_DIV_RATIO_R {
        GPADC_CLK_DIV_RATIO_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:24"]
    #[inline(always)]
    pub fn gpadc_scan_length(&self) -> GPADC_SCAN_LENGTH_R {
        GPADC_SCAN_LENGTH_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn gpadc_scan_en(&self) -> GPADC_SCAN_EN_R {
        GPADC_SCAN_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn gpadc_dither_en(&self) -> GPADC_DITHER_EN_R {
        GPADC_DITHER_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn gpadc_v11_sel(&self) -> GPADC_V11_SEL_R {
        GPADC_V11_SEL_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn gpadc_v18_sel(&self) -> GPADC_V18_SEL_R {
        GPADC_V18_SEL_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_cal_os_en(&mut self) -> GPADC_CAL_OS_EN_W<GPADC_REG_CONFIG1_SPEC> {
        GPADC_CAL_OS_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_cont_conv_en(&mut self) -> GPADC_CONT_CONV_EN_W<GPADC_REG_CONFIG1_SPEC> {
        GPADC_CONT_CONV_EN_W::new(self, 1)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_res_sel(&mut self) -> GPADC_RES_SEL_W<GPADC_REG_CONFIG1_SPEC> {
        GPADC_RES_SEL_W::new(self, 2)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_vcm_sel_en(&mut self) -> GPADC_VCM_SEL_EN_W<GPADC_REG_CONFIG1_SPEC> {
        GPADC_VCM_SEL_EN_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_vcm_hyst_sel(&mut self) -> GPADC_VCM_HYST_SEL_W<GPADC_REG_CONFIG1_SPEC> {
        GPADC_VCM_HYST_SEL_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_lowv_det_en(&mut self) -> GPADC_LOWV_DET_EN_W<GPADC_REG_CONFIG1_SPEC> {
        GPADC_LOWV_DET_EN_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_pwm_trg_en(&mut self) -> GPADC_PWM_TRG_EN_W<GPADC_REG_CONFIG1_SPEC> {
        GPADC_PWM_TRG_EN_W::new(self, 11)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_clk_ana_dly(&mut self) -> GPADC_CLK_ANA_DLY_W<GPADC_REG_CONFIG1_SPEC> {
        GPADC_CLK_ANA_DLY_W::new(self, 12)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_clk_ana_dly_en(&mut self) -> GPADC_CLK_ANA_DLY_EN_W<GPADC_REG_CONFIG1_SPEC> {
        GPADC_CLK_ANA_DLY_EN_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_clk_ana_inv(&mut self) -> GPADC_CLK_ANA_INV_W<GPADC_REG_CONFIG1_SPEC> {
        GPADC_CLK_ANA_INV_W::new(self, 17)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_clk_div_ratio(&mut self) -> GPADC_CLK_DIV_RATIO_W<GPADC_REG_CONFIG1_SPEC> {
        GPADC_CLK_DIV_RATIO_W::new(self, 18)
    }
    #[doc = "Bits 21:24"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_length(&mut self) -> GPADC_SCAN_LENGTH_W<GPADC_REG_CONFIG1_SPEC> {
        GPADC_SCAN_LENGTH_W::new(self, 21)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_en(&mut self) -> GPADC_SCAN_EN_W<GPADC_REG_CONFIG1_SPEC> {
        GPADC_SCAN_EN_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_dither_en(&mut self) -> GPADC_DITHER_EN_W<GPADC_REG_CONFIG1_SPEC> {
        GPADC_DITHER_EN_W::new(self, 26)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_v11_sel(&mut self) -> GPADC_V11_SEL_W<GPADC_REG_CONFIG1_SPEC> {
        GPADC_V11_SEL_W::new(self, 27)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_v18_sel(&mut self) -> GPADC_V18_SEL_W<GPADC_REG_CONFIG1_SPEC> {
        GPADC_V18_SEL_W::new(self, 29)
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
#[doc = "gpadc_reg_config1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpadc_reg_config1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpadc_reg_config1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPADC_REG_CONFIG1_SPEC;
impl crate::RegisterSpec for GPADC_REG_CONFIG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpadc_reg_config1::R`](R) reader structure"]
impl crate::Readable for GPADC_REG_CONFIG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpadc_reg_config1::W`](W) writer structure"]
impl crate::Writable for GPADC_REG_CONFIG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpadc_reg_config1 to value 0"]
impl crate::Resettable for GPADC_REG_CONFIG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
