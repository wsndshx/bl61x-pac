#[doc = "Register `wifi_pll_config_7` reader"]
pub type R = crate::R<WIFI_PLL_CONFIG_7_SPEC>;
#[doc = "Register `wifi_pll_config_7` writer"]
pub type W = crate::W<WIFI_PLL_CONFIG_7_SPEC>;
#[doc = "Field `wifipll_sdm_order_sel` reader - "]
pub type WIFIPLL_SDM_ORDER_SEL_R = crate::FieldReader;
#[doc = "Field `wifipll_sdm_order_sel` writer - "]
pub type WIFIPLL_SDM_ORDER_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wifipll_sdm_noi_prbs_sel` reader - "]
pub type WIFIPLL_SDM_NOI_PRBS_SEL_R = crate::FieldReader;
#[doc = "Field `wifipll_sdm_noi_prbs_sel` writer - "]
pub type WIFIPLL_SDM_NOI_PRBS_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wifipll_sdm_noi_prbs_en` reader - "]
pub type WIFIPLL_SDM_NOI_PRBS_EN_R = crate::BitReader;
#[doc = "Field `wifipll_sdm_noi_prbs_en` writer - "]
pub type WIFIPLL_SDM_NOI_PRBS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_sdm_sig_prbs_sel` reader - "]
pub type WIFIPLL_SDM_SIG_PRBS_SEL_R = crate::FieldReader;
#[doc = "Field `wifipll_sdm_sig_prbs_sel` writer - "]
pub type WIFIPLL_SDM_SIG_PRBS_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wifipll_sdm_sig_dith_sel` reader - "]
pub type WIFIPLL_SDM_SIG_DITH_SEL_R = crate::FieldReader;
#[doc = "Field `wifipll_sdm_sig_dith_sel` writer - "]
pub type WIFIPLL_SDM_SIG_DITH_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn wifipll_sdm_order_sel(&self) -> WIFIPLL_SDM_ORDER_SEL_R {
        WIFIPLL_SDM_ORDER_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn wifipll_sdm_noi_prbs_sel(&self) -> WIFIPLL_SDM_NOI_PRBS_SEL_R {
        WIFIPLL_SDM_NOI_PRBS_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn wifipll_sdm_noi_prbs_en(&self) -> WIFIPLL_SDM_NOI_PRBS_EN_R {
        WIFIPLL_SDM_NOI_PRBS_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn wifipll_sdm_sig_prbs_sel(&self) -> WIFIPLL_SDM_SIG_PRBS_SEL_R {
        WIFIPLL_SDM_SIG_PRBS_SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn wifipll_sdm_sig_dith_sel(&self) -> WIFIPLL_SDM_SIG_DITH_SEL_R {
        WIFIPLL_SDM_SIG_DITH_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_sdm_order_sel(&mut self) -> WIFIPLL_SDM_ORDER_SEL_W<WIFI_PLL_CONFIG_7_SPEC> {
        WIFIPLL_SDM_ORDER_SEL_W::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_sdm_noi_prbs_sel(
        &mut self,
    ) -> WIFIPLL_SDM_NOI_PRBS_SEL_W<WIFI_PLL_CONFIG_7_SPEC> {
        WIFIPLL_SDM_NOI_PRBS_SEL_W::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_sdm_noi_prbs_en(&mut self) -> WIFIPLL_SDM_NOI_PRBS_EN_W<WIFI_PLL_CONFIG_7_SPEC> {
        WIFIPLL_SDM_NOI_PRBS_EN_W::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_sdm_sig_prbs_sel(
        &mut self,
    ) -> WIFIPLL_SDM_SIG_PRBS_SEL_W<WIFI_PLL_CONFIG_7_SPEC> {
        WIFIPLL_SDM_SIG_PRBS_SEL_W::new(self, 12)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_sdm_sig_dith_sel(
        &mut self,
    ) -> WIFIPLL_SDM_SIG_DITH_SEL_W<WIFI_PLL_CONFIG_7_SPEC> {
        WIFIPLL_SDM_SIG_DITH_SEL_W::new(self, 16)
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
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_pll_config_7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_pll_config_7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIFI_PLL_CONFIG_7_SPEC;
impl crate::RegisterSpec for WIFI_PLL_CONFIG_7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_pll_config_7::R`](R) reader structure"]
impl crate::Readable for WIFI_PLL_CONFIG_7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wifi_pll_config_7::W`](W) writer structure"]
impl crate::Writable for WIFI_PLL_CONFIG_7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wifi_pll_config_7 to value 0"]
impl crate::Resettable for WIFI_PLL_CONFIG_7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
