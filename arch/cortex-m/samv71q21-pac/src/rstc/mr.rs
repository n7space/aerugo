#[doc = "Register `MR` reader"]
pub struct R(crate::R<MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR` writer"]
pub struct W(crate::W<MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MR_SPEC>;
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
impl From<crate::W<MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `URSTEN` reader - User Reset Enable"]
pub type URSTEN_R = crate::BitReader;
#[doc = "Field `URSTEN` writer - User Reset Enable"]
pub type URSTEN_W<'a, const O: u8> = crate::BitWriter<'a, MR_SPEC, O>;
#[doc = "Field `URSTIEN` reader - User Reset Interrupt Enable"]
pub type URSTIEN_R = crate::BitReader;
#[doc = "Field `URSTIEN` writer - User Reset Interrupt Enable"]
pub type URSTIEN_W<'a, const O: u8> = crate::BitWriter<'a, MR_SPEC, O>;
#[doc = "Field `ERSTL` reader - External Reset Length"]
pub type ERSTL_R = crate::FieldReader;
#[doc = "Field `ERSTL` writer - External Reset Length"]
pub type ERSTL_W<'a, const O: u8> = crate::FieldWriter<'a, MR_SPEC, 4, O>;
#[doc = "Field `KEY` reader - Write Access Password"]
pub type KEY_R = crate::FieldReader<KEYSELECT_A>;
#[doc = "Write Access Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEYSELECT_A {
    #[doc = "165: Writing any other value in this field aborts the write operation.Always reads as 0."]
    PASSWD = 165,
}
impl From<KEYSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: KEYSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KEYSELECT_A {
    type Ux = u8;
}
impl KEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KEYSELECT_A> {
        match self.bits {
            165 => Some(KEYSELECT_A::PASSWD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PASSWD`"]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        *self == KEYSELECT_A::PASSWD
    }
}
#[doc = "Field `KEY` writer - Write Access Password"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, MR_SPEC, 8, O, KEYSELECT_A>;
impl<'a, const O: u8> KEY_W<'a, O> {
    #[doc = "Writing any other value in this field aborts the write operation.Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(KEYSELECT_A::PASSWD)
    }
}
impl R {
    #[doc = "Bit 0 - User Reset Enable"]
    #[inline(always)]
    pub fn ursten(&self) -> URSTEN_R {
        URSTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - User Reset Interrupt Enable"]
    #[inline(always)]
    pub fn urstien(&self) -> URSTIEN_R {
        URSTIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - External Reset Length"]
    #[inline(always)]
    pub fn erstl(&self) -> ERSTL_R {
        ERSTL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Write Access Password"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - User Reset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ursten(&mut self) -> URSTEN_W<0> {
        URSTEN_W::new(self)
    }
    #[doc = "Bit 4 - User Reset Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn urstien(&mut self) -> URSTIEN_W<4> {
        URSTIEN_W::new(self)
    }
    #[doc = "Bits 8:11 - External Reset Length"]
    #[inline(always)]
    #[must_use]
    pub fn erstl(&mut self) -> ERSTL_W<8> {
        ERSTL_W::new(self)
    }
    #[doc = "Bits 24:31 - Write Access Password"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<24> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr::R](R) reader structure"]
impl crate::Readable for MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr::W](W) writer structure"]
impl crate::Writable for MR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
