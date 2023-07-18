#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WBDIS` reader - Write Back Disable"]
pub type WBDIS_R = crate::BitReader;
#[doc = "Field `WBDIS` writer - Write Back Disable"]
pub type WBDIS_W<'a, const O: u8> = crate::BitWriter<'a, CFG_SPEC, O>;
#[doc = "Field `EOMDIS` reader - End of Monitoring Disable"]
pub type EOMDIS_R = crate::BitReader;
#[doc = "Field `EOMDIS` writer - End of Monitoring Disable"]
pub type EOMDIS_W<'a, const O: u8> = crate::BitWriter<'a, CFG_SPEC, O>;
#[doc = "Field `SLBDIS` reader - Secondary List Branching Disable"]
pub type SLBDIS_R = crate::BitReader;
#[doc = "Field `SLBDIS` writer - Secondary List Branching Disable"]
pub type SLBDIS_W<'a, const O: u8> = crate::BitWriter<'a, CFG_SPEC, O>;
#[doc = "Field `BBC` reader - Bus Burden Control"]
pub type BBC_R = crate::FieldReader;
#[doc = "Field `BBC` writer - Bus Burden Control"]
pub type BBC_W<'a, const O: u8> = crate::FieldWriter<'a, CFG_SPEC, 4, O>;
#[doc = "Field `ASCD` reader - Automatic Switch To Compare Digest"]
pub type ASCD_R = crate::BitReader;
#[doc = "Field `ASCD` writer - Automatic Switch To Compare Digest"]
pub type ASCD_W<'a, const O: u8> = crate::BitWriter<'a, CFG_SPEC, O>;
#[doc = "Field `DUALBUFF` reader - Dual Input Buffer"]
pub type DUALBUFF_R = crate::BitReader;
#[doc = "Field `DUALBUFF` writer - Dual Input Buffer"]
pub type DUALBUFF_W<'a, const O: u8> = crate::BitWriter<'a, CFG_SPEC, O>;
#[doc = "Field `UIHASH` reader - User Initial Hash Value"]
pub type UIHASH_R = crate::BitReader;
#[doc = "Field `UIHASH` writer - User Initial Hash Value"]
pub type UIHASH_W<'a, const O: u8> = crate::BitWriter<'a, CFG_SPEC, O>;
#[doc = "Field `UALGO` reader - User SHA Algorithm"]
pub type UALGO_R = crate::FieldReader<UALGOSELECT_A>;
#[doc = "User SHA Algorithm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UALGOSELECT_A {
    #[doc = "0: SHA1 algorithm processed"]
    SHA1 = 0,
    #[doc = "1: SHA256 algorithm processed"]
    SHA256 = 1,
    #[doc = "4: SHA224 algorithm processed"]
    SHA224 = 4,
}
impl From<UALGOSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: UALGOSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UALGOSELECT_A {
    type Ux = u8;
}
impl UALGO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UALGOSELECT_A> {
        match self.bits {
            0 => Some(UALGOSELECT_A::SHA1),
            1 => Some(UALGOSELECT_A::SHA256),
            4 => Some(UALGOSELECT_A::SHA224),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SHA1`"]
    #[inline(always)]
    pub fn is_sha1(&self) -> bool {
        *self == UALGOSELECT_A::SHA1
    }
    #[doc = "Checks if the value of the field is `SHA256`"]
    #[inline(always)]
    pub fn is_sha256(&self) -> bool {
        *self == UALGOSELECT_A::SHA256
    }
    #[doc = "Checks if the value of the field is `SHA224`"]
    #[inline(always)]
    pub fn is_sha224(&self) -> bool {
        *self == UALGOSELECT_A::SHA224
    }
}
#[doc = "Field `UALGO` writer - User SHA Algorithm"]
pub type UALGO_W<'a, const O: u8> = crate::FieldWriter<'a, CFG_SPEC, 3, O, UALGOSELECT_A>;
impl<'a, const O: u8> UALGO_W<'a, O> {
    #[doc = "SHA1 algorithm processed"]
    #[inline(always)]
    pub fn sha1(self) -> &'a mut W {
        self.variant(UALGOSELECT_A::SHA1)
    }
    #[doc = "SHA256 algorithm processed"]
    #[inline(always)]
    pub fn sha256(self) -> &'a mut W {
        self.variant(UALGOSELECT_A::SHA256)
    }
    #[doc = "SHA224 algorithm processed"]
    #[inline(always)]
    pub fn sha224(self) -> &'a mut W {
        self.variant(UALGOSELECT_A::SHA224)
    }
}
impl R {
    #[doc = "Bit 0 - Write Back Disable"]
    #[inline(always)]
    pub fn wbdis(&self) -> WBDIS_R {
        WBDIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of Monitoring Disable"]
    #[inline(always)]
    pub fn eomdis(&self) -> EOMDIS_R {
        EOMDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Secondary List Branching Disable"]
    #[inline(always)]
    pub fn slbdis(&self) -> SLBDIS_R {
        SLBDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Bus Burden Control"]
    #[inline(always)]
    pub fn bbc(&self) -> BBC_R {
        BBC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Automatic Switch To Compare Digest"]
    #[inline(always)]
    pub fn ascd(&self) -> ASCD_R {
        ASCD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Dual Input Buffer"]
    #[inline(always)]
    pub fn dualbuff(&self) -> DUALBUFF_R {
        DUALBUFF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - User Initial Hash Value"]
    #[inline(always)]
    pub fn uihash(&self) -> UIHASH_R {
        UIHASH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - User SHA Algorithm"]
    #[inline(always)]
    pub fn ualgo(&self) -> UALGO_R {
        UALGO_R::new(((self.bits >> 13) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Write Back Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wbdis(&mut self) -> WBDIS_W<0> {
        WBDIS_W::new(self)
    }
    #[doc = "Bit 1 - End of Monitoring Disable"]
    #[inline(always)]
    #[must_use]
    pub fn eomdis(&mut self) -> EOMDIS_W<1> {
        EOMDIS_W::new(self)
    }
    #[doc = "Bit 2 - Secondary List Branching Disable"]
    #[inline(always)]
    #[must_use]
    pub fn slbdis(&mut self) -> SLBDIS_W<2> {
        SLBDIS_W::new(self)
    }
    #[doc = "Bits 4:7 - Bus Burden Control"]
    #[inline(always)]
    #[must_use]
    pub fn bbc(&mut self) -> BBC_W<4> {
        BBC_W::new(self)
    }
    #[doc = "Bit 8 - Automatic Switch To Compare Digest"]
    #[inline(always)]
    #[must_use]
    pub fn ascd(&mut self) -> ASCD_W<8> {
        ASCD_W::new(self)
    }
    #[doc = "Bit 9 - Dual Input Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn dualbuff(&mut self) -> DUALBUFF_W<9> {
        DUALBUFF_W::new(self)
    }
    #[doc = "Bit 12 - User Initial Hash Value"]
    #[inline(always)]
    #[must_use]
    pub fn uihash(&mut self) -> UIHASH_W<12> {
        UIHASH_W::new(self)
    }
    #[doc = "Bits 13:15 - User SHA Algorithm"]
    #[inline(always)]
    #[must_use]
    pub fn ualgo(&mut self) -> UALGO_W<13> {
        UALGO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
