#[doc = "Register `cpu_core_cfg14` reader"]
pub type R = crate::R<CPU_CORE_CFG14_SPEC>;
#[doc = "Register `cpu_core_cfg14` writer"]
pub type W = crate::W<CPU_CORE_CFG14_SPEC>;
#[doc = "Field `e906_rst_addr` reader - "]
pub type E906_RST_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `e906_rst_addr` writer - "]
pub type E906_RST_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn e906_rst_addr(&self) -> E906_RST_ADDR_R {
        E906_RST_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn e906_rst_addr(&mut self) -> E906_RST_ADDR_W<CPU_CORE_CFG14_SPEC> {
        E906_RST_ADDR_W::new(self, 0)
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
#[doc = "cpu_core_cfg14.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_core_cfg14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_core_cfg14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_CORE_CFG14_SPEC;
impl crate::RegisterSpec for CPU_CORE_CFG14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_core_cfg14::R`](R) reader structure"]
impl crate::Readable for CPU_CORE_CFG14_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_core_cfg14::W`](W) writer structure"]
impl crate::Writable for CPU_CORE_CFG14_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cpu_core_cfg14 to value 0"]
impl crate::Resettable for CPU_CORE_CFG14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
