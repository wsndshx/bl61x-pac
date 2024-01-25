#[doc = "Register `mcu1_log1` reader"]
pub type R = crate::R<MCU1_LOG1_SPEC>;
#[doc = "Register `mcu1_log1` writer"]
pub type W = crate::W<MCU1_LOG1_SPEC>;
#[doc = "Field `sts_mcu1_mcause` reader - MCU1 machine cause. This field contains the cause of the last MCU1 exception."]
pub type STS_MCU1_MCAUSE_R = crate::FieldReader<u32>;
#[doc = "Field `sts_mcu1_mcause` writer - MCU1 machine cause. This field contains the cause of the last MCU1 exception."]
pub type STS_MCU1_MCAUSE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MCU1 machine cause. This field contains the cause of the last MCU1 exception."]
    #[inline(always)]
    pub fn sts_mcu1_mcause(&self) -> STS_MCU1_MCAUSE_R {
        STS_MCU1_MCAUSE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MCU1 machine cause. This field contains the cause of the last MCU1 exception."]
    #[inline(always)]
    #[must_use]
    pub fn sts_mcu1_mcause(&mut self) -> STS_MCU1_MCAUSE_W<MCU1_LOG1_SPEC> {
        STS_MCU1_MCAUSE_W::new(self, 0)
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
#[doc = "MCU1 Log 1 Register. Stores information about the last MCU1 exception.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcu1_log1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcu1_log1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCU1_LOG1_SPEC;
impl crate::RegisterSpec for MCU1_LOG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcu1_log1::R`](R) reader structure"]
impl crate::Readable for MCU1_LOG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcu1_log1::W`](W) writer structure"]
impl crate::Writable for MCU1_LOG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mcu1_log1 to value 0"]
impl crate::Resettable for MCU1_LOG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
