#[doc = "Register `clock_config_0` reader"]
pub type R = crate::R<CLOCK_CONFIG_0_SPEC>;
#[doc = "Register `clock_config_0` writer"]
pub type W = crate::W<CLOCK_CONFIG_0_SPEC>;
#[doc = "Field `pll` reader - Enable or disable Phase-Locked Loop"]
pub type PLL_R = crate::BitReader;
#[doc = "Field `pll` writer - Enable or disable Phase-Locked Loop"]
pub type PLL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fclk` reader - Enable or disable fast clock"]
pub type FCLK_R = crate::BitReader;
#[doc = "Field `fclk` writer - Enable or disable fast clock"]
pub type FCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hclk` reader - Enable or disable hibernate clock"]
pub type HCLK_R = crate::BitReader;
#[doc = "Field `hclk` writer - Enable or disable hibernate clock"]
pub type HCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bclk` reader - Enable or disable bus clock"]
pub type BCLK_R = crate::BitReader;
#[doc = "Field `bclk` writer - Enable or disable bus clock"]
pub type BCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `root_clk_source` reader - Set source of root clock"]
pub type ROOT_CLK_SOURCE_R = crate::FieldReader;
#[doc = "Field `root_clk_source` writer - Set source of root clock"]
pub type ROOT_CLK_SOURCE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `hclk_divide` reader - Set divide factor of hibernate clock"]
pub type HCLK_DIVIDE_R = crate::FieldReader;
#[doc = "Field `hclk_divide` writer - Set divide factor of hibernate clock"]
pub type HCLK_DIVIDE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `bclk_divide` reader - Set divide factor of bus clock"]
pub type BCLK_DIVIDE_R = crate::FieldReader;
#[doc = "Field `bclk_divide` writer - Set divide factor of bus clock"]
pub type BCLK_DIVIDE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Enable or disable Phase-Locked Loop"]
    #[inline(always)]
    pub fn pll(&self) -> PLL_R {
        PLL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable fast clock"]
    #[inline(always)]
    pub fn fclk(&self) -> FCLK_R {
        FCLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable hibernate clock"]
    #[inline(always)]
    pub fn hclk(&self) -> HCLK_R {
        HCLK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable or disable bus clock"]
    #[inline(always)]
    pub fn bclk(&self) -> BCLK_R {
        BCLK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Set source of root clock"]
    #[inline(always)]
    pub fn root_clk_source(&self) -> ROOT_CLK_SOURCE_R {
        ROOT_CLK_SOURCE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - Set divide factor of hibernate clock"]
    #[inline(always)]
    pub fn hclk_divide(&self) -> HCLK_DIVIDE_R {
        HCLK_DIVIDE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Set divide factor of bus clock"]
    #[inline(always)]
    pub fn bclk_divide(&self) -> BCLK_DIVIDE_R {
        BCLK_DIVIDE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable Phase-Locked Loop"]
    #[inline(always)]
    #[must_use]
    pub fn pll(&mut self) -> PLL_W<CLOCK_CONFIG_0_SPEC> {
        PLL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable or disable fast clock"]
    #[inline(always)]
    #[must_use]
    pub fn fclk(&mut self) -> FCLK_W<CLOCK_CONFIG_0_SPEC> {
        FCLK_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable or disable hibernate clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk(&mut self) -> HCLK_W<CLOCK_CONFIG_0_SPEC> {
        HCLK_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable or disable bus clock"]
    #[inline(always)]
    #[must_use]
    pub fn bclk(&mut self) -> BCLK_W<CLOCK_CONFIG_0_SPEC> {
        BCLK_W::new(self, 3)
    }
    #[doc = "Bits 6:7 - Set source of root clock"]
    #[inline(always)]
    #[must_use]
    pub fn root_clk_source(&mut self) -> ROOT_CLK_SOURCE_W<CLOCK_CONFIG_0_SPEC> {
        ROOT_CLK_SOURCE_W::new(self, 6)
    }
    #[doc = "Bits 8:15 - Set divide factor of hibernate clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_divide(&mut self) -> HCLK_DIVIDE_W<CLOCK_CONFIG_0_SPEC> {
        HCLK_DIVIDE_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Set divide factor of bus clock"]
    #[inline(always)]
    #[must_use]
    pub fn bclk_divide(&mut self) -> BCLK_DIVIDE_W<CLOCK_CONFIG_0_SPEC> {
        BCLK_DIVIDE_W::new(self, 16)
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
#[doc = "System clock configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_config_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_config_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLOCK_CONFIG_0_SPEC;
impl crate::RegisterSpec for CLOCK_CONFIG_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clock_config_0::R`](R) reader structure"]
impl crate::Readable for CLOCK_CONFIG_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clock_config_0::W`](W) writer structure"]
impl crate::Writable for CLOCK_CONFIG_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets clock_config_0 to value 0"]
impl crate::Resettable for CLOCK_CONFIG_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
