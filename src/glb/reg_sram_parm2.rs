#[doc = "Register `reg_sram_parm2` reader"]
pub type R = crate::R<REG_SRAM_PARM2_SPEC>;
#[doc = "Register `reg_sram_parm2` writer"]
pub type W = crate::W<REG_SRAM_PARM2_SPEC>;
#[doc = "Field `cr_mcu_cache_dvs` reader - "]
pub type CR_MCU_CACHE_DVS_R = crate::FieldReader;
#[doc = "Field `cr_mcu_cache_dvs` writer - "]
pub type CR_MCU_CACHE_DVS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cr_mcu_hsram_dvs` reader - "]
pub type CR_MCU_HSRAM_DVS_R = crate::FieldReader;
#[doc = "Field `cr_mcu_hsram_dvs` writer - "]
pub type CR_MCU_HSRAM_DVS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cr_mcu_rom_dvs` reader - "]
pub type CR_MCU_ROM_DVS_R = crate::FieldReader;
#[doc = "Field `cr_mcu_rom_dvs` writer - "]
pub type CR_MCU_ROM_DVS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cr_wb_ram_dvs` reader - "]
pub type CR_WB_RAM_DVS_R = crate::FieldReader;
#[doc = "Field `cr_wb_ram_dvs` writer - "]
pub type CR_WB_RAM_DVS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cr_misc_ram_dvs` reader - "]
pub type CR_MISC_RAM_DVS_R = crate::FieldReader;
#[doc = "Field `cr_misc_ram_dvs` writer - "]
pub type CR_MISC_RAM_DVS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cr_ocram_dvs` reader - "]
pub type CR_OCRAM_DVS_R = crate::FieldReader;
#[doc = "Field `cr_ocram_dvs` writer - "]
pub type CR_OCRAM_DVS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cr_wram_dvs` reader - "]
pub type CR_WRAM_DVS_R = crate::FieldReader;
#[doc = "Field `cr_wram_dvs` writer - "]
pub type CR_WRAM_DVS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cr_mcu_cache_dvs(&self) -> CR_MCU_CACHE_DVS_R {
        CR_MCU_CACHE_DVS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn cr_mcu_hsram_dvs(&self) -> CR_MCU_HSRAM_DVS_R {
        CR_MCU_HSRAM_DVS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn cr_mcu_rom_dvs(&self) -> CR_MCU_ROM_DVS_R {
        CR_MCU_ROM_DVS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn cr_wb_ram_dvs(&self) -> CR_WB_RAM_DVS_R {
        CR_WB_RAM_DVS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn cr_misc_ram_dvs(&self) -> CR_MISC_RAM_DVS_R {
        CR_MISC_RAM_DVS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn cr_ocram_dvs(&self) -> CR_OCRAM_DVS_R {
        CR_OCRAM_DVS_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn cr_wram_dvs(&self) -> CR_WRAM_DVS_R {
        CR_WRAM_DVS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn cr_mcu_cache_dvs(&mut self) -> CR_MCU_CACHE_DVS_W<REG_SRAM_PARM2_SPEC> {
        CR_MCU_CACHE_DVS_W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn cr_mcu_hsram_dvs(&mut self) -> CR_MCU_HSRAM_DVS_W<REG_SRAM_PARM2_SPEC> {
        CR_MCU_HSRAM_DVS_W::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn cr_mcu_rom_dvs(&mut self) -> CR_MCU_ROM_DVS_W<REG_SRAM_PARM2_SPEC> {
        CR_MCU_ROM_DVS_W::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn cr_wb_ram_dvs(&mut self) -> CR_WB_RAM_DVS_W<REG_SRAM_PARM2_SPEC> {
        CR_WB_RAM_DVS_W::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn cr_misc_ram_dvs(&mut self) -> CR_MISC_RAM_DVS_W<REG_SRAM_PARM2_SPEC> {
        CR_MISC_RAM_DVS_W::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn cr_ocram_dvs(&mut self) -> CR_OCRAM_DVS_W<REG_SRAM_PARM2_SPEC> {
        CR_OCRAM_DVS_W::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn cr_wram_dvs(&mut self) -> CR_WRAM_DVS_W<REG_SRAM_PARM2_SPEC> {
        CR_WRAM_DVS_W::new(self, 24)
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
#[doc = "reg_sram_parm2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_sram_parm2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_sram_parm2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REG_SRAM_PARM2_SPEC;
impl crate::RegisterSpec for REG_SRAM_PARM2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sram_parm2::R`](R) reader structure"]
impl crate::Readable for REG_SRAM_PARM2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reg_sram_parm2::W`](W) writer structure"]
impl crate::Writable for REG_SRAM_PARM2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets reg_sram_parm2 to value 0"]
impl crate::Resettable for REG_SRAM_PARM2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
