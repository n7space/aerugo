#[doc = "Register `US_LONPRIO` reader"]
pub struct R(crate::R<US_LONPRIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_LONPRIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_LONPRIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_LONPRIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `US_LONPRIO` writer"]
pub struct W(crate::W<US_LONPRIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_LONPRIO_SPEC>;
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
impl From<crate::W<US_LONPRIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_LONPRIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSNB` reader - LON Priority Slot Number"]
pub type PSNB_R = crate::FieldReader;
#[doc = "Field `PSNB` writer - LON Priority Slot Number"]
pub type PSNB_W<'a, const O: u8> = crate::FieldWriter<'a, US_LONPRIO_SPEC, 7, O>;
#[doc = "Field `NPS` reader - LON Node Priority Slot"]
pub type NPS_R = crate::FieldReader;
#[doc = "Field `NPS` writer - LON Node Priority Slot"]
pub type NPS_W<'a, const O: u8> = crate::FieldWriter<'a, US_LONPRIO_SPEC, 7, O>;
impl R {
    #[doc = "Bits 0:6 - LON Priority Slot Number"]
    #[inline(always)]
    pub fn psnb(&self) -> PSNB_R {
        PSNB_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - LON Node Priority Slot"]
    #[inline(always)]
    pub fn nps(&self) -> NPS_R {
        NPS_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - LON Priority Slot Number"]
    #[inline(always)]
    #[must_use]
    pub fn psnb(&mut self) -> PSNB_W<0> {
        PSNB_W::new(self)
    }
    #[doc = "Bits 8:14 - LON Node Priority Slot"]
    #[inline(always)]
    #[must_use]
    pub fn nps(&mut self) -> NPS_W<8> {
        NPS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LON Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_lonprio](index.html) module"]
pub struct US_LONPRIO_SPEC;
impl crate::RegisterSpec for US_LONPRIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_lonprio::R](R) reader structure"]
impl crate::Readable for US_LONPRIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [us_lonprio::W](W) writer structure"]
impl crate::Writable for US_LONPRIO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets US_LONPRIO to value 0"]
impl crate::Resettable for US_LONPRIO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
