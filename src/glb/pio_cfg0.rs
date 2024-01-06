#[doc = "Register `pio_cfg0` reader"]
pub type R = crate::R<PIO_CFG0_SPEC>;
#[doc = "Register `pio_cfg0` writer"]
pub type W = crate::W<PIO_CFG0_SPEC>;
#[doc = "Field `pio_clk_div` reader - "]
pub type PIO_CLK_DIV_R = crate::FieldReader;
#[doc = "Field `pio_clk_div` writer - "]
pub type PIO_CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `pio_clk_en` reader - "]
pub type PIO_CLK_EN_R = crate::BitReader;
#[doc = "Field `pio_clk_en` writer - "]
pub type PIO_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pio_clk_sel` reader - "]
pub type PIO_CLK_SEL_R = crate::BitReader;
#[doc = "Field `pio_clk_sel` writer - "]
pub type PIO_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn pio_clk_div(&self) -> PIO_CLK_DIV_R {
        PIO_CLK_DIV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pio_clk_en(&self) -> PIO_CLK_EN_R {
        PIO_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pio_clk_sel(&self) -> PIO_CLK_SEL_R {
        PIO_CLK_SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn pio_clk_div(&mut self) -> PIO_CLK_DIV_W<PIO_CFG0_SPEC> {
        PIO_CLK_DIV_W::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pio_clk_en(&mut self) -> PIO_CLK_EN_W<PIO_CFG0_SPEC> {
        PIO_CLK_EN_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pio_clk_sel(&mut self) -> PIO_CLK_SEL_W<PIO_CFG0_SPEC> {
        PIO_CLK_SEL_W::new(self, 9)
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
#[doc = "pio_cfg0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIO_CFG0_SPEC;
impl crate::RegisterSpec for PIO_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pio_cfg0::R`](R) reader structure"]
impl crate::Readable for PIO_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pio_cfg0::W`](W) writer structure"]
impl crate::Writable for PIO_CFG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pio_cfg0 to value 0"]
impl crate::Resettable for PIO_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
