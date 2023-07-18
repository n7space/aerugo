#[doc = "Register `DEVEPT` reader"]
pub struct R(crate::R<DEVEPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVEPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVEPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVEPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEVEPT` writer"]
pub struct W(crate::W<DEVEPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVEPT_SPEC>;
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
impl From<crate::W<DEVEPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVEPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPEN0` reader - Endpoint 0 Enable"]
pub type EPEN0_R = crate::BitReader;
#[doc = "Field `EPEN0` writer - Endpoint 0 Enable"]
pub type EPEN0_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPT_SPEC, O>;
#[doc = "Field `EPEN1` reader - Endpoint 1 Enable"]
pub type EPEN1_R = crate::BitReader;
#[doc = "Field `EPEN1` writer - Endpoint 1 Enable"]
pub type EPEN1_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPT_SPEC, O>;
#[doc = "Field `EPEN2` reader - Endpoint 2 Enable"]
pub type EPEN2_R = crate::BitReader;
#[doc = "Field `EPEN2` writer - Endpoint 2 Enable"]
pub type EPEN2_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPT_SPEC, O>;
#[doc = "Field `EPEN3` reader - Endpoint 3 Enable"]
pub type EPEN3_R = crate::BitReader;
#[doc = "Field `EPEN3` writer - Endpoint 3 Enable"]
pub type EPEN3_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPT_SPEC, O>;
#[doc = "Field `EPEN4` reader - Endpoint 4 Enable"]
pub type EPEN4_R = crate::BitReader;
#[doc = "Field `EPEN4` writer - Endpoint 4 Enable"]
pub type EPEN4_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPT_SPEC, O>;
#[doc = "Field `EPEN5` reader - Endpoint 5 Enable"]
pub type EPEN5_R = crate::BitReader;
#[doc = "Field `EPEN5` writer - Endpoint 5 Enable"]
pub type EPEN5_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPT_SPEC, O>;
#[doc = "Field `EPEN6` reader - Endpoint 6 Enable"]
pub type EPEN6_R = crate::BitReader;
#[doc = "Field `EPEN6` writer - Endpoint 6 Enable"]
pub type EPEN6_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPT_SPEC, O>;
#[doc = "Field `EPEN7` reader - Endpoint 7 Enable"]
pub type EPEN7_R = crate::BitReader;
#[doc = "Field `EPEN7` writer - Endpoint 7 Enable"]
pub type EPEN7_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPT_SPEC, O>;
#[doc = "Field `EPEN8` reader - Endpoint 8 Enable"]
pub type EPEN8_R = crate::BitReader;
#[doc = "Field `EPEN8` writer - Endpoint 8 Enable"]
pub type EPEN8_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPT_SPEC, O>;
#[doc = "Field `EPEN9` reader - Endpoint 9 Enable"]
pub type EPEN9_R = crate::BitReader;
#[doc = "Field `EPEN9` writer - Endpoint 9 Enable"]
pub type EPEN9_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPT_SPEC, O>;
#[doc = "Field `EPRST0` reader - Endpoint 0 Reset"]
pub type EPRST0_R = crate::BitReader;
#[doc = "Field `EPRST0` writer - Endpoint 0 Reset"]
pub type EPRST0_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPT_SPEC, O>;
#[doc = "Field `EPRST1` reader - Endpoint 1 Reset"]
pub type EPRST1_R = crate::BitReader;
#[doc = "Field `EPRST1` writer - Endpoint 1 Reset"]
pub type EPRST1_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPT_SPEC, O>;
#[doc = "Field `EPRST2` reader - Endpoint 2 Reset"]
pub type EPRST2_R = crate::BitReader;
#[doc = "Field `EPRST2` writer - Endpoint 2 Reset"]
pub type EPRST2_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPT_SPEC, O>;
#[doc = "Field `EPRST3` reader - Endpoint 3 Reset"]
pub type EPRST3_R = crate::BitReader;
#[doc = "Field `EPRST3` writer - Endpoint 3 Reset"]
pub type EPRST3_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPT_SPEC, O>;
#[doc = "Field `EPRST4` reader - Endpoint 4 Reset"]
pub type EPRST4_R = crate::BitReader;
#[doc = "Field `EPRST4` writer - Endpoint 4 Reset"]
pub type EPRST4_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPT_SPEC, O>;
#[doc = "Field `EPRST5` reader - Endpoint 5 Reset"]
pub type EPRST5_R = crate::BitReader;
#[doc = "Field `EPRST5` writer - Endpoint 5 Reset"]
pub type EPRST5_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPT_SPEC, O>;
#[doc = "Field `EPRST6` reader - Endpoint 6 Reset"]
pub type EPRST6_R = crate::BitReader;
#[doc = "Field `EPRST6` writer - Endpoint 6 Reset"]
pub type EPRST6_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPT_SPEC, O>;
#[doc = "Field `EPRST7` reader - Endpoint 7 Reset"]
pub type EPRST7_R = crate::BitReader;
#[doc = "Field `EPRST7` writer - Endpoint 7 Reset"]
pub type EPRST7_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPT_SPEC, O>;
#[doc = "Field `EPRST8` reader - Endpoint 8 Reset"]
pub type EPRST8_R = crate::BitReader;
#[doc = "Field `EPRST8` writer - Endpoint 8 Reset"]
pub type EPRST8_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPT_SPEC, O>;
#[doc = "Field `EPRST9` reader - Endpoint 9 Reset"]
pub type EPRST9_R = crate::BitReader;
#[doc = "Field `EPRST9` writer - Endpoint 9 Reset"]
pub type EPRST9_W<'a, const O: u8> = crate::BitWriter<'a, DEVEPT_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Endpoint 0 Enable"]
    #[inline(always)]
    pub fn epen0(&self) -> EPEN0_R {
        EPEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint 1 Enable"]
    #[inline(always)]
    pub fn epen1(&self) -> EPEN1_R {
        EPEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint 2 Enable"]
    #[inline(always)]
    pub fn epen2(&self) -> EPEN2_R {
        EPEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Endpoint 3 Enable"]
    #[inline(always)]
    pub fn epen3(&self) -> EPEN3_R {
        EPEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint 4 Enable"]
    #[inline(always)]
    pub fn epen4(&self) -> EPEN4_R {
        EPEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Endpoint 5 Enable"]
    #[inline(always)]
    pub fn epen5(&self) -> EPEN5_R {
        EPEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Endpoint 6 Enable"]
    #[inline(always)]
    pub fn epen6(&self) -> EPEN6_R {
        EPEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Endpoint 7 Enable"]
    #[inline(always)]
    pub fn epen7(&self) -> EPEN7_R {
        EPEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Endpoint 8 Enable"]
    #[inline(always)]
    pub fn epen8(&self) -> EPEN8_R {
        EPEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Endpoint 9 Enable"]
    #[inline(always)]
    pub fn epen9(&self) -> EPEN9_R {
        EPEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Endpoint 0 Reset"]
    #[inline(always)]
    pub fn eprst0(&self) -> EPRST0_R {
        EPRST0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Endpoint 1 Reset"]
    #[inline(always)]
    pub fn eprst1(&self) -> EPRST1_R {
        EPRST1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Endpoint 2 Reset"]
    #[inline(always)]
    pub fn eprst2(&self) -> EPRST2_R {
        EPRST2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Endpoint 3 Reset"]
    #[inline(always)]
    pub fn eprst3(&self) -> EPRST3_R {
        EPRST3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Endpoint 4 Reset"]
    #[inline(always)]
    pub fn eprst4(&self) -> EPRST4_R {
        EPRST4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Endpoint 5 Reset"]
    #[inline(always)]
    pub fn eprst5(&self) -> EPRST5_R {
        EPRST5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Endpoint 6 Reset"]
    #[inline(always)]
    pub fn eprst6(&self) -> EPRST6_R {
        EPRST6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Endpoint 7 Reset"]
    #[inline(always)]
    pub fn eprst7(&self) -> EPRST7_R {
        EPRST7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Endpoint 8 Reset"]
    #[inline(always)]
    pub fn eprst8(&self) -> EPRST8_R {
        EPRST8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Endpoint 9 Reset"]
    #[inline(always)]
    pub fn eprst9(&self) -> EPRST9_R {
        EPRST9_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Endpoint 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epen0(&mut self) -> EPEN0_W<0> {
        EPEN0_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epen1(&mut self) -> EPEN1_W<1> {
        EPEN1_W::new(self)
    }
    #[doc = "Bit 2 - Endpoint 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epen2(&mut self) -> EPEN2_W<2> {
        EPEN2_W::new(self)
    }
    #[doc = "Bit 3 - Endpoint 3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epen3(&mut self) -> EPEN3_W<3> {
        EPEN3_W::new(self)
    }
    #[doc = "Bit 4 - Endpoint 4 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epen4(&mut self) -> EPEN4_W<4> {
        EPEN4_W::new(self)
    }
    #[doc = "Bit 5 - Endpoint 5 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epen5(&mut self) -> EPEN5_W<5> {
        EPEN5_W::new(self)
    }
    #[doc = "Bit 6 - Endpoint 6 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epen6(&mut self) -> EPEN6_W<6> {
        EPEN6_W::new(self)
    }
    #[doc = "Bit 7 - Endpoint 7 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epen7(&mut self) -> EPEN7_W<7> {
        EPEN7_W::new(self)
    }
    #[doc = "Bit 8 - Endpoint 8 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epen8(&mut self) -> EPEN8_W<8> {
        EPEN8_W::new(self)
    }
    #[doc = "Bit 9 - Endpoint 9 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epen9(&mut self) -> EPEN9_W<9> {
        EPEN9_W::new(self)
    }
    #[doc = "Bit 16 - Endpoint 0 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eprst0(&mut self) -> EPRST0_W<16> {
        EPRST0_W::new(self)
    }
    #[doc = "Bit 17 - Endpoint 1 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eprst1(&mut self) -> EPRST1_W<17> {
        EPRST1_W::new(self)
    }
    #[doc = "Bit 18 - Endpoint 2 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eprst2(&mut self) -> EPRST2_W<18> {
        EPRST2_W::new(self)
    }
    #[doc = "Bit 19 - Endpoint 3 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eprst3(&mut self) -> EPRST3_W<19> {
        EPRST3_W::new(self)
    }
    #[doc = "Bit 20 - Endpoint 4 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eprst4(&mut self) -> EPRST4_W<20> {
        EPRST4_W::new(self)
    }
    #[doc = "Bit 21 - Endpoint 5 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eprst5(&mut self) -> EPRST5_W<21> {
        EPRST5_W::new(self)
    }
    #[doc = "Bit 22 - Endpoint 6 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eprst6(&mut self) -> EPRST6_W<22> {
        EPRST6_W::new(self)
    }
    #[doc = "Bit 23 - Endpoint 7 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eprst7(&mut self) -> EPRST7_W<23> {
        EPRST7_W::new(self)
    }
    #[doc = "Bit 24 - Endpoint 8 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eprst8(&mut self) -> EPRST8_W<24> {
        EPRST8_W::new(self)
    }
    #[doc = "Bit 25 - Endpoint 9 Reset"]
    #[inline(always)]
    #[must_use]
    pub fn eprst9(&mut self) -> EPRST9_W<25> {
        EPRST9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Endpoint Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devept](index.html) module"]
pub struct DEVEPT_SPEC;
impl crate::RegisterSpec for DEVEPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devept::R](R) reader structure"]
impl crate::Readable for DEVEPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [devept::W](W) writer structure"]
impl crate::Writable for DEVEPT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEVEPT to value 0"]
impl crate::Resettable for DEVEPT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
