#[doc = "Register `gpadc_reg_cmd` reader"]
pub type R = crate::R<GPADC_REG_CMD_SPEC>;
#[doc = "Register `gpadc_reg_cmd` writer"]
pub type W = crate::W<GPADC_REG_CMD_SPEC>;
#[doc = "Field `gpadc_global_en` reader - "]
pub type GPADC_GLOBAL_EN_R = crate::BitReader;
#[doc = "Field `gpadc_global_en` writer - "]
pub type GPADC_GLOBAL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_conv_start` reader - "]
pub type GPADC_CONV_START_R = crate::BitReader;
#[doc = "Field `gpadc_conv_start` writer - "]
pub type GPADC_CONV_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_soft_rst` reader - "]
pub type GPADC_SOFT_RST_R = crate::BitReader;
#[doc = "Field `gpadc_soft_rst` writer - "]
pub type GPADC_SOFT_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_neg_sel` reader - "]
pub type GPADC_NEG_SEL_R = crate::FieldReader;
#[doc = "Field `gpadc_neg_sel` writer - "]
pub type GPADC_NEG_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_pos_sel` reader - "]
pub type GPADC_POS_SEL_R = crate::FieldReader;
#[doc = "Field `gpadc_pos_sel` writer - "]
pub type GPADC_POS_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_neg_gnd` reader - "]
pub type GPADC_NEG_GND_R = crate::BitReader;
#[doc = "Field `gpadc_neg_gnd` writer - "]
pub type GPADC_NEG_GND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_micbias_en` reader - "]
pub type GPADC_MICBIAS_EN_R = crate::BitReader;
#[doc = "Field `gpadc_micbias_en` writer - "]
pub type GPADC_MICBIAS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_micpga_en` reader - "]
pub type GPADC_MICPGA_EN_R = crate::BitReader;
#[doc = "Field `gpadc_micpga_en` writer - "]
pub type GPADC_MICPGA_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_byp_micboost` reader - "]
pub type GPADC_BYP_MICBOOST_R = crate::BitReader;
#[doc = "Field `gpadc_byp_micboost` writer - "]
pub type GPADC_BYP_MICBOOST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_rcal_en` reader - "]
pub type GPADC_RCAL_EN_R = crate::BitReader;
#[doc = "Field `gpadc_rcal_en` writer - "]
pub type GPADC_RCAL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_dwa_en` reader - "]
pub type GPADC_DWA_EN_R = crate::BitReader;
#[doc = "Field `gpadc_dwa_en` writer - "]
pub type GPADC_DWA_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_mic2_diff` reader - "]
pub type GPADC_MIC2_DIFF_R = crate::BitReader;
#[doc = "Field `gpadc_mic2_diff` writer - "]
pub type GPADC_MIC2_DIFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_mic1_diff` reader - "]
pub type GPADC_MIC1_DIFF_R = crate::BitReader;
#[doc = "Field `gpadc_mic1_diff` writer - "]
pub type GPADC_MIC1_DIFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_mic_pga2_gain` reader - "]
pub type GPADC_MIC_PGA2_GAIN_R = crate::FieldReader;
#[doc = "Field `gpadc_mic_pga2_gain` writer - "]
pub type GPADC_MIC_PGA2_GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gpadc_micboost_32db_en` reader - "]
pub type GPADC_MICBOOST_32DB_EN_R = crate::BitReader;
#[doc = "Field `gpadc_micboost_32db_en` writer - "]
pub type GPADC_MICBOOST_32DB_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_chip_sen_pu` reader - "]
pub type GPADC_CHIP_SEN_PU_R = crate::BitReader;
#[doc = "Field `gpadc_chip_sen_pu` writer - "]
pub type GPADC_CHIP_SEN_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_sen_sel` reader - "]
pub type GPADC_SEN_SEL_R = crate::FieldReader;
#[doc = "Field `gpadc_sen_sel` writer - "]
pub type GPADC_SEN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gpadc_sen_test_en` reader - "]
pub type GPADC_SEN_TEST_EN_R = crate::BitReader;
#[doc = "Field `gpadc_sen_test_en` writer - "]
pub type GPADC_SEN_TEST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_global_en(&self) -> GPADC_GLOBAL_EN_R {
        GPADC_GLOBAL_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpadc_conv_start(&self) -> GPADC_CONV_START_R {
        GPADC_CONV_START_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpadc_soft_rst(&self) -> GPADC_SOFT_RST_R {
        GPADC_SOFT_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    pub fn gpadc_neg_sel(&self) -> GPADC_NEG_SEL_R {
        GPADC_NEG_SEL_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn gpadc_pos_sel(&self) -> GPADC_POS_SEL_R {
        GPADC_POS_SEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpadc_neg_gnd(&self) -> GPADC_NEG_GND_R {
        GPADC_NEG_GND_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpadc_micbias_en(&self) -> GPADC_MICBIAS_EN_R {
        GPADC_MICBIAS_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gpadc_micpga_en(&self) -> GPADC_MICPGA_EN_R {
        GPADC_MICPGA_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gpadc_byp_micboost(&self) -> GPADC_BYP_MICBOOST_R {
        GPADC_BYP_MICBOOST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpadc_rcal_en(&self) -> GPADC_RCAL_EN_R {
        GPADC_RCAL_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpadc_dwa_en(&self) -> GPADC_DWA_EN_R {
        GPADC_DWA_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn gpadc_mic2_diff(&self) -> GPADC_MIC2_DIFF_R {
        GPADC_MIC2_DIFF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn gpadc_mic1_diff(&self) -> GPADC_MIC1_DIFF_R {
        GPADC_MIC1_DIFF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn gpadc_mic_pga2_gain(&self) -> GPADC_MIC_PGA2_GAIN_R {
        GPADC_MIC_PGA2_GAIN_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn gpadc_micboost_32db_en(&self) -> GPADC_MICBOOST_32DB_EN_R {
        GPADC_MICBOOST_32DB_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn gpadc_chip_sen_pu(&self) -> GPADC_CHIP_SEN_PU_R {
        GPADC_CHIP_SEN_PU_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn gpadc_sen_sel(&self) -> GPADC_SEN_SEL_R {
        GPADC_SEN_SEL_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpadc_sen_test_en(&self) -> GPADC_SEN_TEST_EN_R {
        GPADC_SEN_TEST_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_global_en(&mut self) -> GPADC_GLOBAL_EN_W<GPADC_REG_CMD_SPEC> {
        GPADC_GLOBAL_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_conv_start(&mut self) -> GPADC_CONV_START_W<GPADC_REG_CMD_SPEC> {
        GPADC_CONV_START_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_soft_rst(&mut self) -> GPADC_SOFT_RST_W<GPADC_REG_CMD_SPEC> {
        GPADC_SOFT_RST_W::new(self, 2)
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_neg_sel(&mut self) -> GPADC_NEG_SEL_W<GPADC_REG_CMD_SPEC> {
        GPADC_NEG_SEL_W::new(self, 3)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_pos_sel(&mut self) -> GPADC_POS_SEL_W<GPADC_REG_CMD_SPEC> {
        GPADC_POS_SEL_W::new(self, 8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_neg_gnd(&mut self) -> GPADC_NEG_GND_W<GPADC_REG_CMD_SPEC> {
        GPADC_NEG_GND_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_micbias_en(&mut self) -> GPADC_MICBIAS_EN_W<GPADC_REG_CMD_SPEC> {
        GPADC_MICBIAS_EN_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_micpga_en(&mut self) -> GPADC_MICPGA_EN_W<GPADC_REG_CMD_SPEC> {
        GPADC_MICPGA_EN_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_byp_micboost(&mut self) -> GPADC_BYP_MICBOOST_W<GPADC_REG_CMD_SPEC> {
        GPADC_BYP_MICBOOST_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_rcal_en(&mut self) -> GPADC_RCAL_EN_W<GPADC_REG_CMD_SPEC> {
        GPADC_RCAL_EN_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_dwa_en(&mut self) -> GPADC_DWA_EN_W<GPADC_REG_CMD_SPEC> {
        GPADC_DWA_EN_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_mic2_diff(&mut self) -> GPADC_MIC2_DIFF_W<GPADC_REG_CMD_SPEC> {
        GPADC_MIC2_DIFF_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_mic1_diff(&mut self) -> GPADC_MIC1_DIFF_W<GPADC_REG_CMD_SPEC> {
        GPADC_MIC1_DIFF_W::new(self, 20)
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_mic_pga2_gain(&mut self) -> GPADC_MIC_PGA2_GAIN_W<GPADC_REG_CMD_SPEC> {
        GPADC_MIC_PGA2_GAIN_W::new(self, 21)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_micboost_32db_en(&mut self) -> GPADC_MICBOOST_32DB_EN_W<GPADC_REG_CMD_SPEC> {
        GPADC_MICBOOST_32DB_EN_W::new(self, 23)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_chip_sen_pu(&mut self) -> GPADC_CHIP_SEN_PU_W<GPADC_REG_CMD_SPEC> {
        GPADC_CHIP_SEN_PU_W::new(self, 27)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_sen_sel(&mut self) -> GPADC_SEN_SEL_W<GPADC_REG_CMD_SPEC> {
        GPADC_SEN_SEL_W::new(self, 28)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_sen_test_en(&mut self) -> GPADC_SEN_TEST_EN_W<GPADC_REG_CMD_SPEC> {
        GPADC_SEN_TEST_EN_W::new(self, 31)
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
#[doc = "gpadc_reg_cmd.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpadc_reg_cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpadc_reg_cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPADC_REG_CMD_SPEC;
impl crate::RegisterSpec for GPADC_REG_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpadc_reg_cmd::R`](R) reader structure"]
impl crate::Readable for GPADC_REG_CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpadc_reg_cmd::W`](W) writer structure"]
impl crate::Writable for GPADC_REG_CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpadc_reg_cmd to value 0"]
impl crate::Resettable for GPADC_REG_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
