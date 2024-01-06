#[doc = "Register `bmx_cfg2` reader"]
pub type R = crate::R<BMX_CFG2_SPEC>;
#[doc = "Register `bmx_cfg2` writer"]
pub type W = crate::W<BMX_CFG2_SPEC>;
#[doc = "Field `reg_bmx_berr_en` reader - "]
pub type REG_BMX_BERR_EN_R = crate::FieldReader<u16>;
#[doc = "Field `reg_bmx_berr_en` writer - "]
pub type REG_BMX_BERR_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `reg_mcu_berr_en` reader - "]
pub type REG_MCU_BERR_EN_R = crate::BitReader;
#[doc = "Field `reg_mcu_berr_en` writer - "]
pub type REG_MCU_BERR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn reg_bmx_berr_en(&self) -> REG_BMX_BERR_EN_R {
        REG_BMX_BERR_EN_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_mcu_berr_en(&self) -> REG_MCU_BERR_EN_R {
        REG_MCU_BERR_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_berr_en(&mut self) -> REG_BMX_BERR_EN_W<BMX_CFG2_SPEC> {
        REG_BMX_BERR_EN_W::new(self, 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu_berr_en(&mut self) -> REG_MCU_BERR_EN_W<BMX_CFG2_SPEC> {
        REG_MCU_BERR_EN_W::new(self, 16)
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
#[doc = "bmx_cfg2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmx_cfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmx_cfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BMX_CFG2_SPEC;
impl crate::RegisterSpec for BMX_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmx_cfg2::R`](R) reader structure"]
impl crate::Readable for BMX_CFG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bmx_cfg2::W`](W) writer structure"]
impl crate::Writable for BMX_CFG2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets bmx_cfg2 to value 0"]
impl crate::Resettable for BMX_CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
