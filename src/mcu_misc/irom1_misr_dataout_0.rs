#[doc = "Register `irom1_misr_dataout_0` reader"]
pub type R = crate::R<IROM1_MISR_DATAOUT_0_SPEC>;
#[doc = "Register `irom1_misr_dataout_0` writer"]
pub type W = crate::W<IROM1_MISR_DATAOUT_0_SPEC>;
#[doc = "Field `irom_misr_dataout_0` reader - IROM MISR data out 0. This field contains the MISR data output for IROM1."]
pub type IROM_MISR_DATAOUT_0_R = crate::FieldReader<u32>;
#[doc = "Field `irom_misr_dataout_0` writer - IROM MISR data out 0. This field contains the MISR data output for IROM1."]
pub type IROM_MISR_DATAOUT_0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - IROM MISR data out 0. This field contains the MISR data output for IROM1."]
    #[inline(always)]
    pub fn irom_misr_dataout_0(&self) -> IROM_MISR_DATAOUT_0_R {
        IROM_MISR_DATAOUT_0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IROM MISR data out 0. This field contains the MISR data output for IROM1."]
    #[inline(always)]
    #[must_use]
    pub fn irom_misr_dataout_0(&mut self) -> IROM_MISR_DATAOUT_0_W<IROM1_MISR_DATAOUT_0_SPEC> {
        IROM_MISR_DATAOUT_0_W::new(self, 0)
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
#[doc = "IROM1 MISR Data Out 0 Register. Stores the MISR data output for IROM1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irom1_misr_dataout_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irom1_misr_dataout_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IROM1_MISR_DATAOUT_0_SPEC;
impl crate::RegisterSpec for IROM1_MISR_DATAOUT_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irom1_misr_dataout_0::R`](R) reader structure"]
impl crate::Readable for IROM1_MISR_DATAOUT_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`irom1_misr_dataout_0::W`](W) writer structure"]
impl crate::Writable for IROM1_MISR_DATAOUT_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets irom1_misr_dataout_0 to value 0"]
impl crate::Resettable for IROM1_MISR_DATAOUT_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
