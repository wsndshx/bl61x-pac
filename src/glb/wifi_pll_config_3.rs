#[doc = "Register `wifi_pll_config_3` reader"]
pub type R = crate::R<WIFI_PLL_CONFIG_3_SPEC>;
#[doc = "Register `wifi_pll_config_3` writer"]
pub type W = crate::W<WIFI_PLL_CONFIG_3_SPEC>;
#[doc = "Field `wifipll_c4_en` reader - "]
pub type WIFIPLL_C4_EN_R = crate::BitReader;
#[doc = "Field `wifipll_c4_en` writer - "]
pub type WIFIPLL_C4_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_r4` reader - "]
pub type WIFIPLL_R4_R = crate::FieldReader;
#[doc = "Field `wifipll_r4` writer - "]
pub type WIFIPLL_R4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wifipll_r4_short` reader - "]
pub type WIFIPLL_R4_SHORT_R = crate::BitReader;
#[doc = "Field `wifipll_r4_short` writer - "]
pub type WIFIPLL_R4_SHORT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_c3` reader - "]
pub type WIFIPLL_C3_R = crate::FieldReader;
#[doc = "Field `wifipll_c3` writer - "]
pub type WIFIPLL_C3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wifipll_cz` reader - "]
pub type WIFIPLL_CZ_R = crate::FieldReader;
#[doc = "Field `wifipll_cz` writer - "]
pub type WIFIPLL_CZ_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `wifipll_rz` reader - "]
pub type WIFIPLL_RZ_R = crate::FieldReader;
#[doc = "Field `wifipll_rz` writer - "]
pub type WIFIPLL_RZ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wifipll_c4_en(&self) -> WIFIPLL_C4_EN_R {
        WIFIPLL_C4_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn wifipll_r4(&self) -> WIFIPLL_R4_R {
        WIFIPLL_R4_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn wifipll_r4_short(&self) -> WIFIPLL_R4_SHORT_R {
        WIFIPLL_R4_SHORT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn wifipll_c3(&self) -> WIFIPLL_C3_R {
        WIFIPLL_C3_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn wifipll_cz(&self) -> WIFIPLL_CZ_R {
        WIFIPLL_CZ_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn wifipll_rz(&self) -> WIFIPLL_RZ_R {
        WIFIPLL_RZ_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_c4_en(&mut self) -> WIFIPLL_C4_EN_W<WIFI_PLL_CONFIG_3_SPEC> {
        WIFIPLL_C4_EN_W::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_r4(&mut self) -> WIFIPLL_R4_W<WIFI_PLL_CONFIG_3_SPEC> {
        WIFIPLL_R4_W::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_r4_short(&mut self) -> WIFIPLL_R4_SHORT_W<WIFI_PLL_CONFIG_3_SPEC> {
        WIFIPLL_R4_SHORT_W::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_c3(&mut self) -> WIFIPLL_C3_W<WIFI_PLL_CONFIG_3_SPEC> {
        WIFIPLL_C3_W::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_cz(&mut self) -> WIFIPLL_CZ_W<WIFI_PLL_CONFIG_3_SPEC> {
        WIFIPLL_CZ_W::new(self, 14)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_rz(&mut self) -> WIFIPLL_RZ_W<WIFI_PLL_CONFIG_3_SPEC> {
        WIFIPLL_RZ_W::new(self, 16)
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
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_pll_config_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_pll_config_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIFI_PLL_CONFIG_3_SPEC;
impl crate::RegisterSpec for WIFI_PLL_CONFIG_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_pll_config_3::R`](R) reader structure"]
impl crate::Readable for WIFI_PLL_CONFIG_3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wifi_pll_config_3::W`](W) writer structure"]
impl crate::Writable for WIFI_PLL_CONFIG_3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wifi_pll_config_3 to value 0"]
impl crate::Resettable for WIFI_PLL_CONFIG_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
