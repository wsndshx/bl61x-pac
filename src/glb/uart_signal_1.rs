#[doc = "Register `uart_signal_1` reader"]
pub type R = crate::R<UART_SIGNAL_1_SPEC>;
#[doc = "Register `uart_signal_1` writer"]
pub type W = crate::W<UART_SIGNAL_1_SPEC>;
#[doc = "Field `function_1(0-3)` reader - Select peripheral function for UART signal %s (offset by 8)"]
pub use super::uart_signal_0::FUNCTION_0_R as FUNCTION_1_R;
#[doc = "Field `function_1(0-3)` writer - Select peripheral function for UART signal %s (offset by 8)"]
pub use super::uart_signal_0::FUNCTION_0_W as FUNCTION_1_W;
#[doc = "Select peripheral function for UART signal %s (offset by 8)"]
pub use super::uart_signal_0::FUNCTION_A;
impl R {
    #[doc = "Select peripheral function for UART signal (0-3) (offset by 8)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `function_10` field"]
    #[inline(always)]
    pub fn function_1(&self, n: u8) -> FUNCTION_1_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        FUNCTION_1_R::new(((self.bits >> (n * 4)) & 0xff) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Select peripheral function for UART signal (0-3) (offset by 8)"]
    #[inline(always)]
    pub fn function_1_iter(&self) -> impl Iterator<Item = FUNCTION_1_R> + '_ {
        (0..4).map(move |n| FUNCTION_1_R::new(((self.bits >> (n * 4)) & 0xff) as u8))
    }
    #[doc = "Bits 0:7 - Select peripheral function for UART signal 0 (offset by 8)"]
    #[inline(always)]
    pub fn function_10(&self) -> FUNCTION_1_R {
        FUNCTION_1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 4:11 - Select peripheral function for UART signal 1 (offset by 8)"]
    #[inline(always)]
    pub fn function_11(&self) -> FUNCTION_1_R {
        FUNCTION_1_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Select peripheral function for UART signal 2 (offset by 8)"]
    #[inline(always)]
    pub fn function_12(&self) -> FUNCTION_1_R {
        FUNCTION_1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 12:19 - Select peripheral function for UART signal 3 (offset by 8)"]
    #[inline(always)]
    pub fn function_13(&self) -> FUNCTION_1_R {
        FUNCTION_1_R::new(((self.bits >> 12) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Select peripheral function for UART signal (0-3) (offset by 8)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `function_10` field"]
    #[inline(always)]
    #[must_use]
    pub fn function_1(&mut self, n: u8) -> FUNCTION_1_W<UART_SIGNAL_1_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        FUNCTION_1_W::new(self, n * 4)
    }
    #[doc = "Bits 0:7 - Select peripheral function for UART signal 0 (offset by 8)"]
    #[inline(always)]
    #[must_use]
    pub fn function_10(&mut self) -> FUNCTION_1_W<UART_SIGNAL_1_SPEC> {
        FUNCTION_1_W::new(self, 0)
    }
    #[doc = "Bits 4:11 - Select peripheral function for UART signal 1 (offset by 8)"]
    #[inline(always)]
    #[must_use]
    pub fn function_11(&mut self) -> FUNCTION_1_W<UART_SIGNAL_1_SPEC> {
        FUNCTION_1_W::new(self, 4)
    }
    #[doc = "Bits 8:15 - Select peripheral function for UART signal 2 (offset by 8)"]
    #[inline(always)]
    #[must_use]
    pub fn function_12(&mut self) -> FUNCTION_1_W<UART_SIGNAL_1_SPEC> {
        FUNCTION_1_W::new(self, 8)
    }
    #[doc = "Bits 12:19 - Select peripheral function for UART signal 3 (offset by 8)"]
    #[inline(always)]
    #[must_use]
    pub fn function_13(&mut self) -> FUNCTION_1_W<UART_SIGNAL_1_SPEC> {
        FUNCTION_1_W::new(self, 12)
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
#[doc = "Universal Asynchronous Receiver/Transmitter signal configuration 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_signal_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_signal_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART_SIGNAL_1_SPEC;
impl crate::RegisterSpec for UART_SIGNAL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_signal_1::R`](R) reader structure"]
impl crate::Readable for UART_SIGNAL_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart_signal_1::W`](W) writer structure"]
impl crate::Writable for UART_SIGNAL_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uart_signal_1 to value 0"]
impl crate::Resettable for UART_SIGNAL_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
