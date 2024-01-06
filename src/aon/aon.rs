#[doc = "Register `aon` reader"]
pub type R = crate::R<AON_SPEC>;
#[doc = "Register `aon` writer"]
pub type W = crate::W<AON_SPEC>;
#[doc = "Field `aon_resv` reader - "]
pub type AON_RESV_R = crate::FieldReader;
#[doc = "Field `aon_resv` writer - "]
pub type AON_RESV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `pu_aon_dc_tbuf` reader - "]
pub type PU_AON_DC_TBUF_R = crate::BitReader;
#[doc = "Field `pu_aon_dc_tbuf` writer - "]
pub type PU_AON_DC_TBUF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo11_rt_pulldown` reader - "]
pub type LDO11_RT_PULLDOWN_R = crate::BitReader;
#[doc = "Field `ldo11_rt_pulldown` writer - "]
pub type LDO11_RT_PULLDOWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo11_rt_pulldown_sel` reader - "]
pub type LDO11_RT_PULLDOWN_SEL_R = crate::BitReader;
#[doc = "Field `ldo11_rt_pulldown_sel` writer - "]
pub type LDO11_RT_PULLDOWN_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sw_pu_ldo11_rt` reader - "]
pub type SW_PU_LDO11_RT_R = crate::BitReader;
#[doc = "Field `sw_pu_ldo11_rt` writer - "]
pub type SW_PU_LDO11_RT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn aon_resv(&self) -> AON_RESV_R {
        AON_RESV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pu_aon_dc_tbuf(&self) -> PU_AON_DC_TBUF_R {
        PU_AON_DC_TBUF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ldo11_rt_pulldown(&self) -> LDO11_RT_PULLDOWN_R {
        LDO11_RT_PULLDOWN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ldo11_rt_pulldown_sel(&self) -> LDO11_RT_PULLDOWN_SEL_R {
        LDO11_RT_PULLDOWN_SEL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sw_pu_ldo11_rt(&self) -> SW_PU_LDO11_RT_R {
        SW_PU_LDO11_RT_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn aon_resv(&mut self) -> AON_RESV_W<AON_SPEC> {
        AON_RESV_W::new(self, 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn pu_aon_dc_tbuf(&mut self) -> PU_AON_DC_TBUF_W<AON_SPEC> {
        PU_AON_DC_TBUF_W::new(self, 12)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn ldo11_rt_pulldown(&mut self) -> LDO11_RT_PULLDOWN_W<AON_SPEC> {
        LDO11_RT_PULLDOWN_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn ldo11_rt_pulldown_sel(&mut self) -> LDO11_RT_PULLDOWN_SEL_W<AON_SPEC> {
        LDO11_RT_PULLDOWN_SEL_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pu_ldo11_rt(&mut self) -> SW_PU_LDO11_RT_W<AON_SPEC> {
        SW_PU_LDO11_RT_W::new(self, 22)
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
#[doc = "aon.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AON_SPEC;
impl crate::RegisterSpec for AON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon::R`](R) reader structure"]
impl crate::Readable for AON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aon::W`](W) writer structure"]
impl crate::Writable for AON_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets aon to value 0"]
impl crate::Resettable for AON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
