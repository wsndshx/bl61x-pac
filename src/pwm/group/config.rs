#[doc = "Register `config` reader"]
pub type R = crate::R<CONFIG_SPEC>;
#[doc = "Register `config` writer"]
pub type W = crate::W<CONFIG_SPEC>;
#[doc = "Field `stop_on_repeat` reader - Enable or disable stop on one repeat cycle completed"]
pub type STOP_ON_REPEAT_R = crate::BitReader;
#[doc = "Field `stop_on_repeat` writer - Enable or disable stop on one repeat cycle completed"]
pub type STOP_ON_REPEAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `adc_trigger_source` reader - Select channel in Analog-to-Digital Converter to interact with this peripheral\n\n **This field only works with PWM0.** PWM1 does not have this feature."]
pub type ADC_TRIGGER_SOURCE_R = crate::FieldReader;
#[doc = "Field `adc_trigger_source` writer - Select channel in Analog-to-Digital Converter to interact with this peripheral\n\n **This field only works with PWM0.** PWM1 does not have this feature."]
pub type ADC_TRIGGER_SOURCE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `software_break` reader - Enable software signal break"]
pub type SOFTWARE_BREAK_R = crate::BitReader;
#[doc = "Field `software_break` writer - Enable software signal break"]
pub type SOFTWARE_BREAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `external_break` reader - Enable external pin signal break"]
pub type EXTERNAL_BREAK_R = crate::BitReader;
#[doc = "Field `external_break` writer - Enable external pin signal break"]
pub type EXTERNAL_BREAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `external_polarity` reader - Polarity for external pin break function"]
pub type EXTERNAL_POLARITY_R = crate::BitReader;
#[doc = "Field `external_polarity` writer - Polarity for external pin break function"]
pub type EXTERNAL_POLARITY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `stop_function` reader - Enable or disable stop function"]
pub type STOP_FUNCTION_R = crate::BitReader;
#[doc = "Field `stop_function` writer - Enable or disable stop function"]
pub type STOP_FUNCTION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `stop_mode` reader - Mode to stop this peripheral"]
pub type STOP_MODE_R = crate::BitReader;
#[doc = "Field `stop_mode` writer - Mode to stop this peripheral"]
pub type STOP_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `stop_state` reader - Is this peripheral stopped?"]
pub type STOP_STATE_R = crate::BitReader;
#[doc = "Field `stop_state` writer - Is this peripheral stopped?"]
pub type STOP_STATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clock_select` reader - Select group clock source"]
pub type CLOCK_SELECT_R = crate::FieldReader<CLOCK_SOURCE_A>;
#[doc = "Select group clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLOCK_SOURCE_A {
    #[doc = "0: External crystal as clock source"]
    XTAL = 0,
    #[doc = "1: Bus clock as clock source"]
    BCLK = 1,
    #[doc = "2: 32-kHz clock source"]
    F32K = 2,
}
impl From<CLOCK_SOURCE_A> for u8 {
    #[inline(always)]
    fn from(variant: CLOCK_SOURCE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLOCK_SOURCE_A {
    type Ux = u8;
}
impl CLOCK_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLOCK_SOURCE_A> {
        match self.bits {
            0 => Some(CLOCK_SOURCE_A::XTAL),
            1 => Some(CLOCK_SOURCE_A::BCLK),
            2 => Some(CLOCK_SOURCE_A::F32K),
            _ => None,
        }
    }
    #[doc = "External crystal as clock source"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == CLOCK_SOURCE_A::XTAL
    }
    #[doc = "Bus clock as clock source"]
    #[inline(always)]
    pub fn is_bclk(&self) -> bool {
        *self == CLOCK_SOURCE_A::BCLK
    }
    #[doc = "32-kHz clock source"]
    #[inline(always)]
    pub fn is_f32k(&self) -> bool {
        *self == CLOCK_SOURCE_A::F32K
    }
}
#[doc = "Field `clock_select` writer - Select group clock source"]
pub type CLOCK_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CLOCK_SOURCE_A>;
impl<'a, REG> CLOCK_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External crystal as clock source"]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut crate::W<REG> {
        self.variant(CLOCK_SOURCE_A::XTAL)
    }
    #[doc = "Bus clock as clock source"]
    #[inline(always)]
    pub fn bclk(self) -> &'a mut crate::W<REG> {
        self.variant(CLOCK_SOURCE_A::BCLK)
    }
    #[doc = "32-kHz clock source"]
    #[inline(always)]
    pub fn f32k(self) -> &'a mut crate::W<REG> {
        self.variant(CLOCK_SOURCE_A::F32K)
    }
}
impl R {
    #[doc = "Bit 19 - Enable or disable stop on one repeat cycle completed"]
    #[inline(always)]
    pub fn stop_on_repeat(&self) -> STOP_ON_REPEAT_R {
        STOP_ON_REPEAT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Select channel in Analog-to-Digital Converter to interact with this peripheral\n\n **This field only works with PWM0.** PWM1 does not have this feature."]
    #[inline(always)]
    pub fn adc_trigger_source(&self) -> ADC_TRIGGER_SOURCE_R {
        ADC_TRIGGER_SOURCE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Enable software signal break"]
    #[inline(always)]
    pub fn software_break(&self) -> SOFTWARE_BREAK_R {
        SOFTWARE_BREAK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable external pin signal break"]
    #[inline(always)]
    pub fn external_break(&self) -> EXTERNAL_BREAK_R {
        EXTERNAL_BREAK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Polarity for external pin break function"]
    #[inline(always)]
    pub fn external_polarity(&self) -> EXTERNAL_POLARITY_R {
        EXTERNAL_POLARITY_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable or disable stop function"]
    #[inline(always)]
    pub fn stop_function(&self) -> STOP_FUNCTION_R {
        STOP_FUNCTION_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Mode to stop this peripheral"]
    #[inline(always)]
    pub fn stop_mode(&self) -> STOP_MODE_R {
        STOP_MODE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Is this peripheral stopped?"]
    #[inline(always)]
    pub fn stop_state(&self) -> STOP_STATE_R {
        STOP_STATE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Select group clock source"]
    #[inline(always)]
    pub fn clock_select(&self) -> CLOCK_SELECT_R {
        CLOCK_SELECT_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 19 - Enable or disable stop on one repeat cycle completed"]
    #[inline(always)]
    #[must_use]
    pub fn stop_on_repeat(&mut self) -> STOP_ON_REPEAT_W<CONFIG_SPEC> {
        STOP_ON_REPEAT_W::new(self, 19)
    }
    #[doc = "Bits 20:23 - Select channel in Analog-to-Digital Converter to interact with this peripheral\n\n **This field only works with PWM0.** PWM1 does not have this feature."]
    #[inline(always)]
    #[must_use]
    pub fn adc_trigger_source(&mut self) -> ADC_TRIGGER_SOURCE_W<CONFIG_SPEC> {
        ADC_TRIGGER_SOURCE_W::new(self, 20)
    }
    #[doc = "Bit 24 - Enable software signal break"]
    #[inline(always)]
    #[must_use]
    pub fn software_break(&mut self) -> SOFTWARE_BREAK_W<CONFIG_SPEC> {
        SOFTWARE_BREAK_W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable external pin signal break"]
    #[inline(always)]
    #[must_use]
    pub fn external_break(&mut self) -> EXTERNAL_BREAK_W<CONFIG_SPEC> {
        EXTERNAL_BREAK_W::new(self, 25)
    }
    #[doc = "Bit 26 - Polarity for external pin break function"]
    #[inline(always)]
    #[must_use]
    pub fn external_polarity(&mut self) -> EXTERNAL_POLARITY_W<CONFIG_SPEC> {
        EXTERNAL_POLARITY_W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable or disable stop function"]
    #[inline(always)]
    #[must_use]
    pub fn stop_function(&mut self) -> STOP_FUNCTION_W<CONFIG_SPEC> {
        STOP_FUNCTION_W::new(self, 27)
    }
    #[doc = "Bit 28 - Mode to stop this peripheral"]
    #[inline(always)]
    #[must_use]
    pub fn stop_mode(&mut self) -> STOP_MODE_W<CONFIG_SPEC> {
        STOP_MODE_W::new(self, 28)
    }
    #[doc = "Bit 29 - Is this peripheral stopped?"]
    #[inline(always)]
    #[must_use]
    pub fn stop_state(&mut self) -> STOP_STATE_W<CONFIG_SPEC> {
        STOP_STATE_W::new(self, 29)
    }
    #[doc = "Bits 30:31 - Select group clock source"]
    #[inline(always)]
    #[must_use]
    pub fn clock_select(&mut self) -> CLOCK_SELECT_W<CONFIG_SPEC> {
        CLOCK_SELECT_W::new(self, 30)
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
#[doc = "Peripheral group configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets config to value 0"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
