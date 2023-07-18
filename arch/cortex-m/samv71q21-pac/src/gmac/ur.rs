#[doc = "Register `UR` reader"]
pub struct R(crate::R<UR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UR` writer"]
pub struct W(crate::W<UR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<UR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RMII` reader - Reduced MII Mode"]
pub type RMII_R = crate::BitReader;
#[doc = "Field `RMII` writer - Reduced MII Mode"]
pub type RMII_W<'a, const O: u8> = crate::BitWriter<'a, UR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Reduced MII Mode"]
    #[inline(always)]
    pub fn rmii(&self) -> RMII_R {
        RMII_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reduced MII Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rmii(&mut self) -> RMII_W<0> {
        RMII_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "User Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur](index.html) module"]
pub struct UR_SPEC;
impl crate::RegisterSpec for UR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ur::R](R) reader structure"]
impl crate::Readable for UR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ur::W](W) writer structure"]
impl crate::Writable for UR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UR to value 0"]
impl crate::Resettable for UR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
