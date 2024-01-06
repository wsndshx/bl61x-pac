#[doc = "Register `wifi_pll_config_12` reader"]
pub type R = crate::R<WIFI_PLL_CONFIG_12_SPEC>;
#[doc = "Register `wifi_pll_config_12` writer"]
pub type W = crate::W<WIFI_PLL_CONFIG_12_SPEC>;
#[doc = "Field `sscdiv_sdmin` reader - "]
pub type SSCDIV_SDMIN_R = crate::FieldReader<u32>;
#[doc = "Field `sscdiv_sdmin` writer - "]
pub type SSCDIV_SDMIN_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `sscdiv_sdm_bypass` reader - "]
pub type SSCDIV_SDM_BYPASS_R = crate::BitReader;
#[doc = "Field `sscdiv_sdm_bypass` writer - "]
pub type SSCDIV_SDM_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sscdiv_sdm_order_sel` reader - "]
pub type SSCDIV_SDM_ORDER_SEL_R = crate::BitReader;
#[doc = "Field `sscdiv_sdm_order_sel` writer - "]
pub type SSCDIV_SDM_ORDER_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sscdiv_sdm_sig_dith_sel` reader - "]
pub type SSCDIV_SDM_SIG_DITH_SEL_R = crate::FieldReader;
#[doc = "Field `sscdiv_sdm_sig_dith_sel` writer - "]
pub type SSCDIV_SDM_SIG_DITH_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sscdiv_div2_en` reader - "]
pub type SSCDIV_DIV2_EN_R = crate::BitReader;
#[doc = "Field `sscdiv_div2_en` writer - "]
pub type SSCDIV_DIV2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sscdiv_clkout_en` reader - "]
pub type SSCDIV_CLKOUT_EN_R = crate::BitReader;
#[doc = "Field `sscdiv_clkout_en` writer - "]
pub type SSCDIV_CLKOUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sscdiv_sel_sample_clk` reader - "]
pub type SSCDIV_SEL_SAMPLE_CLK_R = crate::FieldReader;
#[doc = "Field `sscdiv_sel_sample_clk` writer - "]
pub type SSCDIV_SEL_SAMPLE_CLK_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sscdiv_rstb` reader - "]
pub type SSCDIV_RSTB_R = crate::BitReader;
#[doc = "Field `sscdiv_rstb` writer - "]
pub type SSCDIV_RSTB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_sscdiv_mmdiv` reader - "]
pub type PU_SSCDIV_MMDIV_R = crate::BitReader;
#[doc = "Field `pu_sscdiv_mmdiv` writer - "]
pub type PU_SSCDIV_MMDIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn sscdiv_sdmin(&self) -> SSCDIV_SDMIN_R {
        SSCDIV_SDMIN_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn sscdiv_sdm_bypass(&self) -> SSCDIV_SDM_BYPASS_R {
        SSCDIV_SDM_BYPASS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn sscdiv_sdm_order_sel(&self) -> SSCDIV_SDM_ORDER_SEL_R {
        SSCDIV_SDM_ORDER_SEL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn sscdiv_sdm_sig_dith_sel(&self) -> SSCDIV_SDM_SIG_DITH_SEL_R {
        SSCDIV_SDM_SIG_DITH_SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sscdiv_div2_en(&self) -> SSCDIV_DIV2_EN_R {
        SSCDIV_DIV2_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sscdiv_clkout_en(&self) -> SSCDIV_CLKOUT_EN_R {
        SSCDIV_CLKOUT_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn sscdiv_sel_sample_clk(&self) -> SSCDIV_SEL_SAMPLE_CLK_R {
        SSCDIV_SEL_SAMPLE_CLK_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn sscdiv_rstb(&self) -> SSCDIV_RSTB_R {
        SSCDIV_RSTB_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pu_sscdiv_mmdiv(&self) -> PU_SSCDIV_MMDIV_R {
        PU_SSCDIV_MMDIV_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    #[must_use]
    pub fn sscdiv_sdmin(&mut self) -> SSCDIV_SDMIN_W<WIFI_PLL_CONFIG_12_SPEC> {
        SSCDIV_SDMIN_W::new(self, 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn sscdiv_sdm_bypass(&mut self) -> SSCDIV_SDM_BYPASS_W<WIFI_PLL_CONFIG_12_SPEC> {
        SSCDIV_SDM_BYPASS_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn sscdiv_sdm_order_sel(&mut self) -> SSCDIV_SDM_ORDER_SEL_W<WIFI_PLL_CONFIG_12_SPEC> {
        SSCDIV_SDM_ORDER_SEL_W::new(self, 21)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    #[must_use]
    pub fn sscdiv_sdm_sig_dith_sel(
        &mut self,
    ) -> SSCDIV_SDM_SIG_DITH_SEL_W<WIFI_PLL_CONFIG_12_SPEC> {
        SSCDIV_SDM_SIG_DITH_SEL_W::new(self, 22)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn sscdiv_div2_en(&mut self) -> SSCDIV_DIV2_EN_W<WIFI_PLL_CONFIG_12_SPEC> {
        SSCDIV_DIV2_EN_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn sscdiv_clkout_en(&mut self) -> SSCDIV_CLKOUT_EN_W<WIFI_PLL_CONFIG_12_SPEC> {
        SSCDIV_CLKOUT_EN_W::new(self, 25)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    #[must_use]
    pub fn sscdiv_sel_sample_clk(&mut self) -> SSCDIV_SEL_SAMPLE_CLK_W<WIFI_PLL_CONFIG_12_SPEC> {
        SSCDIV_SEL_SAMPLE_CLK_W::new(self, 26)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn sscdiv_rstb(&mut self) -> SSCDIV_RSTB_W<WIFI_PLL_CONFIG_12_SPEC> {
        SSCDIV_RSTB_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn pu_sscdiv_mmdiv(&mut self) -> PU_SSCDIV_MMDIV_W<WIFI_PLL_CONFIG_12_SPEC> {
        PU_SSCDIV_MMDIV_W::new(self, 29)
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
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_pll_config_12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_pll_config_12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIFI_PLL_CONFIG_12_SPEC;
impl crate::RegisterSpec for WIFI_PLL_CONFIG_12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_pll_config_12::R`](R) reader structure"]
impl crate::Readable for WIFI_PLL_CONFIG_12_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wifi_pll_config_12::W`](W) writer structure"]
impl crate::Writable for WIFI_PLL_CONFIG_12_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wifi_pll_config_12 to value 0"]
impl crate::Resettable for WIFI_PLL_CONFIG_12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
