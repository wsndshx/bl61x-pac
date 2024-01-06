#[doc = "Register `i2c_config` reader"]
pub type R = crate::R<I2C_CONFIG_SPEC>;
#[doc = "Register `i2c_config` writer"]
pub type W = crate::W<I2C_CONFIG_SPEC>;
#[doc = "Field `clock_divide` reader - Peripheral clock divide factor"]
pub type CLOCK_DIVIDE_R = crate::FieldReader;
#[doc = "Field `clock_divide` writer - Peripheral clock divide factor"]
pub type CLOCK_DIVIDE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `clock_enable` reader - Peripheral level clock gate enable"]
pub type CLOCK_ENABLE_R = crate::BitReader;
#[doc = "Field `clock_enable` writer - Peripheral level clock gate enable"]
pub type CLOCK_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clock_source` reader - Peripheral clock source register"]
pub type CLOCK_SOURCE_R = crate::BitReader;
#[doc = "Field `clock_source` writer - Peripheral clock source register"]
pub type CLOCK_SOURCE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:23 - Peripheral clock divide factor"]
    #[inline(always)]
    pub fn clock_divide(&self) -> CLOCK_DIVIDE_R {
        CLOCK_DIVIDE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Peripheral level clock gate enable"]
    #[inline(always)]
    pub fn clock_enable(&self) -> CLOCK_ENABLE_R {
        CLOCK_ENABLE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Peripheral clock source register"]
    #[inline(always)]
    pub fn clock_source(&self) -> CLOCK_SOURCE_R {
        CLOCK_SOURCE_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:23 - Peripheral clock divide factor"]
    #[inline(always)]
    #[must_use]
    pub fn clock_divide(&mut self) -> CLOCK_DIVIDE_W<I2C_CONFIG_SPEC> {
        CLOCK_DIVIDE_W::new(self, 16)
    }
    #[doc = "Bit 24 - Peripheral level clock gate enable"]
    #[inline(always)]
    #[must_use]
    pub fn clock_enable(&mut self) -> CLOCK_ENABLE_W<I2C_CONFIG_SPEC> {
        CLOCK_ENABLE_W::new(self, 24)
    }
    #[doc = "Bit 25 - Peripheral clock source register"]
    #[inline(always)]
    #[must_use]
    pub fn clock_source(&mut self) -> CLOCK_SOURCE_W<I2C_CONFIG_SPEC> {
        CLOCK_SOURCE_W::new(self, 25)
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
#[doc = "Inter-Integrated Circuit bus configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_CONFIG_SPEC;
impl crate::RegisterSpec for I2C_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_config::R`](R) reader structure"]
impl crate::Readable for I2C_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_config::W`](W) writer structure"]
impl crate::Writable for I2C_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2c_config to value 0"]
impl crate::Resettable for I2C_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
