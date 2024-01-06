#[doc = "Register `gpadc_config` reader"]
pub type R = crate::R<GPADC_CONFIG_SPEC>;
#[doc = "Register `gpadc_config` writer"]
pub type W = crate::W<GPADC_CONFIG_SPEC>;
#[doc = "Field `gpadc_32m_clk_div` reader - "]
pub type GPADC_32M_CLK_DIV_R = crate::FieldReader;
#[doc = "Field `gpadc_32m_clk_div` writer - "]
pub type GPADC_32M_CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `gpadc_32m_clk_sel` reader - "]
pub type GPADC_32M_CLK_SEL_R = crate::BitReader;
#[doc = "Field `gpadc_32m_clk_sel` writer - "]
pub type GPADC_32M_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_32m_div_en` reader - "]
pub type GPADC_32M_DIV_EN_R = crate::BitReader;
#[doc = "Field `gpadc_32m_div_en` writer - "]
pub type GPADC_32M_DIV_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn gpadc_32m_clk_div(&self) -> GPADC_32M_CLK_DIV_R {
        GPADC_32M_CLK_DIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpadc_32m_clk_sel(&self) -> GPADC_32M_CLK_SEL_R {
        GPADC_32M_CLK_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_32m_div_en(&self) -> GPADC_32M_DIV_EN_R {
        GPADC_32M_DIV_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_32m_clk_div(&mut self) -> GPADC_32M_CLK_DIV_W<GPADC_CONFIG_SPEC> {
        GPADC_32M_CLK_DIV_W::new(self, 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_32m_clk_sel(&mut self) -> GPADC_32M_CLK_SEL_W<GPADC_CONFIG_SPEC> {
        GPADC_32M_CLK_SEL_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_32m_div_en(&mut self) -> GPADC_32M_DIV_EN_W<GPADC_CONFIG_SPEC> {
        GPADC_32M_DIV_EN_W::new(self, 8)
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
#[doc = "General Purpose Analog-to-digital convert configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpadc_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpadc_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPADC_CONFIG_SPEC;
impl crate::RegisterSpec for GPADC_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpadc_config::R`](R) reader structure"]
impl crate::Readable for GPADC_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpadc_config::W`](W) writer structure"]
impl crate::Writable for GPADC_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpadc_config to value 0"]
impl crate::Resettable for GPADC_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
