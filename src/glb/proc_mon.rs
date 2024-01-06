#[doc = "Register `proc_mon` reader"]
pub type R = crate::R<PROC_MON_SPEC>;
#[doc = "Register `proc_mon` writer"]
pub type W = crate::W<PROC_MON_SPEC>;
#[doc = "Field `pu_proc_mon` reader - "]
pub type PU_PROC_MON_R = crate::BitReader;
#[doc = "Field `pu_proc_mon` writer - "]
pub type PU_PROC_MON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `osc_en_rvt` reader - "]
pub type OSC_EN_RVT_R = crate::BitReader;
#[doc = "Field `osc_en_rvt` writer - "]
pub type OSC_EN_RVT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `osc_en_lvt` reader - "]
pub type OSC_EN_LVT_R = crate::BitReader;
#[doc = "Field `osc_en_lvt` writer - "]
pub type OSC_EN_LVT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `osc_sel` reader - "]
pub type OSC_SEL_R = crate::BitReader;
#[doc = "Field `osc_sel` writer - "]
pub type OSC_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_ringcount` reader - "]
pub type RSTN_RINGCOUNT_R = crate::BitReader;
#[doc = "Field `rstn_ringcount` writer - "]
pub type RSTN_RINGCOUNT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_refcount` reader - "]
pub type RSTN_REFCOUNT_R = crate::BitReader;
#[doc = "Field `rstn_refcount` writer - "]
pub type RSTN_REFCOUNT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `refcount_div_onehot` reader - "]
pub type REFCOUNT_DIV_ONEHOT_R = crate::FieldReader;
#[doc = "Field `refcount_div_onehot` writer - "]
pub type REFCOUNT_DIV_ONEHOT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ring_freq` reader - "]
pub type RING_FREQ_R = crate::FieldReader<u16>;
#[doc = "Field `ring_freq` writer - "]
pub type RING_FREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ring_freq_rdy` reader - "]
pub type RING_FREQ_RDY_R = crate::BitReader;
#[doc = "Field `ring_freq_rdy` writer - "]
pub type RING_FREQ_RDY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_proc_mon(&self) -> PU_PROC_MON_R {
        PU_PROC_MON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn osc_en_rvt(&self) -> OSC_EN_RVT_R {
        OSC_EN_RVT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn osc_en_lvt(&self) -> OSC_EN_LVT_R {
        OSC_EN_LVT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn osc_sel(&self) -> OSC_SEL_R {
        OSC_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rstn_ringcount(&self) -> RSTN_RINGCOUNT_R {
        RSTN_RINGCOUNT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rstn_refcount(&self) -> RSTN_REFCOUNT_R {
        RSTN_REFCOUNT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn refcount_div_onehot(&self) -> REFCOUNT_DIV_ONEHOT_R {
        REFCOUNT_DIV_ONEHOT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:27"]
    #[inline(always)]
    pub fn ring_freq(&self) -> RING_FREQ_R {
        RING_FREQ_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ring_freq_rdy(&self) -> RING_FREQ_RDY_R {
        RING_FREQ_RDY_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pu_proc_mon(&mut self) -> PU_PROC_MON_W<PROC_MON_SPEC> {
        PU_PROC_MON_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn osc_en_rvt(&mut self) -> OSC_EN_RVT_W<PROC_MON_SPEC> {
        OSC_EN_RVT_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn osc_en_lvt(&mut self) -> OSC_EN_LVT_W<PROC_MON_SPEC> {
        OSC_EN_LVT_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn osc_sel(&mut self) -> OSC_SEL_W<PROC_MON_SPEC> {
        OSC_SEL_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_ringcount(&mut self) -> RSTN_RINGCOUNT_W<PROC_MON_SPEC> {
        RSTN_RINGCOUNT_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_refcount(&mut self) -> RSTN_REFCOUNT_W<PROC_MON_SPEC> {
        RSTN_REFCOUNT_W::new(self, 5)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn refcount_div_onehot(&mut self) -> REFCOUNT_DIV_ONEHOT_W<PROC_MON_SPEC> {
        REFCOUNT_DIV_ONEHOT_W::new(self, 8)
    }
    #[doc = "Bits 12:27"]
    #[inline(always)]
    #[must_use]
    pub fn ring_freq(&mut self) -> RING_FREQ_W<PROC_MON_SPEC> {
        RING_FREQ_W::new(self, 12)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn ring_freq_rdy(&mut self) -> RING_FREQ_RDY_W<PROC_MON_SPEC> {
        RING_FREQ_RDY_W::new(self, 28)
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
#[doc = "proc_mon.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`proc_mon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`proc_mon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PROC_MON_SPEC;
impl crate::RegisterSpec for PROC_MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`proc_mon::R`](R) reader structure"]
impl crate::Readable for PROC_MON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`proc_mon::W`](W) writer structure"]
impl crate::Writable for PROC_MON_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets proc_mon to value 0"]
impl crate::Resettable for PROC_MON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
