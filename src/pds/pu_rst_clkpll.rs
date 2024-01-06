#[doc = "Register `pu_rst_clkpll` reader"]
pub type R = crate::R<PU_RST_CLKPLL_SPEC>;
#[doc = "Register `pu_rst_clkpll` writer"]
pub type W = crate::W<PU_RST_CLKPLL_SPEC>;
#[doc = "Field `cr_pds_pu_clkpll_sfreg` reader - "]
pub type CR_PDS_PU_CLKPLL_SFREG_R = crate::BitReader;
#[doc = "Field `cr_pds_pu_clkpll_sfreg` writer - "]
pub type CR_PDS_PU_CLKPLL_SFREG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_pds_pu_clkpll` reader - "]
pub type CR_PDS_PU_CLKPLL_R = crate::BitReader;
#[doc = "Field `cr_pds_pu_clkpll` writer - "]
pub type CR_PDS_PU_CLKPLL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pds_pu_clkpll_sfreg(&self) -> CR_PDS_PU_CLKPLL_SFREG_R {
        CR_PDS_PU_CLKPLL_SFREG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_pu_clkpll(&self) -> CR_PDS_PU_CLKPLL_R {
        CR_PDS_PU_CLKPLL_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_pu_clkpll_sfreg(&mut self) -> CR_PDS_PU_CLKPLL_SFREG_W<PU_RST_CLKPLL_SPEC> {
        CR_PDS_PU_CLKPLL_SFREG_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_pu_clkpll(&mut self) -> CR_PDS_PU_CLKPLL_W<PU_RST_CLKPLL_SPEC> {
        CR_PDS_PU_CLKPLL_W::new(self, 10)
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
#[doc = "pu_rst_clkpll.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pu_rst_clkpll::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pu_rst_clkpll::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PU_RST_CLKPLL_SPEC;
impl crate::RegisterSpec for PU_RST_CLKPLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pu_rst_clkpll::R`](R) reader structure"]
impl crate::Readable for PU_RST_CLKPLL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pu_rst_clkpll::W`](W) writer structure"]
impl crate::Writable for PU_RST_CLKPLL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pu_rst_clkpll to value 0"]
impl crate::Resettable for PU_RST_CLKPLL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
