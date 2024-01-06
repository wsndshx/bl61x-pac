#[doc = "Register `digit_clock_0` reader"]
pub type R = crate::R<DIGIT_CLOCK_0_SPEC>;
#[doc = "Register `digit_clock_0` writer"]
pub type W = crate::W<DIGIT_CLOCK_0_SPEC>;
#[doc = "Field `dig_32k_div` reader - "]
pub type DIG_32K_DIV_R = crate::FieldReader<u16>;
#[doc = "Field `dig_32k_div` writer - "]
pub type DIG_32K_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `dig_32k_en` reader - "]
pub type DIG_32K_EN_R = crate::BitReader;
#[doc = "Field `dig_32k_en` writer - "]
pub type DIG_32K_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dig_32k_comp` reader - "]
pub type DIG_32K_COMP_R = crate::BitReader;
#[doc = "Field `dig_32k_comp` writer - "]
pub type DIG_32K_COMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dig_512k_div` reader - "]
pub type DIG_512K_DIV_R = crate::FieldReader;
#[doc = "Field `dig_512k_div` writer - "]
pub type DIG_512K_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dig_512k_en` reader - "]
pub type DIG_512K_EN_R = crate::BitReader;
#[doc = "Field `dig_512k_en` writer - "]
pub type DIG_512K_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dig_512k_comp` reader - "]
pub type DIG_512K_COMP_R = crate::BitReader;
#[doc = "Field `dig_512k_comp` writer - "]
pub type DIG_512K_COMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dig_clk_src_sel` reader - "]
pub type DIG_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `dig_clk_src_sel` writer - "]
pub type DIG_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_en_platform_wakeup` reader - "]
pub type REG_EN_PLATFORM_WAKEUP_R = crate::BitReader;
#[doc = "Field `reg_en_platform_wakeup` writer - "]
pub type REG_EN_PLATFORM_WAKEUP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn dig_32k_div(&self) -> DIG_32K_DIV_R {
        DIG_32K_DIV_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dig_32k_en(&self) -> DIG_32K_EN_R {
        DIG_32K_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dig_32k_comp(&self) -> DIG_32K_COMP_R {
        DIG_32K_COMP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn dig_512k_div(&self) -> DIG_512K_DIV_R {
        DIG_512K_DIV_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dig_512k_en(&self) -> DIG_512K_EN_R {
        DIG_512K_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dig_512k_comp(&self) -> DIG_512K_COMP_R {
        DIG_512K_COMP_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn dig_clk_src_sel(&self) -> DIG_CLK_SRC_SEL_R {
        DIG_CLK_SRC_SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reg_en_platform_wakeup(&self) -> REG_EN_PLATFORM_WAKEUP_R {
        REG_EN_PLATFORM_WAKEUP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn dig_32k_div(&mut self) -> DIG_32K_DIV_W<DIGIT_CLOCK_0_SPEC> {
        DIG_32K_DIV_W::new(self, 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn dig_32k_en(&mut self) -> DIG_32K_EN_W<DIGIT_CLOCK_0_SPEC> {
        DIG_32K_EN_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn dig_32k_comp(&mut self) -> DIG_32K_COMP_W<DIGIT_CLOCK_0_SPEC> {
        DIG_32K_COMP_W::new(self, 13)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    #[must_use]
    pub fn dig_512k_div(&mut self) -> DIG_512K_DIV_W<DIGIT_CLOCK_0_SPEC> {
        DIG_512K_DIV_W::new(self, 16)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn dig_512k_en(&mut self) -> DIG_512K_EN_W<DIGIT_CLOCK_0_SPEC> {
        DIG_512K_EN_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn dig_512k_comp(&mut self) -> DIG_512K_COMP_W<DIGIT_CLOCK_0_SPEC> {
        DIG_512K_COMP_W::new(self, 25)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    #[must_use]
    pub fn dig_clk_src_sel(&mut self) -> DIG_CLK_SRC_SEL_W<DIGIT_CLOCK_0_SPEC> {
        DIG_CLK_SRC_SEL_W::new(self, 28)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_en_platform_wakeup(&mut self) -> REG_EN_PLATFORM_WAKEUP_W<DIGIT_CLOCK_0_SPEC> {
        REG_EN_PLATFORM_WAKEUP_W::new(self, 31)
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
#[doc = "Digital clock configuration 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`digit_clock_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`digit_clock_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIGIT_CLOCK_0_SPEC;
impl crate::RegisterSpec for DIGIT_CLOCK_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`digit_clock_0::R`](R) reader structure"]
impl crate::Readable for DIGIT_CLOCK_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`digit_clock_0::W`](W) writer structure"]
impl crate::Writable for DIGIT_CLOCK_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets digit_clock_0 to value 0"]
impl crate::Resettable for DIGIT_CLOCK_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
