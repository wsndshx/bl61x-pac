#[doc = "Register `gpdac_config_0` reader"]
pub type R = crate::R<GPDAC_CONFIG_0_SPEC>;
#[doc = "Register `gpdac_config_0` writer"]
pub type W = crate::W<GPDAC_CONFIG_0_SPEC>;
#[doc = "Field `gpdaca_rstn_ana` reader - "]
pub type GPDACA_RSTN_ANA_R = crate::BitReader;
#[doc = "Field `gpdaca_rstn_ana` writer - "]
pub type GPDACA_RSTN_ANA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpdacb_rstn_ana` reader - "]
pub type GPDACB_RSTN_ANA_R = crate::BitReader;
#[doc = "Field `gpdacb_rstn_ana` writer - "]
pub type GPDACB_RSTN_ANA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpdac_test_en` reader - "]
pub type GPDAC_TEST_EN_R = crate::BitReader;
#[doc = "Field `gpdac_test_en` writer - "]
pub type GPDAC_TEST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpdac_ref_sel` reader - "]
pub type GPDAC_REF_SEL_R = crate::BitReader;
#[doc = "Field `gpdac_ref_sel` writer - "]
pub type GPDAC_REF_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpdac_test_sel` reader - "]
pub type GPDAC_TEST_SEL_R = crate::FieldReader;
#[doc = "Field `gpdac_test_sel` writer - "]
pub type GPDAC_TEST_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gpdac_ana_clk_sel` reader - "]
pub type GPDAC_ANA_CLK_SEL_R = crate::BitReader;
#[doc = "Field `gpdac_ana_clk_sel` writer - "]
pub type GPDAC_ANA_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpdac_dat_cha_sel` reader - "]
pub type GPDAC_DAT_CHA_SEL_R = crate::BitReader;
#[doc = "Field `gpdac_dat_cha_sel` writer - "]
pub type GPDAC_DAT_CHA_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpdac_dat_chb_sel` reader - "]
pub type GPDAC_DAT_CHB_SEL_R = crate::BitReader;
#[doc = "Field `gpdac_dat_chb_sel` writer - "]
pub type GPDAC_DAT_CHB_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdaca_rstn_ana(&self) -> GPDACA_RSTN_ANA_R {
        GPDACA_RSTN_ANA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdacb_rstn_ana(&self) -> GPDACB_RSTN_ANA_R {
        GPDACB_RSTN_ANA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpdac_test_en(&self) -> GPDAC_TEST_EN_R {
        GPDAC_TEST_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpdac_ref_sel(&self) -> GPDAC_REF_SEL_R {
        GPDAC_REF_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn gpdac_test_sel(&self) -> GPDAC_TEST_SEL_R {
        GPDAC_TEST_SEL_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gpdac_ana_clk_sel(&self) -> GPDAC_ANA_CLK_SEL_R {
        GPDAC_ANA_CLK_SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpdac_dat_cha_sel(&self) -> GPDAC_DAT_CHA_SEL_R {
        GPDAC_DAT_CHA_SEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpdac_dat_chb_sel(&self) -> GPDAC_DAT_CHB_SEL_R {
        GPDAC_DAT_CHB_SEL_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn gpdaca_rstn_ana(&mut self) -> GPDACA_RSTN_ANA_W<GPDAC_CONFIG_0_SPEC> {
        GPDACA_RSTN_ANA_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn gpdacb_rstn_ana(&mut self) -> GPDACB_RSTN_ANA_W<GPDAC_CONFIG_0_SPEC> {
        GPDACB_RSTN_ANA_W::new(self, 1)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn gpdac_test_en(&mut self) -> GPDAC_TEST_EN_W<GPDAC_CONFIG_0_SPEC> {
        GPDAC_TEST_EN_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn gpdac_ref_sel(&mut self) -> GPDAC_REF_SEL_W<GPDAC_CONFIG_0_SPEC> {
        GPDAC_REF_SEL_W::new(self, 8)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    #[must_use]
    pub fn gpdac_test_sel(&mut self) -> GPDAC_TEST_SEL_W<GPDAC_CONFIG_0_SPEC> {
        GPDAC_TEST_SEL_W::new(self, 9)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn gpdac_ana_clk_sel(&mut self) -> GPDAC_ANA_CLK_SEL_W<GPDAC_CONFIG_0_SPEC> {
        GPDAC_ANA_CLK_SEL_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn gpdac_dat_cha_sel(&mut self) -> GPDAC_DAT_CHA_SEL_W<GPDAC_CONFIG_0_SPEC> {
        GPDAC_DAT_CHA_SEL_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn gpdac_dat_chb_sel(&mut self) -> GPDAC_DAT_CHB_SEL_W<GPDAC_CONFIG_0_SPEC> {
        GPDAC_DAT_CHB_SEL_W::new(self, 14)
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
#[doc = "General Purpose Digital-to-analog convert configuration 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpdac_config_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpdac_config_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPDAC_CONFIG_0_SPEC;
impl crate::RegisterSpec for GPDAC_CONFIG_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpdac_config_0::R`](R) reader structure"]
impl crate::Readable for GPDAC_CONFIG_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpdac_config_0::W`](W) writer structure"]
impl crate::Writable for GPDAC_CONFIG_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpdac_config_0 to value 0"]
impl crate::Resettable for GPDAC_CONFIG_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
