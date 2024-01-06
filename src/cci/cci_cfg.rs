#[doc = "Register `cci_cfg` reader"]
pub type R = crate::R<CCI_CFG_SPEC>;
#[doc = "Register `cci_cfg` writer"]
pub type W = crate::W<CCI_CFG_SPEC>;
#[doc = "Field `cci_en` reader - "]
pub type CCI_EN_R = crate::BitReader;
#[doc = "Field `cci_en` writer - "]
pub type CCI_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cci_slv_sel_cci2` reader - "]
pub type CCI_SLV_SEL_CCI2_R = crate::BitReader;
#[doc = "Field `cci_slv_sel_cci2` writer - "]
pub type CCI_SLV_SEL_CCI2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cci_mas_sel_cci2` reader - "]
pub type CCI_MAS_SEL_CCI2_R = crate::BitReader;
#[doc = "Field `cci_mas_sel_cci2` writer - "]
pub type CCI_MAS_SEL_CCI2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cci_mas_hw_mode` reader - "]
pub type CCI_MAS_HW_MODE_R = crate::BitReader;
#[doc = "Field `cci_mas_hw_mode` writer - "]
pub type CCI_MAS_HW_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_m_cci_sclk_en` reader - "]
pub type REG_M_CCI_SCLK_EN_R = crate::BitReader;
#[doc = "Field `reg_m_cci_sclk_en` writer - "]
pub type REG_M_CCI_SCLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_div_m_cci_sclk` reader - "]
pub type REG_DIV_M_CCI_SCLK_R = crate::FieldReader;
#[doc = "Field `reg_div_m_cci_sclk` writer - "]
pub type REG_DIV_M_CCI_SCLK_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cfg_cci1_pre_read` reader - "]
pub type CFG_CCI1_PRE_READ_R = crate::BitReader;
#[doc = "Field `cfg_cci1_pre_read` writer - "]
pub type CFG_CCI1_PRE_READ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_scci_clk_inv` reader - "]
pub type REG_SCCI_CLK_INV_R = crate::BitReader;
#[doc = "Field `reg_scci_clk_inv` writer - "]
pub type REG_SCCI_CLK_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_mcci_clk_inv` reader - "]
pub type REG_MCCI_CLK_INV_R = crate::BitReader;
#[doc = "Field `reg_mcci_clk_inv` writer - "]
pub type REG_MCCI_CLK_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_mcci_dly_r` reader - "]
pub type CFG_MCCI_DLY_R_R = crate::BitReader;
#[doc = "Field `cfg_mcci_dly_r` writer - "]
pub type CFG_MCCI_DLY_R_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cci_en(&self) -> CCI_EN_R {
        CCI_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cci_slv_sel_cci2(&self) -> CCI_SLV_SEL_CCI2_R {
        CCI_SLV_SEL_CCI2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cci_mas_sel_cci2(&self) -> CCI_MAS_SEL_CCI2_R {
        CCI_MAS_SEL_CCI2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cci_mas_hw_mode(&self) -> CCI_MAS_HW_MODE_R {
        CCI_MAS_HW_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_m_cci_sclk_en(&self) -> REG_M_CCI_SCLK_EN_R {
        REG_M_CCI_SCLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn reg_div_m_cci_sclk(&self) -> REG_DIV_M_CCI_SCLK_R {
        REG_DIV_M_CCI_SCLK_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cfg_cci1_pre_read(&self) -> CFG_CCI1_PRE_READ_R {
        CFG_CCI1_PRE_READ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_scci_clk_inv(&self) -> REG_SCCI_CLK_INV_R {
        REG_SCCI_CLK_INV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_mcci_clk_inv(&self) -> REG_MCCI_CLK_INV_R {
        REG_MCCI_CLK_INV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cfg_mcci_dly_r(&self) -> CFG_MCCI_DLY_R_R {
        CFG_MCCI_DLY_R_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cci_en(&mut self) -> CCI_EN_W<CCI_CFG_SPEC> {
        CCI_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cci_slv_sel_cci2(&mut self) -> CCI_SLV_SEL_CCI2_W<CCI_CFG_SPEC> {
        CCI_SLV_SEL_CCI2_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cci_mas_sel_cci2(&mut self) -> CCI_MAS_SEL_CCI2_W<CCI_CFG_SPEC> {
        CCI_MAS_SEL_CCI2_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cci_mas_hw_mode(&mut self) -> CCI_MAS_HW_MODE_W<CCI_CFG_SPEC> {
        CCI_MAS_HW_MODE_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn reg_m_cci_sclk_en(&mut self) -> REG_M_CCI_SCLK_EN_W<CCI_CFG_SPEC> {
        REG_M_CCI_SCLK_EN_W::new(self, 4)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    #[must_use]
    pub fn reg_div_m_cci_sclk(&mut self) -> REG_DIV_M_CCI_SCLK_W<CCI_CFG_SPEC> {
        REG_DIV_M_CCI_SCLK_W::new(self, 5)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_cci1_pre_read(&mut self) -> CFG_CCI1_PRE_READ_W<CCI_CFG_SPEC> {
        CFG_CCI1_PRE_READ_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn reg_scci_clk_inv(&mut self) -> REG_SCCI_CLK_INV_W<CCI_CFG_SPEC> {
        REG_SCCI_CLK_INV_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcci_clk_inv(&mut self) -> REG_MCCI_CLK_INV_W<CCI_CFG_SPEC> {
        REG_MCCI_CLK_INV_W::new(self, 9)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_mcci_dly_r(&mut self) -> CFG_MCCI_DLY_R_W<CCI_CFG_SPEC> {
        CFG_MCCI_DLY_R_W::new(self, 16)
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
#[doc = "cci_cfg.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cci_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cci_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCI_CFG_SPEC;
impl crate::RegisterSpec for CCI_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cci_cfg::R`](R) reader structure"]
impl crate::Readable for CCI_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cci_cfg::W`](W) writer structure"]
impl crate::Writable for CCI_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cci_cfg to value 0"]
impl crate::Resettable for CCI_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
