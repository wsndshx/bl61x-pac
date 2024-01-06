#[doc = "Register `chip_inform` reader"]
pub type R = crate::R<CHIP_INFORM_SPEC>;
#[doc = "Register `chip_inform` writer"]
pub type W = crate::W<CHIP_INFORM_SPEC>;
#[doc = "Field `chip_rdy` reader - Chip ready signal"]
pub type CHIP_RDY_R = crate::BitReader;
#[doc = "Field `chip_rdy` writer - Chip ready signal"]
pub type CHIP_RDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `glb_id` reader - "]
pub type GLB_ID_R = crate::FieldReader;
#[doc = "Field `glb_id` writer - "]
pub type GLB_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 27 - Chip ready signal"]
    #[inline(always)]
    pub fn chip_rdy(&self) -> CHIP_RDY_R {
        CHIP_RDY_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn glb_id(&self) -> GLB_ID_R {
        GLB_ID_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 27 - Chip ready signal"]
    #[inline(always)]
    #[must_use]
    pub fn chip_rdy(&mut self) -> CHIP_RDY_W<CHIP_INFORM_SPEC> {
        CHIP_RDY_W::new(self, 27)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn glb_id(&mut self) -> GLB_ID_W<CHIP_INFORM_SPEC> {
        GLB_ID_W::new(self, 28)
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
#[doc = "Chip information register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chip_inform::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chip_inform::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHIP_INFORM_SPEC;
impl crate::RegisterSpec for CHIP_INFORM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chip_inform::R`](R) reader structure"]
impl crate::Readable for CHIP_INFORM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chip_inform::W`](W) writer structure"]
impl crate::Writable for CHIP_INFORM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets chip_inform to value 0"]
impl crate::Resettable for CHIP_INFORM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
