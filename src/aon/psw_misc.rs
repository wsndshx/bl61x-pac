#[doc = "Register `psw_misc` reader"]
pub type R = crate::R<PSW_MISC_SPEC>;
#[doc = "Register `psw_misc` writer"]
pub type W = crate::W<PSW_MISC_SPEC>;
#[doc = "Field `pu_psw_irrcv_aon` reader - "]
pub type PU_PSW_IRRCV_AON_R = crate::BitReader;
#[doc = "Field `pu_psw_irrcv_aon` writer - "]
pub type PU_PSW_IRRCV_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `usb20_rref_ext_en_aon` reader - "]
pub type USB20_RREF_EXT_EN_AON_R = crate::BitReader;
#[doc = "Field `usb20_rref_ext_en_aon` writer - "]
pub type USB20_RREF_EXT_EN_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `en_por33_aon` reader - "]
pub type EN_POR33_AON_R = crate::BitReader;
#[doc = "Field `en_por33_aon` writer - "]
pub type EN_POR33_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `usb20_rref_hiz_aon` reader - "]
pub type USB20_RREF_HIZ_AON_R = crate::BitReader;
#[doc = "Field `usb20_rref_hiz_aon` writer - "]
pub type USB20_RREF_HIZ_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `usb20_rcal_code_aon` reader - "]
pub type USB20_RCAL_CODE_AON_R = crate::FieldReader;
#[doc = "Field `usb20_rcal_code_aon` writer - "]
pub type USB20_RCAL_CODE_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_psw_irrcv_aon(&self) -> PU_PSW_IRRCV_AON_R {
        PU_PSW_IRRCV_AON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn usb20_rref_ext_en_aon(&self) -> USB20_RREF_EXT_EN_AON_R {
        USB20_RREF_EXT_EN_AON_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn en_por33_aon(&self) -> EN_POR33_AON_R {
        EN_POR33_AON_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn usb20_rref_hiz_aon(&self) -> USB20_RREF_HIZ_AON_R {
        USB20_RREF_HIZ_AON_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn usb20_rcal_code_aon(&self) -> USB20_RCAL_CODE_AON_R {
        USB20_RCAL_CODE_AON_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pu_psw_irrcv_aon(&mut self) -> PU_PSW_IRRCV_AON_W<PSW_MISC_SPEC> {
        PU_PSW_IRRCV_AON_W::new(self, 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn usb20_rref_ext_en_aon(&mut self) -> USB20_RREF_EXT_EN_AON_W<PSW_MISC_SPEC> {
        USB20_RREF_EXT_EN_AON_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn en_por33_aon(&mut self) -> EN_POR33_AON_W<PSW_MISC_SPEC> {
        EN_POR33_AON_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn usb20_rref_hiz_aon(&mut self) -> USB20_RREF_HIZ_AON_W<PSW_MISC_SPEC> {
        USB20_RREF_HIZ_AON_W::new(self, 21)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    #[must_use]
    pub fn usb20_rcal_code_aon(&mut self) -> USB20_RCAL_CODE_AON_W<PSW_MISC_SPEC> {
        USB20_RCAL_CODE_AON_W::new(self, 24)
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
#[doc = "psw_misc.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psw_misc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psw_misc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSW_MISC_SPEC;
impl crate::RegisterSpec for PSW_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psw_misc::R`](R) reader structure"]
impl crate::Readable for PSW_MISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psw_misc::W`](W) writer structure"]
impl crate::Writable for PSW_MISC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets psw_misc to value 0"]
impl crate::Resettable for PSW_MISC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
