#[doc = "Register `swrst_cfg3` reader"]
pub type R = crate::R<SWRST_CFG3_SPEC>;
#[doc = "Register `swrst_cfg3` writer"]
pub type W = crate::W<SWRST_CFG3_SPEC>;
#[doc = "Field `disrst_s12` reader - "]
pub type DISRST_S12_R = crate::BitReader;
#[doc = "Field `disrst_s12` writer - "]
pub type DISRST_S12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disrst_s14` reader - "]
pub type DISRST_S14_R = crate::BitReader;
#[doc = "Field `disrst_s14` writer - "]
pub type DISRST_S14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disrst_s18` reader - "]
pub type DISRST_S18_R = crate::BitReader;
#[doc = "Field `disrst_s18` writer - "]
pub type DISRST_S18_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disrst_s1b` reader - "]
pub type DISRST_S1B_R = crate::BitReader;
#[doc = "Field `disrst_s1b` writer - "]
pub type DISRST_S1B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disrst_s1a0` reader - "]
pub type DISRST_S1A0_R = crate::BitReader;
#[doc = "Field `disrst_s1a0` writer - "]
pub type DISRST_S1A0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disrst_s1a1` reader - "]
pub type DISRST_S1A1_R = crate::BitReader;
#[doc = "Field `disrst_s1a1` writer - "]
pub type DISRST_S1A1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disrst_s1a2` reader - "]
pub type DISRST_S1A2_R = crate::BitReader;
#[doc = "Field `disrst_s1a2` writer - "]
pub type DISRST_S1A2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disrst_s1a3` reader - "]
pub type DISRST_S1A3_R = crate::BitReader;
#[doc = "Field `disrst_s1a3` writer - "]
pub type DISRST_S1A3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disrst_s1a4` reader - "]
pub type DISRST_S1A4_R = crate::BitReader;
#[doc = "Field `disrst_s1a4` writer - "]
pub type DISRST_S1A4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disrst_s1a5` reader - "]
pub type DISRST_S1A5_R = crate::BitReader;
#[doc = "Field `disrst_s1a5` writer - "]
pub type DISRST_S1A5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disrst_s1a6` reader - "]
pub type DISRST_S1A6_R = crate::BitReader;
#[doc = "Field `disrst_s1a6` writer - "]
pub type DISRST_S1A6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disrst_s1a7` reader - "]
pub type DISRST_S1A7_R = crate::BitReader;
#[doc = "Field `disrst_s1a7` writer - "]
pub type DISRST_S1A7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disrst_s1a8` reader - "]
pub type DISRST_S1A8_R = crate::BitReader;
#[doc = "Field `disrst_s1a8` writer - "]
pub type DISRST_S1A8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disrst_s1a9` reader - "]
pub type DISRST_S1A9_R = crate::BitReader;
#[doc = "Field `disrst_s1a9` writer - "]
pub type DISRST_S1A9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `disrst_s1aa` reader - "]
pub type DISRST_S1AA_R = crate::BitReader;
#[doc = "Field `disrst_s1aa` writer - "]
pub type DISRST_S1AA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn disrst_s12(&self) -> DISRST_S12_R {
        DISRST_S12_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn disrst_s14(&self) -> DISRST_S14_R {
        DISRST_S14_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn disrst_s18(&self) -> DISRST_S18_R {
        DISRST_S18_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn disrst_s1b(&self) -> DISRST_S1B_R {
        DISRST_S1B_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn disrst_s1a0(&self) -> DISRST_S1A0_R {
        DISRST_S1A0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn disrst_s1a1(&self) -> DISRST_S1A1_R {
        DISRST_S1A1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn disrst_s1a2(&self) -> DISRST_S1A2_R {
        DISRST_S1A2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn disrst_s1a3(&self) -> DISRST_S1A3_R {
        DISRST_S1A3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn disrst_s1a4(&self) -> DISRST_S1A4_R {
        DISRST_S1A4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn disrst_s1a5(&self) -> DISRST_S1A5_R {
        DISRST_S1A5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn disrst_s1a6(&self) -> DISRST_S1A6_R {
        DISRST_S1A6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn disrst_s1a7(&self) -> DISRST_S1A7_R {
        DISRST_S1A7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn disrst_s1a8(&self) -> DISRST_S1A8_R {
        DISRST_S1A8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn disrst_s1a9(&self) -> DISRST_S1A9_R {
        DISRST_S1A9_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn disrst_s1aa(&self) -> DISRST_S1AA_R {
        DISRST_S1AA_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn disrst_s12(&mut self) -> DISRST_S12_W<SWRST_CFG3_SPEC> {
        DISRST_S12_W::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn disrst_s14(&mut self) -> DISRST_S14_W<SWRST_CFG3_SPEC> {
        DISRST_S14_W::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn disrst_s18(&mut self) -> DISRST_S18_W<SWRST_CFG3_SPEC> {
        DISRST_S18_W::new(self, 8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn disrst_s1b(&mut self) -> DISRST_S1B_W<SWRST_CFG3_SPEC> {
        DISRST_S1B_W::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn disrst_s1a0(&mut self) -> DISRST_S1A0_W<SWRST_CFG3_SPEC> {
        DISRST_S1A0_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn disrst_s1a1(&mut self) -> DISRST_S1A1_W<SWRST_CFG3_SPEC> {
        DISRST_S1A1_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn disrst_s1a2(&mut self) -> DISRST_S1A2_W<SWRST_CFG3_SPEC> {
        DISRST_S1A2_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn disrst_s1a3(&mut self) -> DISRST_S1A3_W<SWRST_CFG3_SPEC> {
        DISRST_S1A3_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn disrst_s1a4(&mut self) -> DISRST_S1A4_W<SWRST_CFG3_SPEC> {
        DISRST_S1A4_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn disrst_s1a5(&mut self) -> DISRST_S1A5_W<SWRST_CFG3_SPEC> {
        DISRST_S1A5_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn disrst_s1a6(&mut self) -> DISRST_S1A6_W<SWRST_CFG3_SPEC> {
        DISRST_S1A6_W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn disrst_s1a7(&mut self) -> DISRST_S1A7_W<SWRST_CFG3_SPEC> {
        DISRST_S1A7_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn disrst_s1a8(&mut self) -> DISRST_S1A8_W<SWRST_CFG3_SPEC> {
        DISRST_S1A8_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn disrst_s1a9(&mut self) -> DISRST_S1A9_W<SWRST_CFG3_SPEC> {
        DISRST_S1A9_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn disrst_s1aa(&mut self) -> DISRST_S1AA_W<SWRST_CFG3_SPEC> {
        DISRST_S1AA_W::new(self, 26)
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
#[doc = "Disable hreset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swrst_cfg3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swrst_cfg3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWRST_CFG3_SPEC;
impl crate::RegisterSpec for SWRST_CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swrst_cfg3::R`](R) reader structure"]
impl crate::Readable for SWRST_CFG3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swrst_cfg3::W`](W) writer structure"]
impl crate::Writable for SWRST_CFG3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets swrst_cfg3 to value 0"]
impl crate::Resettable for SWRST_CFG3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
