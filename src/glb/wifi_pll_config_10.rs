#[doc = "Register `wifi_pll_config_10` reader"]
pub type R = crate::R<WIFI_PLL_CONFIG_10_SPEC>;
#[doc = "Register `wifi_pll_config_10` writer"]
pub type W = crate::W<WIFI_PLL_CONFIG_10_SPEC>;
#[doc = "Field `usbpll_sdmin` reader - "]
pub type USBPLL_SDMIN_R = crate::FieldReader<u32>;
#[doc = "Field `usbpll_sdmin` writer - "]
pub type USBPLL_SDMIN_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `usbpll_sdm_bypass` reader - "]
pub type USBPLL_SDM_BYPASS_R = crate::BitReader;
#[doc = "Field `usbpll_sdm_bypass` writer - "]
pub type USBPLL_SDM_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `usbpll_sdm_order_sel` reader - "]
pub type USBPLL_SDM_ORDER_SEL_R = crate::BitReader;
#[doc = "Field `usbpll_sdm_order_sel` writer - "]
pub type USBPLL_SDM_ORDER_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `usbpll_sdm_sig_dith_sel` reader - "]
pub type USBPLL_SDM_SIG_DITH_SEL_R = crate::FieldReader;
#[doc = "Field `usbpll_sdm_sig_dith_sel` writer - "]
pub type USBPLL_SDM_SIG_DITH_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `usbpll_div2_en` reader - "]
pub type USBPLL_DIV2_EN_R = crate::BitReader;
#[doc = "Field `usbpll_div2_en` writer - "]
pub type USBPLL_DIV2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `usbpll_clkout_en` reader - "]
pub type USBPLL_CLKOUT_EN_R = crate::BitReader;
#[doc = "Field `usbpll_clkout_en` writer - "]
pub type USBPLL_CLKOUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `usbpll_sel_sample_clk` reader - "]
pub type USBPLL_SEL_SAMPLE_CLK_R = crate::FieldReader;
#[doc = "Field `usbpll_sel_sample_clk` writer - "]
pub type USBPLL_SEL_SAMPLE_CLK_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `usbpll_rstb` reader - "]
pub type USBPLL_RSTB_R = crate::BitReader;
#[doc = "Field `usbpll_rstb` writer - "]
pub type USBPLL_RSTB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pu_usbpll_mmdiv` reader - "]
pub type PU_USBPLL_MMDIV_R = crate::BitReader;
#[doc = "Field `pu_usbpll_mmdiv` writer - "]
pub type PU_USBPLL_MMDIV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn usbpll_sdmin(&self) -> USBPLL_SDMIN_R {
        USBPLL_SDMIN_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn usbpll_sdm_bypass(&self) -> USBPLL_SDM_BYPASS_R {
        USBPLL_SDM_BYPASS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn usbpll_sdm_order_sel(&self) -> USBPLL_SDM_ORDER_SEL_R {
        USBPLL_SDM_ORDER_SEL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn usbpll_sdm_sig_dith_sel(&self) -> USBPLL_SDM_SIG_DITH_SEL_R {
        USBPLL_SDM_SIG_DITH_SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn usbpll_div2_en(&self) -> USBPLL_DIV2_EN_R {
        USBPLL_DIV2_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn usbpll_clkout_en(&self) -> USBPLL_CLKOUT_EN_R {
        USBPLL_CLKOUT_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn usbpll_sel_sample_clk(&self) -> USBPLL_SEL_SAMPLE_CLK_R {
        USBPLL_SEL_SAMPLE_CLK_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn usbpll_rstb(&self) -> USBPLL_RSTB_R {
        USBPLL_RSTB_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pu_usbpll_mmdiv(&self) -> PU_USBPLL_MMDIV_R {
        PU_USBPLL_MMDIV_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    #[must_use]
    pub fn usbpll_sdmin(&mut self) -> USBPLL_SDMIN_W<WIFI_PLL_CONFIG_10_SPEC> {
        USBPLL_SDMIN_W::new(self, 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn usbpll_sdm_bypass(&mut self) -> USBPLL_SDM_BYPASS_W<WIFI_PLL_CONFIG_10_SPEC> {
        USBPLL_SDM_BYPASS_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn usbpll_sdm_order_sel(&mut self) -> USBPLL_SDM_ORDER_SEL_W<WIFI_PLL_CONFIG_10_SPEC> {
        USBPLL_SDM_ORDER_SEL_W::new(self, 21)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    #[must_use]
    pub fn usbpll_sdm_sig_dith_sel(
        &mut self,
    ) -> USBPLL_SDM_SIG_DITH_SEL_W<WIFI_PLL_CONFIG_10_SPEC> {
        USBPLL_SDM_SIG_DITH_SEL_W::new(self, 22)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn usbpll_div2_en(&mut self) -> USBPLL_DIV2_EN_W<WIFI_PLL_CONFIG_10_SPEC> {
        USBPLL_DIV2_EN_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn usbpll_clkout_en(&mut self) -> USBPLL_CLKOUT_EN_W<WIFI_PLL_CONFIG_10_SPEC> {
        USBPLL_CLKOUT_EN_W::new(self, 25)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    #[must_use]
    pub fn usbpll_sel_sample_clk(&mut self) -> USBPLL_SEL_SAMPLE_CLK_W<WIFI_PLL_CONFIG_10_SPEC> {
        USBPLL_SEL_SAMPLE_CLK_W::new(self, 26)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn usbpll_rstb(&mut self) -> USBPLL_RSTB_W<WIFI_PLL_CONFIG_10_SPEC> {
        USBPLL_RSTB_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn pu_usbpll_mmdiv(&mut self) -> PU_USBPLL_MMDIV_W<WIFI_PLL_CONFIG_10_SPEC> {
        PU_USBPLL_MMDIV_W::new(self, 29)
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
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_pll_config_10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_pll_config_10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIFI_PLL_CONFIG_10_SPEC;
impl crate::RegisterSpec for WIFI_PLL_CONFIG_10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_pll_config_10::R`](R) reader structure"]
impl crate::Readable for WIFI_PLL_CONFIG_10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wifi_pll_config_10::W`](W) writer structure"]
impl crate::Writable for WIFI_PLL_CONFIG_10_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wifi_pll_config_10 to value 0"]
impl crate::Resettable for WIFI_PLL_CONFIG_10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
