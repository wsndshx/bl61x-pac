#[doc = "Register `interrupt_mode` reader"]
pub type R = crate::R<INTERRUPT_MODE_SPEC>;
#[doc = "Register `interrupt_mode` writer"]
pub type W = crate::W<INTERRUPT_MODE_SPEC>;
#[doc = "Field `hbn_pin_wakeup_mode` reader - "]
pub type HBN_PIN_WAKEUP_MODE_R = crate::FieldReader;
#[doc = "Field `hbn_pin_wakeup_mode` writer - "]
pub type HBN_PIN_WAKEUP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `hbn_pin_wakeup_mask` reader - "]
pub type HBN_PIN_WAKEUP_MASK_R = crate::FieldReader;
#[doc = "Field `hbn_pin_wakeup_mask` writer - "]
pub type HBN_PIN_WAKEUP_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_en_hw_pu_pd` reader - "]
pub type REG_EN_HW_PU_PD_R = crate::BitReader;
#[doc = "Field `reg_en_hw_pu_pd` writer - "]
pub type REG_EN_HW_PU_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `irq_bor_en` reader - "]
pub type IRQ_BOR_EN_R = crate::BitReader;
#[doc = "Field `irq_bor_en` writer - "]
pub type IRQ_BOR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `irq_acomp0_en` reader - "]
pub type IRQ_ACOMP0_EN_R = crate::FieldReader;
#[doc = "Field `irq_acomp0_en` writer - "]
pub type IRQ_ACOMP0_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `irq_acomp1_en` reader - "]
pub type IRQ_ACOMP1_EN_R = crate::FieldReader;
#[doc = "Field `irq_acomp1_en` writer - "]
pub type IRQ_ACOMP1_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pin_wakeup_sel` reader - "]
pub type PIN_WAKEUP_SEL_R = crate::FieldReader;
#[doc = "Field `pin_wakeup_sel` writer - "]
pub type PIN_WAKEUP_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pin_wakeup_en` reader - "]
pub type PIN_WAKEUP_EN_R = crate::BitReader;
#[doc = "Field `pin_wakeup_en` writer - "]
pub type PIN_WAKEUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn hbn_pin_wakeup_mode(&self) -> HBN_PIN_WAKEUP_MODE_R {
        HBN_PIN_WAKEUP_MODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn hbn_pin_wakeup_mask(&self) -> HBN_PIN_WAKEUP_MASK_R {
        HBN_PIN_WAKEUP_MASK_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_en_hw_pu_pd(&self) -> REG_EN_HW_PU_PD_R {
        REG_EN_HW_PU_PD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn irq_bor_en(&self) -> IRQ_BOR_EN_R {
        IRQ_BOR_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn irq_acomp0_en(&self) -> IRQ_ACOMP0_EN_R {
        IRQ_ACOMP0_EN_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn irq_acomp1_en(&self) -> IRQ_ACOMP1_EN_R {
        IRQ_ACOMP1_EN_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn pin_wakeup_sel(&self) -> PIN_WAKEUP_SEL_R {
        PIN_WAKEUP_SEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn pin_wakeup_en(&self) -> PIN_WAKEUP_EN_R {
        PIN_WAKEUP_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_pin_wakeup_mode(&mut self) -> HBN_PIN_WAKEUP_MODE_W<INTERRUPT_MODE_SPEC> {
        HBN_PIN_WAKEUP_MODE_W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_pin_wakeup_mask(&mut self) -> HBN_PIN_WAKEUP_MASK_W<INTERRUPT_MODE_SPEC> {
        HBN_PIN_WAKEUP_MASK_W::new(self, 4)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn reg_en_hw_pu_pd(&mut self) -> REG_EN_HW_PU_PD_W<INTERRUPT_MODE_SPEC> {
        REG_EN_HW_PU_PD_W::new(self, 16)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn irq_bor_en(&mut self) -> IRQ_BOR_EN_W<INTERRUPT_MODE_SPEC> {
        IRQ_BOR_EN_W::new(self, 18)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn irq_acomp0_en(&mut self) -> IRQ_ACOMP0_EN_W<INTERRUPT_MODE_SPEC> {
        IRQ_ACOMP0_EN_W::new(self, 20)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    #[must_use]
    pub fn irq_acomp1_en(&mut self) -> IRQ_ACOMP1_EN_W<INTERRUPT_MODE_SPEC> {
        IRQ_ACOMP1_EN_W::new(self, 22)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    #[must_use]
    pub fn pin_wakeup_sel(&mut self) -> PIN_WAKEUP_SEL_W<INTERRUPT_MODE_SPEC> {
        PIN_WAKEUP_SEL_W::new(self, 24)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn pin_wakeup_en(&mut self) -> PIN_WAKEUP_EN_W<INTERRUPT_MODE_SPEC> {
        PIN_WAKEUP_EN_W::new(self, 27)
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
#[doc = "Hibernate interrupt contol\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERRUPT_MODE_SPEC;
impl crate::RegisterSpec for INTERRUPT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_mode::R`](R) reader structure"]
impl crate::Readable for INTERRUPT_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interrupt_mode::W`](W) writer structure"]
impl crate::Writable for INTERRUPT_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets interrupt_mode to value 0"]
impl crate::Resettable for INTERRUPT_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
