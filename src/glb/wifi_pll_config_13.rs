#[doc = "Register `wifi_pll_config_13` reader"]
pub type R = crate::R<WIFI_PLL_CONFIG_13_SPEC>;
#[doc = "Register `wifi_pll_config_13` writer"]
pub type W = crate::W<WIFI_PLL_CONFIG_13_SPEC>;
#[doc = "Field `sscdiv_ssc_cnt` reader - "]
pub type SSCDIV_SSC_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `sscdiv_ssc_cnt` writer - "]
pub type SSCDIV_SSC_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `sscdiv_ssc_start` reader - "]
pub type SSCDIV_SSC_START_R = crate::BitReader;
#[doc = "Field `sscdiv_ssc_start` writer - "]
pub type SSCDIV_SSC_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sscdiv_ssc_start_gate_en` reader - "]
pub type SSCDIV_SSC_START_GATE_EN_R = crate::BitReader;
#[doc = "Field `sscdiv_ssc_start_gate_en` writer - "]
pub type SSCDIV_SSC_START_GATE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sscdiv_ssc_gain` reader - "]
pub type SSCDIV_SSC_GAIN_R = crate::FieldReader;
#[doc = "Field `sscdiv_ssc_gain` writer - "]
pub type SSCDIV_SSC_GAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `sscdiv_ssc_en` reader - "]
pub type SSCDIV_SSC_EN_R = crate::BitReader;
#[doc = "Field `sscdiv_ssc_en` writer - "]
pub type SSCDIV_SSC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn sscdiv_ssc_cnt(&self) -> SSCDIV_SSC_CNT_R {
        SSCDIV_SSC_CNT_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sscdiv_ssc_start(&self) -> SSCDIV_SSC_START_R {
        SSCDIV_SSC_START_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sscdiv_ssc_start_gate_en(&self) -> SSCDIV_SSC_START_GATE_EN_R {
        SSCDIV_SSC_START_GATE_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn sscdiv_ssc_gain(&self) -> SSCDIV_SSC_GAIN_R {
        SSCDIV_SSC_GAIN_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sscdiv_ssc_en(&self) -> SSCDIV_SSC_EN_R {
        SSCDIV_SSC_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    #[must_use]
    pub fn sscdiv_ssc_cnt(&mut self) -> SSCDIV_SSC_CNT_W<WIFI_PLL_CONFIG_13_SPEC> {
        SSCDIV_SSC_CNT_W::new(self, 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn sscdiv_ssc_start(&mut self) -> SSCDIV_SSC_START_W<WIFI_PLL_CONFIG_13_SPEC> {
        SSCDIV_SSC_START_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn sscdiv_ssc_start_gate_en(
        &mut self,
    ) -> SSCDIV_SSC_START_GATE_EN_W<WIFI_PLL_CONFIG_13_SPEC> {
        SSCDIV_SSC_START_GATE_EN_W::new(self, 10)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn sscdiv_ssc_gain(&mut self) -> SSCDIV_SSC_GAIN_W<WIFI_PLL_CONFIG_13_SPEC> {
        SSCDIV_SSC_GAIN_W::new(self, 12)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn sscdiv_ssc_en(&mut self) -> SSCDIV_SSC_EN_W<WIFI_PLL_CONFIG_13_SPEC> {
        SSCDIV_SSC_EN_W::new(self, 16)
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
#[doc = "Wireless Fidelity Phase-Locked Loop configuration 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_pll_config_13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_pll_config_13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIFI_PLL_CONFIG_13_SPEC;
impl crate::RegisterSpec for WIFI_PLL_CONFIG_13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_pll_config_13::R`](R) reader structure"]
impl crate::Readable for WIFI_PLL_CONFIG_13_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wifi_pll_config_13::W`](W) writer structure"]
impl crate::Writable for WIFI_PLL_CONFIG_13_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets wifi_pll_config_13 to value 0"]
impl crate::Resettable for WIFI_PLL_CONFIG_13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
