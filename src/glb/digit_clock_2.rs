#[doc = "Register `digit_clock_2` reader"]
pub type R = crate::R<DIGIT_CLOCK_2_SPEC>;
#[doc = "Register `digit_clock_2` writer"]
pub type W = crate::W<DIGIT_CLOCK_2_SPEC>;
#[doc = "Field `chip_clk_out_0_sel` reader - "]
pub type CHIP_CLK_OUT_0_SEL_R = crate::FieldReader;
#[doc = "Field `chip_clk_out_0_sel` writer - "]
pub type CHIP_CLK_OUT_0_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `chip_clk_out_1_sel` reader - "]
pub type CHIP_CLK_OUT_1_SEL_R = crate::FieldReader;
#[doc = "Field `chip_clk_out_1_sel` writer - "]
pub type CHIP_CLK_OUT_1_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `chip_clk_out_2_sel` reader - "]
pub type CHIP_CLK_OUT_2_SEL_R = crate::FieldReader;
#[doc = "Field `chip_clk_out_2_sel` writer - "]
pub type CHIP_CLK_OUT_2_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `chip_clk_out_3_sel` reader - "]
pub type CHIP_CLK_OUT_3_SEL_R = crate::FieldReader;
#[doc = "Field `chip_clk_out_3_sel` writer - "]
pub type CHIP_CLK_OUT_3_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `chip_clk_out_0_en` reader - "]
pub type CHIP_CLK_OUT_0_EN_R = crate::BitReader;
#[doc = "Field `chip_clk_out_0_en` writer - "]
pub type CHIP_CLK_OUT_0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `chip_clk_out_1_en` reader - "]
pub type CHIP_CLK_OUT_1_EN_R = crate::BitReader;
#[doc = "Field `chip_clk_out_1_en` writer - "]
pub type CHIP_CLK_OUT_1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `chip_clk_out_2_en` reader - "]
pub type CHIP_CLK_OUT_2_EN_R = crate::BitReader;
#[doc = "Field `chip_clk_out_2_en` writer - "]
pub type CHIP_CLK_OUT_2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `chip_clk_out_3_en` reader - "]
pub type CHIP_CLK_OUT_3_EN_R = crate::BitReader;
#[doc = "Field `chip_clk_out_3_en` writer - "]
pub type CHIP_CLK_OUT_3_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpio_tmr_clk_sel` reader - "]
pub type GPIO_TMR_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `gpio_tmr_clk_sel` writer - "]
pub type GPIO_TMR_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn chip_clk_out_0_sel(&self) -> CHIP_CLK_OUT_0_SEL_R {
        CHIP_CLK_OUT_0_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn chip_clk_out_1_sel(&self) -> CHIP_CLK_OUT_1_SEL_R {
        CHIP_CLK_OUT_1_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn chip_clk_out_2_sel(&self) -> CHIP_CLK_OUT_2_SEL_R {
        CHIP_CLK_OUT_2_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn chip_clk_out_3_sel(&self) -> CHIP_CLK_OUT_3_SEL_R {
        CHIP_CLK_OUT_3_SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn chip_clk_out_0_en(&self) -> CHIP_CLK_OUT_0_EN_R {
        CHIP_CLK_OUT_0_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn chip_clk_out_1_en(&self) -> CHIP_CLK_OUT_1_EN_R {
        CHIP_CLK_OUT_1_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn chip_clk_out_2_en(&self) -> CHIP_CLK_OUT_2_EN_R {
        CHIP_CLK_OUT_2_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn chip_clk_out_3_en(&self) -> CHIP_CLK_OUT_3_EN_R {
        CHIP_CLK_OUT_3_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn gpio_tmr_clk_sel(&self) -> GPIO_TMR_CLK_SEL_R {
        GPIO_TMR_CLK_SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn chip_clk_out_0_sel(&mut self) -> CHIP_CLK_OUT_0_SEL_W<DIGIT_CLOCK_2_SPEC> {
        CHIP_CLK_OUT_0_SEL_W::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn chip_clk_out_1_sel(&mut self) -> CHIP_CLK_OUT_1_SEL_W<DIGIT_CLOCK_2_SPEC> {
        CHIP_CLK_OUT_1_SEL_W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn chip_clk_out_2_sel(&mut self) -> CHIP_CLK_OUT_2_SEL_W<DIGIT_CLOCK_2_SPEC> {
        CHIP_CLK_OUT_2_SEL_W::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn chip_clk_out_3_sel(&mut self) -> CHIP_CLK_OUT_3_SEL_W<DIGIT_CLOCK_2_SPEC> {
        CHIP_CLK_OUT_3_SEL_W::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn chip_clk_out_0_en(&mut self) -> CHIP_CLK_OUT_0_EN_W<DIGIT_CLOCK_2_SPEC> {
        CHIP_CLK_OUT_0_EN_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn chip_clk_out_1_en(&mut self) -> CHIP_CLK_OUT_1_EN_W<DIGIT_CLOCK_2_SPEC> {
        CHIP_CLK_OUT_1_EN_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn chip_clk_out_2_en(&mut self) -> CHIP_CLK_OUT_2_EN_W<DIGIT_CLOCK_2_SPEC> {
        CHIP_CLK_OUT_2_EN_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn chip_clk_out_3_en(&mut self) -> CHIP_CLK_OUT_3_EN_W<DIGIT_CLOCK_2_SPEC> {
        CHIP_CLK_OUT_3_EN_W::new(self, 11)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_tmr_clk_sel(&mut self) -> GPIO_TMR_CLK_SEL_W<DIGIT_CLOCK_2_SPEC> {
        GPIO_TMR_CLK_SEL_W::new(self, 12)
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
#[doc = "Digital clock configuration 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`digit_clock_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`digit_clock_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIGIT_CLOCK_2_SPEC;
impl crate::RegisterSpec for DIGIT_CLOCK_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`digit_clock_2::R`](R) reader structure"]
impl crate::Readable for DIGIT_CLOCK_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`digit_clock_2::W`](W) writer structure"]
impl crate::Writable for DIGIT_CLOCK_2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets digit_clock_2 to value 0"]
impl crate::Resettable for DIGIT_CLOCK_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
