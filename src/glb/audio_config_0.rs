#[doc = "Register `audio_config_0` reader"]
pub type R = crate::R<AUDIO_CONFIG_0_SPEC>;
#[doc = "Register `audio_config_0` writer"]
pub type W = crate::W<AUDIO_CONFIG_0_SPEC>;
#[doc = "Field `reg_audio_adc_clk_div` reader - "]
pub type REG_AUDIO_ADC_CLK_DIV_R = crate::FieldReader;
#[doc = "Field `reg_audio_adc_clk_div` writer - "]
pub type REG_AUDIO_ADC_CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `reg_audio_adc_clk_en` reader - "]
pub type REG_AUDIO_ADC_CLK_EN_R = crate::BitReader;
#[doc = "Field `reg_audio_adc_clk_en` writer - "]
pub type REG_AUDIO_ADC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_audio_auto_div_en` reader - "]
pub type REG_AUDIO_AUTO_DIV_EN_R = crate::BitReader;
#[doc = "Field `reg_audio_auto_div_en` writer - "]
pub type REG_AUDIO_AUTO_DIV_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn reg_audio_adc_clk_div(&self) -> REG_AUDIO_ADC_CLK_DIV_R {
        REG_AUDIO_ADC_CLK_DIV_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_audio_adc_clk_en(&self) -> REG_AUDIO_ADC_CLK_EN_R {
        REG_AUDIO_ADC_CLK_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reg_audio_auto_div_en(&self) -> REG_AUDIO_AUTO_DIV_EN_R {
        REG_AUDIO_AUTO_DIV_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:13"]
    #[inline(always)]
    #[must_use]
    pub fn reg_audio_adc_clk_div(&mut self) -> REG_AUDIO_ADC_CLK_DIV_W<AUDIO_CONFIG_0_SPEC> {
        REG_AUDIO_ADC_CLK_DIV_W::new(self, 8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn reg_audio_adc_clk_en(&mut self) -> REG_AUDIO_ADC_CLK_EN_W<AUDIO_CONFIG_0_SPEC> {
        REG_AUDIO_ADC_CLK_EN_W::new(self, 15)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_audio_auto_div_en(&mut self) -> REG_AUDIO_AUTO_DIV_EN_W<AUDIO_CONFIG_0_SPEC> {
        REG_AUDIO_AUTO_DIV_EN_W::new(self, 31)
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
#[doc = "Audio configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`audio_config_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`audio_config_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AUDIO_CONFIG_0_SPEC;
impl crate::RegisterSpec for AUDIO_CONFIG_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audio_config_0::R`](R) reader structure"]
impl crate::Readable for AUDIO_CONFIG_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`audio_config_0::W`](W) writer structure"]
impl crate::Writable for AUDIO_CONFIG_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets audio_config_0 to value 0"]
impl crate::Resettable for AUDIO_CONFIG_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
