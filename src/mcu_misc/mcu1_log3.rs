#[doc = "Register `mcu1_log3` reader"]
pub type R = crate::R<MCU1_LOG3_SPEC>;
#[doc = "Register `mcu1_log3` writer"]
pub type W = crate::W<MCU1_LOG3_SPEC>;
#[doc = "Field `sts_mcu1_mstatus` reader - MCU1 machine status. This field contains the status of the last MCU1 machine instruction."]
pub type STS_MCU1_MSTATUS_R = crate::FieldReader<u32>;
#[doc = "Field `sts_mcu1_mstatus` writer - MCU1 machine status. This field contains the status of the last MCU1 machine instruction."]
pub type STS_MCU1_MSTATUS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MCU1 machine status. This field contains the status of the last MCU1 machine instruction."]
    #[inline(always)]
    pub fn sts_mcu1_mstatus(&self) -> STS_MCU1_MSTATUS_R {
        STS_MCU1_MSTATUS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MCU1 machine status. This field contains the status of the last MCU1 machine instruction."]
    #[inline(always)]
    #[must_use]
    pub fn sts_mcu1_mstatus(&mut self) -> STS_MCU1_MSTATUS_W<MCU1_LOG3_SPEC> {
        STS_MCU1_MSTATUS_W::new(self, 0)
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
#[doc = "MCU1 Log 3 Register. Stores information about the last MCU1 machine status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcu1_log3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcu1_log3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCU1_LOG3_SPEC;
impl crate::RegisterSpec for MCU1_LOG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcu1_log3::R`](R) reader structure"]
impl crate::Readable for MCU1_LOG3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcu1_log3::W`](W) writer structure"]
impl crate::Writable for MCU1_LOG3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mcu1_log3 to value 0"]
impl crate::Resettable for MCU1_LOG3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
