#[doc = "Register `wifi_pll_config_8` reader"]
pub type R = crate::R<WIFI_PLL_CONFIG_8_SPEC>;
#[doc = "Register `wifi_pll_config_8` writer"]
pub type W = crate::W<WIFI_PLL_CONFIG_8_SPEC>;
#[doc = "Field `wifipll_en_rf_div3` reader - "]
pub type WIFIPLL_EN_RF_DIV3_R = crate::BitReader;
#[doc = "Field `wifipll_en_rf_div3` writer - "]
pub type WIFIPLL_EN_RF_DIV3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_en_rf_div6` reader - "]
pub type WIFIPLL_EN_RF_DIV6_R = crate::BitReader;
#[doc = "Field `wifipll_en_rf_div6` writer - "]
pub type WIFIPLL_EN_RF_DIV6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_sel_div3_div6` reader - "]
pub type WIFIPLL_SEL_DIV3_DIV6_R = crate::BitReader;
#[doc = "Field `wifipll_sel_div3_div6` writer - "]
pub type WIFIPLL_SEL_DIV3_DIV6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_sel_div6_div12` reader - "]
pub type WIFIPLL_SEL_DIV6_DIV12_R = crate::BitReader;
#[doc = "Field `wifipll_sel_div6_div12` writer - "]
pub type WIFIPLL_SEL_DIV6_DIV12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_en_div3` reader - "]
pub type WIFIPLL_EN_DIV3_R = crate::BitReader;
#[doc = "Field `wifipll_en_div3` writer - "]
pub type WIFIPLL_EN_DIV3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_en_div4` reader - "]
pub type WIFIPLL_EN_DIV4_R = crate::BitReader;
#[doc = "Field `wifipll_en_div4` writer - "]
pub type WIFIPLL_EN_DIV4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_en_div5` reader - "]
pub type WIFIPLL_EN_DIV5_R = crate::BitReader;
#[doc = "Field `wifipll_en_div5` writer - "]
pub type WIFIPLL_EN_DIV5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_en_div6` reader - "]
pub type WIFIPLL_EN_DIV6_R = crate::BitReader;
#[doc = "Field `wifipll_en_div6` writer - "]
pub type WIFIPLL_EN_DIV6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_en_div8` reader - "]
pub type WIFIPLL_EN_DIV8_R = crate::BitReader;
#[doc = "Field `wifipll_en_div8` writer - "]
pub type WIFIPLL_EN_DIV8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_en_div10` reader - "]
pub type WIFIPLL_EN_DIV10_R = crate::BitReader;
#[doc = "Field `wifipll_en_div10` writer - "]
pub type WIFIPLL_EN_DIV10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_en_div12` reader - "]
pub type WIFIPLL_EN_DIV12_R = crate::BitReader;
#[doc = "Field `wifipll_en_div12` writer - "]
pub type WIFIPLL_EN_DIV12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_en_div20` reader - "]
pub type WIFIPLL_EN_DIV20_R = crate::BitReader;
#[doc = "Field `wifipll_en_div20` writer - "]
pub type WIFIPLL_EN_DIV20_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_en_div30` reader - "]
pub type WIFIPLL_EN_DIV30_R = crate::BitReader;
#[doc = "Field `wifipll_en_div30` writer - "]
pub type WIFIPLL_EN_DIV30_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_en_rf_div3_hw` reader - "]
pub type WIFIPLL_EN_RF_DIV3_HW_R = crate::BitReader;
#[doc = "Field `wifipll_en_rf_div3_hw` writer - "]
pub type WIFIPLL_EN_RF_DIV3_HW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_en_ctrl_hw` reader - "]
pub type WIFIPLL_EN_CTRL_HW_R = crate::BitReader;
#[doc = "Field `wifipll_en_ctrl_hw` writer - "]
pub type WIFIPLL_EN_CTRL_HW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wifipll_en_rf_div3(&self) -> WIFIPLL_EN_RF_DIV3_R {
        WIFIPLL_EN_RF_DIV3_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wifipll_en_rf_div6(&self) -> WIFIPLL_EN_RF_DIV6_R {
        WIFIPLL_EN_RF_DIV6_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wifipll_sel_div3_div6(&self) -> WIFIPLL_SEL_DIV3_DIV6_R {
        WIFIPLL_SEL_DIV3_DIV6_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wifipll_sel_div6_div12(&self) -> WIFIPLL_SEL_DIV6_DIV12_R {
        WIFIPLL_SEL_DIV6_DIV12_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wifipll_en_div3(&self) -> WIFIPLL_EN_DIV3_R {
        WIFIPLL_EN_DIV3_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn wifipll_en_div4(&self) -> WIFIPLL_EN_DIV4_R {
        WIFIPLL_EN_DIV4_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn wifipll_en_div5(&self) -> WIFIPLL_EN_DIV5_R {
        WIFIPLL_EN_DIV5_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn wifipll_en_div6(&self) -> WIFIPLL_EN_DIV6_R {
        WIFIPLL_EN_DIV6_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn wifipll_en_div8(&self) -> WIFIPLL_EN_DIV8_R {
        WIFIPLL_EN_DIV8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn wifipll_en_div10(&self) -> WIFIPLL_EN_DIV10_R {
        WIFIPLL_EN_DIV10_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn wifipll_en_div12(&self) -> WIFIPLL_EN_DIV12_R {
        WIFIPLL_EN_DIV12_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn wifipll_en_div20(&self) -> WIFIPLL_EN_DIV20_R {
        WIFIPLL_EN_DIV20_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn wifipll_en_div30(&self) -> WIFIPLL_EN_DIV30_R {
        WIFIPLL_EN_DIV30_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn wifipll_en_rf_div3_hw(&self) -> WIFIPLL_EN_RF_DIV3_HW_R {
        WIFIPLL_EN_RF_DIV3_HW_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn wifipll_en_ctrl_hw(&self) -> WIFIPLL_EN_CTRL_HW_R {
        WIFIPLL_EN_CTRL_HW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_en_rf_div3(&mut self) -> WIFIPLL_EN_RF_DIV3_W<WIFI_PLL_CONFIG_8_SPEC> {
        WIFIPLL_EN_RF_DIV3_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_en_rf_div6(&mut self) -> WIFIPLL_EN_RF_DIV6_W<WIFI_PLL_CONFIG_8_SPEC> {
        WIFIPLL_EN_RF_DIV6_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_sel_div3_div6(&mut self) -> WIFIPLL_SEL_DIV3_DIV6_W<WIFI_PLL_CONFIG_8_SPEC> {
        WIFIPLL_SEL_DIV3_DIV6_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_sel_div6_div12(&mut self) -> WIFIPLL_SEL_DIV6_DIV12_W<WIFI_PLL_CONFIG_8_SPEC> {
        WIFIPLL_SEL_DIV6_DIV12_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_en_div3(&mut self) -> WIFIPLL_EN_DIV3_W<WIFI_PLL_CONFIG_8_SPEC> {
        WIFIPLL_EN_DIV3_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_en_div4(&mut self) -> WIFIPLL_EN_DIV4_W<WIFI_PLL_CONFIG_8_SPEC> {
        WIFIPLL_EN_DIV4_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_en_div5(&mut self) -> WIFIPLL_EN_DIV5_W<WIFI_PLL_CONFIG_8_SPEC> {
        WIFIPLL_EN_DIV5_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_en_div6(&mut self) -> WIFIPLL_EN_DIV6_W<WIFI_PLL_CONFIG_8_SPEC> {
        WIFIPLL_EN_DIV6_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_en_div8(&mut self) -> WIFIPLL_EN_DIV8_W<WIFI_PLL_CONFIG_8_SPEC> {
        WIFIPLL_EN_DIV8_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_en_div10(&mut self) -> WIFIPLL_EN_DIV10_W<WIFI_PLL_CONFIG_8_SPEC> {
        WIFIPLL_EN_DIV10_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_en_div12(&mut self) -> WIFIPLL_EN_DIV12_W<WIFI_PLL_CONFIG_8_SPEC> {
        WIFIPLL_EN_DIV12_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_en_div20(&mut self) -> WIFIPLL_EN_DIV20_W<WIFI_PLL_CONFIG_8_SPEC> {
        WIFIPLL_EN_DIV20_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_en_div30(&mut self) -> WIFIPLL_EN_DIV30_W<WIFI_PLL_CONFIG_8_SPEC> {
        WIFIPLL_EN_DIV30_W::new(self, 12)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_en_rf_div3_hw(&mut self) -> WIFIPLL_EN_RF_DIV3_HW_W<WIFI_PLL_CONFIG_8_SPEC> {
        WIFIPLL_EN_RF_DIV3_HW_W::new(self, 20)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_en_ctrl_hw(&mut self) -> WIFIPLL_EN_CTRL_HW_W<WIFI_PLL_CONFIG_8_SPEC> {
        WIFIPLL_EN_CTRL_HW_W::new(self, 31)
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
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_pll_config_8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_pll_config_8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIFI_PLL_CONFIG_8_SPEC;
impl crate::RegisterSpec for WIFI_PLL_CONFIG_8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_pll_config_8::R`](R) reader structure"]
impl crate::Readable for WIFI_PLL_CONFIG_8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wifi_pll_config_8::W`](W) writer structure"]
impl crate::Writable for WIFI_PLL_CONFIG_8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wifi_pll_config_8 to value 0"]
impl crate::Resettable for WIFI_PLL_CONFIG_8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
