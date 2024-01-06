#[doc = "Register `hw_rsv3` reader"]
pub type R = crate::R<HW_RSV3_SPEC>;
#[doc = "Register `hw_rsv3` writer"]
pub type W = crate::W<HW_RSV3_SPEC>;
#[doc = "Field `rsvd_31_0` reader - "]
pub type RSVD_31_0_R = crate::FieldReader<u32>;
#[doc = "Field `rsvd_31_0` writer - "]
pub type RSVD_31_0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rsvd_31_0(&self) -> RSVD_31_0_R {
        RSVD_31_0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_31_0(&mut self) -> RSVD_31_0_W<HW_RSV3_SPEC> {
        RSVD_31_0_W::new(self, 0)
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
#[doc = "hw_rsv3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hw_rsv3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hw_rsv3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HW_RSV3_SPEC;
impl crate::RegisterSpec for HW_RSV3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_rsv3::R`](R) reader structure"]
impl crate::Readable for HW_RSV3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hw_rsv3::W`](W) writer structure"]
impl crate::Writable for HW_RSV3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hw_rsv3 to value 0"]
impl crate::Resettable for HW_RSV3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
