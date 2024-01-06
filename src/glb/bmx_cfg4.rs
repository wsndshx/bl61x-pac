#[doc = "Register `bmx_cfg4` reader"]
pub type R = crate::R<BMX_CFG4_SPEC>;
#[doc = "Register `bmx_cfg4` writer"]
pub type W = crate::W<BMX_CFG4_SPEC>;
#[doc = "Field `sts_bmx_berr_src` reader - "]
pub type STS_BMX_BERR_SRC_R = crate::FieldReader<u16>;
#[doc = "Field `sts_bmx_berr_src` writer - "]
pub type STS_BMX_BERR_SRC_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `sts_mcu_berr_src` reader - "]
pub type STS_MCU_BERR_SRC_R = crate::BitReader;
#[doc = "Field `sts_mcu_berr_src` writer - "]
pub type STS_MCU_BERR_SRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sts_mcu_berr_id` reader - "]
pub type STS_MCU_BERR_ID_R = crate::FieldReader;
#[doc = "Field `sts_mcu_berr_id` writer - "]
pub type STS_MCU_BERR_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn sts_bmx_berr_src(&self) -> STS_BMX_BERR_SRC_R {
        STS_BMX_BERR_SRC_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sts_mcu_berr_src(&self) -> STS_MCU_BERR_SRC_R {
        STS_MCU_BERR_SRC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sts_mcu_berr_id(&self) -> STS_MCU_BERR_ID_R {
        STS_MCU_BERR_ID_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn sts_bmx_berr_src(&mut self) -> STS_BMX_BERR_SRC_W<BMX_CFG4_SPEC> {
        STS_BMX_BERR_SRC_W::new(self, 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn sts_mcu_berr_src(&mut self) -> STS_MCU_BERR_SRC_W<BMX_CFG4_SPEC> {
        STS_MCU_BERR_SRC_W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn sts_mcu_berr_id(&mut self) -> STS_MCU_BERR_ID_W<BMX_CFG4_SPEC> {
        STS_MCU_BERR_ID_W::new(self, 24)
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
#[doc = "bmx_cfg4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmx_cfg4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmx_cfg4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BMX_CFG4_SPEC;
impl crate::RegisterSpec for BMX_CFG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmx_cfg4::R`](R) reader structure"]
impl crate::Readable for BMX_CFG4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bmx_cfg4::W`](W) writer structure"]
impl crate::Writable for BMX_CFG4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets bmx_cfg4 to value 0"]
impl crate::Resettable for BMX_CFG4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
