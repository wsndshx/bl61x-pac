#[doc = "Register `HBN_PAD_CTRL_1` reader"]
pub type R = crate::R<HBN_PAD_CTRL_1_SPEC>;
#[doc = "Register `HBN_PAD_CTRL_1` writer"]
pub type W = crate::W<HBN_PAD_CTRL_1_SPEC>;
#[doc = "Field `reg_aon_pad_oe` reader - "]
pub type REG_AON_PAD_OE_R = crate::FieldReader;
#[doc = "Field `reg_aon_pad_oe` writer - "]
pub type REG_AON_PAD_OE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_aon_pad_pd` reader - "]
pub type REG_AON_PAD_PD_R = crate::FieldReader;
#[doc = "Field `reg_aon_pad_pd` writer - "]
pub type REG_AON_PAD_PD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_aon_pad_pu` reader - "]
pub type REG_AON_PAD_PU_R = crate::FieldReader;
#[doc = "Field `reg_aon_pad_pu` writer - "]
pub type REG_AON_PAD_PU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn reg_aon_pad_oe(&self) -> REG_AON_PAD_OE_R {
        REG_AON_PAD_OE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 10:13"]
    #[inline(always)]
    pub fn reg_aon_pad_pd(&self) -> REG_AON_PAD_PD_R {
        REG_AON_PAD_PD_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn reg_aon_pad_pu(&self) -> REG_AON_PAD_PU_R {
        REG_AON_PAD_PU_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn reg_aon_pad_oe(&mut self) -> REG_AON_PAD_OE_W<HBN_PAD_CTRL_1_SPEC> {
        REG_AON_PAD_OE_W::new(self, 0)
    }
    #[doc = "Bits 10:13"]
    #[inline(always)]
    #[must_use]
    pub fn reg_aon_pad_pd(&mut self) -> REG_AON_PAD_PD_W<HBN_PAD_CTRL_1_SPEC> {
        REG_AON_PAD_PD_W::new(self, 10)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn reg_aon_pad_pu(&mut self) -> REG_AON_PAD_PU_W<HBN_PAD_CTRL_1_SPEC> {
        REG_AON_PAD_PU_W::new(self, 20)
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
#[doc = "HBN_PAD_CTRL_1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hbn_pad_ctrl_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hbn_pad_ctrl_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HBN_PAD_CTRL_1_SPEC;
impl crate::RegisterSpec for HBN_PAD_CTRL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hbn_pad_ctrl_1::R`](R) reader structure"]
impl crate::Readable for HBN_PAD_CTRL_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hbn_pad_ctrl_1::W`](W) writer structure"]
impl crate::Writable for HBN_PAD_CTRL_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HBN_PAD_CTRL_1 to value 0"]
impl crate::Resettable for HBN_PAD_CTRL_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
