#[doc = "Register `signal_override` reader"]
pub type R = crate::R<SIGNAL_OVERRIDE_SPEC>;
#[doc = "Register `signal_override` writer"]
pub type W = crate::W<SIGNAL_OVERRIDE_SPEC>;
#[doc = "Field `transmit_signal` reader - Enable manual override of transmit signal"]
pub use RTS_SIGNAL_R as TRANSMIT_SIGNAL_R;
#[doc = "Field `transmit_signal` writer - Enable manual override of transmit signal"]
pub use RTS_SIGNAL_W as TRANSMIT_SIGNAL_W;
#[doc = "Field `transmit_value` reader - Value to override transmit signal if override is enabled"]
pub use RTS_VALUE_R as TRANSMIT_VALUE_R;
#[doc = "Field `transmit_value` writer - Value to override transmit signal if override is enabled"]
pub use RTS_VALUE_W as TRANSMIT_VALUE_W;
#[doc = "Field `rts_signal` reader - Enable manual override of Request-to-Send flow control signal"]
pub type RTS_SIGNAL_R = crate::BitReader<OVERRIDE_ENABLE_A>;
#[doc = "Enable manual override of Request-to-Send flow control signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVERRIDE_ENABLE_A {
    #[doc = "1: Enable manual override of this signal"]
    ENABLE = 1,
    #[doc = "0: Disable manual override of this signal"]
    DISABLE = 0,
}
impl From<OVERRIDE_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: OVERRIDE_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl RTS_SIGNAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVERRIDE_ENABLE_A {
        match self.bits {
            true => OVERRIDE_ENABLE_A::ENABLE,
            false => OVERRIDE_ENABLE_A::DISABLE,
        }
    }
    #[doc = "Enable manual override of this signal"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == OVERRIDE_ENABLE_A::ENABLE
    }
    #[doc = "Disable manual override of this signal"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OVERRIDE_ENABLE_A::DISABLE
    }
}
#[doc = "Field `rts_signal` writer - Enable manual override of Request-to-Send flow control signal"]
pub type RTS_SIGNAL_W<'a, REG> = crate::BitWriter<'a, REG, OVERRIDE_ENABLE_A>;
impl<'a, REG> RTS_SIGNAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable manual override of this signal"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(OVERRIDE_ENABLE_A::ENABLE)
    }
    #[doc = "Disable manual override of this signal"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(OVERRIDE_ENABLE_A::DISABLE)
    }
}
#[doc = "Field `rts_value` reader - Value to override Request-to-Send signal if override is enabled"]
pub type RTS_VALUE_R = crate::BitReader<SIGNAL_ASSERT_A>;
#[doc = "Value to override Request-to-Send signal if override is enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIGNAL_ASSERT_A {
    #[doc = "1: Assert this signal"]
    HIGH = 1,
    #[doc = "0: Deassert this signal"]
    LOW = 0,
}
impl From<SIGNAL_ASSERT_A> for bool {
    #[inline(always)]
    fn from(variant: SIGNAL_ASSERT_A) -> Self {
        variant as u8 != 0
    }
}
impl RTS_VALUE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SIGNAL_ASSERT_A {
        match self.bits {
            true => SIGNAL_ASSERT_A::HIGH,
            false => SIGNAL_ASSERT_A::LOW,
        }
    }
    #[doc = "Assert this signal"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SIGNAL_ASSERT_A::HIGH
    }
    #[doc = "Deassert this signal"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SIGNAL_ASSERT_A::LOW
    }
}
#[doc = "Field `rts_value` writer - Value to override Request-to-Send signal if override is enabled"]
pub type RTS_VALUE_W<'a, REG> = crate::BitWriter<'a, REG, SIGNAL_ASSERT_A>;
impl<'a, REG> RTS_VALUE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Assert this signal"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(SIGNAL_ASSERT_A::HIGH)
    }
    #[doc = "Deassert this signal"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(SIGNAL_ASSERT_A::LOW)
    }
}
impl R {
    #[doc = "Bit 0 - Enable manual override of transmit signal"]
    #[inline(always)]
    pub fn transmit_signal(&self) -> TRANSMIT_SIGNAL_R {
        TRANSMIT_SIGNAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Value to override transmit signal if override is enabled"]
    #[inline(always)]
    pub fn transmit_value(&self) -> TRANSMIT_VALUE_R {
        TRANSMIT_VALUE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable manual override of Request-to-Send flow control signal"]
    #[inline(always)]
    pub fn rts_signal(&self) -> RTS_SIGNAL_R {
        RTS_SIGNAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Value to override Request-to-Send signal if override is enabled"]
    #[inline(always)]
    pub fn rts_value(&self) -> RTS_VALUE_R {
        RTS_VALUE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable manual override of transmit signal"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_signal(&mut self) -> TRANSMIT_SIGNAL_W<SIGNAL_OVERRIDE_SPEC> {
        TRANSMIT_SIGNAL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Value to override transmit signal if override is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn transmit_value(&mut self) -> TRANSMIT_VALUE_W<SIGNAL_OVERRIDE_SPEC> {
        TRANSMIT_VALUE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable manual override of Request-to-Send flow control signal"]
    #[inline(always)]
    #[must_use]
    pub fn rts_signal(&mut self) -> RTS_SIGNAL_W<SIGNAL_OVERRIDE_SPEC> {
        RTS_SIGNAL_W::new(self, 2)
    }
    #[doc = "Bit 3 - Value to override Request-to-Send signal if override is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn rts_value(&mut self) -> RTS_VALUE_W<SIGNAL_OVERRIDE_SPEC> {
        RTS_VALUE_W::new(self, 3)
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
#[doc = "Manual override of flow control signal\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`signal_override::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`signal_override::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIGNAL_OVERRIDE_SPEC;
impl crate::RegisterSpec for SIGNAL_OVERRIDE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`signal_override::R`](R) reader structure"]
impl crate::Readable for SIGNAL_OVERRIDE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`signal_override::W`](W) writer structure"]
impl crate::Writable for SIGNAL_OVERRIDE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets signal_override to value 0"]
impl crate::Resettable for SIGNAL_OVERRIDE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
