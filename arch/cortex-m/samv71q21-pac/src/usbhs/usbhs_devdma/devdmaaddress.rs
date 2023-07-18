#[doc = "Register `DEVDMAADDRESS` reader"]
pub struct R(crate::R<DEVDMAADDRESS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVDMAADDRESS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVDMAADDRESS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVDMAADDRESS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEVDMAADDRESS` writer"]
pub struct W(crate::W<DEVDMAADDRESS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVDMAADDRESS_SPEC>;
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
impl From<crate::W<DEVDMAADDRESS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVDMAADDRESS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUFF_ADD` reader - Buffer Address"]
pub type BUFF_ADD_R = crate::FieldReader<u32>;
#[doc = "Field `BUFF_ADD` writer - Buffer Address"]
pub type BUFF_ADD_W<'a, const O: u8> = crate::FieldWriter<'a, DEVDMAADDRESS_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Address"]
    #[inline(always)]
    pub fn buff_add(&self) -> BUFF_ADD_R {
        BUFF_ADD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Address"]
    #[inline(always)]
    #[must_use]
    pub fn buff_add(&mut self) -> BUFF_ADD_W<0> {
        BUFF_ADD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device DMA Channel Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devdmaaddress](index.html) module"]
pub struct DEVDMAADDRESS_SPEC;
impl crate::RegisterSpec for DEVDMAADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devdmaaddress::R](R) reader structure"]
impl crate::Readable for DEVDMAADDRESS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [devdmaaddress::W](W) writer structure"]
impl crate::Writable for DEVDMAADDRESS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEVDMAADDRESS to value 0"]
impl crate::Resettable for DEVDMAADDRESS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
