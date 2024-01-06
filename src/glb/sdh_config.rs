#[doc = "Register `sdh_config` reader"]
pub type R = crate::R<SDH_CONFIG_SPEC>;
#[doc = "Register `sdh_config` writer"]
pub type W = crate::W<SDH_CONFIG_SPEC>;
#[doc = "Field `reg_sdh_clk_div` reader - "]
pub type REG_SDH_CLK_DIV_R = crate::FieldReader;
#[doc = "Field `reg_sdh_clk_div` writer - "]
pub type REG_SDH_CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_sdh_clk_sel` reader - "]
pub type REG_SDH_CLK_SEL_R = crate::BitReader;
#[doc = "Field `reg_sdh_clk_sel` writer - "]
pub type REG_SDH_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_sdh_clk_en` reader - "]
pub type REG_SDH_CLK_EN_R = crate::BitReader;
#[doc = "Field `reg_sdh_clk_en` writer - "]
pub type REG_SDH_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn reg_sdh_clk_div(&self) -> REG_SDH_CLK_DIV_R {
        REG_SDH_CLK_DIV_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_sdh_clk_sel(&self) -> REG_SDH_CLK_SEL_R {
        REG_SDH_CLK_SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_sdh_clk_en(&self) -> REG_SDH_CLK_EN_R {
        REG_SDH_CLK_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 9:11"]
    #[inline(always)]
    #[must_use]
    pub fn reg_sdh_clk_div(&mut self) -> REG_SDH_CLK_DIV_W<SDH_CONFIG_SPEC> {
        REG_SDH_CLK_DIV_W::new(self, 9)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn reg_sdh_clk_sel(&mut self) -> REG_SDH_CLK_SEL_W<SDH_CONFIG_SPEC> {
        REG_SDH_CLK_SEL_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn reg_sdh_clk_en(&mut self) -> REG_SDH_CLK_EN_W<SDH_CONFIG_SPEC> {
        REG_SDH_CLK_EN_W::new(self, 13)
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
#[doc = "Secure Digital host configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdh_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdh_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDH_CONFIG_SPEC;
impl crate::RegisterSpec for SDH_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdh_config::R`](R) reader structure"]
impl crate::Readable for SDH_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdh_config::W`](W) writer structure"]
impl crate::Writable for SDH_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sdh_config to value 0"]
impl crate::Resettable for SDH_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
