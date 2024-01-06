#[doc = "Register `config` reader"]
pub type R = crate::R<CONFIG_SPEC>;
#[doc = "Register `config` writer"]
pub type W = crate::W<CONFIG_SPEC>;
#[doc = "Write 1 to clear internal checksum states\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLEAR_AW {
    #[doc = "1: Write 1 to clear internal states"]
    CLEAR = 1,
}
impl From<CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: CLEAR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `clear` writer - Write 1 to clear internal checksum states"]
pub type CLEAR_W<'a, REG> = crate::BitWriter<'a, REG, CLEAR_AW>;
impl<'a, REG> CLEAR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write 1 to clear internal states"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CLEAR_AW::CLEAR)
    }
}
#[doc = "Field `endian` reader - Sets endian of input data"]
pub type ENDIAN_R = crate::BitReader<ENDIAN_A>;
#[doc = "Sets endian of input data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDIAN_A {
    #[doc = "0: Little endian"]
    LITTLE = 0,
    #[doc = "1: Big endian"]
    BIG = 1,
}
impl From<ENDIAN_A> for bool {
    #[inline(always)]
    fn from(variant: ENDIAN_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDIAN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENDIAN_A {
        match self.bits {
            false => ENDIAN_A::LITTLE,
            true => ENDIAN_A::BIG,
        }
    }
    #[doc = "Little endian"]
    #[inline(always)]
    pub fn is_little(&self) -> bool {
        *self == ENDIAN_A::LITTLE
    }
    #[doc = "Big endian"]
    #[inline(always)]
    pub fn is_big(&self) -> bool {
        *self == ENDIAN_A::BIG
    }
}
#[doc = "Field `endian` writer - Sets endian of input data"]
pub type ENDIAN_W<'a, REG> = crate::BitWriter<'a, REG, ENDIAN_A>;
impl<'a, REG> ENDIAN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Little endian"]
    #[inline(always)]
    pub fn little(self) -> &'a mut crate::W<REG> {
        self.variant(ENDIAN_A::LITTLE)
    }
    #[doc = "Big endian"]
    #[inline(always)]
    pub fn big(self) -> &'a mut crate::W<REG> {
        self.variant(ENDIAN_A::BIG)
    }
}
impl R {
    #[doc = "Bit 1 - Sets endian of input data"]
    #[inline(always)]
    pub fn endian(&self) -> ENDIAN_R {
        ENDIAN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to clear internal checksum states"]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> CLEAR_W<CONFIG_SPEC> {
        CLEAR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Sets endian of input data"]
    #[inline(always)]
    #[must_use]
    pub fn endian(&mut self) -> ENDIAN_W<CONFIG_SPEC> {
        ENDIAN_W::new(self, 1)
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
#[doc = "Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets config to value 0"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
