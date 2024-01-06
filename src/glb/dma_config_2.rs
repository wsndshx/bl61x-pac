#[doc = "Register `dma_config_2` reader"]
pub type R = crate::R<DMA_CONFIG_2_SPEC>;
#[doc = "Register `dma_config_2` writer"]
pub type W = crate::W<DMA_CONFIG_2_SPEC>;
#[doc = "Field `reg_dma_cn_sel` reader - "]
pub type REG_DMA_CN_SEL_R = crate::FieldReader<u32>;
#[doc = "Field `reg_dma_cn_sel` writer - "]
pub type REG_DMA_CN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_dma_cn_sel(&self) -> REG_DMA_CN_SEL_R {
        REG_DMA_CN_SEL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_dma_cn_sel(&mut self) -> REG_DMA_CN_SEL_W<DMA_CONFIG_2_SPEC> {
        REG_DMA_CN_SEL_W::new(self, 0)
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
#[doc = "Direct Memory Access configuration 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_config_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_config_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_CONFIG_2_SPEC;
impl crate::RegisterSpec for DMA_CONFIG_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_config_2::R`](R) reader structure"]
impl crate::Readable for DMA_CONFIG_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_config_2::W`](W) writer structure"]
impl crate::Writable for DMA_CONFIG_2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dma_config_2 to value 0"]
impl crate::Resettable for DMA_CONFIG_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
