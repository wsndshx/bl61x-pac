#[doc = "Register `pwm_cfg0` reader"]
pub type R = crate::R<PWM_CFG0_SPEC>;
#[doc = "Register `pwm_cfg0` writer"]
pub type W = crate::W<PWM_CFG0_SPEC>;
#[doc = "Field `reg_pwm1_io_sel` reader - "]
pub type REG_PWM1_IO_SEL_R = crate::BitReader;
#[doc = "Field `reg_pwm1_io_sel` writer - "]
pub type REG_PWM1_IO_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_pwm1_io_sel(&self) -> REG_PWM1_IO_SEL_R {
        REG_PWM1_IO_SEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pwm1_io_sel(&mut self) -> REG_PWM1_IO_SEL_W<PWM_CFG0_SPEC> {
        REG_PWM1_IO_SEL_W::new(self, 0)
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
#[doc = "pwm_cfg0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWM_CFG0_SPEC;
impl crate::RegisterSpec for PWM_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm_cfg0::R`](R) reader structure"]
impl crate::Readable for PWM_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwm_cfg0::W`](W) writer structure"]
impl crate::Writable for PWM_CFG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pwm_cfg0 to value 0"]
impl crate::Resettable for PWM_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
