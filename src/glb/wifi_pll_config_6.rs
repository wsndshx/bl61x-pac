#[doc = "Register `wifi_pll_config_6` reader"]
pub type R = crate::R<WIFI_PLL_CONFIG_6_SPEC>;
#[doc = "Register `wifi_pll_config_6` writer"]
pub type W = crate::W<WIFI_PLL_CONFIG_6_SPEC>;
#[doc = "Field `wifipll_sdmin` reader - "]
pub type WIFIPLL_SDMIN_R = crate::FieldReader<u32>;
#[doc = "Field `wifipll_sdmin` writer - "]
pub type WIFIPLL_SDMIN_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
#[doc = "Field `wifipll_sdm_bypass` reader - "]
pub type WIFIPLL_SDM_BYPASS_R = crate::BitReader;
#[doc = "Field `wifipll_sdm_bypass` writer - "]
pub type WIFIPLL_SDM_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_sdm_bypass_hw` reader - "]
pub type WIFIPLL_SDM_BYPASS_HW_R = crate::BitReader;
#[doc = "Field `wifipll_sdm_bypass_hw` writer - "]
pub type WIFIPLL_SDM_BYPASS_HW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_sdm_ctrl_hw` reader - "]
pub type WIFIPLL_SDM_CTRL_HW_R = crate::BitReader;
#[doc = "Field `wifipll_sdm_ctrl_hw` writer - "]
pub type WIFIPLL_SDM_CTRL_HW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn wifipll_sdmin(&self) -> WIFIPLL_SDMIN_R {
        WIFIPLL_SDMIN_R::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn wifipll_sdm_bypass(&self) -> WIFIPLL_SDM_BYPASS_R {
        WIFIPLL_SDM_BYPASS_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn wifipll_sdm_bypass_hw(&self) -> WIFIPLL_SDM_BYPASS_HW_R {
        WIFIPLL_SDM_BYPASS_HW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn wifipll_sdm_ctrl_hw(&self) -> WIFIPLL_SDM_CTRL_HW_R {
        WIFIPLL_SDM_CTRL_HW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_sdmin(&mut self) -> WIFIPLL_SDMIN_W<WIFI_PLL_CONFIG_6_SPEC> {
        WIFIPLL_SDMIN_W::new(self, 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_sdm_bypass(&mut self) -> WIFIPLL_SDM_BYPASS_W<WIFI_PLL_CONFIG_6_SPEC> {
        WIFIPLL_SDM_BYPASS_W::new(self, 26)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_sdm_bypass_hw(&mut self) -> WIFIPLL_SDM_BYPASS_HW_W<WIFI_PLL_CONFIG_6_SPEC> {
        WIFIPLL_SDM_BYPASS_HW_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_sdm_ctrl_hw(&mut self) -> WIFIPLL_SDM_CTRL_HW_W<WIFI_PLL_CONFIG_6_SPEC> {
        WIFIPLL_SDM_CTRL_HW_W::new(self, 31)
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
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_pll_config_6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_pll_config_6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIFI_PLL_CONFIG_6_SPEC;
impl crate::RegisterSpec for WIFI_PLL_CONFIG_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_pll_config_6::R`](R) reader structure"]
impl crate::Readable for WIFI_PLL_CONFIG_6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wifi_pll_config_6::W`](W) writer structure"]
impl crate::Writable for WIFI_PLL_CONFIG_6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wifi_pll_config_6 to value 0"]
impl crate::Resettable for WIFI_PLL_CONFIG_6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
