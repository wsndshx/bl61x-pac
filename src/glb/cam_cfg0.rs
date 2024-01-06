#[doc = "Register `cam_cfg0` reader"]
pub type R = crate::R<CAM_CFG0_SPEC>;
#[doc = "Register `cam_cfg0` writer"]
pub type W = crate::W<CAM_CFG0_SPEC>;
#[doc = "Field `reg_cam_ref_clk_en` reader - "]
pub type REG_CAM_REF_CLK_EN_R = crate::BitReader;
#[doc = "Field `reg_cam_ref_clk_en` writer - "]
pub type REG_CAM_REF_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_cam_ref_clk_src_sel` reader - "]
pub type REG_CAM_REF_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `reg_cam_ref_clk_src_sel` writer - "]
pub type REG_CAM_REF_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_cam_ref_clk_div` reader - "]
pub type REG_CAM_REF_CLK_DIV_R = crate::FieldReader;
#[doc = "Field `reg_cam_ref_clk_div` writer - "]
pub type REG_CAM_REF_CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn reg_cam_ref_clk_en(&self) -> REG_CAM_REF_CLK_EN_R {
        REG_CAM_REF_CLK_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn reg_cam_ref_clk_src_sel(&self) -> REG_CAM_REF_CLK_SRC_SEL_R {
        REG_CAM_REF_CLK_SRC_SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn reg_cam_ref_clk_div(&self) -> REG_CAM_REF_CLK_DIV_R {
        REG_CAM_REF_CLK_DIV_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn reg_cam_ref_clk_en(&mut self) -> REG_CAM_REF_CLK_EN_W<CAM_CFG0_SPEC> {
        REG_CAM_REF_CLK_EN_W::new(self, 27)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    #[must_use]
    pub fn reg_cam_ref_clk_src_sel(&mut self) -> REG_CAM_REF_CLK_SRC_SEL_W<CAM_CFG0_SPEC> {
        REG_CAM_REF_CLK_SRC_SEL_W::new(self, 28)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_cam_ref_clk_div(&mut self) -> REG_CAM_REF_CLK_DIV_W<CAM_CFG0_SPEC> {
        REG_CAM_REF_CLK_DIV_W::new(self, 30)
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
#[doc = "cam_cfg0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cam_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cam_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAM_CFG0_SPEC;
impl crate::RegisterSpec for CAM_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cam_cfg0::R`](R) reader structure"]
impl crate::Readable for CAM_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cam_cfg0::W`](W) writer structure"]
impl crate::Writable for CAM_CFG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cam_cfg0 to value 0"]
impl crate::Resettable for CAM_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
