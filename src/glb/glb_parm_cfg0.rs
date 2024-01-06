#[doc = "Register `glb_parm_cfg0` reader"]
pub type R = crate::R<GLB_PARM_CFG0_SPEC>;
#[doc = "Register `glb_parm_cfg0` writer"]
pub type W = crate::W<GLB_PARM_CFG0_SPEC>;
#[doc = "Field `uart_swap_set` reader - "]
pub type UART_SWAP_SET_R = crate::FieldReader;
#[doc = "Field `uart_swap_set` writer - "]
pub type UART_SWAP_SET_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `swap_sflash_io_3_io_0` reader - "]
pub type SWAP_SFLASH_IO_3_IO_0_R = crate::BitReader;
#[doc = "Field `swap_sflash_io_3_io_0` writer - "]
pub type SWAP_SFLASH_IO_3_IO_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sel_embedded_sflash` reader - "]
pub type SEL_EMBEDDED_SFLASH_R = crate::BitReader;
#[doc = "Field `sel_embedded_sflash` writer - "]
pub type SEL_EMBEDDED_SFLASH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swap_sflash_io_2_cs` reader - "]
pub type SWAP_SFLASH_IO_2_CS_R = crate::BitReader;
#[doc = "Field `swap_sflash_io_2_cs` writer - "]
pub type SWAP_SFLASH_IO_2_CS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `swap_sflash2_io_3_io_0` reader - "]
pub type SWAP_SFLASH2_IO_3_IO_0_R = crate::BitReader;
#[doc = "Field `swap_sflash2_io_3_io_0` writer - "]
pub type SWAP_SFLASH2_IO_3_IO_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_spi_0_master_mode` reader - "]
pub type REG_SPI_0_MASTER_MODE_R = crate::BitReader;
#[doc = "Field `reg_spi_0_master_mode` writer - "]
pub type REG_SPI_0_MASTER_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_spi_0_swap` reader - "]
pub type REG_SPI_0_SWAP_R = crate::BitReader;
#[doc = "Field `reg_spi_0_swap` writer - "]
pub type REG_SPI_0_SWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ant_switch_sel` reader - "]
pub type ANT_SWITCH_SEL_R = crate::BitReader;
#[doc = "Field `ant_switch_sel` writer - "]
pub type ANT_SWITCH_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `p3_cci_use_io_10_13` reader - "]
pub type P3_CCI_USE_IO_10_13_R = crate::BitReader;
#[doc = "Field `p3_cci_use_io_10_13` writer - "]
pub type P3_CCI_USE_IO_10_13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `audio_test_mode` reader - "]
pub type AUDIO_TEST_MODE_R = crate::BitReader;
#[doc = "Field `audio_test_mode` writer - "]
pub type AUDIO_TEST_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sel_rf_audio_test` reader - "]
pub type SEL_RF_AUDIO_TEST_R = crate::FieldReader;
#[doc = "Field `sel_rf_audio_test` writer - "]
pub type SEL_RF_AUDIO_TEST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn uart_swap_set(&self) -> UART_SWAP_SET_R {
        UART_SWAP_SET_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swap_sflash_io_3_io_0(&self) -> SWAP_SFLASH_IO_3_IO_0_R {
        SWAP_SFLASH_IO_3_IO_0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sel_embedded_sflash(&self) -> SEL_EMBEDDED_SFLASH_R {
        SEL_EMBEDDED_SFLASH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn swap_sflash_io_2_cs(&self) -> SWAP_SFLASH_IO_2_CS_R {
        SWAP_SFLASH_IO_2_CS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn swap_sflash2_io_3_io_0(&self) -> SWAP_SFLASH2_IO_3_IO_0_R {
        SWAP_SFLASH2_IO_3_IO_0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_spi_0_master_mode(&self) -> REG_SPI_0_MASTER_MODE_R {
        REG_SPI_0_MASTER_MODE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_spi_0_swap(&self) -> REG_SPI_0_SWAP_R {
        REG_SPI_0_SWAP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ant_switch_sel(&self) -> ANT_SWITCH_SEL_R {
        ANT_SWITCH_SEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn p3_cci_use_io_10_13(&self) -> P3_CCI_USE_IO_10_13_R {
        P3_CCI_USE_IO_10_13_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn audio_test_mode(&self) -> AUDIO_TEST_MODE_R {
        AUDIO_TEST_MODE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn sel_rf_audio_test(&self) -> SEL_RF_AUDIO_TEST_R {
        SEL_RF_AUDIO_TEST_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 2:5"]
    #[inline(always)]
    #[must_use]
    pub fn uart_swap_set(&mut self) -> UART_SWAP_SET_W<GLB_PARM_CFG0_SPEC> {
        UART_SWAP_SET_W::new(self, 2)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn swap_sflash_io_3_io_0(&mut self) -> SWAP_SFLASH_IO_3_IO_0_W<GLB_PARM_CFG0_SPEC> {
        SWAP_SFLASH_IO_3_IO_0_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn sel_embedded_sflash(&mut self) -> SEL_EMBEDDED_SFLASH_W<GLB_PARM_CFG0_SPEC> {
        SEL_EMBEDDED_SFLASH_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn swap_sflash_io_2_cs(&mut self) -> SWAP_SFLASH_IO_2_CS_W<GLB_PARM_CFG0_SPEC> {
        SWAP_SFLASH_IO_2_CS_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn swap_sflash2_io_3_io_0(&mut self) -> SWAP_SFLASH2_IO_3_IO_0_W<GLB_PARM_CFG0_SPEC> {
        SWAP_SFLASH2_IO_3_IO_0_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn reg_spi_0_master_mode(&mut self) -> REG_SPI_0_MASTER_MODE_W<GLB_PARM_CFG0_SPEC> {
        REG_SPI_0_MASTER_MODE_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn reg_spi_0_swap(&mut self) -> REG_SPI_0_SWAP_W<GLB_PARM_CFG0_SPEC> {
        REG_SPI_0_SWAP_W::new(self, 13)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn ant_switch_sel(&mut self) -> ANT_SWITCH_SEL_W<GLB_PARM_CFG0_SPEC> {
        ANT_SWITCH_SEL_W::new(self, 15)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn p3_cci_use_io_10_13(&mut self) -> P3_CCI_USE_IO_10_13_W<GLB_PARM_CFG0_SPEC> {
        P3_CCI_USE_IO_10_13_W::new(self, 19)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn audio_test_mode(&mut self) -> AUDIO_TEST_MODE_W<GLB_PARM_CFG0_SPEC> {
        AUDIO_TEST_MODE_W::new(self, 29)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn sel_rf_audio_test(&mut self) -> SEL_RF_AUDIO_TEST_W<GLB_PARM_CFG0_SPEC> {
        SEL_RF_AUDIO_TEST_W::new(self, 30)
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
#[doc = "glb_parm_cfg0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_parm_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_parm_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GLB_PARM_CFG0_SPEC;
impl crate::RegisterSpec for GLB_PARM_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`glb_parm_cfg0::R`](R) reader structure"]
impl crate::Readable for GLB_PARM_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`glb_parm_cfg0::W`](W) writer structure"]
impl crate::Writable for GLB_PARM_CFG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets glb_parm_cfg0 to value 0"]
impl crate::Resettable for GLB_PARM_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
