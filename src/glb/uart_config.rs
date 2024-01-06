#[doc = "Register `uart_config` reader"]
pub type R = crate::R<UART_CONFIG_SPEC>;
#[doc = "Register `uart_config` writer"]
pub type W = crate::W<UART_CONFIG_SPEC>;
#[doc = "Field `clock_divide` reader - Peripheral clock divide factor"]
pub type CLOCK_DIVIDE_R = crate::FieldReader;
#[doc = "Field `clock_divide` writer - Peripheral clock divide factor"]
pub type CLOCK_DIVIDE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `clock_enable` reader - Peripheral level clock gate enable"]
pub type CLOCK_ENABLE_R = crate::BitReader;
#[doc = "Field `clock_enable` writer - Peripheral level clock gate enable"]
pub type CLOCK_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hibernate_clock_source` reader - Reads clock source from hibernate registers"]
pub type HIBERNATE_CLOCK_SOURCE_R = crate::BitReader;
#[doc = "Field `hibernate_clock_source_2` reader - Reads clock source from hibernate registers"]
pub type HIBERNATE_CLOCK_SOURCE_2_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Peripheral clock divide factor"]
    #[inline(always)]
    pub fn clock_divide(&self) -> CLOCK_DIVIDE_R {
        CLOCK_DIVIDE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Peripheral level clock gate enable"]
    #[inline(always)]
    pub fn clock_enable(&self) -> CLOCK_ENABLE_R {
        CLOCK_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Reads clock source from hibernate registers"]
    #[inline(always)]
    pub fn hibernate_clock_source(&self) -> HIBERNATE_CLOCK_SOURCE_R {
        HIBERNATE_CLOCK_SOURCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 22 - Reads clock source from hibernate registers"]
    #[inline(always)]
    pub fn hibernate_clock_source_2(&self) -> HIBERNATE_CLOCK_SOURCE_2_R {
        HIBERNATE_CLOCK_SOURCE_2_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Peripheral clock divide factor"]
    #[inline(always)]
    #[must_use]
    pub fn clock_divide(&mut self) -> CLOCK_DIVIDE_W<UART_CONFIG_SPEC> {
        CLOCK_DIVIDE_W::new(self, 0)
    }
    #[doc = "Bit 4 - Peripheral level clock gate enable"]
    #[inline(always)]
    #[must_use]
    pub fn clock_enable(&mut self) -> CLOCK_ENABLE_W<UART_CONFIG_SPEC> {
        CLOCK_ENABLE_W::new(self, 4)
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
#[doc = "Universal Asynchronous Receiver/Transmitter configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART_CONFIG_SPEC;
impl crate::RegisterSpec for UART_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_config::R`](R) reader structure"]
impl crate::Readable for UART_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart_config::W`](W) writer structure"]
impl crate::Writable for UART_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uart_config to value 0"]
impl crate::Resettable for UART_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
