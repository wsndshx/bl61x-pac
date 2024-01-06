#[doc = "Register `wifi_pll_config_9` reader"]
pub type R = crate::R<WIFI_PLL_CONFIG_9_SPEC>;
#[doc = "Register `wifi_pll_config_9` writer"]
pub type W = crate::W<WIFI_PLL_CONFIG_9_SPEC>;
#[doc = "Field `wifipll_dc_tp_out_en` reader - "]
pub type WIFIPLL_DC_TP_OUT_EN_R = crate::BitReader;
#[doc = "Field `wifipll_dc_tp_out_en` writer - "]
pub type WIFIPLL_DC_TP_OUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_wifipll` reader - "]
pub type TEN_WIFIPLL_R = crate::BitReader;
#[doc = "Field `ten_wifipll` writer - "]
pub type TEN_WIFIPLL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_wifipll_sfreg` reader - "]
pub type TEN_WIFIPLL_SFREG_R = crate::BitReader;
#[doc = "Field `ten_wifipll_sfreg` writer - "]
pub type TEN_WIFIPLL_SFREG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_wifipll_fin` reader - "]
pub type DTEN_WIFIPLL_FIN_R = crate::BitReader;
#[doc = "Field `dten_wifipll_fin` writer - "]
pub type DTEN_WIFIPLL_FIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_wifipll_fref` reader - "]
pub type DTEN_WIFIPLL_FREF_R = crate::BitReader;
#[doc = "Field `dten_wifipll_fref` writer - "]
pub type DTEN_WIFIPLL_FREF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_wifipll_fsdm` reader - "]
pub type DTEN_WIFIPLL_FSDM_R = crate::BitReader;
#[doc = "Field `dten_wifipll_fsdm` writer - "]
pub type DTEN_WIFIPLL_FSDM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_wifipll_div30` reader - "]
pub type DTEN_WIFIPLL_DIV30_R = crate::BitReader;
#[doc = "Field `dten_wifipll_div30` writer - "]
pub type DTEN_WIFIPLL_DIV30_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_wifipll_div10` reader - "]
pub type DTEN_WIFIPLL_DIV10_R = crate::BitReader;
#[doc = "Field `dten_wifipll_div10` writer - "]
pub type DTEN_WIFIPLL_DIV10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_wifipll_postdiv_clk` reader - "]
pub type DTEN_WIFIPLL_POSTDIV_CLK_R = crate::BitReader;
#[doc = "Field `dten_wifipll_postdiv_clk` writer - "]
pub type DTEN_WIFIPLL_POSTDIV_CLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_usbpll_pclk` reader - "]
pub type DTEN_USBPLL_PCLK_R = crate::BitReader;
#[doc = "Field `dten_usbpll_pclk` writer - "]
pub type DTEN_USBPLL_PCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_usbpll_clkout` reader - "]
pub type DTEN_USBPLL_CLKOUT_R = crate::BitReader;
#[doc = "Field `dten_usbpll_clkout` writer - "]
pub type DTEN_USBPLL_CLKOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_sscdiv_pclk` reader - "]
pub type DTEN_SSCDIV_PCLK_R = crate::BitReader;
#[doc = "Field `dten_sscdiv_pclk` writer - "]
pub type DTEN_SSCDIV_PCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_sscdiv_clkout` reader - "]
pub type DTEN_SSCDIV_CLKOUT_R = crate::BitReader;
#[doc = "Field `dten_sscdiv_clkout` writer - "]
pub type DTEN_SSCDIV_CLKOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dtest_pulldown` reader - "]
pub type DTEST_PULLDOWN_R = crate::BitReader;
#[doc = "Field `dtest_pulldown` writer - "]
pub type DTEST_PULLDOWN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wifipll_dc_tp_out_en(&self) -> WIFIPLL_DC_TP_OUT_EN_R {
        WIFIPLL_DC_TP_OUT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ten_wifipll(&self) -> TEN_WIFIPLL_R {
        TEN_WIFIPLL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ten_wifipll_sfreg(&self) -> TEN_WIFIPLL_SFREG_R {
        TEN_WIFIPLL_SFREG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dten_wifipll_fin(&self) -> DTEN_WIFIPLL_FIN_R {
        DTEN_WIFIPLL_FIN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dten_wifipll_fref(&self) -> DTEN_WIFIPLL_FREF_R {
        DTEN_WIFIPLL_FREF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dten_wifipll_fsdm(&self) -> DTEN_WIFIPLL_FSDM_R {
        DTEN_WIFIPLL_FSDM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dten_wifipll_div30(&self) -> DTEN_WIFIPLL_DIV30_R {
        DTEN_WIFIPLL_DIV30_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dten_wifipll_div10(&self) -> DTEN_WIFIPLL_DIV10_R {
        DTEN_WIFIPLL_DIV10_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dten_wifipll_postdiv_clk(&self) -> DTEN_WIFIPLL_POSTDIV_CLK_R {
        DTEN_WIFIPLL_POSTDIV_CLK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dten_usbpll_pclk(&self) -> DTEN_USBPLL_PCLK_R {
        DTEN_USBPLL_PCLK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dten_usbpll_clkout(&self) -> DTEN_USBPLL_CLKOUT_R {
        DTEN_USBPLL_CLKOUT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dten_sscdiv_pclk(&self) -> DTEN_SSCDIV_PCLK_R {
        DTEN_SSCDIV_PCLK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dten_sscdiv_clkout(&self) -> DTEN_SSCDIV_CLKOUT_R {
        DTEN_SSCDIV_CLKOUT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dtest_pulldown(&self) -> DTEST_PULLDOWN_R {
        DTEST_PULLDOWN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_dc_tp_out_en(&mut self) -> WIFIPLL_DC_TP_OUT_EN_W<WIFI_PLL_CONFIG_9_SPEC> {
        WIFIPLL_DC_TP_OUT_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ten_wifipll(&mut self) -> TEN_WIFIPLL_W<WIFI_PLL_CONFIG_9_SPEC> {
        TEN_WIFIPLL_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ten_wifipll_sfreg(&mut self) -> TEN_WIFIPLL_SFREG_W<WIFI_PLL_CONFIG_9_SPEC> {
        TEN_WIFIPLL_SFREG_W::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn dten_wifipll_fin(&mut self) -> DTEN_WIFIPLL_FIN_W<WIFI_PLL_CONFIG_9_SPEC> {
        DTEN_WIFIPLL_FIN_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn dten_wifipll_fref(&mut self) -> DTEN_WIFIPLL_FREF_W<WIFI_PLL_CONFIG_9_SPEC> {
        DTEN_WIFIPLL_FREF_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn dten_wifipll_fsdm(&mut self) -> DTEN_WIFIPLL_FSDM_W<WIFI_PLL_CONFIG_9_SPEC> {
        DTEN_WIFIPLL_FSDM_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn dten_wifipll_div30(&mut self) -> DTEN_WIFIPLL_DIV30_W<WIFI_PLL_CONFIG_9_SPEC> {
        DTEN_WIFIPLL_DIV30_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn dten_wifipll_div10(&mut self) -> DTEN_WIFIPLL_DIV10_W<WIFI_PLL_CONFIG_9_SPEC> {
        DTEN_WIFIPLL_DIV10_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn dten_wifipll_postdiv_clk(
        &mut self,
    ) -> DTEN_WIFIPLL_POSTDIV_CLK_W<WIFI_PLL_CONFIG_9_SPEC> {
        DTEN_WIFIPLL_POSTDIV_CLK_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn dten_usbpll_pclk(&mut self) -> DTEN_USBPLL_PCLK_W<WIFI_PLL_CONFIG_9_SPEC> {
        DTEN_USBPLL_PCLK_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn dten_usbpll_clkout(&mut self) -> DTEN_USBPLL_CLKOUT_W<WIFI_PLL_CONFIG_9_SPEC> {
        DTEN_USBPLL_CLKOUT_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn dten_sscdiv_pclk(&mut self) -> DTEN_SSCDIV_PCLK_W<WIFI_PLL_CONFIG_9_SPEC> {
        DTEN_SSCDIV_PCLK_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn dten_sscdiv_clkout(&mut self) -> DTEN_SSCDIV_CLKOUT_W<WIFI_PLL_CONFIG_9_SPEC> {
        DTEN_SSCDIV_CLKOUT_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn dtest_pulldown(&mut self) -> DTEST_PULLDOWN_W<WIFI_PLL_CONFIG_9_SPEC> {
        DTEST_PULLDOWN_W::new(self, 14)
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
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_pll_config_9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_pll_config_9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIFI_PLL_CONFIG_9_SPEC;
impl crate::RegisterSpec for WIFI_PLL_CONFIG_9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_pll_config_9::R`](R) reader structure"]
impl crate::Readable for WIFI_PLL_CONFIG_9_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wifi_pll_config_9::W`](W) writer structure"]
impl crate::Writable for WIFI_PLL_CONFIG_9_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wifi_pll_config_9 to value 0"]
impl crate::Resettable for WIFI_PLL_CONFIG_9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
