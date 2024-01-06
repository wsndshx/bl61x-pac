#[doc = "Register `wifi_pll_config_2` reader"]
pub type R = crate::R<WIFI_PLL_CONFIG_2_SPEC>;
#[doc = "Register `wifi_pll_config_2` writer"]
pub type W = crate::W<WIFI_PLL_CONFIG_2_SPEC>;
#[doc = "Field `wifipll_sel_cp_bias` reader - "]
pub type WIFIPLL_SEL_CP_BIAS_R = crate::BitReader;
#[doc = "Field `wifipll_sel_cp_bias` writer - "]
pub type WIFIPLL_SEL_CP_BIAS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_icp_5u` reader - "]
pub type WIFIPLL_ICP_5U_R = crate::FieldReader;
#[doc = "Field `wifipll_icp_5u` writer - "]
pub type WIFIPLL_ICP_5U_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wifipll_icp_1u` reader - "]
pub type WIFIPLL_ICP_1U_R = crate::FieldReader;
#[doc = "Field `wifipll_icp_1u` writer - "]
pub type WIFIPLL_ICP_1U_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wifipll_int_frac_sw` reader - "]
pub type WIFIPLL_INT_FRAC_SW_R = crate::BitReader;
#[doc = "Field `wifipll_int_frac_sw` writer - "]
pub type WIFIPLL_INT_FRAC_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_cp_startup_en` reader - "]
pub type WIFIPLL_CP_STARTUP_EN_R = crate::BitReader;
#[doc = "Field `wifipll_cp_startup_en` writer - "]
pub type WIFIPLL_CP_STARTUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_cp_opamp_en` reader - "]
pub type WIFIPLL_CP_OPAMP_EN_R = crate::BitReader;
#[doc = "Field `wifipll_cp_opamp_en` writer - "]
pub type WIFIPLL_CP_OPAMP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wifipll_sel_cp_bias(&self) -> WIFIPLL_SEL_CP_BIAS_R {
        WIFIPLL_SEL_CP_BIAS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn wifipll_icp_5u(&self) -> WIFIPLL_ICP_5U_R {
        WIFIPLL_ICP_5U_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn wifipll_icp_1u(&self) -> WIFIPLL_ICP_1U_R {
        WIFIPLL_ICP_1U_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn wifipll_int_frac_sw(&self) -> WIFIPLL_INT_FRAC_SW_R {
        WIFIPLL_INT_FRAC_SW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn wifipll_cp_startup_en(&self) -> WIFIPLL_CP_STARTUP_EN_R {
        WIFIPLL_CP_STARTUP_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn wifipll_cp_opamp_en(&self) -> WIFIPLL_CP_OPAMP_EN_R {
        WIFIPLL_CP_OPAMP_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_sel_cp_bias(&mut self) -> WIFIPLL_SEL_CP_BIAS_W<WIFI_PLL_CONFIG_2_SPEC> {
        WIFIPLL_SEL_CP_BIAS_W::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_icp_5u(&mut self) -> WIFIPLL_ICP_5U_W<WIFI_PLL_CONFIG_2_SPEC> {
        WIFIPLL_ICP_5U_W::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_icp_1u(&mut self) -> WIFIPLL_ICP_1U_W<WIFI_PLL_CONFIG_2_SPEC> {
        WIFIPLL_ICP_1U_W::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_int_frac_sw(&mut self) -> WIFIPLL_INT_FRAC_SW_W<WIFI_PLL_CONFIG_2_SPEC> {
        WIFIPLL_INT_FRAC_SW_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_cp_startup_en(&mut self) -> WIFIPLL_CP_STARTUP_EN_W<WIFI_PLL_CONFIG_2_SPEC> {
        WIFIPLL_CP_STARTUP_EN_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_cp_opamp_en(&mut self) -> WIFIPLL_CP_OPAMP_EN_W<WIFI_PLL_CONFIG_2_SPEC> {
        WIFIPLL_CP_OPAMP_EN_W::new(self, 10)
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
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_pll_config_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_pll_config_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIFI_PLL_CONFIG_2_SPEC;
impl crate::RegisterSpec for WIFI_PLL_CONFIG_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_pll_config_2::R`](R) reader structure"]
impl crate::Readable for WIFI_PLL_CONFIG_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wifi_pll_config_2::W`](W) writer structure"]
impl crate::Writable for WIFI_PLL_CONFIG_2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wifi_pll_config_2 to value 0"]
impl crate::Resettable for WIFI_PLL_CONFIG_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
