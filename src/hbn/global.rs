#[doc = "Register `global` reader"]
pub type R = crate::R<GLOBAL_SPEC>;
#[doc = "Register `global` writer"]
pub type W = crate::W<GLOBAL_SPEC>;
#[doc = "Field `hbn_root_clk_sel` reader - "]
pub type HBN_ROOT_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `hbn_root_clk_sel` writer - "]
pub type HBN_ROOT_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `hbn_uart_clk_sel` reader - "]
pub type HBN_UART_CLK_SEL_R = crate::BitReader;
#[doc = "Field `hbn_uart_clk_sel` writer - "]
pub type HBN_UART_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hbn_f32k_sel` reader - "]
pub type HBN_F32K_SEL_R = crate::FieldReader;
#[doc = "Field `hbn_f32k_sel` writer - "]
pub type HBN_F32K_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `hbn_pu_rc32k` reader - "]
pub type HBN_PU_RC32K_R = crate::BitReader;
#[doc = "Field `hbn_pu_rc32k` writer - "]
pub type HBN_PU_RC32K_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hbn_reset_event` reader - "]
pub type HBN_RESET_EVENT_R = crate::FieldReader;
#[doc = "Field `hbn_reset_event` writer - "]
pub type HBN_RESET_EVENT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `hbn_uart_clk_sel2` reader - "]
pub type HBN_UART_CLK_SEL2_R = crate::BitReader;
#[doc = "Field `hbn_uart_clk_sel2` writer - "]
pub type HBN_UART_CLK_SEL2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sw_ldo11soc_vout_sel_aon` reader - "]
pub type SW_LDO11SOC_VOUT_SEL_AON_R = crate::FieldReader;
#[doc = "Field `sw_ldo11soc_vout_sel_aon` writer - "]
pub type SW_LDO11SOC_VOUT_SEL_AON_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `sw_ldo11_rt_vout_sel` reader - "]
pub type SW_LDO11_RT_VOUT_SEL_R = crate::FieldReader;
#[doc = "Field `sw_ldo11_rt_vout_sel` writer - "]
pub type SW_LDO11_RT_VOUT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `sw_ldo11_aon_vout_sel` reader - "]
pub type SW_LDO11_AON_VOUT_SEL_R = crate::FieldReader;
#[doc = "Field `sw_ldo11_aon_vout_sel` writer - "]
pub type SW_LDO11_AON_VOUT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn hbn_root_clk_sel(&self) -> HBN_ROOT_CLK_SEL_R {
        HBN_ROOT_CLK_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn hbn_uart_clk_sel(&self) -> HBN_UART_CLK_SEL_R {
        HBN_UART_CLK_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn hbn_f32k_sel(&self) -> HBN_F32K_SEL_R {
        HBN_F32K_SEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn hbn_pu_rc32k(&self) -> HBN_PU_RC32K_R {
        HBN_PU_RC32K_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 7:11"]
    #[inline(always)]
    pub fn hbn_reset_event(&self) -> HBN_RESET_EVENT_R {
        HBN_RESET_EVENT_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn hbn_uart_clk_sel2(&self) -> HBN_UART_CLK_SEL2_R {
        HBN_UART_CLK_SEL2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn sw_ldo11soc_vout_sel_aon(&self) -> SW_LDO11SOC_VOUT_SEL_AON_R {
        SW_LDO11SOC_VOUT_SEL_AON_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn sw_ldo11_rt_vout_sel(&self) -> SW_LDO11_RT_VOUT_SEL_R {
        SW_LDO11_RT_VOUT_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn sw_ldo11_aon_vout_sel(&self) -> SW_LDO11_AON_VOUT_SEL_R {
        SW_LDO11_AON_VOUT_SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_root_clk_sel(&mut self) -> HBN_ROOT_CLK_SEL_W<GLOBAL_SPEC> {
        HBN_ROOT_CLK_SEL_W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_uart_clk_sel(&mut self) -> HBN_UART_CLK_SEL_W<GLOBAL_SPEC> {
        HBN_UART_CLK_SEL_W::new(self, 2)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_f32k_sel(&mut self) -> HBN_F32K_SEL_W<GLOBAL_SPEC> {
        HBN_F32K_SEL_W::new(self, 3)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_pu_rc32k(&mut self) -> HBN_PU_RC32K_W<GLOBAL_SPEC> {
        HBN_PU_RC32K_W::new(self, 5)
    }
    #[doc = "Bits 7:11"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_reset_event(&mut self) -> HBN_RESET_EVENT_W<GLOBAL_SPEC> {
        HBN_RESET_EVENT_W::new(self, 7)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_uart_clk_sel2(&mut self) -> HBN_UART_CLK_SEL2_W<GLOBAL_SPEC> {
        HBN_UART_CLK_SEL2_W::new(self, 15)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ldo11soc_vout_sel_aon(&mut self) -> SW_LDO11SOC_VOUT_SEL_AON_W<GLOBAL_SPEC> {
        SW_LDO11SOC_VOUT_SEL_AON_W::new(self, 16)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ldo11_rt_vout_sel(&mut self) -> SW_LDO11_RT_VOUT_SEL_W<GLOBAL_SPEC> {
        SW_LDO11_RT_VOUT_SEL_W::new(self, 24)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ldo11_aon_vout_sel(&mut self) -> SW_LDO11_AON_VOUT_SEL_W<GLOBAL_SPEC> {
        SW_LDO11_AON_VOUT_SEL_W::new(self, 28)
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
#[doc = "Global hibernate configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`global::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`global::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GLOBAL_SPEC;
impl crate::RegisterSpec for GLOBAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`global::R`](R) reader structure"]
impl crate::Readable for GLOBAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`global::W`](W) writer structure"]
impl crate::Writable for GLOBAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets global to value 0"]
impl crate::Resettable for GLOBAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
