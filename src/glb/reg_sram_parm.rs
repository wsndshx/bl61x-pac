#[doc = "Register `reg_sram_parm` reader"]
pub type R = crate::R<REG_SRAM_PARM_SPEC>;
#[doc = "Register `reg_sram_parm` writer"]
pub type W = crate::W<REG_SRAM_PARM_SPEC>;
#[doc = "Field `cr_mcu_cache_dvse` reader - "]
pub type CR_MCU_CACHE_DVSE_R = crate::BitReader;
#[doc = "Field `cr_mcu_cache_dvse` writer - "]
pub type CR_MCU_CACHE_DVSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_mcu_hsram_dvse` reader - "]
pub type CR_MCU_HSRAM_DVSE_R = crate::BitReader;
#[doc = "Field `cr_mcu_hsram_dvse` writer - "]
pub type CR_MCU_HSRAM_DVSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_mcu_rom_dvse` reader - "]
pub type CR_MCU_ROM_DVSE_R = crate::BitReader;
#[doc = "Field `cr_mcu_rom_dvse` writer - "]
pub type CR_MCU_ROM_DVSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_wb_ram_dvse` reader - "]
pub type CR_WB_RAM_DVSE_R = crate::BitReader;
#[doc = "Field `cr_wb_ram_dvse` writer - "]
pub type CR_WB_RAM_DVSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_misc_ram_dvse` reader - "]
pub type CR_MISC_RAM_DVSE_R = crate::BitReader;
#[doc = "Field `cr_misc_ram_dvse` writer - "]
pub type CR_MISC_RAM_DVSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_ocram_dvse` reader - "]
pub type CR_OCRAM_DVSE_R = crate::BitReader;
#[doc = "Field `cr_ocram_dvse` writer - "]
pub type CR_OCRAM_DVSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_wram_dvse` reader - "]
pub type CR_WRAM_DVSE_R = crate::BitReader;
#[doc = "Field `cr_wram_dvse` writer - "]
pub type CR_WRAM_DVSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_mcu_cache_nap` reader - "]
pub type CR_MCU_CACHE_NAP_R = crate::BitReader;
#[doc = "Field `cr_mcu_cache_nap` writer - "]
pub type CR_MCU_CACHE_NAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_mcu_hsram_nap` reader - "]
pub type CR_MCU_HSRAM_NAP_R = crate::BitReader;
#[doc = "Field `cr_mcu_hsram_nap` writer - "]
pub type CR_MCU_HSRAM_NAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_wb_ram_nap` reader - "]
pub type CR_WB_RAM_NAP_R = crate::BitReader;
#[doc = "Field `cr_wb_ram_nap` writer - "]
pub type CR_WB_RAM_NAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_misc_ram_nap` reader - "]
pub type CR_MISC_RAM_NAP_R = crate::BitReader;
#[doc = "Field `cr_misc_ram_nap` writer - "]
pub type CR_MISC_RAM_NAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_ocram_nap` reader - "]
pub type CR_OCRAM_NAP_R = crate::BitReader;
#[doc = "Field `cr_ocram_nap` writer - "]
pub type CR_OCRAM_NAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_wram_nap` reader - "]
pub type CR_WRAM_NAP_R = crate::BitReader;
#[doc = "Field `cr_wram_nap` writer - "]
pub type CR_WRAM_NAP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_mcu_cache_dvse(&self) -> CR_MCU_CACHE_DVSE_R {
        CR_MCU_CACHE_DVSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_mcu_hsram_dvse(&self) -> CR_MCU_HSRAM_DVSE_R {
        CR_MCU_HSRAM_DVSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_mcu_rom_dvse(&self) -> CR_MCU_ROM_DVSE_R {
        CR_MCU_ROM_DVSE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_wb_ram_dvse(&self) -> CR_WB_RAM_DVSE_R {
        CR_WB_RAM_DVSE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_misc_ram_dvse(&self) -> CR_MISC_RAM_DVSE_R {
        CR_MISC_RAM_DVSE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_ocram_dvse(&self) -> CR_OCRAM_DVSE_R {
        CR_OCRAM_DVSE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_wram_dvse(&self) -> CR_WRAM_DVSE_R {
        CR_WRAM_DVSE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_mcu_cache_nap(&self) -> CR_MCU_CACHE_NAP_R {
        CR_MCU_CACHE_NAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_mcu_hsram_nap(&self) -> CR_MCU_HSRAM_NAP_R {
        CR_MCU_HSRAM_NAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_wb_ram_nap(&self) -> CR_WB_RAM_NAP_R {
        CR_WB_RAM_NAP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_misc_ram_nap(&self) -> CR_MISC_RAM_NAP_R {
        CR_MISC_RAM_NAP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_ocram_nap(&self) -> CR_OCRAM_NAP_R {
        CR_OCRAM_NAP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_wram_nap(&self) -> CR_WRAM_NAP_R {
        CR_WRAM_NAP_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cr_mcu_cache_dvse(&mut self) -> CR_MCU_CACHE_DVSE_W<REG_SRAM_PARM_SPEC> {
        CR_MCU_CACHE_DVSE_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cr_mcu_hsram_dvse(&mut self) -> CR_MCU_HSRAM_DVSE_W<REG_SRAM_PARM_SPEC> {
        CR_MCU_HSRAM_DVSE_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cr_mcu_rom_dvse(&mut self) -> CR_MCU_ROM_DVSE_W<REG_SRAM_PARM_SPEC> {
        CR_MCU_ROM_DVSE_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cr_wb_ram_dvse(&mut self) -> CR_WB_RAM_DVSE_W<REG_SRAM_PARM_SPEC> {
        CR_WB_RAM_DVSE_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cr_misc_ram_dvse(&mut self) -> CR_MISC_RAM_DVSE_W<REG_SRAM_PARM_SPEC> {
        CR_MISC_RAM_DVSE_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn cr_ocram_dvse(&mut self) -> CR_OCRAM_DVSE_W<REG_SRAM_PARM_SPEC> {
        CR_OCRAM_DVSE_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn cr_wram_dvse(&mut self) -> CR_WRAM_DVSE_W<REG_SRAM_PARM_SPEC> {
        CR_WRAM_DVSE_W::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn cr_mcu_cache_nap(&mut self) -> CR_MCU_CACHE_NAP_W<REG_SRAM_PARM_SPEC> {
        CR_MCU_CACHE_NAP_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn cr_mcu_hsram_nap(&mut self) -> CR_MCU_HSRAM_NAP_W<REG_SRAM_PARM_SPEC> {
        CR_MCU_HSRAM_NAP_W::new(self, 9)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn cr_wb_ram_nap(&mut self) -> CR_WB_RAM_NAP_W<REG_SRAM_PARM_SPEC> {
        CR_WB_RAM_NAP_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn cr_misc_ram_nap(&mut self) -> CR_MISC_RAM_NAP_W<REG_SRAM_PARM_SPEC> {
        CR_MISC_RAM_NAP_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn cr_ocram_nap(&mut self) -> CR_OCRAM_NAP_W<REG_SRAM_PARM_SPEC> {
        CR_OCRAM_NAP_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn cr_wram_nap(&mut self) -> CR_WRAM_NAP_W<REG_SRAM_PARM_SPEC> {
        CR_WRAM_NAP_W::new(self, 14)
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
#[doc = "reg_sram_parm.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_sram_parm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_sram_parm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REG_SRAM_PARM_SPEC;
impl crate::RegisterSpec for REG_SRAM_PARM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sram_parm::R`](R) reader structure"]
impl crate::Readable for REG_SRAM_PARM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reg_sram_parm::W`](W) writer structure"]
impl crate::Writable for REG_SRAM_PARM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets reg_sram_parm to value 0"]
impl crate::Resettable for REG_SRAM_PARM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
