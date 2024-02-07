#[doc = "Register `swrst_cfg2` reader"]
pub type R = crate::R<SWRST_CFG2_SPEC>;
#[doc = "Register `swrst_cfg2` writer"]
pub type W = crate::W<SWRST_CFG2_SPEC>;
#[doc = "Field `reg_ctrl_pwron_rst` reader - "]
pub type REG_CTRL_PWRON_RST_R = crate::BitReader;
#[doc = "Field `reg_ctrl_pwron_rst` writer - "]
pub type REG_CTRL_PWRON_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ctrl_cpu_reset` reader - "]
pub type REG_CTRL_CPU_RESET_R = crate::BitReader;
#[doc = "Field `reg_ctrl_cpu_reset` writer - "]
pub type REG_CTRL_CPU_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ctrl_sys_reset` reader - "]
pub type REG_CTRL_SYS_RESET_R = crate::BitReader;
#[doc = "Field `reg_ctrl_sys_reset` writer - "]
pub type REG_CTRL_SYS_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ctrl_pico_reset` reader - "]
pub type REG_CTRL_PICO_RESET_R = crate::BitReader;
#[doc = "Field `reg_ctrl_pico_reset` writer - "]
pub type REG_CTRL_PICO_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ctrl_cpu2_reset` reader - "]
pub type REG_CTRL_CPU2_RESET_R = crate::BitReader;
#[doc = "Field `reg_ctrl_cpu2_reset` writer - "]
pub type REG_CTRL_CPU2_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ctrl_chip_reset` reader - "]
pub type REG_CTRL_CHIP_RESET_R = crate::BitReader;
#[doc = "Field `reg_ctrl_chip_reset` writer - "]
pub type REG_CTRL_CHIP_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pka_clk_sel` reader - "]
pub type PKA_CLK_SEL_R = crate::BitReader;
#[doc = "Field `pka_clk_sel` writer - "]
pub type PKA_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_ctrl_reset_dummy` reader - "]
pub type REG_CTRL_RESET_DUMMY_R = crate::FieldReader;
#[doc = "Field `reg_ctrl_reset_dummy` writer - "]
pub type REG_CTRL_RESET_DUMMY_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_ctrl_pwron_rst(&self) -> REG_CTRL_PWRON_RST_R {
        REG_CTRL_PWRON_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_ctrl_cpu_reset(&self) -> REG_CTRL_CPU_RESET_R {
        REG_CTRL_CPU_RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_ctrl_sys_reset(&self) -> REG_CTRL_SYS_RESET_R {
        REG_CTRL_SYS_RESET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_ctrl_pico_reset(&self) -> REG_CTRL_PICO_RESET_R {
        REG_CTRL_PICO_RESET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_ctrl_cpu2_reset(&self) -> REG_CTRL_CPU2_RESET_R {
        REG_CTRL_CPU2_RESET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_ctrl_chip_reset(&self) -> REG_CTRL_CHIP_RESET_R {
        REG_CTRL_CHIP_RESET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pka_clk_sel(&self) -> PKA_CLK_SEL_R {
        PKA_CLK_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn reg_ctrl_reset_dummy(&self) -> REG_CTRL_RESET_DUMMY_R {
        REG_CTRL_RESET_DUMMY_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ctrl_pwron_rst(&mut self) -> REG_CTRL_PWRON_RST_W<SWRST_CFG2_SPEC> {
        REG_CTRL_PWRON_RST_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ctrl_cpu_reset(&mut self) -> REG_CTRL_CPU_RESET_W<SWRST_CFG2_SPEC> {
        REG_CTRL_CPU_RESET_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ctrl_sys_reset(&mut self) -> REG_CTRL_SYS_RESET_W<SWRST_CFG2_SPEC> {
        REG_CTRL_SYS_RESET_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ctrl_pico_reset(&mut self) -> REG_CTRL_PICO_RESET_W<SWRST_CFG2_SPEC> {
        REG_CTRL_PICO_RESET_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ctrl_cpu2_reset(&mut self) -> REG_CTRL_CPU2_RESET_W<SWRST_CFG2_SPEC> {
        REG_CTRL_CPU2_RESET_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ctrl_chip_reset(&mut self) -> REG_CTRL_CHIP_RESET_W<SWRST_CFG2_SPEC> {
        REG_CTRL_CHIP_RESET_W::new(self, 5)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn pka_clk_sel(&mut self) -> PKA_CLK_SEL_W<SWRST_CFG2_SPEC> {
        PKA_CLK_SEL_W::new(self, 24)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ctrl_reset_dummy(&mut self) -> REG_CTRL_RESET_DUMMY_W<SWRST_CFG2_SPEC> {
        REG_CTRL_RESET_DUMMY_W::new(self, 28)
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
#[doc = "swrst_cfg2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swrst_cfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swrst_cfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWRST_CFG2_SPEC;
impl crate::RegisterSpec for SWRST_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swrst_cfg2::R`](R) reader structure"]
impl crate::Readable for SWRST_CFG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swrst_cfg2::W`](W) writer structure"]
impl crate::Writable for SWRST_CFG2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets swrst_cfg2 to value 0"]
impl crate::Resettable for SWRST_CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
