#[doc = "Register `pwm_config` reader"]
pub type R = crate::R<PWM_CONFIG_SPEC>;
#[doc = "Register `pwm_config` writer"]
pub type W = crate::W<PWM_CONFIG_SPEC>;
#[doc = "Field `reg_pdm_io_sel` reader - "]
pub type REG_PDM_IO_SEL_R = crate::BitReader;
#[doc = "Field `reg_pdm_io_sel` writer - "]
pub type REG_PDM_IO_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_pdm_io_sel(&self) -> REG_PDM_IO_SEL_R {
        REG_PDM_IO_SEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pdm_io_sel(&mut self) -> REG_PDM_IO_SEL_W<PWM_CONFIG_SPEC> {
        REG_PDM_IO_SEL_W::new(self, 0)
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
#[doc = "Pulse-Width configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWM_CONFIG_SPEC;
impl crate::RegisterSpec for PWM_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm_config::R`](R) reader structure"]
impl crate::Readable for PWM_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwm_config::W`](W) writer structure"]
impl crate::Writable for PWM_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pwm_config to value 0"]
impl crate::Resettable for PWM_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
