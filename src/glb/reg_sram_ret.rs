#[doc = "Register `reg_sram_ret` reader"]
pub type R = crate::R<REG_SRAM_RET_SPEC>;
#[doc = "Register `reg_sram_ret` writer"]
pub type W = crate::W<REG_SRAM_RET_SPEC>;
#[doc = "Field `cr_mcu_cache_ret` reader - "]
pub type CR_MCU_CACHE_RET_R = crate::FieldReader;
#[doc = "Field `cr_mcu_cache_ret` writer - "]
pub type CR_MCU_CACHE_RET_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cr_mcu_hsram_ret` reader - "]
pub type CR_MCU_HSRAM_RET_R = crate::FieldReader;
#[doc = "Field `cr_mcu_hsram_ret` writer - "]
pub type CR_MCU_HSRAM_RET_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cr_wb_ram_ret` reader - "]
pub type CR_WB_RAM_RET_R = crate::BitReader;
#[doc = "Field `cr_wb_ram_ret` writer - "]
pub type CR_WB_RAM_RET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_misc_ram_ret` reader - "]
pub type CR_MISC_RAM_RET_R = crate::FieldReader;
#[doc = "Field `cr_misc_ram_ret` writer - "]
pub type CR_MISC_RAM_RET_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cr_mcu_cache_ret(&self) -> CR_MCU_CACHE_RET_R {
        CR_MCU_CACHE_RET_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn cr_mcu_hsram_ret(&self) -> CR_MCU_HSRAM_RET_R {
        CR_MCU_HSRAM_RET_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_wb_ram_ret(&self) -> CR_WB_RAM_RET_R {
        CR_WB_RAM_RET_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn cr_misc_ram_ret(&self) -> CR_MISC_RAM_RET_R {
        CR_MISC_RAM_RET_R::new(((self.bits >> 9) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn cr_mcu_cache_ret(&mut self) -> CR_MCU_CACHE_RET_W<REG_SRAM_RET_SPEC> {
        CR_MCU_CACHE_RET_W::new(self, 0)
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    #[must_use]
    pub fn cr_mcu_hsram_ret(&mut self) -> CR_MCU_HSRAM_RET_W<REG_SRAM_RET_SPEC> {
        CR_MCU_HSRAM_RET_W::new(self, 2)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn cr_wb_ram_ret(&mut self) -> CR_WB_RAM_RET_W<REG_SRAM_RET_SPEC> {
        CR_WB_RAM_RET_W::new(self, 8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    #[must_use]
    pub fn cr_misc_ram_ret(&mut self) -> CR_MISC_RAM_RET_W<REG_SRAM_RET_SPEC> {
        CR_MISC_RAM_RET_W::new(self, 9)
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
#[doc = "reg_sram_ret.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_sram_ret::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_sram_ret::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REG_SRAM_RET_SPEC;
impl crate::RegisterSpec for REG_SRAM_RET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sram_ret::R`](R) reader structure"]
impl crate::Readable for REG_SRAM_RET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reg_sram_ret::W`](W) writer structure"]
impl crate::Writable for REG_SRAM_RET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets reg_sram_ret to value 0"]
impl crate::Resettable for REG_SRAM_RET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
