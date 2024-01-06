#[doc = "Register `PDS_RAM3` reader"]
pub type R = crate::R<PDS_RAM3_SPEC>;
#[doc = "Register `PDS_RAM3` writer"]
pub type W = crate::W<PDS_RAM3_SPEC>;
#[doc = "Field `cr_ocram_ret` reader - "]
pub type CR_OCRAM_RET_R = crate::FieldReader<u32>;
#[doc = "Field `cr_ocram_ret` writer - "]
pub type CR_OCRAM_RET_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn cr_ocram_ret(&self) -> CR_OCRAM_RET_R {
        CR_OCRAM_RET_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    #[must_use]
    pub fn cr_ocram_ret(&mut self) -> CR_OCRAM_RET_W<PDS_RAM3_SPEC> {
        CR_OCRAM_RET_W::new(self, 0)
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
#[doc = "PDS_RAM3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pds_ram3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pds_ram3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDS_RAM3_SPEC;
impl crate::RegisterSpec for PDS_RAM3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pds_ram3::R`](R) reader structure"]
impl crate::Readable for PDS_RAM3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pds_ram3::W`](W) writer structure"]
impl crate::Writable for PDS_RAM3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDS_RAM3 to value 0"]
impl crate::Resettable for PDS_RAM3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
