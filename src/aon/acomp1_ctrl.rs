#[doc = "Register `acomp1_ctrl` reader"]
pub type R = crate::R<ACOMP1_CTRL_SPEC>;
#[doc = "Register `acomp1_ctrl` writer"]
pub type W = crate::W<ACOMP1_CTRL_SPEC>;
#[doc = "Field `acomp1_en` reader - "]
pub type ACOMP1_EN_R = crate::BitReader;
#[doc = "Field `acomp1_en` writer - "]
pub type ACOMP1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `acomp1_hyst_seln` reader - "]
pub type ACOMP1_HYST_SELN_R = crate::FieldReader;
#[doc = "Field `acomp1_hyst_seln` writer - "]
pub type ACOMP1_HYST_SELN_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `acomp1_hyst_selp` reader - "]
pub type ACOMP1_HYST_SELP_R = crate::FieldReader;
#[doc = "Field `acomp1_hyst_selp` writer - "]
pub type ACOMP1_HYST_SELP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `acomp1_bias_prog` reader - "]
pub type ACOMP1_BIAS_PROG_R = crate::FieldReader;
#[doc = "Field `acomp1_bias_prog` writer - "]
pub type ACOMP1_BIAS_PROG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `acomp1_level_sel` reader - "]
pub type ACOMP1_LEVEL_SEL_R = crate::FieldReader;
#[doc = "Field `acomp1_level_sel` writer - "]
pub type ACOMP1_LEVEL_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `acomp1_neg_sel` reader - "]
pub type ACOMP1_NEG_SEL_R = crate::FieldReader;
#[doc = "Field `acomp1_neg_sel` writer - "]
pub type ACOMP1_NEG_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `acomp1_pos_sel` reader - "]
pub type ACOMP1_POS_SEL_R = crate::FieldReader;
#[doc = "Field `acomp1_pos_sel` writer - "]
pub type ACOMP1_POS_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `acomp1_muxen` reader - "]
pub type ACOMP1_MUXEN_R = crate::BitReader;
#[doc = "Field `acomp1_muxen` writer - "]
pub type ACOMP1_MUXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn acomp1_en(&self) -> ACOMP1_EN_R {
        ACOMP1_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn acomp1_hyst_seln(&self) -> ACOMP1_HYST_SELN_R {
        ACOMP1_HYST_SELN_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn acomp1_hyst_selp(&self) -> ACOMP1_HYST_SELP_R {
        ACOMP1_HYST_SELP_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn acomp1_bias_prog(&self) -> ACOMP1_BIAS_PROG_R {
        ACOMP1_BIAS_PROG_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn acomp1_level_sel(&self) -> ACOMP1_LEVEL_SEL_R {
        ACOMP1_LEVEL_SEL_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn acomp1_neg_sel(&self) -> ACOMP1_NEG_SEL_R {
        ACOMP1_NEG_SEL_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn acomp1_pos_sel(&self) -> ACOMP1_POS_SEL_R {
        ACOMP1_POS_SEL_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn acomp1_muxen(&self) -> ACOMP1_MUXEN_R {
        ACOMP1_MUXEN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn acomp1_en(&mut self) -> ACOMP1_EN_W<ACOMP1_CTRL_SPEC> {
        ACOMP1_EN_W::new(self, 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn acomp1_hyst_seln(&mut self) -> ACOMP1_HYST_SELN_W<ACOMP1_CTRL_SPEC> {
        ACOMP1_HYST_SELN_W::new(self, 4)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    #[must_use]
    pub fn acomp1_hyst_selp(&mut self) -> ACOMP1_HYST_SELP_W<ACOMP1_CTRL_SPEC> {
        ACOMP1_HYST_SELP_W::new(self, 7)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn acomp1_bias_prog(&mut self) -> ACOMP1_BIAS_PROG_W<ACOMP1_CTRL_SPEC> {
        ACOMP1_BIAS_PROG_W::new(self, 10)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    #[must_use]
    pub fn acomp1_level_sel(&mut self) -> ACOMP1_LEVEL_SEL_W<ACOMP1_CTRL_SPEC> {
        ACOMP1_LEVEL_SEL_W::new(self, 12)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    #[must_use]
    pub fn acomp1_neg_sel(&mut self) -> ACOMP1_NEG_SEL_W<ACOMP1_CTRL_SPEC> {
        ACOMP1_NEG_SEL_W::new(self, 18)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    #[must_use]
    pub fn acomp1_pos_sel(&mut self) -> ACOMP1_POS_SEL_W<ACOMP1_CTRL_SPEC> {
        ACOMP1_POS_SEL_W::new(self, 22)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn acomp1_muxen(&mut self) -> ACOMP1_MUXEN_W<ACOMP1_CTRL_SPEC> {
        ACOMP1_MUXEN_W::new(self, 26)
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
#[doc = "acomp1_ctrl.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acomp1_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acomp1_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACOMP1_CTRL_SPEC;
impl crate::RegisterSpec for ACOMP1_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acomp1_ctrl::R`](R) reader structure"]
impl crate::Readable for ACOMP1_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`acomp1_ctrl::W`](W) writer structure"]
impl crate::Writable for ACOMP1_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets acomp1_ctrl to value 0"]
impl crate::Resettable for ACOMP1_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
