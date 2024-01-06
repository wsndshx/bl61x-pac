#[doc = "Register `debug_config_0` reader"]
pub type R = crate::R<DEBUG_CONFIG_0_SPEC>;
#[doc = "Register `debug_config_0` writer"]
pub type W = crate::W<DEBUG_CONFIG_0_SPEC>;
#[doc = "Field `reg_dbg_ll_ctrl` reader - "]
pub type REG_DBG_LL_CTRL_R = crate::FieldReader<u32>;
#[doc = "Field `reg_dbg_ll_ctrl` writer - "]
pub type REG_DBG_LL_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
#[doc = "Field `reg_dbg_ll_sel` reader - "]
pub type REG_DBG_LL_SEL_R = crate::FieldReader;
#[doc = "Field `reg_dbg_ll_sel` writer - "]
pub type REG_DBG_LL_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn reg_dbg_ll_ctrl(&self) -> REG_DBG_LL_CTRL_R {
        REG_DBG_LL_CTRL_R::new(self.bits & 0x3fff_ffff)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn reg_dbg_ll_sel(&self) -> REG_DBG_LL_SEL_R {
        REG_DBG_LL_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    #[must_use]
    pub fn reg_dbg_ll_ctrl(&mut self) -> REG_DBG_LL_CTRL_W<DEBUG_CONFIG_0_SPEC> {
        REG_DBG_LL_CTRL_W::new(self, 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_dbg_ll_sel(&mut self) -> REG_DBG_LL_SEL_W<DEBUG_CONFIG_0_SPEC> {
        REG_DBG_LL_SEL_W::new(self, 30)
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
#[doc = "Debug configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_config_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_config_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEBUG_CONFIG_0_SPEC;
impl crate::RegisterSpec for DEBUG_CONFIG_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_config_0::R`](R) reader structure"]
impl crate::Readable for DEBUG_CONFIG_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`debug_config_0::W`](W) writer structure"]
impl crate::Writable for DEBUG_CONFIG_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets debug_config_0 to value 0"]
impl crate::Resettable for DEBUG_CONFIG_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
