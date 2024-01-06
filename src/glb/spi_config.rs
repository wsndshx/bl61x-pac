#[doc = "Register `spi_config` reader"]
pub type R = crate::R<SPI_CONFIG_SPEC>;
#[doc = "Register `spi_config` writer"]
pub type W = crate::W<SPI_CONFIG_SPEC>;
#[doc = "Field `clock_divide` reader - Peripheral clock divide factor"]
pub type CLOCK_DIVIDE_R = crate::FieldReader;
#[doc = "Field `clock_divide` writer - Peripheral clock divide factor"]
pub type CLOCK_DIVIDE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `clock_enable` reader - Peripheral level clock gate enable"]
pub type CLOCK_ENABLE_R = crate::BitReader;
#[doc = "Field `clock_enable` writer - Peripheral level clock gate enable"]
pub type CLOCK_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clock_source` reader - Peripheral clock source register"]
pub type CLOCK_SOURCE_R = crate::BitReader;
#[doc = "Field `clock_source` writer - Peripheral clock source register"]
pub type CLOCK_SOURCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pin_swap` reader - Swap Serial Peripheral Interface pin signals"]
pub type PIN_SWAP_R = crate::FieldReader;
#[doc = "Field `pin_swap` writer - Swap Serial Peripheral Interface pin signals"]
pub type PIN_SWAP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - Peripheral clock divide factor"]
    #[inline(always)]
    pub fn clock_divide(&self) -> CLOCK_DIVIDE_R {
        CLOCK_DIVIDE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Peripheral level clock gate enable"]
    #[inline(always)]
    pub fn clock_enable(&self) -> CLOCK_ENABLE_R {
        CLOCK_ENABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Peripheral clock source register"]
    #[inline(always)]
    pub fn clock_source(&self) -> CLOCK_SOURCE_R {
        CLOCK_SOURCE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Swap Serial Peripheral Interface pin signals"]
    #[inline(always)]
    pub fn pin_swap(&self) -> PIN_SWAP_R {
        PIN_SWAP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Peripheral clock divide factor"]
    #[inline(always)]
    #[must_use]
    pub fn clock_divide(&mut self) -> CLOCK_DIVIDE_W<SPI_CONFIG_SPEC> {
        CLOCK_DIVIDE_W::new(self, 0)
    }
    #[doc = "Bit 8 - Peripheral level clock gate enable"]
    #[inline(always)]
    #[must_use]
    pub fn clock_enable(&mut self) -> CLOCK_ENABLE_W<SPI_CONFIG_SPEC> {
        CLOCK_ENABLE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Peripheral clock source register"]
    #[inline(always)]
    #[must_use]
    pub fn clock_source(&mut self) -> CLOCK_SOURCE_W<SPI_CONFIG_SPEC> {
        CLOCK_SOURCE_W::new(self, 9)
    }
    #[doc = "Bits 16:19 - Swap Serial Peripheral Interface pin signals"]
    #[inline(always)]
    #[must_use]
    pub fn pin_swap(&mut self) -> PIN_SWAP_W<SPI_CONFIG_SPEC> {
        PIN_SWAP_W::new(self, 16)
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
#[doc = "Serial Peripheral Interface configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_CONFIG_SPEC;
impl crate::RegisterSpec for SPI_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_config::R`](R) reader structure"]
impl crate::Readable for SPI_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_config::W`](W) writer structure"]
impl crate::Writable for SPI_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spi_config to value 0"]
impl crate::Resettable for SPI_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
