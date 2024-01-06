#[doc = "Register `gpadc_reg_isr` reader"]
pub type R = crate::R<GPADC_REG_ISR_SPEC>;
#[doc = "Register `gpadc_reg_isr` writer"]
pub type W = crate::W<GPADC_REG_ISR_SPEC>;
#[doc = "Field `gpadc_neg_satur` reader - "]
pub type GPADC_NEG_SATUR_R = crate::BitReader;
#[doc = "Field `gpadc_neg_satur` writer - "]
pub type GPADC_NEG_SATUR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_pos_satur` reader - "]
pub type GPADC_POS_SATUR_R = crate::BitReader;
#[doc = "Field `gpadc_pos_satur` writer - "]
pub type GPADC_POS_SATUR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_neg_satur_clr` reader - "]
pub type GPADC_NEG_SATUR_CLR_R = crate::BitReader;
#[doc = "Field `gpadc_neg_satur_clr` writer - "]
pub type GPADC_NEG_SATUR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_pos_satur_clr` reader - "]
pub type GPADC_POS_SATUR_CLR_R = crate::BitReader;
#[doc = "Field `gpadc_pos_satur_clr` writer - "]
pub type GPADC_POS_SATUR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_neg_satur_mask` reader - "]
pub type GPADC_NEG_SATUR_MASK_R = crate::BitReader;
#[doc = "Field `gpadc_neg_satur_mask` writer - "]
pub type GPADC_NEG_SATUR_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gpadc_pos_satur_mask` reader - "]
pub type GPADC_POS_SATUR_MASK_R = crate::BitReader;
#[doc = "Field `gpadc_pos_satur_mask` writer - "]
pub type GPADC_POS_SATUR_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_neg_satur(&self) -> GPADC_NEG_SATUR_R {
        GPADC_NEG_SATUR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpadc_pos_satur(&self) -> GPADC_POS_SATUR_R {
        GPADC_POS_SATUR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpadc_neg_satur_clr(&self) -> GPADC_NEG_SATUR_CLR_R {
        GPADC_NEG_SATUR_CLR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpadc_pos_satur_clr(&self) -> GPADC_POS_SATUR_CLR_R {
        GPADC_POS_SATUR_CLR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_neg_satur_mask(&self) -> GPADC_NEG_SATUR_MASK_R {
        GPADC_NEG_SATUR_MASK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpadc_pos_satur_mask(&self) -> GPADC_POS_SATUR_MASK_R {
        GPADC_POS_SATUR_MASK_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_neg_satur(&mut self) -> GPADC_NEG_SATUR_W<GPADC_REG_ISR_SPEC> {
        GPADC_NEG_SATUR_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_pos_satur(&mut self) -> GPADC_POS_SATUR_W<GPADC_REG_ISR_SPEC> {
        GPADC_POS_SATUR_W::new(self, 1)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_neg_satur_clr(&mut self) -> GPADC_NEG_SATUR_CLR_W<GPADC_REG_ISR_SPEC> {
        GPADC_NEG_SATUR_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_pos_satur_clr(&mut self) -> GPADC_POS_SATUR_CLR_W<GPADC_REG_ISR_SPEC> {
        GPADC_POS_SATUR_CLR_W::new(self, 5)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_neg_satur_mask(&mut self) -> GPADC_NEG_SATUR_MASK_W<GPADC_REG_ISR_SPEC> {
        GPADC_NEG_SATUR_MASK_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn gpadc_pos_satur_mask(&mut self) -> GPADC_POS_SATUR_MASK_W<GPADC_REG_ISR_SPEC> {
        GPADC_POS_SATUR_MASK_W::new(self, 9)
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
#[doc = "gpadc_reg_isr.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpadc_reg_isr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpadc_reg_isr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPADC_REG_ISR_SPEC;
impl crate::RegisterSpec for GPADC_REG_ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpadc_reg_isr::R`](R) reader structure"]
impl crate::Readable for GPADC_REG_ISR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpadc_reg_isr::W`](W) writer structure"]
impl crate::Writable for GPADC_REG_ISR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpadc_reg_isr to value 0"]
impl crate::Resettable for GPADC_REG_ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
