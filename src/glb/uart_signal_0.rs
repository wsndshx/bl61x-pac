#[doc = "Register `uart_signal_0` reader"]
pub type R = crate::R<UART_SIGNAL_0_SPEC>;
#[doc = "Register `uart_signal_0` writer"]
pub type W = crate::W<UART_SIGNAL_0_SPEC>;
#[doc = "Field `function_0(0-7)` reader - Select peripheral function for UART signal %s"]
pub type FUNCTION_0_R = crate::FieldReader<FUNCTION_A>;
#[doc = "Select peripheral function for UART signal %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FUNCTION_A {
    #[doc = "0: UART0 Request-to-Send flow control"]
    UART0_RTS = 0,
    #[doc = "1: UART0 Clear-to-Send flow control"]
    UART0_CTS = 1,
    #[doc = "2: UART0 transmit data"]
    UART0_TXD = 2,
    #[doc = "3: UART0 receive data"]
    UART0_RXD = 3,
    #[doc = "4: UART1 Request-to-Send flow control"]
    UART1_RTS = 4,
    #[doc = "5: UART1 Clear-to-Send flow control"]
    UART1_CTS = 5,
    #[doc = "6: UART1 transmit data"]
    UART1_TXD = 6,
    #[doc = "7: UART1 receive data"]
    UART1_RXD = 7,
}
impl From<FUNCTION_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNCTION_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FUNCTION_A {
    type Ux = u8;
}
impl FUNCTION_0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FUNCTION_A> {
        match self.bits {
            0 => Some(FUNCTION_A::UART0_RTS),
            1 => Some(FUNCTION_A::UART0_CTS),
            2 => Some(FUNCTION_A::UART0_TXD),
            3 => Some(FUNCTION_A::UART0_RXD),
            4 => Some(FUNCTION_A::UART1_RTS),
            5 => Some(FUNCTION_A::UART1_CTS),
            6 => Some(FUNCTION_A::UART1_TXD),
            7 => Some(FUNCTION_A::UART1_RXD),
            _ => None,
        }
    }
    #[doc = "UART0 Request-to-Send flow control"]
    #[inline(always)]
    pub fn is_uart0_rts(&self) -> bool {
        *self == FUNCTION_A::UART0_RTS
    }
    #[doc = "UART0 Clear-to-Send flow control"]
    #[inline(always)]
    pub fn is_uart0_cts(&self) -> bool {
        *self == FUNCTION_A::UART0_CTS
    }
    #[doc = "UART0 transmit data"]
    #[inline(always)]
    pub fn is_uart0_txd(&self) -> bool {
        *self == FUNCTION_A::UART0_TXD
    }
    #[doc = "UART0 receive data"]
    #[inline(always)]
    pub fn is_uart0_rxd(&self) -> bool {
        *self == FUNCTION_A::UART0_RXD
    }
    #[doc = "UART1 Request-to-Send flow control"]
    #[inline(always)]
    pub fn is_uart1_rts(&self) -> bool {
        *self == FUNCTION_A::UART1_RTS
    }
    #[doc = "UART1 Clear-to-Send flow control"]
    #[inline(always)]
    pub fn is_uart1_cts(&self) -> bool {
        *self == FUNCTION_A::UART1_CTS
    }
    #[doc = "UART1 transmit data"]
    #[inline(always)]
    pub fn is_uart1_txd(&self) -> bool {
        *self == FUNCTION_A::UART1_TXD
    }
    #[doc = "UART1 receive data"]
    #[inline(always)]
    pub fn is_uart1_rxd(&self) -> bool {
        *self == FUNCTION_A::UART1_RXD
    }
}
#[doc = "Field `function_0(0-7)` writer - Select peripheral function for UART signal %s"]
pub type FUNCTION_0_W<'a, REG> = crate::FieldWriter<'a, REG, 8, FUNCTION_A>;
impl<'a, REG> FUNCTION_0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "UART0 Request-to-Send flow control"]
    #[inline(always)]
    pub fn uart0_rts(self) -> &'a mut crate::W<REG> {
        self.variant(FUNCTION_A::UART0_RTS)
    }
    #[doc = "UART0 Clear-to-Send flow control"]
    #[inline(always)]
    pub fn uart0_cts(self) -> &'a mut crate::W<REG> {
        self.variant(FUNCTION_A::UART0_CTS)
    }
    #[doc = "UART0 transmit data"]
    #[inline(always)]
    pub fn uart0_txd(self) -> &'a mut crate::W<REG> {
        self.variant(FUNCTION_A::UART0_TXD)
    }
    #[doc = "UART0 receive data"]
    #[inline(always)]
    pub fn uart0_rxd(self) -> &'a mut crate::W<REG> {
        self.variant(FUNCTION_A::UART0_RXD)
    }
    #[doc = "UART1 Request-to-Send flow control"]
    #[inline(always)]
    pub fn uart1_rts(self) -> &'a mut crate::W<REG> {
        self.variant(FUNCTION_A::UART1_RTS)
    }
    #[doc = "UART1 Clear-to-Send flow control"]
    #[inline(always)]
    pub fn uart1_cts(self) -> &'a mut crate::W<REG> {
        self.variant(FUNCTION_A::UART1_CTS)
    }
    #[doc = "UART1 transmit data"]
    #[inline(always)]
    pub fn uart1_txd(self) -> &'a mut crate::W<REG> {
        self.variant(FUNCTION_A::UART1_TXD)
    }
    #[doc = "UART1 receive data"]
    #[inline(always)]
    pub fn uart1_rxd(self) -> &'a mut crate::W<REG> {
        self.variant(FUNCTION_A::UART1_RXD)
    }
}
impl R {
    #[doc = "Select peripheral function for UART signal (0-7)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `function_00` field"]
    #[inline(always)]
    pub fn function_0(&self, n: u8) -> FUNCTION_0_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        FUNCTION_0_R::new(((self.bits >> (n * 4)) & 0xff) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Select peripheral function for UART signal (0-7)"]
    #[inline(always)]
    pub fn function_0_iter(&self) -> impl Iterator<Item = FUNCTION_0_R> + '_ {
        (0..8).map(move |n| FUNCTION_0_R::new(((self.bits >> (n * 4)) & 0xff) as u8))
    }
    #[doc = "Bits 0:7 - Select peripheral function for UART signal 0"]
    #[inline(always)]
    pub fn function_00(&self) -> FUNCTION_0_R {
        FUNCTION_0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 4:11 - Select peripheral function for UART signal 1"]
    #[inline(always)]
    pub fn function_01(&self) -> FUNCTION_0_R {
        FUNCTION_0_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Select peripheral function for UART signal 2"]
    #[inline(always)]
    pub fn function_02(&self) -> FUNCTION_0_R {
        FUNCTION_0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 12:19 - Select peripheral function for UART signal 3"]
    #[inline(always)]
    pub fn function_03(&self) -> FUNCTION_0_R {
        FUNCTION_0_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Select peripheral function for UART signal 4"]
    #[inline(always)]
    pub fn function_04(&self) -> FUNCTION_0_R {
        FUNCTION_0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 20:27 - Select peripheral function for UART signal 5"]
    #[inline(always)]
    pub fn function_05(&self) -> FUNCTION_0_R {
        FUNCTION_0_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Select peripheral function for UART signal 6"]
    #[inline(always)]
    pub fn function_06(&self) -> FUNCTION_0_R {
        FUNCTION_0_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 28:35 - Select peripheral function for UART signal 7"]
    #[inline(always)]
    pub fn function_07(&self) -> FUNCTION_0_R {
        FUNCTION_0_R::new(((self.bits >> 28) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Select peripheral function for UART signal (0-7)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `function_00` field"]
    #[inline(always)]
    #[must_use]
    pub fn function_0(&mut self, n: u8) -> FUNCTION_0_W<UART_SIGNAL_0_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        FUNCTION_0_W::new(self, n * 4)
    }
    #[doc = "Bits 0:7 - Select peripheral function for UART signal 0"]
    #[inline(always)]
    #[must_use]
    pub fn function_00(&mut self) -> FUNCTION_0_W<UART_SIGNAL_0_SPEC> {
        FUNCTION_0_W::new(self, 0)
    }
    #[doc = "Bits 4:11 - Select peripheral function for UART signal 1"]
    #[inline(always)]
    #[must_use]
    pub fn function_01(&mut self) -> FUNCTION_0_W<UART_SIGNAL_0_SPEC> {
        FUNCTION_0_W::new(self, 4)
    }
    #[doc = "Bits 8:15 - Select peripheral function for UART signal 2"]
    #[inline(always)]
    #[must_use]
    pub fn function_02(&mut self) -> FUNCTION_0_W<UART_SIGNAL_0_SPEC> {
        FUNCTION_0_W::new(self, 8)
    }
    #[doc = "Bits 12:19 - Select peripheral function for UART signal 3"]
    #[inline(always)]
    #[must_use]
    pub fn function_03(&mut self) -> FUNCTION_0_W<UART_SIGNAL_0_SPEC> {
        FUNCTION_0_W::new(self, 12)
    }
    #[doc = "Bits 16:23 - Select peripheral function for UART signal 4"]
    #[inline(always)]
    #[must_use]
    pub fn function_04(&mut self) -> FUNCTION_0_W<UART_SIGNAL_0_SPEC> {
        FUNCTION_0_W::new(self, 16)
    }
    #[doc = "Bits 20:27 - Select peripheral function for UART signal 5"]
    #[inline(always)]
    #[must_use]
    pub fn function_05(&mut self) -> FUNCTION_0_W<UART_SIGNAL_0_SPEC> {
        FUNCTION_0_W::new(self, 20)
    }
    #[doc = "Bits 24:31 - Select peripheral function for UART signal 6"]
    #[inline(always)]
    #[must_use]
    pub fn function_06(&mut self) -> FUNCTION_0_W<UART_SIGNAL_0_SPEC> {
        FUNCTION_0_W::new(self, 24)
    }
    #[doc = "Bits 28:35 - Select peripheral function for UART signal 7"]
    #[inline(always)]
    #[must_use]
    pub fn function_07(&mut self) -> FUNCTION_0_W<UART_SIGNAL_0_SPEC> {
        FUNCTION_0_W::new(self, 28)
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
#[doc = "Universal Asynchronous Receiver/Transmitter signal configuration 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_signal_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_signal_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART_SIGNAL_0_SPEC;
impl crate::RegisterSpec for UART_SIGNAL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_signal_0::R`](R) reader structure"]
impl crate::Readable for UART_SIGNAL_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart_signal_0::W`](W) writer structure"]
impl crate::Writable for UART_SIGNAL_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uart_signal_0 to value 0"]
impl crate::Resettable for UART_SIGNAL_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
