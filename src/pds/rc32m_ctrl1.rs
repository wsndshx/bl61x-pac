#[doc = "Register `rc32m_ctrl1` reader"]
pub type R = crate::R<RC32M_CTRL1_SPEC>;
#[doc = "Register `rc32m_ctrl1` writer"]
pub type W = crate::W<RC32M_CTRL1_SPEC>;
#[doc = "Field `rc32m_test_en` reader - "]
pub type RC32M_TEST_EN_R = crate::BitReader;
#[doc = "Field `rc32m_test_en` writer - "]
pub type RC32M_TEST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32m_soft_rst` reader - "]
pub type RC32M_SOFT_RST_R = crate::BitReader;
#[doc = "Field `rc32m_soft_rst` writer - "]
pub type RC32M_SOFT_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32m_clk_soft_rst` reader - "]
pub type RC32M_CLK_SOFT_RST_R = crate::BitReader;
#[doc = "Field `rc32m_clk_soft_rst` writer - "]
pub type RC32M_CLK_SOFT_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32m_clk_inv` reader - "]
pub type RC32M_CLK_INV_R = crate::BitReader;
#[doc = "Field `rc32m_clk_inv` writer - "]
pub type RC32M_CLK_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rc32m_clk_force_on` reader - "]
pub type RC32M_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `rc32m_clk_force_on` writer - "]
pub type RC32M_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rc32m_test_en(&self) -> RC32M_TEST_EN_R {
        RC32M_TEST_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rc32m_soft_rst(&self) -> RC32M_SOFT_RST_R {
        RC32M_SOFT_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rc32m_clk_soft_rst(&self) -> RC32M_CLK_SOFT_RST_R {
        RC32M_CLK_SOFT_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rc32m_clk_inv(&self) -> RC32M_CLK_INV_R {
        RC32M_CLK_INV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rc32m_clk_force_on(&self) -> RC32M_CLK_FORCE_ON_R {
        RC32M_CLK_FORCE_ON_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rc32m_test_en(&mut self) -> RC32M_TEST_EN_W<RC32M_CTRL1_SPEC> {
        RC32M_TEST_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rc32m_soft_rst(&mut self) -> RC32M_SOFT_RST_W<RC32M_CTRL1_SPEC> {
        RC32M_SOFT_RST_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rc32m_clk_soft_rst(&mut self) -> RC32M_CLK_SOFT_RST_W<RC32M_CTRL1_SPEC> {
        RC32M_CLK_SOFT_RST_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rc32m_clk_inv(&mut self) -> RC32M_CLK_INV_W<RC32M_CTRL1_SPEC> {
        RC32M_CLK_INV_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rc32m_clk_force_on(&mut self) -> RC32M_CLK_FORCE_ON_W<RC32M_CTRL1_SPEC> {
        RC32M_CLK_FORCE_ON_W::new(self, 4)
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
#[doc = "rc32m_ctrl1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc32m_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rc32m_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RC32M_CTRL1_SPEC;
impl crate::RegisterSpec for RC32M_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rc32m_ctrl1::R`](R) reader structure"]
impl crate::Readable for RC32M_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rc32m_ctrl1::W`](W) writer structure"]
impl crate::Writable for RC32M_CTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rc32m_ctrl1 to value 0"]
impl crate::Resettable for RC32M_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
