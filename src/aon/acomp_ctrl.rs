#[doc = "Register `acomp_ctrl` reader"]
pub type R = crate::R<ACOMP_CTRL_SPEC>;
#[doc = "Register `acomp_ctrl` writer"]
pub type W = crate::W<ACOMP_CTRL_SPEC>;
#[doc = "Field `acomp1_rstn_ana` reader - "]
pub type ACOMP1_RSTN_ANA_R = crate::BitReader;
#[doc = "Field `acomp1_rstn_ana` writer - "]
pub type ACOMP1_RSTN_ANA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `acomp0_rstn_ana` reader - "]
pub type ACOMP0_RSTN_ANA_R = crate::BitReader;
#[doc = "Field `acomp0_rstn_ana` writer - "]
pub type ACOMP0_RSTN_ANA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `acomp1_test_en` reader - "]
pub type ACOMP1_TEST_EN_R = crate::BitReader;
#[doc = "Field `acomp1_test_en` writer - "]
pub type ACOMP1_TEST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `acomp0_test_en` reader - "]
pub type ACOMP0_TEST_EN_R = crate::BitReader;
#[doc = "Field `acomp0_test_en` writer - "]
pub type ACOMP0_TEST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `acomp1_test_sel` reader - "]
pub type ACOMP1_TEST_SEL_R = crate::FieldReader;
#[doc = "Field `acomp1_test_sel` writer - "]
pub type ACOMP1_TEST_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `acomp0_test_sel` reader - "]
pub type ACOMP0_TEST_SEL_R = crate::FieldReader;
#[doc = "Field `acomp0_test_sel` writer - "]
pub type ACOMP0_TEST_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `acomp1_out_raw` reader - "]
pub type ACOMP1_OUT_RAW_R = crate::BitReader;
#[doc = "Field `acomp1_out_raw` writer - "]
pub type ACOMP1_OUT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `acomp0_out_raw` reader - "]
pub type ACOMP0_OUT_RAW_R = crate::BitReader;
#[doc = "Field `acomp0_out_raw` writer - "]
pub type ACOMP0_OUT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `acomp_vref_sel` reader - "]
pub type ACOMP_VREF_SEL_R = crate::FieldReader;
#[doc = "Field `acomp_vref_sel` writer - "]
pub type ACOMP_VREF_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn acomp1_rstn_ana(&self) -> ACOMP1_RSTN_ANA_R {
        ACOMP1_RSTN_ANA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn acomp0_rstn_ana(&self) -> ACOMP0_RSTN_ANA_R {
        ACOMP0_RSTN_ANA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn acomp1_test_en(&self) -> ACOMP1_TEST_EN_R {
        ACOMP1_TEST_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn acomp0_test_en(&self) -> ACOMP0_TEST_EN_R {
        ACOMP0_TEST_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn acomp1_test_sel(&self) -> ACOMP1_TEST_SEL_R {
        ACOMP1_TEST_SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn acomp0_test_sel(&self) -> ACOMP0_TEST_SEL_R {
        ACOMP0_TEST_SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn acomp1_out_raw(&self) -> ACOMP1_OUT_RAW_R {
        ACOMP1_OUT_RAW_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn acomp0_out_raw(&self) -> ACOMP0_OUT_RAW_R {
        ACOMP0_OUT_RAW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn acomp_vref_sel(&self) -> ACOMP_VREF_SEL_R {
        ACOMP_VREF_SEL_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn acomp1_rstn_ana(&mut self) -> ACOMP1_RSTN_ANA_W<ACOMP_CTRL_SPEC> {
        ACOMP1_RSTN_ANA_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn acomp0_rstn_ana(&mut self) -> ACOMP0_RSTN_ANA_W<ACOMP_CTRL_SPEC> {
        ACOMP0_RSTN_ANA_W::new(self, 1)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn acomp1_test_en(&mut self) -> ACOMP1_TEST_EN_W<ACOMP_CTRL_SPEC> {
        ACOMP1_TEST_EN_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn acomp0_test_en(&mut self) -> ACOMP0_TEST_EN_W<ACOMP_CTRL_SPEC> {
        ACOMP0_TEST_EN_W::new(self, 9)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn acomp1_test_sel(&mut self) -> ACOMP1_TEST_SEL_W<ACOMP_CTRL_SPEC> {
        ACOMP1_TEST_SEL_W::new(self, 10)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn acomp0_test_sel(&mut self) -> ACOMP0_TEST_SEL_W<ACOMP_CTRL_SPEC> {
        ACOMP0_TEST_SEL_W::new(self, 12)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn acomp1_out_raw(&mut self) -> ACOMP1_OUT_RAW_W<ACOMP_CTRL_SPEC> {
        ACOMP1_OUT_RAW_W::new(self, 17)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn acomp0_out_raw(&mut self) -> ACOMP0_OUT_RAW_W<ACOMP_CTRL_SPEC> {
        ACOMP0_OUT_RAW_W::new(self, 19)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    #[must_use]
    pub fn acomp_vref_sel(&mut self) -> ACOMP_VREF_SEL_W<ACOMP_CTRL_SPEC> {
        ACOMP_VREF_SEL_W::new(self, 24)
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
#[doc = "acomp_ctrl.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acomp_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acomp_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACOMP_CTRL_SPEC;
impl crate::RegisterSpec for ACOMP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acomp_ctrl::R`](R) reader structure"]
impl crate::Readable for ACOMP_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`acomp_ctrl::W`](W) writer structure"]
impl crate::Writable for ACOMP_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets acomp_ctrl to value 0"]
impl crate::Resettable for ACOMP_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
