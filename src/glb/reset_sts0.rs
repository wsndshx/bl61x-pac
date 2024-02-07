#[doc = "Register `reset_sts0` reader"]
pub type R = crate::R<RESET_STS0_SPEC>;
#[doc = "Register `reset_sts0` writer"]
pub type W = crate::W<RESET_STS0_SPEC>;
#[doc = "Field `top_reset_recorder` reader - "]
pub type TOP_RESET_RECORDER_R = crate::FieldReader;
#[doc = "Field `top_reset_recorder` writer - "]
pub type TOP_RESET_RECORDER_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `clr_top_reset_recorder` reader - "]
pub type CLR_TOP_RESET_RECORDER_R = crate::BitReader;
#[doc = "Field `clr_top_reset_recorder` writer - "]
pub type CLR_TOP_RESET_RECORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn top_reset_recorder(&self) -> TOP_RESET_RECORDER_R {
        TOP_RESET_RECORDER_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clr_top_reset_recorder(&self) -> CLR_TOP_RESET_RECORDER_R {
        CLR_TOP_RESET_RECORDER_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn top_reset_recorder(&mut self) -> TOP_RESET_RECORDER_W<RESET_STS0_SPEC> {
        TOP_RESET_RECORDER_W::new(self, 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn clr_top_reset_recorder(&mut self) -> CLR_TOP_RESET_RECORDER_W<RESET_STS0_SPEC> {
        CLR_TOP_RESET_RECORDER_W::new(self, 7)
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
#[doc = "reset_sts0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_sts0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_sts0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESET_STS0_SPEC;
impl crate::RegisterSpec for RESET_STS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset_sts0::R`](R) reader structure"]
impl crate::Readable for RESET_STS0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reset_sts0::W`](W) writer structure"]
impl crate::Writable for RESET_STS0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets reset_sts0 to value 0"]
impl crate::Resettable for RESET_STS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
