#[doc = "Register `OCMS` reader"]
pub type R = crate::R<OCMS_SPEC>;
#[doc = "Register `OCMS` writer"]
pub type W = crate::W<OCMS_SPEC>;
#[doc = "Field `SMSE` reader - Static Memory Controller Scrambling Enable"]
pub type SMSE_R = crate::BitReader;
#[doc = "Field `SMSE` writer - Static Memory Controller Scrambling Enable"]
pub type SMSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CS0SE` reader - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type CS0SE_R = crate::BitReader;
#[doc = "Field `CS0SE` writer - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type CS0SE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CS1SE` reader - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type CS1SE_R = crate::BitReader;
#[doc = "Field `CS1SE` writer - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type CS1SE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CS2SE` reader - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type CS2SE_R = crate::BitReader;
#[doc = "Field `CS2SE` writer - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type CS2SE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CS3SE` reader - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type CS3SE_R = crate::BitReader;
#[doc = "Field `CS3SE` writer - Chip Select (x = 0 to 3) Scrambling Enable"]
pub type CS3SE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Static Memory Controller Scrambling Enable"]
    #[inline(always)]
    pub fn smse(&self) -> SMSE_R {
        SMSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    pub fn cs0se(&self) -> CS0SE_R {
        CS0SE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    pub fn cs1se(&self) -> CS1SE_R {
        CS1SE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    pub fn cs2se(&self) -> CS2SE_R {
        CS2SE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    pub fn cs3se(&self) -> CS3SE_R {
        CS3SE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Static Memory Controller Scrambling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smse(&mut self) -> SMSE_W<OCMS_SPEC, 0> {
        SMSE_W::new(self)
    }
    #[doc = "Bit 8 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cs0se(&mut self) -> CS0SE_W<OCMS_SPEC, 8> {
        CS0SE_W::new(self)
    }
    #[doc = "Bit 9 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cs1se(&mut self) -> CS1SE_W<OCMS_SPEC, 9> {
        CS1SE_W::new(self)
    }
    #[doc = "Bit 10 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cs2se(&mut self) -> CS2SE_W<OCMS_SPEC, 10> {
        CS2SE_W::new(self)
    }
    #[doc = "Bit 11 - Chip Select (x = 0 to 3) Scrambling Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cs3se(&mut self) -> CS3SE_W<OCMS_SPEC, 11> {
        CS3SE_W::new(self)
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
#[doc = "SMC Off-Chip Memory Scrambling Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocms::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocms::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCMS_SPEC;
impl crate::RegisterSpec for OCMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ocms::R`](R) reader structure"]
impl crate::Readable for OCMS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ocms::W`](W) writer structure"]
impl crate::Writable for OCMS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCMS to value 0"]
impl crate::Resettable for OCMS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
