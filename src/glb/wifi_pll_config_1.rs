#[doc = "Register `wifi_pll_config_1` reader"]
pub type R = crate::R<WIFI_PLL_CONFIG_1_SPEC>;
#[doc = "Register `wifi_pll_config_1` writer"]
pub type W = crate::W<WIFI_PLL_CONFIG_1_SPEC>;
#[doc = "Field `wifipll_postdiv` reader - "]
pub type WIFIPLL_POSTDIV_R = crate::FieldReader;
#[doc = "Field `wifipll_postdiv` writer - "]
pub type WIFIPLL_POSTDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `wifipll_refdiv_ratio` reader - "]
pub type WIFIPLL_REFDIV_RATIO_R = crate::FieldReader;
#[doc = "Field `wifipll_refdiv_ratio` writer - "]
pub type WIFIPLL_REFDIV_RATIO_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `wifipll_refclk_sel` reader - "]
pub type WIFIPLL_REFCLK_SEL_R = crate::FieldReader;
#[doc = "Field `wifipll_refclk_sel` writer - "]
pub type WIFIPLL_REFCLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wifipll_vg11_sel` reader - "]
pub type WIFIPLL_VG11_SEL_R = crate::FieldReader;
#[doc = "Field `wifipll_vg11_sel` writer - "]
pub type WIFIPLL_VG11_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wifipll_vg13_sel` reader - "]
pub type WIFIPLL_VG13_SEL_R = crate::FieldReader;
#[doc = "Field `wifipll_vg13_sel` writer - "]
pub type WIFIPLL_VG13_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn wifipll_postdiv(&self) -> WIFIPLL_POSTDIV_R {
        WIFIPLL_POSTDIV_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn wifipll_refdiv_ratio(&self) -> WIFIPLL_REFDIV_RATIO_R {
        WIFIPLL_REFDIV_RATIO_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn wifipll_refclk_sel(&self) -> WIFIPLL_REFCLK_SEL_R {
        WIFIPLL_REFCLK_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn wifipll_vg11_sel(&self) -> WIFIPLL_VG11_SEL_R {
        WIFIPLL_VG11_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn wifipll_vg13_sel(&self) -> WIFIPLL_VG13_SEL_R {
        WIFIPLL_VG13_SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_postdiv(&mut self) -> WIFIPLL_POSTDIV_W<WIFI_PLL_CONFIG_1_SPEC> {
        WIFIPLL_POSTDIV_W::new(self, 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_refdiv_ratio(&mut self) -> WIFIPLL_REFDIV_RATIO_W<WIFI_PLL_CONFIG_1_SPEC> {
        WIFIPLL_REFDIV_RATIO_W::new(self, 8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_refclk_sel(&mut self) -> WIFIPLL_REFCLK_SEL_W<WIFI_PLL_CONFIG_1_SPEC> {
        WIFIPLL_REFCLK_SEL_W::new(self, 16)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_vg11_sel(&mut self) -> WIFIPLL_VG11_SEL_W<WIFI_PLL_CONFIG_1_SPEC> {
        WIFIPLL_VG11_SEL_W::new(self, 20)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_vg13_sel(&mut self) -> WIFIPLL_VG13_SEL_W<WIFI_PLL_CONFIG_1_SPEC> {
        WIFIPLL_VG13_SEL_W::new(self, 24)
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
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_pll_config_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_pll_config_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIFI_PLL_CONFIG_1_SPEC;
impl crate::RegisterSpec for WIFI_PLL_CONFIG_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_pll_config_1::R`](R) reader structure"]
impl crate::Readable for WIFI_PLL_CONFIG_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wifi_pll_config_1::W`](W) writer structure"]
impl crate::Writable for WIFI_PLL_CONFIG_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wifi_pll_config_1 to value 0"]
impl crate::Resettable for WIFI_PLL_CONFIG_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
