#[doc = "Register `emac_config` reader"]
pub type R = crate::R<EMAC_CONFIG_SPEC>;
#[doc = "Register `emac_config` writer"]
pub type W = crate::W<EMAC_CONFIG_SPEC>;
#[doc = "Field `cfg_sel_eth_ref_clk_o` reader - "]
pub type CFG_SEL_ETH_REF_CLK_O_R = crate::BitReader;
#[doc = "Field `cfg_sel_eth_ref_clk_o` writer - "]
pub type CFG_SEL_ETH_REF_CLK_O_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_inv_eth_ref_clk_o` reader - "]
pub type CFG_INV_ETH_REF_CLK_O_R = crate::BitReader;
#[doc = "Field `cfg_inv_eth_ref_clk_o` writer - "]
pub type CFG_INV_ETH_REF_CLK_O_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_inv_eth_tx_clk` reader - "]
pub type CFG_INV_ETH_TX_CLK_R = crate::BitReader;
#[doc = "Field `cfg_inv_eth_tx_clk` writer - "]
pub type CFG_INV_ETH_TX_CLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfg_inv_eth_rx_clk` reader - "]
pub type CFG_INV_ETH_RX_CLK_R = crate::BitReader;
#[doc = "Field `cfg_inv_eth_rx_clk` writer - "]
pub type CFG_INV_ETH_RX_CLK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cfg_sel_eth_ref_clk_o(&self) -> CFG_SEL_ETH_REF_CLK_O_R {
        CFG_SEL_ETH_REF_CLK_O_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cfg_inv_eth_ref_clk_o(&self) -> CFG_INV_ETH_REF_CLK_O_R {
        CFG_INV_ETH_REF_CLK_O_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cfg_inv_eth_tx_clk(&self) -> CFG_INV_ETH_TX_CLK_R {
        CFG_INV_ETH_TX_CLK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cfg_inv_eth_rx_clk(&self) -> CFG_INV_ETH_RX_CLK_R {
        CFG_INV_ETH_RX_CLK_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_sel_eth_ref_clk_o(&mut self) -> CFG_SEL_ETH_REF_CLK_O_W<EMAC_CONFIG_SPEC> {
        CFG_SEL_ETH_REF_CLK_O_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_inv_eth_ref_clk_o(&mut self) -> CFG_INV_ETH_REF_CLK_O_W<EMAC_CONFIG_SPEC> {
        CFG_INV_ETH_REF_CLK_O_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_inv_eth_tx_clk(&mut self) -> CFG_INV_ETH_TX_CLK_W<EMAC_CONFIG_SPEC> {
        CFG_INV_ETH_TX_CLK_W::new(self, 7)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_inv_eth_rx_clk(&mut self) -> CFG_INV_ETH_RX_CLK_W<EMAC_CONFIG_SPEC> {
        CFG_INV_ETH_RX_CLK_W::new(self, 10)
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
#[doc = "Ethernet Media Access Control configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emac_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emac_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMAC_CONFIG_SPEC;
impl crate::RegisterSpec for EMAC_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emac_config::R`](R) reader structure"]
impl crate::Readable for EMAC_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emac_config::W`](W) writer structure"]
impl crate::Writable for EMAC_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets emac_config to value 0"]
impl crate::Resettable for EMAC_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
