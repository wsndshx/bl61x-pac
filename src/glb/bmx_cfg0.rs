#[doc = "Register `bmx_cfg0` reader"]
pub type R = crate::R<BMX_CFG0_SPEC>;
#[doc = "Register `bmx_cfg0` writer"]
pub type W = crate::W<BMX_CFG0_SPEC>;
#[doc = "Field `reg_bmx_timeout_en` reader - "]
pub type REG_BMX_TIMEOUT_EN_R = crate::FieldReader;
#[doc = "Field `reg_bmx_timeout_en` writer - "]
pub type REG_BMX_TIMEOUT_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_bmx_arb_mode` reader - "]
pub type REG_BMX_ARB_MODE_R = crate::BitReader;
#[doc = "Field `reg_bmx_arb_mode` writer - "]
pub type REG_BMX_ARB_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bmx_timeout_clr` reader - "]
pub type REG_BMX_TIMEOUT_CLR_R = crate::BitReader;
#[doc = "Field `reg_bmx_timeout_clr` writer - "]
pub type REG_BMX_TIMEOUT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sts_bmx_timeout_sts` reader - "]
pub type STS_BMX_TIMEOUT_STS_R = crate::FieldReader;
#[doc = "Field `sts_bmx_timeout_sts` writer - "]
pub type STS_BMX_TIMEOUT_STS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `pds_apb_cfg` reader - "]
pub type PDS_APB_CFG_R = crate::FieldReader;
#[doc = "Field `pds_apb_cfg` writer - "]
pub type PDS_APB_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `hbn_apb_cfg` reader - "]
pub type HBN_APB_CFG_R = crate::FieldReader;
#[doc = "Field `hbn_apb_cfg` writer - "]
pub type HBN_APB_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn reg_bmx_timeout_en(&self) -> REG_BMX_TIMEOUT_EN_R {
        REG_BMX_TIMEOUT_EN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_bmx_arb_mode(&self) -> REG_BMX_ARB_MODE_R {
        REG_BMX_ARB_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_bmx_timeout_clr(&self) -> REG_BMX_TIMEOUT_CLR_R {
        REG_BMX_TIMEOUT_CLR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    pub fn sts_bmx_timeout_sts(&self) -> STS_BMX_TIMEOUT_STS_R {
        STS_BMX_TIMEOUT_STS_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn pds_apb_cfg(&self) -> PDS_APB_CFG_R {
        PDS_APB_CFG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn hbn_apb_cfg(&self) -> HBN_APB_CFG_R {
        HBN_APB_CFG_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_timeout_en(&mut self) -> REG_BMX_TIMEOUT_EN_W<BMX_CFG0_SPEC> {
        REG_BMX_TIMEOUT_EN_W::new(self, 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_arb_mode(&mut self) -> REG_BMX_ARB_MODE_W<BMX_CFG0_SPEC> {
        REG_BMX_ARB_MODE_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_timeout_clr(&mut self) -> REG_BMX_TIMEOUT_CLR_W<BMX_CFG0_SPEC> {
        REG_BMX_TIMEOUT_CLR_W::new(self, 6)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    #[must_use]
    pub fn sts_bmx_timeout_sts(&mut self) -> STS_BMX_TIMEOUT_STS_W<BMX_CFG0_SPEC> {
        STS_BMX_TIMEOUT_STS_W::new(self, 11)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn pds_apb_cfg(&mut self) -> PDS_APB_CFG_W<BMX_CFG0_SPEC> {
        PDS_APB_CFG_W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_apb_cfg(&mut self) -> HBN_APB_CFG_W<BMX_CFG0_SPEC> {
        HBN_APB_CFG_W::new(self, 24)
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
#[doc = "bmx_cfg0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmx_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmx_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BMX_CFG0_SPEC;
impl crate::RegisterSpec for BMX_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmx_cfg0::R`](R) reader structure"]
impl crate::Readable for BMX_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bmx_cfg0::W`](W) writer structure"]
impl crate::Writable for BMX_CFG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets bmx_cfg0 to value 0"]
impl crate::Resettable for BMX_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
