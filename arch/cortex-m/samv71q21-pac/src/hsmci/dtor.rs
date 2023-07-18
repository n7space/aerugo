#[doc = "Register `DTOR` reader"]
pub struct R(crate::R<DTOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTOR` writer"]
pub struct W(crate::W<DTOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTOR_SPEC>;
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
impl From<crate::W<DTOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTOCYC` reader - Data Timeout Cycle Number"]
pub type DTOCYC_R = crate::FieldReader;
#[doc = "Field `DTOCYC` writer - Data Timeout Cycle Number"]
pub type DTOCYC_W<'a, const O: u8> = crate::FieldWriter<'a, DTOR_SPEC, 4, O>;
#[doc = "Field `DTOMUL` reader - Data Timeout Multiplier"]
pub type DTOMUL_R = crate::FieldReader<DTOMULSELECT_A>;
#[doc = "Data Timeout Multiplier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTOMULSELECT_A {
    #[doc = "0: DTOCYC"]
    _1 = 0,
    #[doc = "1: DTOCYC x 16"]
    _16 = 1,
    #[doc = "2: DTOCYC x 128"]
    _128 = 2,
    #[doc = "3: DTOCYC x 256"]
    _256 = 3,
    #[doc = "4: DTOCYC x 1024"]
    _1024 = 4,
    #[doc = "5: DTOCYC x 4096"]
    _4096 = 5,
    #[doc = "6: DTOCYC x 65536"]
    _65536 = 6,
    #[doc = "7: DTOCYC x 1048576"]
    _1048576 = 7,
}
impl From<DTOMULSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DTOMULSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DTOMULSELECT_A {
    type Ux = u8;
}
impl DTOMUL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTOMULSELECT_A {
        match self.bits {
            0 => DTOMULSELECT_A::_1,
            1 => DTOMULSELECT_A::_16,
            2 => DTOMULSELECT_A::_128,
            3 => DTOMULSELECT_A::_256,
            4 => DTOMULSELECT_A::_1024,
            5 => DTOMULSELECT_A::_4096,
            6 => DTOMULSELECT_A::_65536,
            7 => DTOMULSELECT_A::_1048576,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTOMULSELECT_A::_1
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == DTOMULSELECT_A::_16
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == DTOMULSELECT_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == DTOMULSELECT_A::_256
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == DTOMULSELECT_A::_1024
    }
    #[doc = "Checks if the value of the field is `_4096`"]
    #[inline(always)]
    pub fn is_4096(&self) -> bool {
        *self == DTOMULSELECT_A::_4096
    }
    #[doc = "Checks if the value of the field is `_65536`"]
    #[inline(always)]
    pub fn is_65536(&self) -> bool {
        *self == DTOMULSELECT_A::_65536
    }
    #[doc = "Checks if the value of the field is `_1048576`"]
    #[inline(always)]
    pub fn is_1048576(&self) -> bool {
        *self == DTOMULSELECT_A::_1048576
    }
}
#[doc = "Field `DTOMUL` writer - Data Timeout Multiplier"]
pub type DTOMUL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, DTOR_SPEC, 3, O, DTOMULSELECT_A>;
impl<'a, const O: u8> DTOMUL_W<'a, O> {
    #[doc = "DTOCYC"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTOMULSELECT_A::_1)
    }
    #[doc = "DTOCYC x 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(DTOMULSELECT_A::_16)
    }
    #[doc = "DTOCYC x 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(DTOMULSELECT_A::_128)
    }
    #[doc = "DTOCYC x 256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(DTOMULSELECT_A::_256)
    }
    #[doc = "DTOCYC x 1024"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut W {
        self.variant(DTOMULSELECT_A::_1024)
    }
    #[doc = "DTOCYC x 4096"]
    #[inline(always)]
    pub fn _4096(self) -> &'a mut W {
        self.variant(DTOMULSELECT_A::_4096)
    }
    #[doc = "DTOCYC x 65536"]
    #[inline(always)]
    pub fn _65536(self) -> &'a mut W {
        self.variant(DTOMULSELECT_A::_65536)
    }
    #[doc = "DTOCYC x 1048576"]
    #[inline(always)]
    pub fn _1048576(self) -> &'a mut W {
        self.variant(DTOMULSELECT_A::_1048576)
    }
}
impl R {
    #[doc = "Bits 0:3 - Data Timeout Cycle Number"]
    #[inline(always)]
    pub fn dtocyc(&self) -> DTOCYC_R {
        DTOCYC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Data Timeout Multiplier"]
    #[inline(always)]
    pub fn dtomul(&self) -> DTOMUL_R {
        DTOMUL_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Timeout Cycle Number"]
    #[inline(always)]
    #[must_use]
    pub fn dtocyc(&mut self) -> DTOCYC_W<0> {
        DTOCYC_W::new(self)
    }
    #[doc = "Bits 4:6 - Data Timeout Multiplier"]
    #[inline(always)]
    #[must_use]
    pub fn dtomul(&mut self) -> DTOMUL_W<4> {
        DTOMUL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Timeout Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtor](index.html) module"]
pub struct DTOR_SPEC;
impl crate::RegisterSpec for DTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtor::R](R) reader structure"]
impl crate::Readable for DTOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtor::W](W) writer structure"]
impl crate::Writable for DTOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTOR to value 0"]
impl crate::Resettable for DTOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
