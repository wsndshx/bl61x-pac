#[doc = "Register `gpadc_reg_config2` reader"]
pub type R = crate::R<GPADC_REG_CONFIG2_SPEC>;
#[doc = "Register `gpadc_reg_config2` writer"]
pub type W = crate::W<GPADC_REG_CONFIG2_SPEC>;
#[doc = "Field `gpadc_scan_pos_0` reader - "]
pub type GPADC_SCAN_POS_0_R = crate::FieldReader;
#[doc = "Field `gpadc_scan_pos_0` writer - "]
pub type GPADC_SCAN_POS_0_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_pos_6` reader - "]
pub type GPADC_SCAN_POS_6_R = crate::FieldReader;
#[doc = "Field `gpadc_scan_pos_6` writer - "]
pub type GPADC_SCAN_POS_6_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_neg_0` reader - "]
pub type GPADC_SCAN_NEG_0_R = crate::FieldReader;
#[doc = "Field `gpadc_scan_neg_0` writer - "]
pub type GPADC_SCAN_NEG_0_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_neg_6` reader - "]
pub type GPADC_SCAN_NEG_6_R = crate::FieldReader;
#[doc = "Field `gpadc_scan_neg_6` writer - "]
pub type GPADC_SCAN_NEG_6_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_diff_mode` reader - "]
pub type GPADC_DIFF_MODE_R = crate::BitReader;
#[doc = "Field `gpadc_diff_mode` writer - "]
pub type GPADC_DIFF_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_vref_sel` reader - "]
pub type GPADC_VREF_SEL_R = crate::BitReader;
#[doc = "Field `gpadc_vref_sel` writer - "]
pub type GPADC_VREF_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_vbat_en` reader - "]
pub type GPADC_VBAT_EN_R = crate::BitReader;
#[doc = "Field `gpadc_vbat_en` writer - "]
pub type GPADC_VBAT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_tsext_sel` reader - "]
pub type GPADC_TSEXT_SEL_R = crate::BitReader;
#[doc = "Field `gpadc_tsext_sel` writer - "]
pub type GPADC_TSEXT_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_scan_pos_1` reader - "]
pub type GPADC_SCAN_POS_1_R = crate::FieldReader;
#[doc = "Field `gpadc_scan_pos_1` writer - "]
pub type GPADC_SCAN_POS_1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_pos_7` reader - "]
pub type GPADC_SCAN_POS_7_R = crate::FieldReader;
#[doc = "Field `gpadc_scan_pos_7` writer - "]
pub type GPADC_SCAN_POS_7_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_neg_1` reader - "]
pub type GPADC_SCAN_NEG_1_R = crate::FieldReader;
#[doc = "Field `gpadc_scan_neg_1` writer - "]
pub type GPADC_SCAN_NEG_1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_neg_7` reader - "]
pub type GPADC_SCAN_NEG_7_R = crate::FieldReader;
#[doc = "Field `gpadc_scan_neg_7` writer - "]
pub type GPADC_SCAN_NEG_7_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_ts_en` reader - "]
pub type GPADC_TS_EN_R = crate::BitReader;
#[doc = "Field `gpadc_ts_en` writer - "]
pub type GPADC_TS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_pga_vcm` reader - "]
pub type GPADC_PGA_VCM_R = crate::FieldReader;
#[doc = "Field `gpadc_pga_vcm` writer - "]
pub type GPADC_PGA_VCM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gpadc_pga_os_cal` reader - "]
pub type GPADC_PGA_OS_CAL_R = crate::FieldReader;
#[doc = "Field `gpadc_pga_os_cal` writer - "]
pub type GPADC_PGA_OS_CAL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `gpadc_scan_pos_2` reader - "]
pub type GPADC_SCAN_POS_2_R = crate::FieldReader;
#[doc = "Field `gpadc_scan_pos_2` writer - "]
pub type GPADC_SCAN_POS_2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_pos_8` reader - "]
pub type GPADC_SCAN_POS_8_R = crate::FieldReader;
#[doc = "Field `gpadc_scan_pos_8` writer - "]
pub type GPADC_SCAN_POS_8_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_neg_2` reader - "]
pub type GPADC_SCAN_NEG_2_R = crate::FieldReader;
#[doc = "Field `gpadc_scan_neg_2` writer - "]
pub type GPADC_SCAN_NEG_2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_neg_8` reader - "]
pub type GPADC_SCAN_NEG_8_R = crate::FieldReader;
#[doc = "Field `gpadc_scan_neg_8` writer - "]
pub type GPADC_SCAN_NEG_8_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_pga_en` reader - "]
pub type GPADC_PGA_EN_R = crate::BitReader;
#[doc = "Field `gpadc_pga_en` writer - "]
pub type GPADC_PGA_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_pga_vcmi_en` reader - "]
pub type GPADC_PGA_VCMI_EN_R = crate::BitReader;
#[doc = "Field `gpadc_pga_vcmi_en` writer - "]
pub type GPADC_PGA_VCMI_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_chop_mode` reader - "]
pub type GPADC_CHOP_MODE_R = crate::FieldReader;
#[doc = "Field `gpadc_chop_mode` writer - "]
pub type GPADC_CHOP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gpadc_scan_pos_3` reader - "]
pub type GPADC_SCAN_POS_3_R = crate::FieldReader;
#[doc = "Field `gpadc_scan_pos_3` writer - "]
pub type GPADC_SCAN_POS_3_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_pos_9` reader - "]
pub type GPADC_SCAN_POS_9_R = crate::FieldReader;
#[doc = "Field `gpadc_scan_pos_9` writer - "]
pub type GPADC_SCAN_POS_9_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_neg_3` reader - "]
pub type GPADC_SCAN_NEG_3_R = crate::FieldReader;
#[doc = "Field `gpadc_scan_neg_3` writer - "]
pub type GPADC_SCAN_NEG_3_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_neg_9` reader - "]
pub type GPADC_SCAN_NEG_9_R = crate::FieldReader;
#[doc = "Field `gpadc_scan_neg_9` writer - "]
pub type GPADC_SCAN_NEG_9_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_bias_sel` reader - "]
pub type GPADC_BIAS_SEL_R = crate::BitReader;
#[doc = "Field `gpadc_bias_sel` writer - "]
pub type GPADC_BIAS_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_test_en` reader - "]
pub type GPADC_TEST_EN_R = crate::BitReader;
#[doc = "Field `gpadc_test_en` writer - "]
pub type GPADC_TEST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_test_sel` reader - "]
pub type GPADC_TEST_SEL_R = crate::FieldReader;
#[doc = "Field `gpadc_test_sel` writer - "]
pub type GPADC_TEST_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gpadc_scan_pos_4` reader - "]
pub type GPADC_SCAN_POS_4_R = crate::FieldReader;
#[doc = "Field `gpadc_scan_pos_4` writer - "]
pub type GPADC_SCAN_POS_4_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_pos_10` reader - "]
pub type GPADC_SCAN_POS_10_R = crate::FieldReader;
#[doc = "Field `gpadc_scan_pos_10` writer - "]
pub type GPADC_SCAN_POS_10_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_neg_4` reader - "]
pub type GPADC_SCAN_NEG_4_R = crate::FieldReader;
#[doc = "Field `gpadc_scan_neg_4` writer - "]
pub type GPADC_SCAN_NEG_4_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_neg_10` reader - "]
pub type GPADC_SCAN_NEG_10_R = crate::FieldReader;
#[doc = "Field `gpadc_scan_neg_10` writer - "]
pub type GPADC_SCAN_NEG_10_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_pga2_gain` reader - "]
pub type GPADC_PGA2_GAIN_R = crate::FieldReader;
#[doc = "Field `gpadc_pga2_gain` writer - "]
pub type GPADC_PGA2_GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gpadc_pga1_gain` reader - "]
pub type GPADC_PGA1_GAIN_R = crate::FieldReader;
#[doc = "Field `gpadc_pga1_gain` writer - "]
pub type GPADC_PGA1_GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gpadc_scan_pos_5` reader - "]
pub type GPADC_SCAN_POS_5_R = crate::FieldReader;
#[doc = "Field `gpadc_scan_pos_5` writer - "]
pub type GPADC_SCAN_POS_5_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_pos_11` reader - "]
pub type GPADC_SCAN_POS_11_R = crate::FieldReader;
#[doc = "Field `gpadc_scan_pos_11` writer - "]
pub type GPADC_SCAN_POS_11_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_neg_5` reader - "]
pub type GPADC_SCAN_NEG_5_R = crate::FieldReader;
#[doc = "Field `gpadc_scan_neg_5` writer - "]
pub type GPADC_SCAN_NEG_5_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_scan_neg_11` reader - "]
pub type GPADC_SCAN_NEG_11_R = crate::FieldReader;
#[doc = "Field `gpadc_scan_neg_11` writer - "]
pub type GPADC_SCAN_NEG_11_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `gpadc_dly_sel` reader - "]
pub type GPADC_DLY_SEL_R = crate::FieldReader;
#[doc = "Field `gpadc_dly_sel` writer - "]
pub type GPADC_DLY_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gpadc_tsvbe_low` reader - "]
pub type GPADC_TSVBE_LOW_R = crate::BitReader;
#[doc = "Field `gpadc_tsvbe_low` writer - "]
pub type GPADC_TSVBE_LOW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gpadc_scan_pos_0(&self) -> GPADC_SCAN_POS_0_R {
        GPADC_SCAN_POS_0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gpadc_scan_pos_6(&self) -> GPADC_SCAN_POS_6_R {
        GPADC_SCAN_POS_6_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gpadc_scan_neg_0(&self) -> GPADC_SCAN_NEG_0_R {
        GPADC_SCAN_NEG_0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gpadc_scan_neg_6(&self) -> GPADC_SCAN_NEG_6_R {
        GPADC_SCAN_NEG_6_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpadc_diff_mode(&self) -> GPADC_DIFF_MODE_R {
        GPADC_DIFF_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpadc_vref_sel(&self) -> GPADC_VREF_SEL_R {
        GPADC_VREF_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpadc_vbat_en(&self) -> GPADC_VBAT_EN_R {
        GPADC_VBAT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpadc_tsext_sel(&self) -> GPADC_TSEXT_SEL_R {
        GPADC_TSEXT_SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn gpadc_scan_pos_1(&self) -> GPADC_SCAN_POS_1_R {
        GPADC_SCAN_POS_1_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn gpadc_scan_pos_7(&self) -> GPADC_SCAN_POS_7_R {
        GPADC_SCAN_POS_7_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn gpadc_scan_neg_1(&self) -> GPADC_SCAN_NEG_1_R {
        GPADC_SCAN_NEG_1_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn gpadc_scan_neg_7(&self) -> GPADC_SCAN_NEG_7_R {
        GPADC_SCAN_NEG_7_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpadc_ts_en(&self) -> GPADC_TS_EN_R {
        GPADC_TS_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn gpadc_pga_vcm(&self) -> GPADC_PGA_VCM_R {
        GPADC_PGA_VCM_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:12"]
    #[inline(always)]
    pub fn gpadc_pga_os_cal(&self) -> GPADC_PGA_OS_CAL_R {
        GPADC_PGA_OS_CAL_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn gpadc_scan_pos_2(&self) -> GPADC_SCAN_POS_2_R {
        GPADC_SCAN_POS_2_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn gpadc_scan_pos_8(&self) -> GPADC_SCAN_POS_8_R {
        GPADC_SCAN_POS_8_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn gpadc_scan_neg_2(&self) -> GPADC_SCAN_NEG_2_R {
        GPADC_SCAN_NEG_2_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn gpadc_scan_neg_8(&self) -> GPADC_SCAN_NEG_8_R {
        GPADC_SCAN_NEG_8_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpadc_pga_en(&self) -> GPADC_PGA_EN_R {
        GPADC_PGA_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpadc_pga_vcmi_en(&self) -> GPADC_PGA_VCMI_EN_R {
        GPADC_PGA_VCMI_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn gpadc_chop_mode(&self) -> GPADC_CHOP_MODE_R {
        GPADC_CHOP_MODE_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn gpadc_scan_pos_3(&self) -> GPADC_SCAN_POS_3_R {
        GPADC_SCAN_POS_3_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn gpadc_scan_pos_9(&self) -> GPADC_SCAN_POS_9_R {
        GPADC_SCAN_POS_9_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn gpadc_scan_neg_3(&self) -> GPADC_SCAN_NEG_3_R {
        GPADC_SCAN_NEG_3_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn gpadc_scan_neg_9(&self) -> GPADC_SCAN_NEG_9_R {
        GPADC_SCAN_NEG_9_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpadc_bias_sel(&self) -> GPADC_BIAS_SEL_R {
        GPADC_BIAS_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpadc_test_en(&self) -> GPADC_TEST_EN_R {
        GPADC_TEST_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21"]
    #[inline(always)]
    pub fn gpadc_test_sel(&self) -> GPADC_TEST_SEL_R {
        GPADC_TEST_SEL_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn gpadc_scan_pos_4(&self) -> GPADC_SCAN_POS_4_R {
        GPADC_SCAN_POS_4_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn gpadc_scan_pos_10(&self) -> GPADC_SCAN_POS_10_R {
        GPADC_SCAN_POS_10_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn gpadc_scan_neg_4(&self) -> GPADC_SCAN_NEG_4_R {
        GPADC_SCAN_NEG_4_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn gpadc_scan_neg_10(&self) -> GPADC_SCAN_NEG_10_R {
        GPADC_SCAN_NEG_10_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 22:24"]
    #[inline(always)]
    pub fn gpadc_pga2_gain(&self) -> GPADC_PGA2_GAIN_R {
        GPADC_PGA2_GAIN_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn gpadc_pga1_gain(&self) -> GPADC_PGA1_GAIN_R {
        GPADC_PGA1_GAIN_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 25:29"]
    #[inline(always)]
    pub fn gpadc_scan_pos_5(&self) -> GPADC_SCAN_POS_5_R {
        GPADC_SCAN_POS_5_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29"]
    #[inline(always)]
    pub fn gpadc_scan_pos_11(&self) -> GPADC_SCAN_POS_11_R {
        GPADC_SCAN_POS_11_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29"]
    #[inline(always)]
    pub fn gpadc_scan_neg_5(&self) -> GPADC_SCAN_NEG_5_R {
        GPADC_SCAN_NEG_5_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29"]
    #[inline(always)]
    pub fn gpadc_scan_neg_11(&self) -> GPADC_SCAN_NEG_11_R {
        GPADC_SCAN_NEG_11_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn gpadc_dly_sel(&self) -> GPADC_DLY_SEL_R {
        GPADC_DLY_SEL_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpadc_tsvbe_low(&self) -> GPADC_TSVBE_LOW_R {
        GPADC_TSVBE_LOW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_pos_0(&mut self) -> GPADC_SCAN_POS_0_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_SCAN_POS_0_W::new(self, 0)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_pos_6(&mut self) -> GPADC_SCAN_POS_6_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_SCAN_POS_6_W::new(self, 0)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_neg_0(&mut self) -> GPADC_SCAN_NEG_0_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_SCAN_NEG_0_W::new(self, 0)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_neg_6(&mut self) -> GPADC_SCAN_NEG_6_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_SCAN_NEG_6_W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_diff_mode(&mut self) -> GPADC_DIFF_MODE_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_DIFF_MODE_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_vref_sel(&mut self) -> GPADC_VREF_SEL_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_VREF_SEL_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_vbat_en(&mut self) -> GPADC_VBAT_EN_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_VBAT_EN_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_tsext_sel(&mut self) -> GPADC_TSEXT_SEL_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_TSEXT_SEL_W::new(self, 5)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_pos_1(&mut self) -> GPADC_SCAN_POS_1_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_SCAN_POS_1_W::new(self, 5)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_pos_7(&mut self) -> GPADC_SCAN_POS_7_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_SCAN_POS_7_W::new(self, 5)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_neg_1(&mut self) -> GPADC_SCAN_NEG_1_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_SCAN_NEG_1_W::new(self, 5)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_neg_7(&mut self) -> GPADC_SCAN_NEG_7_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_SCAN_NEG_7_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_ts_en(&mut self) -> GPADC_TS_EN_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_TS_EN_W::new(self, 6)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_pga_vcm(&mut self) -> GPADC_PGA_VCM_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_PGA_VCM_W::new(self, 7)
    }
    #[doc = "Bits 9:12"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_pga_os_cal(&mut self) -> GPADC_PGA_OS_CAL_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_PGA_OS_CAL_W::new(self, 9)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_pos_2(&mut self) -> GPADC_SCAN_POS_2_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_SCAN_POS_2_W::new(self, 10)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_pos_8(&mut self) -> GPADC_SCAN_POS_8_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_SCAN_POS_8_W::new(self, 10)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_neg_2(&mut self) -> GPADC_SCAN_NEG_2_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_SCAN_NEG_2_W::new(self, 10)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_neg_8(&mut self) -> GPADC_SCAN_NEG_8_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_SCAN_NEG_8_W::new(self, 10)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_pga_en(&mut self) -> GPADC_PGA_EN_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_PGA_EN_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_pga_vcmi_en(&mut self) -> GPADC_PGA_VCMI_EN_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_PGA_VCMI_EN_W::new(self, 14)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_chop_mode(&mut self) -> GPADC_CHOP_MODE_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_CHOP_MODE_W::new(self, 15)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_pos_3(&mut self) -> GPADC_SCAN_POS_3_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_SCAN_POS_3_W::new(self, 15)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_pos_9(&mut self) -> GPADC_SCAN_POS_9_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_SCAN_POS_9_W::new(self, 15)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_neg_3(&mut self) -> GPADC_SCAN_NEG_3_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_SCAN_NEG_3_W::new(self, 15)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_neg_9(&mut self) -> GPADC_SCAN_NEG_9_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_SCAN_NEG_9_W::new(self, 15)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_bias_sel(&mut self) -> GPADC_BIAS_SEL_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_BIAS_SEL_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_test_en(&mut self) -> GPADC_TEST_EN_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_TEST_EN_W::new(self, 18)
    }
    #[doc = "Bits 19:21"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_test_sel(&mut self) -> GPADC_TEST_SEL_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_TEST_SEL_W::new(self, 19)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_pos_4(&mut self) -> GPADC_SCAN_POS_4_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_SCAN_POS_4_W::new(self, 20)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_pos_10(&mut self) -> GPADC_SCAN_POS_10_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_SCAN_POS_10_W::new(self, 20)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_neg_4(&mut self) -> GPADC_SCAN_NEG_4_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_SCAN_NEG_4_W::new(self, 20)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_neg_10(&mut self) -> GPADC_SCAN_NEG_10_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_SCAN_NEG_10_W::new(self, 20)
    }
    #[doc = "Bits 22:24"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_pga2_gain(&mut self) -> GPADC_PGA2_GAIN_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_PGA2_GAIN_W::new(self, 22)
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_pga1_gain(&mut self) -> GPADC_PGA1_GAIN_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_PGA1_GAIN_W::new(self, 25)
    }
    #[doc = "Bits 25:29"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_pos_5(&mut self) -> GPADC_SCAN_POS_5_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_SCAN_POS_5_W::new(self, 25)
    }
    #[doc = "Bits 25:29"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_pos_11(&mut self) -> GPADC_SCAN_POS_11_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_SCAN_POS_11_W::new(self, 25)
    }
    #[doc = "Bits 25:29"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_neg_5(&mut self) -> GPADC_SCAN_NEG_5_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_SCAN_NEG_5_W::new(self, 25)
    }
    #[doc = "Bits 25:29"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_scan_neg_11(&mut self) -> GPADC_SCAN_NEG_11_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_SCAN_NEG_11_W::new(self, 25)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_dly_sel(&mut self) -> GPADC_DLY_SEL_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_DLY_SEL_W::new(self, 28)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_tsvbe_low(&mut self) -> GPADC_TSVBE_LOW_W<GPADC_REG_CONFIG2_SPEC> {
        GPADC_TSVBE_LOW_W::new(self, 31)
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
#[doc = "gpadc_reg_config2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpadc_reg_config2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpadc_reg_config2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPADC_REG_CONFIG2_SPEC;
impl crate::RegisterSpec for GPADC_REG_CONFIG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpadc_reg_config2::R`](R) reader structure"]
impl crate::Readable for GPADC_REG_CONFIG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpadc_reg_config2::W`](W) writer structure"]
impl crate::Writable for GPADC_REG_CONFIG2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpadc_reg_config2 to value 0"]
impl crate::Resettable for GPADC_REG_CONFIG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
