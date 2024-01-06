#[doc = "Register `aon_common` reader"]
pub type R = crate::R<AON_COMMON_SPEC>;
#[doc = "Register `aon_common` writer"]
pub type W = crate::W<AON_COMMON_SPEC>;
#[doc = "Field `tmux_aon` reader - "]
pub type TMUX_AON_R = crate::FieldReader;
#[doc = "Field `tmux_aon` writer - "]
pub type TMUX_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ten_aon` reader - "]
pub type TEN_AON_R = crate::BitReader;
#[doc = "Field `ten_aon` writer - "]
pub type TEN_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_vddcore_aon` reader - "]
pub type TEN_VDDCORE_AON_R = crate::BitReader;
#[doc = "Field `ten_vddcore_aon` writer - "]
pub type TEN_VDDCORE_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_ldo11soc_aon` reader - "]
pub type TEN_LDO11SOC_AON_R = crate::BitReader;
#[doc = "Field `ten_ldo11soc_aon` writer - "]
pub type TEN_LDO11SOC_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_dcdc_0_aon` reader - "]
pub type TEN_DCDC_0_AON_R = crate::BitReader;
#[doc = "Field `ten_dcdc_0_aon` writer - "]
pub type TEN_DCDC_0_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_dcdc_1_aon` reader - "]
pub type TEN_DCDC_1_AON_R = crate::BitReader;
#[doc = "Field `ten_dcdc_1_aon` writer - "]
pub type TEN_DCDC_1_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_bg_sys_aon` reader - "]
pub type TEN_BG_SYS_AON_R = crate::BitReader;
#[doc = "Field `ten_bg_sys_aon` writer - "]
pub type TEN_BG_SYS_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_ldo15rf_aon` reader - "]
pub type TEN_LDO15RF_AON_R = crate::BitReader;
#[doc = "Field `ten_ldo15rf_aon` writer - "]
pub type TEN_LDO15RF_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_xtal_aon` reader - "]
pub type TEN_XTAL_AON_R = crate::BitReader;
#[doc = "Field `ten_xtal_aon` writer - "]
pub type TEN_XTAL_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dten_xtal_aon` reader - "]
pub type DTEN_XTAL_AON_R = crate::BitReader;
#[doc = "Field `dten_xtal_aon` writer - "]
pub type DTEN_XTAL_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_mbg_aon` reader - "]
pub type TEN_MBG_AON_R = crate::BitReader;
#[doc = "Field `ten_mbg_aon` writer - "]
pub type TEN_MBG_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ten_cip_misc_aon` reader - "]
pub type TEN_CIP_MISC_AON_R = crate::BitReader;
#[doc = "Field `ten_cip_misc_aon` writer - "]
pub type TEN_CIP_MISC_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tmux_aon(&self) -> TMUX_AON_R {
        TMUX_AON_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ten_aon(&self) -> TEN_AON_R {
        TEN_AON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ten_vddcore_aon(&self) -> TEN_VDDCORE_AON_R {
        TEN_VDDCORE_AON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ten_ldo11soc_aon(&self) -> TEN_LDO11SOC_AON_R {
        TEN_LDO11SOC_AON_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ten_dcdc_0_aon(&self) -> TEN_DCDC_0_AON_R {
        TEN_DCDC_0_AON_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ten_dcdc_1_aon(&self) -> TEN_DCDC_1_AON_R {
        TEN_DCDC_1_AON_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ten_bg_sys_aon(&self) -> TEN_BG_SYS_AON_R {
        TEN_BG_SYS_AON_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ten_ldo15rf_aon(&self) -> TEN_LDO15RF_AON_R {
        TEN_LDO15RF_AON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ten_xtal_aon(&self) -> TEN_XTAL_AON_R {
        TEN_XTAL_AON_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn dten_xtal_aon(&self) -> DTEN_XTAL_AON_R {
        DTEN_XTAL_AON_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ten_mbg_aon(&self) -> TEN_MBG_AON_R {
        TEN_MBG_AON_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ten_cip_misc_aon(&self) -> TEN_CIP_MISC_AON_R {
        TEN_CIP_MISC_AON_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn tmux_aon(&mut self) -> TMUX_AON_W<AON_COMMON_SPEC> {
        TMUX_AON_W::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn ten_aon(&mut self) -> TEN_AON_W<AON_COMMON_SPEC> {
        TEN_AON_W::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn ten_vddcore_aon(&mut self) -> TEN_VDDCORE_AON_W<AON_COMMON_SPEC> {
        TEN_VDDCORE_AON_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn ten_ldo11soc_aon(&mut self) -> TEN_LDO11SOC_AON_W<AON_COMMON_SPEC> {
        TEN_LDO11SOC_AON_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn ten_dcdc_0_aon(&mut self) -> TEN_DCDC_0_AON_W<AON_COMMON_SPEC> {
        TEN_DCDC_0_AON_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn ten_dcdc_1_aon(&mut self) -> TEN_DCDC_1_AON_W<AON_COMMON_SPEC> {
        TEN_DCDC_1_AON_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn ten_bg_sys_aon(&mut self) -> TEN_BG_SYS_AON_W<AON_COMMON_SPEC> {
        TEN_BG_SYS_AON_W::new(self, 12)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn ten_ldo15rf_aon(&mut self) -> TEN_LDO15RF_AON_W<AON_COMMON_SPEC> {
        TEN_LDO15RF_AON_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn ten_xtal_aon(&mut self) -> TEN_XTAL_AON_W<AON_COMMON_SPEC> {
        TEN_XTAL_AON_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn dten_xtal_aon(&mut self) -> DTEN_XTAL_AON_W<AON_COMMON_SPEC> {
        DTEN_XTAL_AON_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn ten_mbg_aon(&mut self) -> TEN_MBG_AON_W<AON_COMMON_SPEC> {
        TEN_MBG_AON_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn ten_cip_misc_aon(&mut self) -> TEN_CIP_MISC_AON_W<AON_COMMON_SPEC> {
        TEN_CIP_MISC_AON_W::new(self, 20)
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
#[doc = "aon_common.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_common::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_common::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AON_COMMON_SPEC;
impl crate::RegisterSpec for AON_COMMON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_common::R`](R) reader structure"]
impl crate::Readable for AON_COMMON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aon_common::W`](W) writer structure"]
impl crate::Writable for AON_COMMON_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets aon_common to value 0"]
impl crate::Resettable for AON_COMMON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
