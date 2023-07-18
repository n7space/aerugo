#[doc = "Register `HCTL` reader"]
pub struct R(crate::R<HCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCTL` writer"]
pub struct W(crate::W<HCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCTL_SPEC>;
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
impl From<crate::W<HCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RST0` reader - Address Generation Unit 0 Software Reset"]
pub type RST0_R = crate::BitReader;
#[doc = "Field `RST0` writer - Address Generation Unit 0 Software Reset"]
pub type RST0_W<'a, const O: u8> = crate::BitWriter<'a, HCTL_SPEC, O>;
#[doc = "Field `RST1` reader - Address Generation Unit 1 Software Reset"]
pub type RST1_R = crate::BitReader;
#[doc = "Field `RST1` writer - Address Generation Unit 1 Software Reset"]
pub type RST1_W<'a, const O: u8> = crate::BitWriter<'a, HCTL_SPEC, O>;
#[doc = "Field `EN` reader - HBI Enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - HBI Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, HCTL_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Address Generation Unit 0 Software Reset"]
    #[inline(always)]
    pub fn rst0(&self) -> RST0_R {
        RST0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address Generation Unit 1 Software Reset"]
    #[inline(always)]
    pub fn rst1(&self) -> RST1_R {
        RST1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 15 - HBI Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Address Generation Unit 0 Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst0(&mut self) -> RST0_W<0> {
        RST0_W::new(self)
    }
    #[doc = "Bit 1 - Address Generation Unit 1 Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst1(&mut self) -> RST1_W<1> {
        RST1_W::new(self)
    }
    #[doc = "Bit 15 - HBI Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<15> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HBI Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hctl](index.html) module"]
pub struct HCTL_SPEC;
impl crate::RegisterSpec for HCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hctl::R](R) reader structure"]
impl crate::Readable for HCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hctl::W](W) writer structure"]
impl crate::Writable for HCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCTL to value 0"]
impl crate::Resettable for HCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
