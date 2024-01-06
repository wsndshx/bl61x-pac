#[doc = "Register `digit_clock_1` reader"]
pub type R = crate::R<DIGIT_CLOCK_1_SPEC>;
#[doc = "Register `digit_clock_1` writer"]
pub type W = crate::W<DIGIT_CLOCK_1_SPEC>;
#[doc = "Field `reg_isp_muxpll_80m_sel` reader - "]
pub type REG_ISP_MUXPLL_80M_SEL_R = crate::FieldReader;
#[doc = "Field `reg_isp_muxpll_80m_sel` writer - "]
pub type REG_ISP_MUXPLL_80M_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_top_muxpll_80m_sel` reader - "]
pub type REG_TOP_MUXPLL_80M_SEL_R = crate::FieldReader;
#[doc = "Field `reg_top_muxpll_80m_sel` writer - "]
pub type REG_TOP_MUXPLL_80M_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_top_muxpll_160m_sel` reader - "]
pub type REG_TOP_MUXPLL_160M_SEL_R = crate::FieldReader;
#[doc = "Field `reg_top_muxpll_160m_sel` writer - "]
pub type REG_TOP_MUXPLL_160M_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn reg_isp_muxpll_80m_sel(&self) -> REG_ISP_MUXPLL_80M_SEL_R {
        REG_ISP_MUXPLL_80M_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn reg_top_muxpll_80m_sel(&self) -> REG_TOP_MUXPLL_80M_SEL_R {
        REG_TOP_MUXPLL_80M_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn reg_top_muxpll_160m_sel(&self) -> REG_TOP_MUXPLL_160M_SEL_R {
        REG_TOP_MUXPLL_160M_SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_isp_muxpll_80m_sel(&mut self) -> REG_ISP_MUXPLL_80M_SEL_W<DIGIT_CLOCK_1_SPEC> {
        REG_ISP_MUXPLL_80M_SEL_W::new(self, 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn reg_top_muxpll_80m_sel(&mut self) -> REG_TOP_MUXPLL_80M_SEL_W<DIGIT_CLOCK_1_SPEC> {
        REG_TOP_MUXPLL_80M_SEL_W::new(self, 8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn reg_top_muxpll_160m_sel(&mut self) -> REG_TOP_MUXPLL_160M_SEL_W<DIGIT_CLOCK_1_SPEC> {
        REG_TOP_MUXPLL_160M_SEL_W::new(self, 10)
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
#[doc = "Digital clock configuration 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`digit_clock_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`digit_clock_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIGIT_CLOCK_1_SPEC;
impl crate::RegisterSpec for DIGIT_CLOCK_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`digit_clock_1::R`](R) reader structure"]
impl crate::Readable for DIGIT_CLOCK_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`digit_clock_1::W`](W) writer structure"]
impl crate::Writable for DIGIT_CLOCK_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets digit_clock_1 to value 0"]
impl crate::Resettable for DIGIT_CLOCK_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
