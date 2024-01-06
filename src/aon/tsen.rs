#[doc = "Register `tsen` reader"]
pub type R = crate::R<TSEN_SPEC>;
#[doc = "Register `tsen` writer"]
pub type W = crate::W<TSEN_SPEC>;
#[doc = "Field `tsen_refcode_corner` reader - "]
pub type TSEN_REFCODE_CORNER_R = crate::FieldReader<u16>;
#[doc = "Field `tsen_refcode_corner` writer - "]
pub type TSEN_REFCODE_CORNER_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `tsen_refcode_rfcal` reader - "]
pub type TSEN_REFCODE_RFCAL_R = crate::FieldReader<u16>;
#[doc = "Field `tsen_refcode_rfcal` writer - "]
pub type TSEN_REFCODE_RFCAL_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `xtal_rdy` reader - "]
pub type XTAL_RDY_R = crate::BitReader;
#[doc = "Field `xtal_rdy` writer - "]
pub type XTAL_RDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xtal_inn_cfg_en_aon` reader - "]
pub type XTAL_INN_CFG_EN_AON_R = crate::BitReader;
#[doc = "Field `xtal_inn_cfg_en_aon` writer - "]
pub type XTAL_INN_CFG_EN_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xtal_rdy_int_sel_aon` reader - "]
pub type XTAL_RDY_INT_SEL_AON_R = crate::FieldReader;
#[doc = "Field `xtal_rdy_int_sel_aon` writer - "]
pub type XTAL_RDY_INT_SEL_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn tsen_refcode_corner(&self) -> TSEN_REFCODE_CORNER_R {
        TSEN_REFCODE_CORNER_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn tsen_refcode_rfcal(&self) -> TSEN_REFCODE_RFCAL_R {
        TSEN_REFCODE_RFCAL_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn xtal_rdy(&self) -> XTAL_RDY_R {
        XTAL_RDY_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn xtal_inn_cfg_en_aon(&self) -> XTAL_INN_CFG_EN_AON_R {
        XTAL_INN_CFG_EN_AON_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn xtal_rdy_int_sel_aon(&self) -> XTAL_RDY_INT_SEL_AON_R {
        XTAL_RDY_INT_SEL_AON_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn tsen_refcode_corner(&mut self) -> TSEN_REFCODE_CORNER_W<TSEN_SPEC> {
        TSEN_REFCODE_CORNER_W::new(self, 0)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    #[must_use]
    pub fn tsen_refcode_rfcal(&mut self) -> TSEN_REFCODE_RFCAL_W<TSEN_SPEC> {
        TSEN_REFCODE_RFCAL_W::new(self, 16)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_rdy(&mut self) -> XTAL_RDY_W<TSEN_SPEC> {
        XTAL_RDY_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_inn_cfg_en_aon(&mut self) -> XTAL_INN_CFG_EN_AON_W<TSEN_SPEC> {
        XTAL_INN_CFG_EN_AON_W::new(self, 29)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn xtal_rdy_int_sel_aon(&mut self) -> XTAL_RDY_INT_SEL_AON_W<TSEN_SPEC> {
        XTAL_RDY_INT_SEL_AON_W::new(self, 30)
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
#[doc = "tsen.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSEN_SPEC;
impl crate::RegisterSpec for TSEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsen::R`](R) reader structure"]
impl crate::Readable for TSEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tsen::W`](W) writer structure"]
impl crate::Writable for TSEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tsen to value 0"]
impl crate::Resettable for TSEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
