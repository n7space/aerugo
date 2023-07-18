#[doc = "Register `WPMR` reader"]
pub struct R(crate::R<WPMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WPMR` writer"]
pub struct W(crate::W<WPMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPMR_SPEC>;
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
impl From<crate::W<WPMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WPEN` reader - Write Protection Enable"]
pub type WPEN_R = crate::BitReader;
#[doc = "Field `WPEN` writer - Write Protection Enable"]
pub type WPEN_W<'a, const O: u8> = crate::BitWriter<'a, WPMR_SPEC, O>;
#[doc = "Field `WPKEY` reader - Write Protection Key Password"]
pub type WPKEY_R = crate::FieldReader<WPKEYSELECT_A>;
#[doc = "Write Protection Key Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum WPKEYSELECT_A {
    #[doc = "4805449: Writing any other value in this field aborts the write operation of the WPEN bit.Always reads as 0."]
    PASSWD = 4805449,
}
impl From<WPKEYSELECT_A> for u32 {
    #[inline(always)]
    fn from(variant: WPKEYSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WPKEYSELECT_A {
    type Ux = u32;
}
impl WPKEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WPKEYSELECT_A> {
        match self.bits {
            4805449 => Some(WPKEYSELECT_A::PASSWD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PASSWD`"]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        *self == WPKEYSELECT_A::PASSWD
    }
}
#[doc = "Field `WPKEY` writer - Write Protection Key Password"]
pub type WPKEY_W<'a, const O: u8> = crate::FieldWriter<'a, WPMR_SPEC, 24, O, WPKEYSELECT_A>;
impl<'a, const O: u8> WPKEY_W<'a, O> {
    #[doc = "Writing any other value in this field aborts the write operation of the WPEN bit.Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(WPKEYSELECT_A::PASSWD)
    }
}
impl R {
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline(always)]
    pub fn wpen(&self) -> WPEN_R {
        WPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:31 - Write Protection Key Password"]
    #[inline(always)]
    pub fn wpkey(&self) -> WPKEY_R {
        WPKEY_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wpen(&mut self) -> WPEN_W<0> {
        WPEN_W::new(self)
    }
    #[doc = "Bits 8:31 - Write Protection Key Password"]
    #[inline(always)]
    #[must_use]
    pub fn wpkey(&mut self) -> WPKEY_W<8> {
        WPKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write Protection Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpmr](index.html) module"]
pub struct WPMR_SPEC;
impl crate::RegisterSpec for WPMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpmr::R](R) reader structure"]
impl crate::Readable for WPMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wpmr::W](W) writer structure"]
impl crate::Writable for WPMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WPMR to value 0"]
impl crate::Resettable for WPMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
