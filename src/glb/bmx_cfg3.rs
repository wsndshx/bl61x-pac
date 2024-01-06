#[doc = "Register `bmx_cfg3` reader"]
pub type R = crate::R<BMX_CFG3_SPEC>;
#[doc = "Register `bmx_cfg3` writer"]
pub type W = crate::W<BMX_CFG3_SPEC>;
#[doc = "Field `reg_bmx_berr_clr` reader - "]
pub type REG_BMX_BERR_CLR_R = crate::BitReader;
#[doc = "Field `reg_bmx_berr_clr` writer - "]
pub type REG_BMX_BERR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_bmx_berr_last` reader - "]
pub type REG_BMX_BERR_LAST_R = crate::BitReader;
#[doc = "Field `reg_bmx_berr_last` writer - "]
pub type REG_BMX_BERR_LAST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mcu_berr_clr` reader - "]
pub type REG_MCU_BERR_CLR_R = crate::BitReader;
#[doc = "Field `reg_mcu_berr_clr` writer - "]
pub type REG_MCU_BERR_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mcu_berr_last` reader - "]
pub type REG_MCU_BERR_LAST_R = crate::BitReader;
#[doc = "Field `reg_mcu_berr_last` writer - "]
pub type REG_MCU_BERR_LAST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sts_bmx_berr` reader - "]
pub type STS_BMX_BERR_R = crate::BitReader;
#[doc = "Field `sts_bmx_berr` writer - "]
pub type STS_BMX_BERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sts_mcu_berr` reader - "]
pub type STS_MCU_BERR_R = crate::BitReader;
#[doc = "Field `sts_mcu_berr` writer - "]
pub type STS_MCU_BERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sts_bmx_berr_write` reader - "]
pub type STS_BMX_BERR_WRITE_R = crate::BitReader;
#[doc = "Field `sts_bmx_berr_write` writer - "]
pub type STS_BMX_BERR_WRITE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sts_mcu_berr_write` reader - "]
pub type STS_MCU_BERR_WRITE_R = crate::BitReader;
#[doc = "Field `sts_mcu_berr_write` writer - "]
pub type STS_MCU_BERR_WRITE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_bmx_berr_clr(&self) -> REG_BMX_BERR_CLR_R {
        REG_BMX_BERR_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_bmx_berr_last(&self) -> REG_BMX_BERR_LAST_R {
        REG_BMX_BERR_LAST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_mcu_berr_clr(&self) -> REG_MCU_BERR_CLR_R {
        REG_MCU_BERR_CLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_mcu_berr_last(&self) -> REG_MCU_BERR_LAST_R {
        REG_MCU_BERR_LAST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sts_bmx_berr(&self) -> STS_BMX_BERR_R {
        STS_BMX_BERR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sts_mcu_berr(&self) -> STS_MCU_BERR_R {
        STS_MCU_BERR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sts_bmx_berr_write(&self) -> STS_BMX_BERR_WRITE_R {
        STS_BMX_BERR_WRITE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sts_mcu_berr_write(&self) -> STS_MCU_BERR_WRITE_R {
        STS_MCU_BERR_WRITE_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_berr_clr(&mut self) -> REG_BMX_BERR_CLR_W<BMX_CFG3_SPEC> {
        REG_BMX_BERR_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_berr_last(&mut self) -> REG_BMX_BERR_LAST_W<BMX_CFG3_SPEC> {
        REG_BMX_BERR_LAST_W::new(self, 1)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu_berr_clr(&mut self) -> REG_MCU_BERR_CLR_W<BMX_CFG3_SPEC> {
        REG_MCU_BERR_CLR_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu_berr_last(&mut self) -> REG_MCU_BERR_LAST_W<BMX_CFG3_SPEC> {
        REG_MCU_BERR_LAST_W::new(self, 9)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn sts_bmx_berr(&mut self) -> STS_BMX_BERR_W<BMX_CFG3_SPEC> {
        STS_BMX_BERR_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn sts_mcu_berr(&mut self) -> STS_MCU_BERR_W<BMX_CFG3_SPEC> {
        STS_MCU_BERR_W::new(self, 17)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn sts_bmx_berr_write(&mut self) -> STS_BMX_BERR_WRITE_W<BMX_CFG3_SPEC> {
        STS_BMX_BERR_WRITE_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn sts_mcu_berr_write(&mut self) -> STS_MCU_BERR_WRITE_W<BMX_CFG3_SPEC> {
        STS_MCU_BERR_WRITE_W::new(self, 25)
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
#[doc = "bmx_cfg3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmx_cfg3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmx_cfg3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BMX_CFG3_SPEC;
impl crate::RegisterSpec for BMX_CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmx_cfg3::R`](R) reader structure"]
impl crate::Readable for BMX_CFG3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bmx_cfg3::W`](W) writer structure"]
impl crate::Writable for BMX_CFG3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets bmx_cfg3 to value 0"]
impl crate::Resettable for BMX_CFG3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
