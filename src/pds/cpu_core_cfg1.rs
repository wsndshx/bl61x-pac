#[doc = "Register `cpu_core_cfg1` reader"]
pub type R = crate::R<CPU_CORE_CFG1_SPEC>;
#[doc = "Register `cpu_core_cfg1` writer"]
pub type W = crate::W<CPU_CORE_CFG1_SPEC>;
#[doc = "Field `reg_pll_sel` reader - "]
pub type REG_PLL_SEL_R = crate::FieldReader;
#[doc = "Field `reg_pll_sel` writer - "]
pub type REG_PLL_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_mcu1_clk_en` reader - "]
pub type REG_MCU1_CLK_EN_R = crate::BitReader;
#[doc = "Field `reg_mcu1_clk_en` writer - "]
pub type REG_MCU1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn reg_pll_sel(&self) -> REG_PLL_SEL_R {
        REG_PLL_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_mcu1_clk_en(&self) -> REG_MCU1_CLK_EN_R {
        REG_MCU1_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pll_sel(&mut self) -> REG_PLL_SEL_W<CPU_CORE_CFG1_SPEC> {
        REG_PLL_SEL_W::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu1_clk_en(&mut self) -> REG_MCU1_CLK_EN_W<CPU_CORE_CFG1_SPEC> {
        REG_MCU1_CLK_EN_W::new(self, 8)
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
#[doc = "cpu_core_cfg1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_core_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_core_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_CORE_CFG1_SPEC;
impl crate::RegisterSpec for CPU_CORE_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_core_cfg1::R`](R) reader structure"]
impl crate::Readable for CPU_CORE_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_core_cfg1::W`](W) writer structure"]
impl crate::Writable for CPU_CORE_CFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cpu_core_cfg1 to value 0"]
impl crate::Resettable for CPU_CORE_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
