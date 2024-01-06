#[doc = "Register `bg_sys_top` reader"]
pub type R = crate::R<BG_SYS_TOP_SPEC>;
#[doc = "Register `bg_sys_top` writer"]
pub type W = crate::W<BG_SYS_TOP_SPEC>;
#[doc = "Field `pu_bg_sys_aon` reader - "]
pub type PU_BG_SYS_AON_R = crate::BitReader;
#[doc = "Field `pu_bg_sys_aon` writer - "]
pub type PU_BG_SYS_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `istart_ctrl_aon` reader - "]
pub type ISTART_CTRL_AON_R = crate::BitReader;
#[doc = "Field `istart_ctrl_aon` writer - "]
pub type ISTART_CTRL_AON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pmip_resv_aon` reader - "]
pub type PMIP_RESV_AON_R = crate::FieldReader;
#[doc = "Field `pmip_resv_aon` writer - "]
pub type PMIP_RESV_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_bg_sys_aon(&self) -> PU_BG_SYS_AON_R {
        PU_BG_SYS_AON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn istart_ctrl_aon(&self) -> ISTART_CTRL_AON_R {
        ISTART_CTRL_AON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn pmip_resv_aon(&self) -> PMIP_RESV_AON_R {
        PMIP_RESV_AON_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pu_bg_sys_aon(&mut self) -> PU_BG_SYS_AON_W<BG_SYS_TOP_SPEC> {
        PU_BG_SYS_AON_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn istart_ctrl_aon(&mut self) -> ISTART_CTRL_AON_W<BG_SYS_TOP_SPEC> {
        ISTART_CTRL_AON_W::new(self, 1)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn pmip_resv_aon(&mut self) -> PMIP_RESV_AON_W<BG_SYS_TOP_SPEC> {
        PMIP_RESV_AON_W::new(self, 8)
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
#[doc = "bg_sys_top.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bg_sys_top::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bg_sys_top::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BG_SYS_TOP_SPEC;
impl crate::RegisterSpec for BG_SYS_TOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bg_sys_top::R`](R) reader structure"]
impl crate::Readable for BG_SYS_TOP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bg_sys_top::W`](W) writer structure"]
impl crate::Writable for BG_SYS_TOP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets bg_sys_top to value 0"]
impl crate::Resettable for BG_SYS_TOP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
