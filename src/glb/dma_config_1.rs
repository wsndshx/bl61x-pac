#[doc = "Register `dma_config_1` reader"]
pub type R = crate::R<DMA_CONFIG_1_SPEC>;
#[doc = "Register `dma_config_1` writer"]
pub type W = crate::W<DMA_CONFIG_1_SPEC>;
#[doc = "Field `dma2_clk_en` reader - "]
pub type DMA2_CLK_EN_R = crate::FieldReader;
#[doc = "Field `dma2_clk_en` writer - "]
pub type DMA2_CLK_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn dma2_clk_en(&self) -> DMA2_CLK_EN_R {
        DMA2_CLK_EN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn dma2_clk_en(&mut self) -> DMA2_CLK_EN_W<DMA_CONFIG_1_SPEC> {
        DMA2_CLK_EN_W::new(self, 24)
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
#[doc = "Direct Memory Access configuration 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_config_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_config_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_CONFIG_1_SPEC;
impl crate::RegisterSpec for DMA_CONFIG_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_config_1::R`](R) reader structure"]
impl crate::Readable for DMA_CONFIG_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_config_1::W`](W) writer structure"]
impl crate::Writable for DMA_CONFIG_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dma_config_1 to value 0"]
impl crate::Resettable for DMA_CONFIG_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
