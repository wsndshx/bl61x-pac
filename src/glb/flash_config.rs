#[doc = "Register `flash_config` reader"]
pub type R = crate::R<FLASH_CONFIG_SPEC>;
#[doc = "Register `flash_config` writer"]
pub type W = crate::W<FLASH_CONFIG_SPEC>;
#[doc = "Field `clock_divide` reader - Peripheral clock divide factor"]
pub type CLOCK_DIVIDE_R = crate::FieldReader;
#[doc = "Field `clock_divide` writer - Peripheral clock divide factor"]
pub type CLOCK_DIVIDE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `clock_enable` reader - Peripheral level clock gate enable"]
pub type CLOCK_ENABLE_R = crate::BitReader;
#[doc = "Field `clock_enable` writer - Peripheral level clock gate enable"]
pub type CLOCK_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clock_source_0` reader - Peripheral clock source register 0"]
pub type CLOCK_SOURCE_0_R = crate::FieldReader;
#[doc = "Field `clock_source_0` writer - Peripheral clock source register 0"]
pub type CLOCK_SOURCE_0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `clock_source_1` reader - Peripheral clock source register 1"]
pub type CLOCK_SOURCE_1_R = crate::FieldReader;
#[doc = "Field `clock_source_1` writer - Peripheral clock source register 1"]
pub type CLOCK_SOURCE_1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 8:10 - Peripheral clock divide factor"]
    #[inline(always)]
    pub fn clock_divide(&self) -> CLOCK_DIVIDE_R {
        CLOCK_DIVIDE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Peripheral level clock gate enable"]
    #[inline(always)]
    pub fn clock_enable(&self) -> CLOCK_ENABLE_R {
        CLOCK_ENABLE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Peripheral clock source register 0"]
    #[inline(always)]
    pub fn clock_source_0(&self) -> CLOCK_SOURCE_0_R {
        CLOCK_SOURCE_0_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Peripheral clock source register 1"]
    #[inline(always)]
    pub fn clock_source_1(&self) -> CLOCK_SOURCE_1_R {
        CLOCK_SOURCE_1_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - Peripheral clock divide factor"]
    #[inline(always)]
    #[must_use]
    pub fn clock_divide(&mut self) -> CLOCK_DIVIDE_W<FLASH_CONFIG_SPEC> {
        CLOCK_DIVIDE_W::new(self, 8)
    }
    #[doc = "Bit 11 - Peripheral level clock gate enable"]
    #[inline(always)]
    #[must_use]
    pub fn clock_enable(&mut self) -> CLOCK_ENABLE_W<FLASH_CONFIG_SPEC> {
        CLOCK_ENABLE_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - Peripheral clock source register 0"]
    #[inline(always)]
    #[must_use]
    pub fn clock_source_0(&mut self) -> CLOCK_SOURCE_0_W<FLASH_CONFIG_SPEC> {
        CLOCK_SOURCE_0_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Peripheral clock source register 1"]
    #[inline(always)]
    #[must_use]
    pub fn clock_source_1(&mut self) -> CLOCK_SOURCE_1_W<FLASH_CONFIG_SPEC> {
        CLOCK_SOURCE_1_W::new(self, 14)
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
#[doc = "Serial flash configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_CONFIG_SPEC;
impl crate::RegisterSpec for FLASH_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_config::R`](R) reader structure"]
impl crate::Readable for FLASH_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flash_config::W`](W) writer structure"]
impl crate::Writable for FLASH_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets flash_config to value 0"]
impl crate::Resettable for FLASH_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
