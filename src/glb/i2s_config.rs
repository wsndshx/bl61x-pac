#[doc = "Register `i2s_config` reader"]
pub type R = crate::R<I2S_CONFIG_SPEC>;
#[doc = "Register `i2s_config` writer"]
pub type W = crate::W<I2S_CONFIG_SPEC>;
#[doc = "Field `reg_i2s_ref_clk_div` reader - "]
pub type REG_I2S_REF_CLK_DIV_R = crate::FieldReader;
#[doc = "Field `reg_i2s_ref_clk_div` writer - "]
pub type REG_I2S_REF_CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `reg_i2s_di_ref_clk_sel` reader - "]
pub type REG_I2S_DI_REF_CLK_SEL_R = crate::BitReader;
#[doc = "Field `reg_i2s_di_ref_clk_sel` writer - "]
pub type REG_I2S_DI_REF_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_i2s_ref_clk_en` reader - "]
pub type REG_I2S_REF_CLK_EN_R = crate::BitReader;
#[doc = "Field `reg_i2s_ref_clk_en` writer - "]
pub type REG_I2S_REF_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_i2s_do_ref_clk_sel` reader - "]
pub type REG_I2S_DO_REF_CLK_SEL_R = crate::BitReader;
#[doc = "Field `reg_i2s_do_ref_clk_sel` writer - "]
pub type REG_I2S_DO_REF_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn reg_i2s_ref_clk_div(&self) -> REG_I2S_REF_CLK_DIV_R {
        REG_I2S_REF_CLK_DIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_i2s_di_ref_clk_sel(&self) -> REG_I2S_DI_REF_CLK_SEL_R {
        REG_I2S_DI_REF_CLK_SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_i2s_ref_clk_en(&self) -> REG_I2S_REF_CLK_EN_R {
        REG_I2S_REF_CLK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_i2s_do_ref_clk_sel(&self) -> REG_I2S_DO_REF_CLK_SEL_R {
        REG_I2S_DO_REF_CLK_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn reg_i2s_ref_clk_div(&mut self) -> REG_I2S_REF_CLK_DIV_W<I2S_CONFIG_SPEC> {
        REG_I2S_REF_CLK_DIV_W::new(self, 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn reg_i2s_di_ref_clk_sel(&mut self) -> REG_I2S_DI_REF_CLK_SEL_W<I2S_CONFIG_SPEC> {
        REG_I2S_DI_REF_CLK_SEL_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn reg_i2s_ref_clk_en(&mut self) -> REG_I2S_REF_CLK_EN_W<I2S_CONFIG_SPEC> {
        REG_I2S_REF_CLK_EN_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn reg_i2s_do_ref_clk_sel(&mut self) -> REG_I2S_DO_REF_CLK_SEL_W<I2S_CONFIG_SPEC> {
        REG_I2S_DO_REF_CLK_SEL_W::new(self, 8)
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
#[doc = "Inter-IC Sound configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2S_CONFIG_SPEC;
impl crate::RegisterSpec for I2S_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_config::R`](R) reader structure"]
impl crate::Readable for I2S_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2s_config::W`](W) writer structure"]
impl crate::Writable for I2S_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2s_config to value 0"]
impl crate::Resettable for I2S_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
