#[doc = "Register `HBN_RSV2` reader"]
pub type R = crate::R<HBN_RSV2_SPEC>;
#[doc = "Register `HBN_RSV2` writer"]
pub type W = crate::W<HBN_RSV2_SPEC>;
#[doc = "Field `hbn_ldo18io_power_on_dly` reader - "]
pub type HBN_LDO18IO_POWER_ON_DLY_R = crate::FieldReader<u16>;
#[doc = "Field `hbn_ldo18io_power_on_dly` writer - "]
pub type HBN_LDO18IO_POWER_ON_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `hbn_ldo18io_power_off_dly` reader - "]
pub type HBN_LDO18IO_POWER_OFF_DLY_R = crate::FieldReader;
#[doc = "Field `hbn_ldo18io_power_off_dly` writer - "]
pub type HBN_LDO18IO_POWER_OFF_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `hbn_ldo18io_power_dly_sts` reader - "]
pub type HBN_LDO18IO_POWER_DLY_STS_R = crate::FieldReader;
#[doc = "Field `hbn_ldo18io_power_dly_sts` writer - "]
pub type HBN_LDO18IO_POWER_DLY_STS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `hbn_core_unhalt` reader - "]
pub type HBN_CORE_UNHALT_R = crate::BitReader;
#[doc = "Field `hbn_core_unhalt` writer - "]
pub type HBN_CORE_UNHALT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hbn_user_boot_sel` reader - "]
pub type HBN_USER_BOOT_SEL_R = crate::FieldReader;
#[doc = "Field `hbn_user_boot_sel` writer - "]
pub type HBN_USER_BOOT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `hbn_release_core` reader - "]
pub type HBN_RELEASE_CORE_R = crate::FieldReader;
#[doc = "Field `hbn_release_core` writer - "]
pub type HBN_RELEASE_CORE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn hbn_ldo18io_power_on_dly(&self) -> HBN_LDO18IO_POWER_ON_DLY_R {
        HBN_LDO18IO_POWER_ON_DLY_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn hbn_ldo18io_power_off_dly(&self) -> HBN_LDO18IO_POWER_OFF_DLY_R {
        HBN_LDO18IO_POWER_OFF_DLY_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn hbn_ldo18io_power_dly_sts(&self) -> HBN_LDO18IO_POWER_DLY_STS_R {
        HBN_LDO18IO_POWER_DLY_STS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn hbn_core_unhalt(&self) -> HBN_CORE_UNHALT_R {
        HBN_CORE_UNHALT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn hbn_user_boot_sel(&self) -> HBN_USER_BOOT_SEL_R {
        HBN_USER_BOOT_SEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn hbn_release_core(&self) -> HBN_RELEASE_CORE_R {
        HBN_RELEASE_CORE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_ldo18io_power_on_dly(&mut self) -> HBN_LDO18IO_POWER_ON_DLY_W<HBN_RSV2_SPEC> {
        HBN_LDO18IO_POWER_ON_DLY_W::new(self, 0)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_ldo18io_power_off_dly(&mut self) -> HBN_LDO18IO_POWER_OFF_DLY_W<HBN_RSV2_SPEC> {
        HBN_LDO18IO_POWER_OFF_DLY_W::new(self, 11)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_ldo18io_power_dly_sts(&mut self) -> HBN_LDO18IO_POWER_DLY_STS_W<HBN_RSV2_SPEC> {
        HBN_LDO18IO_POWER_DLY_STS_W::new(self, 16)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_core_unhalt(&mut self) -> HBN_CORE_UNHALT_W<HBN_RSV2_SPEC> {
        HBN_CORE_UNHALT_W::new(self, 25)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_user_boot_sel(&mut self) -> HBN_USER_BOOT_SEL_W<HBN_RSV2_SPEC> {
        HBN_USER_BOOT_SEL_W::new(self, 26)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_release_core(&mut self) -> HBN_RELEASE_CORE_W<HBN_RSV2_SPEC> {
        HBN_RELEASE_CORE_W::new(self, 28)
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
#[doc = "HBN_RSV2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hbn_rsv2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hbn_rsv2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HBN_RSV2_SPEC;
impl crate::RegisterSpec for HBN_RSV2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hbn_rsv2::R`](R) reader structure"]
impl crate::Readable for HBN_RSV2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hbn_rsv2::W`](W) writer structure"]
impl crate::Writable for HBN_RSV2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HBN_RSV2 to value 0"]
impl crate::Resettable for HBN_RSV2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
