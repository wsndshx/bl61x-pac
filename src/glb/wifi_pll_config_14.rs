#[doc = "Register `wifi_pll_config_14` reader"]
pub type R = crate::R<WIFI_PLL_CONFIG_14_SPEC>;
#[doc = "Register `wifi_pll_config_14` writer"]
pub type W = crate::W<WIFI_PLL_CONFIG_14_SPEC>;
#[doc = "Field `wifipll_resv` reader - "]
pub type WIFIPLL_RESV_R = crate::FieldReader<u16>;
#[doc = "Field `wifipll_resv` writer - "]
pub type WIFIPLL_RESV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `sscdiv_dl_ctrl` reader - "]
pub type SSCDIV_DL_CTRL_R = crate::BitReader;
#[doc = "Field `sscdiv_dl_ctrl` writer - "]
pub type SSCDIV_DL_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `usbpll_dl_ctrl` reader - "]
pub type USBPLL_DL_CTRL_R = crate::BitReader;
#[doc = "Field `usbpll_dl_ctrl` writer - "]
pub type USBPLL_DL_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_dl_ctrl_30` reader - "]
pub type WIFIPLL_DL_CTRL_30_R = crate::BitReader;
#[doc = "Field `wifipll_dl_ctrl_30` writer - "]
pub type WIFIPLL_DL_CTRL_30_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_dl_ctrl_20` reader - "]
pub type WIFIPLL_DL_CTRL_20_R = crate::BitReader;
#[doc = "Field `wifipll_dl_ctrl_20` writer - "]
pub type WIFIPLL_DL_CTRL_20_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_dl_ctrl_12` reader - "]
pub type WIFIPLL_DL_CTRL_12_R = crate::BitReader;
#[doc = "Field `wifipll_dl_ctrl_12` writer - "]
pub type WIFIPLL_DL_CTRL_12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_dl_ctrl_10` reader - "]
pub type WIFIPLL_DL_CTRL_10_R = crate::BitReader;
#[doc = "Field `wifipll_dl_ctrl_10` writer - "]
pub type WIFIPLL_DL_CTRL_10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_dl_ctrl_8` reader - "]
pub type WIFIPLL_DL_CTRL_8_R = crate::BitReader;
#[doc = "Field `wifipll_dl_ctrl_8` writer - "]
pub type WIFIPLL_DL_CTRL_8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_dl_ctrl_6_rf` reader - "]
pub type WIFIPLL_DL_CTRL_6_RF_R = crate::BitReader;
#[doc = "Field `wifipll_dl_ctrl_6_rf` writer - "]
pub type WIFIPLL_DL_CTRL_6_RF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_dl_ctrl_6` reader - "]
pub type WIFIPLL_DL_CTRL_6_R = crate::BitReader;
#[doc = "Field `wifipll_dl_ctrl_6` writer - "]
pub type WIFIPLL_DL_CTRL_6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_dl_ctrl_5` reader - "]
pub type WIFIPLL_DL_CTRL_5_R = crate::BitReader;
#[doc = "Field `wifipll_dl_ctrl_5` writer - "]
pub type WIFIPLL_DL_CTRL_5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_dl_ctrl_4` reader - "]
pub type WIFIPLL_DL_CTRL_4_R = crate::BitReader;
#[doc = "Field `wifipll_dl_ctrl_4` writer - "]
pub type WIFIPLL_DL_CTRL_4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_dl_ctrl_3_rf` reader - "]
pub type WIFIPLL_DL_CTRL_3_RF_R = crate::BitReader;
#[doc = "Field `wifipll_dl_ctrl_3_rf` writer - "]
pub type WIFIPLL_DL_CTRL_3_RF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wifipll_dl_ctrl_3` reader - "]
pub type WIFIPLL_DL_CTRL_3_R = crate::BitReader;
#[doc = "Field `wifipll_dl_ctrl_3` writer - "]
pub type WIFIPLL_DL_CTRL_3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wifipll_resv(&self) -> WIFIPLL_RESV_R {
        WIFIPLL_RESV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sscdiv_dl_ctrl(&self) -> SSCDIV_DL_CTRL_R {
        SSCDIV_DL_CTRL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn usbpll_dl_ctrl(&self) -> USBPLL_DL_CTRL_R {
        USBPLL_DL_CTRL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_30(&self) -> WIFIPLL_DL_CTRL_30_R {
        WIFIPLL_DL_CTRL_30_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_20(&self) -> WIFIPLL_DL_CTRL_20_R {
        WIFIPLL_DL_CTRL_20_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_12(&self) -> WIFIPLL_DL_CTRL_12_R {
        WIFIPLL_DL_CTRL_12_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_10(&self) -> WIFIPLL_DL_CTRL_10_R {
        WIFIPLL_DL_CTRL_10_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_8(&self) -> WIFIPLL_DL_CTRL_8_R {
        WIFIPLL_DL_CTRL_8_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_6_rf(&self) -> WIFIPLL_DL_CTRL_6_RF_R {
        WIFIPLL_DL_CTRL_6_RF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_6(&self) -> WIFIPLL_DL_CTRL_6_R {
        WIFIPLL_DL_CTRL_6_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_5(&self) -> WIFIPLL_DL_CTRL_5_R {
        WIFIPLL_DL_CTRL_5_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_4(&self) -> WIFIPLL_DL_CTRL_4_R {
        WIFIPLL_DL_CTRL_4_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_3_rf(&self) -> WIFIPLL_DL_CTRL_3_RF_R {
        WIFIPLL_DL_CTRL_3_RF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn wifipll_dl_ctrl_3(&self) -> WIFIPLL_DL_CTRL_3_R {
        WIFIPLL_DL_CTRL_3_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_resv(&mut self) -> WIFIPLL_RESV_W<WIFI_PLL_CONFIG_14_SPEC> {
        WIFIPLL_RESV_W::new(self, 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn sscdiv_dl_ctrl(&mut self) -> SSCDIV_DL_CTRL_W<WIFI_PLL_CONFIG_14_SPEC> {
        SSCDIV_DL_CTRL_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn usbpll_dl_ctrl(&mut self) -> USBPLL_DL_CTRL_W<WIFI_PLL_CONFIG_14_SPEC> {
        USBPLL_DL_CTRL_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_dl_ctrl_30(&mut self) -> WIFIPLL_DL_CTRL_30_W<WIFI_PLL_CONFIG_14_SPEC> {
        WIFIPLL_DL_CTRL_30_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_dl_ctrl_20(&mut self) -> WIFIPLL_DL_CTRL_20_W<WIFI_PLL_CONFIG_14_SPEC> {
        WIFIPLL_DL_CTRL_20_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_dl_ctrl_12(&mut self) -> WIFIPLL_DL_CTRL_12_W<WIFI_PLL_CONFIG_14_SPEC> {
        WIFIPLL_DL_CTRL_12_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_dl_ctrl_10(&mut self) -> WIFIPLL_DL_CTRL_10_W<WIFI_PLL_CONFIG_14_SPEC> {
        WIFIPLL_DL_CTRL_10_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_dl_ctrl_8(&mut self) -> WIFIPLL_DL_CTRL_8_W<WIFI_PLL_CONFIG_14_SPEC> {
        WIFIPLL_DL_CTRL_8_W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_dl_ctrl_6_rf(&mut self) -> WIFIPLL_DL_CTRL_6_RF_W<WIFI_PLL_CONFIG_14_SPEC> {
        WIFIPLL_DL_CTRL_6_RF_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_dl_ctrl_6(&mut self) -> WIFIPLL_DL_CTRL_6_W<WIFI_PLL_CONFIG_14_SPEC> {
        WIFIPLL_DL_CTRL_6_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_dl_ctrl_5(&mut self) -> WIFIPLL_DL_CTRL_5_W<WIFI_PLL_CONFIG_14_SPEC> {
        WIFIPLL_DL_CTRL_5_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_dl_ctrl_4(&mut self) -> WIFIPLL_DL_CTRL_4_W<WIFI_PLL_CONFIG_14_SPEC> {
        WIFIPLL_DL_CTRL_4_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_dl_ctrl_3_rf(&mut self) -> WIFIPLL_DL_CTRL_3_RF_W<WIFI_PLL_CONFIG_14_SPEC> {
        WIFIPLL_DL_CTRL_3_RF_W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn wifipll_dl_ctrl_3(&mut self) -> WIFIPLL_DL_CTRL_3_W<WIFI_PLL_CONFIG_14_SPEC> {
        WIFIPLL_DL_CTRL_3_W::new(self, 28)
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
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_pll_config_14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_pll_config_14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIFI_PLL_CONFIG_14_SPEC;
impl crate::RegisterSpec for WIFI_PLL_CONFIG_14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_pll_config_14::R`](R) reader structure"]
impl crate::Readable for WIFI_PLL_CONFIG_14_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wifi_pll_config_14::W`](W) writer structure"]
impl crate::Writable for WIFI_PLL_CONFIG_14_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wifi_pll_config_14 to value 0"]
impl crate::Resettable for WIFI_PLL_CONFIG_14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
