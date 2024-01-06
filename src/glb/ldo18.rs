#[doc = "Register `ldo18` reader"]
pub type R = crate::R<LDO18_SPEC>;
#[doc = "Register `ldo18` writer"]
pub type W = crate::W<LDO18_SPEC>;
#[doc = "Field `pu_ldo18io` reader - "]
pub type PU_LDO18IO_R = crate::BitReader;
#[doc = "Field `pu_ldo18io` writer - "]
pub type PU_LDO18IO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo18io_bypass` reader - "]
pub type LDO18IO_BYPASS_R = crate::BitReader;
#[doc = "Field `ldo18io_bypass` writer - "]
pub type LDO18IO_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_ldo18io` reader - "]
pub type TEN_LDO18IO_R = crate::BitReader;
#[doc = "Field `ten_ldo18io` writer - "]
pub type TEN_LDO18IO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo18io_ocp_out` reader - "]
pub type LDO18IO_OCP_OUT_R = crate::BitReader;
#[doc = "Field `ldo18io_ocp_out` writer - "]
pub type LDO18IO_OCP_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo18io_bm` reader - "]
pub type LDO18IO_BM_R = crate::FieldReader;
#[doc = "Field `ldo18io_bm` writer - "]
pub type LDO18IO_BM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ldo18io_cc` reader - "]
pub type LDO18IO_CC_R = crate::FieldReader;
#[doc = "Field `ldo18io_cc` writer - "]
pub type LDO18IO_CC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ldo18io_ocp_th` reader - "]
pub type LDO18IO_OCP_TH_R = crate::FieldReader;
#[doc = "Field `ldo18io_ocp_th` writer - "]
pub type LDO18IO_OCP_TH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ldo18io_ocp_en` reader - "]
pub type LDO18IO_OCP_EN_R = crate::BitReader;
#[doc = "Field `ldo18io_ocp_en` writer - "]
pub type LDO18IO_OCP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo18io_pulldown` reader - "]
pub type LDO18IO_PULLDOWN_R = crate::BitReader;
#[doc = "Field `ldo18io_pulldown` writer - "]
pub type LDO18IO_PULLDOWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo18io_pulldown_sel` reader - "]
pub type LDO18IO_PULLDOWN_SEL_R = crate::BitReader;
#[doc = "Field `ldo18io_pulldown_sel` writer - "]
pub type LDO18IO_PULLDOWN_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo18io_sstart_delay` reader - "]
pub type LDO18IO_SSTART_DELAY_R = crate::FieldReader;
#[doc = "Field `ldo18io_sstart_delay` writer - "]
pub type LDO18IO_SSTART_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ldo18io_sstart_en` reader - "]
pub type LDO18IO_SSTART_EN_R = crate::BitReader;
#[doc = "Field `ldo18io_sstart_en` writer - "]
pub type LDO18IO_SSTART_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo18io_vout_sel` reader - "]
pub type LDO18IO_VOUT_SEL_R = crate::FieldReader;
#[doc = "Field `ldo18io_vout_sel` writer - "]
pub type LDO18IO_VOUT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ldo18io_vout_trim` reader - "]
pub type LDO18IO_VOUT_TRIM_R = crate::FieldReader;
#[doc = "Field `ldo18io_vout_trim` writer - "]
pub type LDO18IO_VOUT_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ldo18io(&self) -> PU_LDO18IO_R {
        PU_LDO18IO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ldo18io_bypass(&self) -> LDO18IO_BYPASS_R {
        LDO18IO_BYPASS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ten_ldo18io(&self) -> TEN_LDO18IO_R {
        TEN_LDO18IO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ldo18io_ocp_out(&self) -> LDO18IO_OCP_OUT_R {
        LDO18IO_OCP_OUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn ldo18io_bm(&self) -> LDO18IO_BM_R {
        LDO18IO_BM_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn ldo18io_cc(&self) -> LDO18IO_CC_R {
        LDO18IO_CC_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn ldo18io_ocp_th(&self) -> LDO18IO_OCP_TH_R {
        LDO18IO_OCP_TH_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ldo18io_ocp_en(&self) -> LDO18IO_OCP_EN_R {
        LDO18IO_OCP_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ldo18io_pulldown(&self) -> LDO18IO_PULLDOWN_R {
        LDO18IO_PULLDOWN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ldo18io_pulldown_sel(&self) -> LDO18IO_PULLDOWN_SEL_R {
        LDO18IO_PULLDOWN_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn ldo18io_sstart_delay(&self) -> LDO18IO_SSTART_DELAY_R {
        LDO18IO_SSTART_DELAY_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ldo18io_sstart_en(&self) -> LDO18IO_SSTART_EN_R {
        LDO18IO_SSTART_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn ldo18io_vout_sel(&self) -> LDO18IO_VOUT_SEL_R {
        LDO18IO_VOUT_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn ldo18io_vout_trim(&self) -> LDO18IO_VOUT_TRIM_R {
        LDO18IO_VOUT_TRIM_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pu_ldo18io(&mut self) -> PU_LDO18IO_W<LDO18_SPEC> {
        PU_LDO18IO_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18io_bypass(&mut self) -> LDO18IO_BYPASS_W<LDO18_SPEC> {
        LDO18IO_BYPASS_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ten_ldo18io(&mut self) -> TEN_LDO18IO_W<LDO18_SPEC> {
        TEN_LDO18IO_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18io_ocp_out(&mut self) -> LDO18IO_OCP_OUT_W<LDO18_SPEC> {
        LDO18IO_OCP_OUT_W::new(self, 3)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18io_bm(&mut self) -> LDO18IO_BM_W<LDO18_SPEC> {
        LDO18IO_BM_W::new(self, 4)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18io_cc(&mut self) -> LDO18IO_CC_W<LDO18_SPEC> {
        LDO18IO_CC_W::new(self, 8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18io_ocp_th(&mut self) -> LDO18IO_OCP_TH_W<LDO18_SPEC> {
        LDO18IO_OCP_TH_W::new(self, 12)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18io_ocp_en(&mut self) -> LDO18IO_OCP_EN_W<LDO18_SPEC> {
        LDO18IO_OCP_EN_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18io_pulldown(&mut self) -> LDO18IO_PULLDOWN_W<LDO18_SPEC> {
        LDO18IO_PULLDOWN_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18io_pulldown_sel(&mut self) -> LDO18IO_PULLDOWN_SEL_W<LDO18_SPEC> {
        LDO18IO_PULLDOWN_SEL_W::new(self, 17)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18io_sstart_delay(&mut self) -> LDO18IO_SSTART_DELAY_W<LDO18_SPEC> {
        LDO18IO_SSTART_DELAY_W::new(self, 20)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18io_sstart_en(&mut self) -> LDO18IO_SSTART_EN_W<LDO18_SPEC> {
        LDO18IO_SSTART_EN_W::new(self, 23)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18io_vout_sel(&mut self) -> LDO18IO_VOUT_SEL_W<LDO18_SPEC> {
        LDO18IO_VOUT_SEL_W::new(self, 24)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn ldo18io_vout_trim(&mut self) -> LDO18IO_VOUT_TRIM_W<LDO18_SPEC> {
        LDO18IO_VOUT_TRIM_W::new(self, 28)
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
#[doc = "1.8-V Low Dropout Linear Regulator configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ldo18::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ldo18::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LDO18_SPEC;
impl crate::RegisterSpec for LDO18_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ldo18::R`](R) reader structure"]
impl crate::Readable for LDO18_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ldo18::W`](W) writer structure"]
impl crate::Writable for LDO18_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ldo18 to value 0"]
impl crate::Resettable for LDO18_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
