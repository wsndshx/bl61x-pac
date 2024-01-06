#[doc = "Register `wifi_pll_config_5` reader"]
pub type R = crate::R<WIFI_PLL_CONFIG_5_SPEC>;
#[doc = "Register `wifi_pll_config_5` writer"]
pub type W = crate::W<WIFI_PLL_CONFIG_5_SPEC>;
#[doc = "Field `wifipll_vco_speed` reader - "]
pub type WIFIPLL_VCO_SPEED_R = crate::FieldReader;
#[doc = "Field `wifipll_vco_speed` writer - "]
pub type WIFIPLL_VCO_SPEED_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `wifipll_vco_div3_en` reader - "]
pub type WIFIPLL_VCO_DIV3_EN_R = crate::BitReader;
#[doc = "Field `wifipll_vco_div3_en` writer - "]
pub type WIFIPLL_VCO_DIV3_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_vco_div2_en` reader - "]
pub type WIFIPLL_VCO_DIV2_EN_R = crate::BitReader;
#[doc = "Field `wifipll_vco_div2_en` writer - "]
pub type WIFIPLL_VCO_DIV2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_vco_div1_en` reader - "]
pub type WIFIPLL_VCO_DIV1_EN_R = crate::BitReader;
#[doc = "Field `wifipll_vco_div1_en` writer - "]
pub type WIFIPLL_VCO_DIV1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn wifipll_vco_speed(&self) -> WIFIPLL_VCO_SPEED_R {
        WIFIPLL_VCO_SPEED_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wifipll_vco_div3_en(&self) -> WIFIPLL_VCO_DIV3_EN_R {
        WIFIPLL_VCO_DIV3_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wifipll_vco_div2_en(&self) -> WIFIPLL_VCO_DIV2_EN_R {
        WIFIPLL_VCO_DIV2_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn wifipll_vco_div1_en(&self) -> WIFIPLL_VCO_DIV1_EN_R {
        WIFIPLL_VCO_DIV1_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_vco_speed(&mut self) -> WIFIPLL_VCO_SPEED_W<WIFI_PLL_CONFIG_5_SPEC> {
        WIFIPLL_VCO_SPEED_W::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_vco_div3_en(&mut self) -> WIFIPLL_VCO_DIV3_EN_W<WIFI_PLL_CONFIG_5_SPEC> {
        WIFIPLL_VCO_DIV3_EN_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_vco_div2_en(&mut self) -> WIFIPLL_VCO_DIV2_EN_W<WIFI_PLL_CONFIG_5_SPEC> {
        WIFIPLL_VCO_DIV2_EN_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_vco_div1_en(&mut self) -> WIFIPLL_VCO_DIV1_EN_W<WIFI_PLL_CONFIG_5_SPEC> {
        WIFIPLL_VCO_DIV1_EN_W::new(self, 5)
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
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_pll_config_5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_pll_config_5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIFI_PLL_CONFIG_5_SPEC;
impl crate::RegisterSpec for WIFI_PLL_CONFIG_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_pll_config_5::R`](R) reader structure"]
impl crate::Readable for WIFI_PLL_CONFIG_5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wifi_pll_config_5::W`](W) writer structure"]
impl crate::Writable for WIFI_PLL_CONFIG_5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wifi_pll_config_5 to value 0"]
impl crate::Resettable for WIFI_PLL_CONFIG_5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
