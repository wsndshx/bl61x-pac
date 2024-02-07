#[doc = "Register `cgen_m` reader"]
pub type R = crate::R<CGEN_M_SPEC>;
#[doc = "Register `cgen_m` writer"]
pub type W = crate::W<CGEN_M_SPEC>;
#[doc = "Field `cgen_m_cpu` reader - "]
pub type CGEN_M_CPU_R = crate::BitReader;
#[doc = "Field `cgen_m_cpu` writer - "]
pub type CGEN_M_CPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_m_sdu` reader - "]
pub type CGEN_M_SDU_R = crate::BitReader;
#[doc = "Field `cgen_m_sdu` writer - "]
pub type CGEN_M_SDU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_m_sec` reader - "]
pub type CGEN_M_SEC_R = crate::BitReader;
#[doc = "Field `cgen_m_sec` writer - "]
pub type CGEN_M_SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_m_dma` reader - "]
pub type CGEN_M_DMA_R = crate::BitReader;
#[doc = "Field `cgen_m_dma` writer - "]
pub type CGEN_M_DMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cgen_m_cci` reader - "]
pub type CGEN_M_CCI_R = crate::BitReader;
#[doc = "Field `cgen_m_cci` writer - "]
pub type CGEN_M_CCI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cgen_m_cpu(&self) -> CGEN_M_CPU_R {
        CGEN_M_CPU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cgen_m_sdu(&self) -> CGEN_M_SDU_R {
        CGEN_M_SDU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cgen_m_sec(&self) -> CGEN_M_SEC_R {
        CGEN_M_SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cgen_m_dma(&self) -> CGEN_M_DMA_R {
        CGEN_M_DMA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cgen_m_cci(&self) -> CGEN_M_CCI_R {
        CGEN_M_CCI_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_m_cpu(&mut self) -> CGEN_M_CPU_W<CGEN_M_SPEC> {
        CGEN_M_CPU_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_m_sdu(&mut self) -> CGEN_M_SDU_W<CGEN_M_SPEC> {
        CGEN_M_SDU_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_m_sec(&mut self) -> CGEN_M_SEC_W<CGEN_M_SPEC> {
        CGEN_M_SEC_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_m_dma(&mut self) -> CGEN_M_DMA_W<CGEN_M_SPEC> {
        CGEN_M_DMA_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cgen_m_cci(&mut self) -> CGEN_M_CCI_W<CGEN_M_SPEC> {
        CGEN_M_CCI_W::new(self, 4)
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
#[doc = "cgen_m.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cgen_m::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgen_m::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CGEN_M_SPEC;
impl crate::RegisterSpec for CGEN_M_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgen_m::R`](R) reader structure"]
impl crate::Readable for CGEN_M_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cgen_m::W`](W) writer structure"]
impl crate::Writable for CGEN_M_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cgen_m to value 0"]
impl crate::Resettable for CGEN_M_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
