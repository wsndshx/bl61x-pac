#[doc = "Register `cpu_mbist` reader"]
pub type R = crate::R<CPU_MBIST_SPEC>;
#[doc = "Register `cpu_mbist` writer"]
pub type W = crate::W<CPU_MBIST_SPEC>;
#[doc = "Field `cpu_mbist_mode` reader - CPU MBIST mode. This field controls the MBIST mode for the CPU."]
pub type CPU_MBIST_MODE_R = crate::BitReader;
#[doc = "Field `cpu_mbist_mode` writer - CPU MBIST mode. This field controls the MBIST mode for the CPU."]
pub type CPU_MBIST_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `irom_mbist_mode` reader - IROM MBIST mode. This field controls the MBIST mode for the IROM."]
pub type IROM_MBIST_MODE_R = crate::BitReader;
#[doc = "Field `irom_mbist_mode` writer - IROM MBIST mode. This field controls the MBIST mode for the IROM."]
pub type IROM_MBIST_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_cpu_mbist_rst_n` reader - CPU MBIST reset. Writing 1 to this bit resets the CPU MBIST."]
pub type REG_CPU_MBIST_RST_N_R = crate::BitReader;
#[doc = "Field `reg_cpu_mbist_rst_n` writer - CPU MBIST reset. Writing 1 to this bit resets the CPU MBIST."]
pub type REG_CPU_MBIST_RST_N_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpu_mbist_done` reader - CPU MBIST done. This field indicates the status of the CPU MBIST."]
pub type CPU_MBIST_DONE_R = crate::FieldReader;
#[doc = "Field `cpu_mbist_done` writer - CPU MBIST done. This field indicates the status of the CPU MBIST."]
pub type CPU_MBIST_DONE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `irom_mbist_done` reader - IROM MBIST done. This field indicates the status of the IROM MBIST."]
pub type IROM_MBIST_DONE_R = crate::BitReader;
#[doc = "Field `irom_mbist_done` writer - IROM MBIST done. This field indicates the status of the IROM MBIST."]
pub type IROM_MBIST_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpu_mbist_fail` reader - CPU MBIST fail. This field indicates the status of the CPU MBIST."]
pub type CPU_MBIST_FAIL_R = crate::FieldReader;
#[doc = "Field `cpu_mbist_fail` writer - CPU MBIST fail. This field indicates the status of the CPU MBIST."]
pub type CPU_MBIST_FAIL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `irom_mbist_fail` reader - IROM MBIST fail. This field indicates the status of the IROM MBIST."]
pub type IROM_MBIST_FAIL_R = crate::BitReader;
#[doc = "Field `irom_mbist_fail` writer - IROM MBIST fail. This field indicates the status of the IROM MBIST."]
pub type IROM_MBIST_FAIL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CPU MBIST mode. This field controls the MBIST mode for the CPU."]
    #[inline(always)]
    pub fn cpu_mbist_mode(&self) -> CPU_MBIST_MODE_R {
        CPU_MBIST_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IROM MBIST mode. This field controls the MBIST mode for the IROM."]
    #[inline(always)]
    pub fn irom_mbist_mode(&self) -> IROM_MBIST_MODE_R {
        IROM_MBIST_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - CPU MBIST reset. Writing 1 to this bit resets the CPU MBIST."]
    #[inline(always)]
    pub fn reg_cpu_mbist_rst_n(&self) -> REG_CPU_MBIST_RST_N_R {
        REG_CPU_MBIST_RST_N_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:21 - CPU MBIST done. This field indicates the status of the CPU MBIST."]
    #[inline(always)]
    pub fn cpu_mbist_done(&self) -> CPU_MBIST_DONE_R {
        CPU_MBIST_DONE_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - IROM MBIST done. This field indicates the status of the IROM MBIST."]
    #[inline(always)]
    pub fn irom_mbist_done(&self) -> IROM_MBIST_DONE_R {
        IROM_MBIST_DONE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:29 - CPU MBIST fail. This field indicates the status of the CPU MBIST."]
    #[inline(always)]
    pub fn cpu_mbist_fail(&self) -> CPU_MBIST_FAIL_R {
        CPU_MBIST_FAIL_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - IROM MBIST fail. This field indicates the status of the IROM MBIST."]
    #[inline(always)]
    pub fn irom_mbist_fail(&self) -> IROM_MBIST_FAIL_R {
        IROM_MBIST_FAIL_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU MBIST mode. This field controls the MBIST mode for the CPU."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_mbist_mode(&mut self) -> CPU_MBIST_MODE_W<CPU_MBIST_SPEC> {
        CPU_MBIST_MODE_W::new(self, 0)
    }
    #[doc = "Bit 1 - IROM MBIST mode. This field controls the MBIST mode for the IROM."]
    #[inline(always)]
    #[must_use]
    pub fn irom_mbist_mode(&mut self) -> IROM_MBIST_MODE_W<CPU_MBIST_SPEC> {
        IROM_MBIST_MODE_W::new(self, 1)
    }
    #[doc = "Bit 8 - CPU MBIST reset. Writing 1 to this bit resets the CPU MBIST."]
    #[inline(always)]
    #[must_use]
    pub fn reg_cpu_mbist_rst_n(&mut self) -> REG_CPU_MBIST_RST_N_W<CPU_MBIST_SPEC> {
        REG_CPU_MBIST_RST_N_W::new(self, 8)
    }
    #[doc = "Bits 16:21 - CPU MBIST done. This field indicates the status of the CPU MBIST."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_mbist_done(&mut self) -> CPU_MBIST_DONE_W<CPU_MBIST_SPEC> {
        CPU_MBIST_DONE_W::new(self, 16)
    }
    #[doc = "Bit 22 - IROM MBIST done. This field indicates the status of the IROM MBIST."]
    #[inline(always)]
    #[must_use]
    pub fn irom_mbist_done(&mut self) -> IROM_MBIST_DONE_W<CPU_MBIST_SPEC> {
        IROM_MBIST_DONE_W::new(self, 22)
    }
    #[doc = "Bits 24:29 - CPU MBIST fail. This field indicates the status of the CPU MBIST."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_mbist_fail(&mut self) -> CPU_MBIST_FAIL_W<CPU_MBIST_SPEC> {
        CPU_MBIST_FAIL_W::new(self, 24)
    }
    #[doc = "Bit 30 - IROM MBIST fail. This field indicates the status of the IROM MBIST."]
    #[inline(always)]
    #[must_use]
    pub fn irom_mbist_fail(&mut self) -> IROM_MBIST_FAIL_W<CPU_MBIST_SPEC> {
        IROM_MBIST_FAIL_W::new(self, 30)
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
#[doc = "CPU MBIST Register. Controls the CPU memory built-in self-test (MBIST).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_mbist::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_mbist::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_MBIST_SPEC;
impl crate::RegisterSpec for CPU_MBIST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_mbist::R`](R) reader structure"]
impl crate::Readable for CPU_MBIST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_mbist::W`](W) writer structure"]
impl crate::Writable for CPU_MBIST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cpu_mbist to value 0"]
impl crate::Resettable for CPU_MBIST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
