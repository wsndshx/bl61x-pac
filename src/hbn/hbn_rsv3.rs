#[doc = "Register `HBN_RSV3` reader"]
pub type R = crate::R<HBN_RSV3_SPEC>;
#[doc = "Register `HBN_RSV3` writer"]
pub type W = crate::W<HBN_RSV3_SPEC>;
#[doc = "Field `HBN_RSV3` reader - "]
pub type HBN_RSV3_R = crate::FieldReader<u32>;
#[doc = "Field `HBN_RSV3` writer - "]
pub type HBN_RSV3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hbn_rsv3(&self) -> HBN_RSV3_R {
        HBN_RSV3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_rsv3(&mut self) -> HBN_RSV3_W<HBN_RSV3_SPEC> {
        HBN_RSV3_W::new(self, 0)
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
#[doc = "HBN_RSV3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hbn_rsv3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hbn_rsv3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HBN_RSV3_SPEC;
impl crate::RegisterSpec for HBN_RSV3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hbn_rsv3::R`](R) reader structure"]
impl crate::Readable for HBN_RSV3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hbn_rsv3::W`](W) writer structure"]
impl crate::Writable for HBN_RSV3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HBN_RSV3 to value 0"]
impl crate::Resettable for HBN_RSV3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
