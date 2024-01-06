#[doc = "Register `sub_address` reader"]
pub type R = crate::R<SUB_ADDRESS_SPEC>;
#[doc = "Register `sub_address` writer"]
pub type W = crate::W<SUB_ADDRESS_SPEC>;
#[doc = "Field `byte(0-3)` reader - I2C sub-address byte %s"]
pub type BYTE_R = crate::FieldReader;
#[doc = "Field `byte(0-3)` writer - I2C sub-address byte %s"]
pub type BYTE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "I2C sub-address byte (0-3)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `byte0` field"]
    #[inline(always)]
    pub fn byte(&self, n: u8) -> BYTE_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        BYTE_R::new(((self.bits >> (n * 8)) & 0xff) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "I2C sub-address byte (0-3)"]
    #[inline(always)]
    pub fn byte_iter(&self) -> impl Iterator<Item = BYTE_R> + '_ {
        (0..4).map(move |n| BYTE_R::new(((self.bits >> (n * 8)) & 0xff) as u8))
    }
    #[doc = "Bits 0:7 - I2C sub-address byte 0"]
    #[inline(always)]
    pub fn byte0(&self) -> BYTE_R {
        BYTE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - I2C sub-address byte 1"]
    #[inline(always)]
    pub fn byte1(&self) -> BYTE_R {
        BYTE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - I2C sub-address byte 2"]
    #[inline(always)]
    pub fn byte2(&self) -> BYTE_R {
        BYTE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - I2C sub-address byte 3"]
    #[inline(always)]
    pub fn byte3(&self) -> BYTE_R {
        BYTE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "I2C sub-address byte (0-3)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `byte0` field"]
    #[inline(always)]
    #[must_use]
    pub fn byte(&mut self, n: u8) -> BYTE_W<SUB_ADDRESS_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        BYTE_W::new(self, n * 8)
    }
    #[doc = "Bits 0:7 - I2C sub-address byte 0"]
    #[inline(always)]
    #[must_use]
    pub fn byte0(&mut self) -> BYTE_W<SUB_ADDRESS_SPEC> {
        BYTE_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - I2C sub-address byte 1"]
    #[inline(always)]
    #[must_use]
    pub fn byte1(&mut self) -> BYTE_W<SUB_ADDRESS_SPEC> {
        BYTE_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - I2C sub-address byte 2"]
    #[inline(always)]
    #[must_use]
    pub fn byte2(&mut self) -> BYTE_W<SUB_ADDRESS_SPEC> {
        BYTE_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - I2C sub-address byte 3"]
    #[inline(always)]
    #[must_use]
    pub fn byte3(&mut self) -> BYTE_W<SUB_ADDRESS_SPEC> {
        BYTE_W::new(self, 24)
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
#[doc = "Register address of slave device\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sub_address::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sub_address::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SUB_ADDRESS_SPEC;
impl crate::RegisterSpec for SUB_ADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sub_address::R`](R) reader structure"]
impl crate::Readable for SUB_ADDRESS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sub_address::W`](W) writer structure"]
impl crate::Writable for SUB_ADDRESS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sub_address to value 0"]
impl crate::Resettable for SUB_ADDRESS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
