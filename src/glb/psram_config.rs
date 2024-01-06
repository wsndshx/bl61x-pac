#[doc = "Register `psram_config` reader"]
pub type R = crate::R<PSRAM_CONFIG_SPEC>;
#[doc = "Register `psram_config` writer"]
pub type W = crate::W<PSRAM_CONFIG_SPEC>;
#[doc = "Field `reg_psramB_clk_en` reader - "]
pub type REG_PSRAM_B_CLK_EN_R = crate::BitReader;
#[doc = "Field `reg_psramB_clk_en` writer - "]
pub type REG_PSRAM_B_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_psramB_clk_sel` reader - "]
pub type REG_PSRAM_B_CLK_SEL_R = crate::BitReader;
#[doc = "Field `reg_psramB_clk_sel` writer - "]
pub type REG_PSRAM_B_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_psramB_clk_div` reader - "]
pub type REG_PSRAM_B_CLK_DIV_R = crate::FieldReader;
#[doc = "Field `reg_psramB_clk_div` writer - "]
pub type REG_PSRAM_B_CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn reg_psram_b_clk_en(&self) -> REG_PSRAM_B_CLK_EN_R {
        REG_PSRAM_B_CLK_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn reg_psram_b_clk_sel(&self) -> REG_PSRAM_B_CLK_SEL_R {
        REG_PSRAM_B_CLK_SEL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn reg_psram_b_clk_div(&self) -> REG_PSRAM_B_CLK_DIV_R {
        REG_PSRAM_B_CLK_DIV_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn reg_psram_b_clk_en(&mut self) -> REG_PSRAM_B_CLK_EN_W<PSRAM_CONFIG_SPEC> {
        REG_PSRAM_B_CLK_EN_W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn reg_psram_b_clk_sel(&mut self) -> REG_PSRAM_B_CLK_SEL_W<PSRAM_CONFIG_SPEC> {
        REG_PSRAM_B_CLK_SEL_W::new(self, 28)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_psram_b_clk_div(&mut self) -> REG_PSRAM_B_CLK_DIV_W<PSRAM_CONFIG_SPEC> {
        REG_PSRAM_B_CLK_DIV_W::new(self, 30)
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
#[doc = "Pseudo Static Random-Access Memory configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psram_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psram_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSRAM_CONFIG_SPEC;
impl crate::RegisterSpec for PSRAM_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_config::R`](R) reader structure"]
impl crate::Readable for PSRAM_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psram_config::W`](W) writer structure"]
impl crate::Writable for PSRAM_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets psram_config to value 0"]
impl crate::Resettable for PSRAM_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
