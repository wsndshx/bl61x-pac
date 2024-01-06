#[doc = "Register `rs485_transmit` reader"]
pub type R = crate::R<RS485_TRANSMIT_SPEC>;
#[doc = "Register `rs485_transmit` writer"]
pub type W = crate::W<RS485_TRANSMIT_SPEC>;
#[doc = "Field `function` reader - RS-485 transceiver mode enable"]
pub type FUNCTION_R = crate::BitReader<FUNCTION_A>;
#[doc = "RS-485 transceiver mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FUNCTION_A {
    #[doc = "1: Enable RS-485 transceiver mode\n\n The peripheral is connected to RS-485 transceiver, and RTS signal output becomes Driver Enable (DE) signal."]
    ENABLE = 1,
    #[doc = "0: Disable RS-485 transceiver mode\n\n The peripheral operates as normal UART."]
    DISABLE = 0,
}
impl From<FUNCTION_A> for bool {
    #[inline(always)]
    fn from(variant: FUNCTION_A) -> Self {
        variant as u8 != 0
    }
}
impl FUNCTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FUNCTION_A {
        match self.bits {
            true => FUNCTION_A::ENABLE,
            false => FUNCTION_A::DISABLE,
        }
    }
    #[doc = "Enable RS-485 transceiver mode\n\n The peripheral is connected to RS-485 transceiver, and RTS signal output becomes Driver Enable (DE) signal."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FUNCTION_A::ENABLE
    }
    #[doc = "Disable RS-485 transceiver mode\n\n The peripheral operates as normal UART."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FUNCTION_A::DISABLE
    }
}
#[doc = "Field `function` writer - RS-485 transceiver mode enable"]
pub type FUNCTION_W<'a, REG> = crate::BitWriter<'a, REG, FUNCTION_A>;
impl<'a, REG> FUNCTION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable RS-485 transceiver mode\n\n The peripheral is connected to RS-485 transceiver, and RTS signal output becomes Driver Enable (DE) signal."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FUNCTION_A::ENABLE)
    }
    #[doc = "Disable RS-485 transceiver mode\n\n The peripheral operates as normal UART."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FUNCTION_A::DISABLE)
    }
}
#[doc = "Field `polarity` reader - RS-485 pin polarity of Driver Enable (DE) pin"]
pub type POLARITY_R = crate::BitReader<POLARITY_A>;
#[doc = "RS-485 pin polarity of Driver Enable (DE) pin\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POLARITY_A {
    #[doc = "1: Driver Enable (DE) pin is active high"]
    ACTIVE_HIGH = 1,
    #[doc = "0: Driver Enable (DE) pin is active low"]
    ACTIVE_LOW = 0,
}
impl From<POLARITY_A> for bool {
    #[inline(always)]
    fn from(variant: POLARITY_A) -> Self {
        variant as u8 != 0
    }
}
impl POLARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> POLARITY_A {
        match self.bits {
            true => POLARITY_A::ACTIVE_HIGH,
            false => POLARITY_A::ACTIVE_LOW,
        }
    }
    #[doc = "Driver Enable (DE) pin is active high"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == POLARITY_A::ACTIVE_HIGH
    }
    #[doc = "Driver Enable (DE) pin is active low"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == POLARITY_A::ACTIVE_LOW
    }
}
#[doc = "Field `polarity` writer - RS-485 pin polarity of Driver Enable (DE) pin"]
pub type POLARITY_W<'a, REG> = crate::BitWriter<'a, REG, POLARITY_A>;
impl<'a, REG> POLARITY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Driver Enable (DE) pin is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(POLARITY_A::ACTIVE_HIGH)
    }
    #[doc = "Driver Enable (DE) pin is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(POLARITY_A::ACTIVE_LOW)
    }
}
impl R {
    #[doc = "Bit 0 - RS-485 transceiver mode enable"]
    #[inline(always)]
    pub fn function(&self) -> FUNCTION_R {
        FUNCTION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RS-485 pin polarity of Driver Enable (DE) pin"]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RS-485 transceiver mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn function(&mut self) -> FUNCTION_W<RS485_TRANSMIT_SPEC> {
        FUNCTION_W::new(self, 0)
    }
    #[doc = "Bit 1 - RS-485 pin polarity of Driver Enable (DE) pin"]
    #[inline(always)]
    #[must_use]
    pub fn polarity(&mut self) -> POLARITY_W<RS485_TRANSMIT_SPEC> {
        POLARITY_W::new(self, 1)
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
#[doc = "RS-485 mode transmit configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rs485_transmit::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rs485_transmit::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RS485_TRANSMIT_SPEC;
impl crate::RegisterSpec for RS485_TRANSMIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rs485_transmit::R`](R) reader structure"]
impl crate::Readable for RS485_TRANSMIT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rs485_transmit::W`](W) writer structure"]
impl crate::Writable for RS485_TRANSMIT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rs485_transmit to value 0x02"]
impl crate::Resettable for RS485_TRANSMIT_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
