#[doc = "Register `debug_config_4` reader"]
pub type R = crate::R<DEBUG_CONFIG_4_SPEC>;
#[doc = "Register `debug_config_4` writer"]
pub type W = crate::W<DEBUG_CONFIG_4_SPEC>;
#[doc = "Field `debug_oe` reader - "]
pub type DEBUG_OE_R = crate::BitReader;
#[doc = "Field `debug_oe` writer - "]
pub type DEBUG_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `debug_i` reader - "]
pub type DEBUG_I_R = crate::FieldReader<u32>;
#[doc = "Field `debug_i` writer - "]
pub type DEBUG_I_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn debug_oe(&self) -> DEBUG_OE_R {
        DEBUG_OE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn debug_i(&self) -> DEBUG_I_R {
        DEBUG_I_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn debug_oe(&mut self) -> DEBUG_OE_W<DEBUG_CONFIG_4_SPEC> {
        DEBUG_OE_W::new(self, 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    #[must_use]
    pub fn debug_i(&mut self) -> DEBUG_I_W<DEBUG_CONFIG_4_SPEC> {
        DEBUG_I_W::new(self, 1)
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
#[doc = "Debug configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_config_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_config_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEBUG_CONFIG_4_SPEC;
impl crate::RegisterSpec for DEBUG_CONFIG_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_config_4::R`](R) reader structure"]
impl crate::Readable for DEBUG_CONFIG_4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`debug_config_4::W`](W) writer structure"]
impl crate::Writable for DEBUG_CONFIG_4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets debug_config_4 to value 0"]
impl crate::Resettable for DEBUG_CONFIG_4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
