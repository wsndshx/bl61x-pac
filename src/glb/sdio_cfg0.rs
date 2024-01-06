#[doc = "Register `sdio_cfg0` reader"]
pub type R = crate::R<SDIO_CFG0_SPEC>;
#[doc = "Register `sdio_cfg0` writer"]
pub type W = crate::W<SDIO_CFG0_SPEC>;
#[doc = "Field `reg_sdio_int_sys_dis` reader - "]
pub type REG_SDIO_INT_SYS_DIS_R = crate::BitReader;
#[doc = "Field `reg_sdio_int_sys_dis` writer - "]
pub type REG_SDIO_INT_SYS_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_sd_rst_sd_dis` reader - "]
pub type REG_SD_RST_SD_DIS_R = crate::BitReader;
#[doc = "Field `reg_sd_rst_sd_dis` writer - "]
pub type REG_SD_RST_SD_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_sdu_rst_sd_dis` reader - "]
pub type REG_SDU_RST_SD_DIS_R = crate::BitReader;
#[doc = "Field `reg_sdu_rst_sd_dis` writer - "]
pub type REG_SDU_RST_SD_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_sys_rst_sd_en` reader - "]
pub type REG_SYS_RST_SD_EN_R = crate::BitReader;
#[doc = "Field `reg_sys_rst_sd_en` writer - "]
pub type REG_SYS_RST_SD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdu_cmdRcvd` reader - "]
pub type SDU_CMD_RCVD_R = crate::BitReader;
#[doc = "Field `sdu_cmdRcvd` writer - "]
pub type SDU_CMD_RCVD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdu_clk_switch_ok` reader - "]
pub type SDU_CLK_SWITCH_OK_R = crate::BitReader;
#[doc = "Field `sdu_clk_switch_ok` writer - "]
pub type SDU_CLK_SWITCH_OK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sd_pwup` reader - "]
pub type SD_PWUP_R = crate::BitReader;
#[doc = "Field `sd_pwup` writer - "]
pub type SD_PWUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdu_dbg` reader - "]
pub type SDU_DBG_R = crate::FieldReader<u16>;
#[doc = "Field `sdu_dbg` writer - "]
pub type SDU_DBG_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_sdio_int_sys_dis(&self) -> REG_SDIO_INT_SYS_DIS_R {
        REG_SDIO_INT_SYS_DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_sd_rst_sd_dis(&self) -> REG_SD_RST_SD_DIS_R {
        REG_SD_RST_SD_DIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_sdu_rst_sd_dis(&self) -> REG_SDU_RST_SD_DIS_R {
        REG_SDU_RST_SD_DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_sys_rst_sd_en(&self) -> REG_SYS_RST_SD_EN_R {
        REG_SYS_RST_SD_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sdu_cmd_rcvd(&self) -> SDU_CMD_RCVD_R {
        SDU_CMD_RCVD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn sdu_clk_switch_ok(&self) -> SDU_CLK_SWITCH_OK_R {
        SDU_CLK_SWITCH_OK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn sd_pwup(&self) -> SD_PWUP_R {
        SD_PWUP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn sdu_dbg(&self) -> SDU_DBG_R {
        SDU_DBG_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_sdio_int_sys_dis(&mut self) -> REG_SDIO_INT_SYS_DIS_W<SDIO_CFG0_SPEC> {
        REG_SDIO_INT_SYS_DIS_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_sd_rst_sd_dis(&mut self) -> REG_SD_RST_SD_DIS_W<SDIO_CFG0_SPEC> {
        REG_SD_RST_SD_DIS_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn reg_sdu_rst_sd_dis(&mut self) -> REG_SDU_RST_SD_DIS_W<SDIO_CFG0_SPEC> {
        REG_SDU_RST_SD_DIS_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn reg_sys_rst_sd_en(&mut self) -> REG_SYS_RST_SD_EN_W<SDIO_CFG0_SPEC> {
        REG_SYS_RST_SD_EN_W::new(self, 3)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn sdu_cmd_rcvd(&mut self) -> SDU_CMD_RCVD_W<SDIO_CFG0_SPEC> {
        SDU_CMD_RCVD_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn sdu_clk_switch_ok(&mut self) -> SDU_CLK_SWITCH_OK_W<SDIO_CFG0_SPEC> {
        SDU_CLK_SWITCH_OK_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn sd_pwup(&mut self) -> SD_PWUP_W<SDIO_CFG0_SPEC> {
        SD_PWUP_W::new(self, 14)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn sdu_dbg(&mut self) -> SDU_DBG_W<SDIO_CFG0_SPEC> {
        SDU_DBG_W::new(self, 16)
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
#[doc = "sdio_cfg0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDIO_CFG0_SPEC;
impl crate::RegisterSpec for SDIO_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio_cfg0::R`](R) reader structure"]
impl crate::Readable for SDIO_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdio_cfg0::W`](W) writer structure"]
impl crate::Writable for SDIO_CFG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sdio_cfg0 to value 0"]
impl crate::Resettable for SDIO_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
