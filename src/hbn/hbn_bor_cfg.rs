#[doc = "Register `HBN_BOR_CFG` reader"]
pub type R = crate::R<HBN_BOR_CFG_SPEC>;
#[doc = "Register `HBN_BOR_CFG` writer"]
pub type W = crate::W<HBN_BOR_CFG_SPEC>;
#[doc = "Field `bod_sel` reader - "]
pub type BOD_SEL_R = crate::BitReader;
#[doc = "Field `bod_sel` writer - "]
pub type BOD_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bod_vth` reader - "]
pub type BOD_VTH_R = crate::FieldReader;
#[doc = "Field `bod_vth` writer - "]
pub type BOD_VTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pu_bod` reader - "]
pub type PU_BOD_R = crate::BitReader;
#[doc = "Field `pu_bod` writer - "]
pub type PU_BOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_bod_out` reader - "]
pub type R_BOD_OUT_R = crate::BitReader;
#[doc = "Field `r_bod_out` writer - "]
pub type R_BOD_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bod_sel(&self) -> BOD_SEL_R {
        BOD_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn bod_vth(&self) -> BOD_VTH_R {
        BOD_VTH_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pu_bod(&self) -> PU_BOD_R {
        PU_BOD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn r_bod_out(&self) -> R_BOD_OUT_R {
        R_BOD_OUT_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn bod_sel(&mut self) -> BOD_SEL_W<HBN_BOR_CFG_SPEC> {
        BOD_SEL_W::new(self, 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    #[must_use]
    pub fn bod_vth(&mut self) -> BOD_VTH_W<HBN_BOR_CFG_SPEC> {
        BOD_VTH_W::new(self, 1)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pu_bod(&mut self) -> PU_BOD_W<HBN_BOR_CFG_SPEC> {
        PU_BOD_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn r_bod_out(&mut self) -> R_BOD_OUT_W<HBN_BOR_CFG_SPEC> {
        R_BOD_OUT_W::new(self, 5)
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
#[doc = "HBN_BOR_CFG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hbn_bor_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hbn_bor_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HBN_BOR_CFG_SPEC;
impl crate::RegisterSpec for HBN_BOR_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hbn_bor_cfg::R`](R) reader structure"]
impl crate::Readable for HBN_BOR_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hbn_bor_cfg::W`](W) writer structure"]
impl crate::Writable for HBN_BOR_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HBN_BOR_CFG to value 0"]
impl crate::Resettable for HBN_BOR_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
