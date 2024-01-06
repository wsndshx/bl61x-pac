#[doc = "Register `psw_irrcv` reader"]
pub type R = crate::R<PSW_IRRCV_SPEC>;
#[doc = "Register `psw_irrcv` writer"]
pub type W = crate::W<PSW_IRRCV_SPEC>;
#[doc = "Field `pu_ir_psw_aon` reader - "]
pub type PU_IR_PSW_AON_R = crate::BitReader;
#[doc = "Field `pu_ir_psw_aon` writer - "]
pub type PU_IR_PSW_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ir_psw_aon(&self) -> PU_IR_PSW_AON_R {
        PU_IR_PSW_AON_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pu_ir_psw_aon(&mut self) -> PU_IR_PSW_AON_W<PSW_IRRCV_SPEC> {
        PU_IR_PSW_AON_W::new(self, 0)
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
#[doc = "psw_irrcv.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psw_irrcv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psw_irrcv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSW_IRRCV_SPEC;
impl crate::RegisterSpec for PSW_IRRCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psw_irrcv::R`](R) reader structure"]
impl crate::Readable for PSW_IRRCV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psw_irrcv::W`](W) writer structure"]
impl crate::Writable for PSW_IRRCV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets psw_irrcv to value 0"]
impl crate::Resettable for PSW_IRRCV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
