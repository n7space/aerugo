#[doc = "Register `ST2CW0` reader"]
pub type R = crate::R<ST2CW0_SPEC>;
#[doc = "Register `ST2CW0` writer"]
pub type W = crate::W<ST2CW0_SPEC>;
#[doc = "Field `MASKVAL` reader - Mask Value"]
pub type MASKVAL_R = crate::FieldReader<u16>;
#[doc = "Field `MASKVAL` writer - Mask Value"]
pub type MASKVAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `COMPVAL` reader - Compare Value"]
pub type COMPVAL_R = crate::FieldReader<u16>;
#[doc = "Field `COMPVAL` writer - Compare Value"]
pub type COMPVAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Mask Value"]
    #[inline(always)]
    pub fn maskval(&self) -> MASKVAL_R {
        MASKVAL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Compare Value"]
    #[inline(always)]
    pub fn compval(&self) -> COMPVAL_R {
        COMPVAL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Mask Value"]
    #[inline(always)]
    #[must_use]
    pub fn maskval(&mut self) -> MASKVAL_W<ST2CW0_SPEC, 0> {
        MASKVAL_W::new(self)
    }
    #[doc = "Bits 16:31 - Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn compval(&mut self) -> COMPVAL_W<ST2CW0_SPEC, 16> {
        COMPVAL_W::new(self)
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
#[doc = "Screening Type 2 Compare Word 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st2cw0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st2cw0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST2CW0_SPEC;
impl crate::RegisterSpec for ST2CW0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st2cw0::R`](R) reader structure"]
impl crate::Readable for ST2CW0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`st2cw0::W`](W) writer structure"]
impl crate::Writable for ST2CW0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST2CW0 to value 0"]
impl crate::Resettable for ST2CW0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
