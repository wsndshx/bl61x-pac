#[doc = "Register `mcu1_log4` reader"]
pub type R = crate::R<MCU1_LOG4_SPEC>;
#[doc = "Register `mcu1_log4` writer"]
pub type W = crate::W<MCU1_LOG4_SPEC>;
#[doc = "Field `sts_mcu1_sp` reader - MCU1 stack pointer. This field contains the value of the MCU1 stack pointer at the time of the last exception."]
pub type STS_MCU1_SP_R = crate::BitReader;
#[doc = "Field `sts_mcu1_sp` writer - MCU1 stack pointer. This field contains the value of the MCU1 stack pointer at the time of the last exception."]
pub type STS_MCU1_SP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sts_mcu1_pc` reader - MCU1 program counter. This field contains the value of the MCU1 program counter at the time of the last exception."]
pub type STS_MCU1_PC_R = crate::FieldReader<u32>;
#[doc = "Field `sts_mcu1_pc` writer - MCU1 program counter. This field contains the value of the MCU1 program counter at the time of the last exception."]
pub type STS_MCU1_PC_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - MCU1 stack pointer. This field contains the value of the MCU1 stack pointer at the time of the last exception."]
    #[inline(always)]
    pub fn sts_mcu1_sp(&self) -> STS_MCU1_SP_R {
        STS_MCU1_SP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - MCU1 program counter. This field contains the value of the MCU1 program counter at the time of the last exception."]
    #[inline(always)]
    pub fn sts_mcu1_pc(&self) -> STS_MCU1_PC_R {
        STS_MCU1_PC_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - MCU1 stack pointer. This field contains the value of the MCU1 stack pointer at the time of the last exception."]
    #[inline(always)]
    #[must_use]
    pub fn sts_mcu1_sp(&mut self) -> STS_MCU1_SP_W<MCU1_LOG4_SPEC> {
        STS_MCU1_SP_W::new(self, 0)
    }
    #[doc = "Bits 1:31 - MCU1 program counter. This field contains the value of the MCU1 program counter at the time of the last exception."]
    #[inline(always)]
    #[must_use]
    pub fn sts_mcu1_pc(&mut self) -> STS_MCU1_PC_W<MCU1_LOG4_SPEC> {
        STS_MCU1_PC_W::new(self, 1)
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
#[doc = "MCU1 Log 4 Register. Stores information about the last MCU1 program counter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcu1_log4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcu1_log4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCU1_LOG4_SPEC;
impl crate::RegisterSpec for MCU1_LOG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcu1_log4::R`](R) reader structure"]
impl crate::Readable for MCU1_LOG4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcu1_log4::W`](W) writer structure"]
impl crate::Writable for MCU1_LOG4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mcu1_log4 to value 0"]
impl crate::Resettable for MCU1_LOG4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
