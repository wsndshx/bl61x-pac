#[doc = "Register `rc32m_ctrl2` reader"]
pub type R = crate::R<RC32M_CTRL2_SPEC>;
#[doc = "Register `rc32m_ctrl2` writer"]
pub type W = crate::W<RC32M_CTRL2_SPEC>;
#[doc = "Field `rc32m_code_fr_ext2` reader - "]
pub type RC32M_CODE_FR_EXT2_R = crate::FieldReader;
#[doc = "Field `rc32m_code_fr_ext2` writer - "]
pub type RC32M_CODE_FR_EXT2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `rc32m_ext_code_sel` reader - "]
pub type RC32M_EXT_CODE_SEL_R = crate::BitReader;
#[doc = "Field `rc32m_ext_code_sel` writer - "]
pub type RC32M_EXT_CODE_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 22:29"]
    #[inline(always)]
    pub fn rc32m_code_fr_ext2(&self) -> RC32M_CODE_FR_EXT2_R {
        RC32M_CODE_FR_EXT2_R::new(((self.bits >> 22) & 0xff) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rc32m_ext_code_sel(&self) -> RC32M_EXT_CODE_SEL_R {
        RC32M_EXT_CODE_SEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 22:29"]
    #[inline(always)]
    #[must_use]
    pub fn rc32m_code_fr_ext2(&mut self) -> RC32M_CODE_FR_EXT2_W<RC32M_CTRL2_SPEC> {
        RC32M_CODE_FR_EXT2_W::new(self, 22)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn rc32m_ext_code_sel(&mut self) -> RC32M_EXT_CODE_SEL_W<RC32M_CTRL2_SPEC> {
        RC32M_EXT_CODE_SEL_W::new(self, 31)
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
#[doc = "rc32m_ctrl2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc32m_ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rc32m_ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RC32M_CTRL2_SPEC;
impl crate::RegisterSpec for RC32M_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rc32m_ctrl2::R`](R) reader structure"]
impl crate::Readable for RC32M_CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rc32m_ctrl2::W`](W) writer structure"]
impl crate::Writable for RC32M_CTRL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rc32m_ctrl2 to value 0"]
impl crate::Resettable for RC32M_CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
