#[doc = "Register `GRS` reader"]
pub struct R(crate::R<GRS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GRS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GRS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GRS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GRS` writer"]
pub struct W(crate::W<GRS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GRS_SPEC>;
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
impl From<crate::W<GRS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GRS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RS0` reader - XDMAC Channel 0 Read Suspend Bit"]
pub type RS0_R = crate::BitReader;
#[doc = "Field `RS0` writer - XDMAC Channel 0 Read Suspend Bit"]
pub type RS0_W<'a, const O: u8> = crate::BitWriter<'a, GRS_SPEC, O>;
#[doc = "Field `RS1` reader - XDMAC Channel 1 Read Suspend Bit"]
pub type RS1_R = crate::BitReader;
#[doc = "Field `RS1` writer - XDMAC Channel 1 Read Suspend Bit"]
pub type RS1_W<'a, const O: u8> = crate::BitWriter<'a, GRS_SPEC, O>;
#[doc = "Field `RS2` reader - XDMAC Channel 2 Read Suspend Bit"]
pub type RS2_R = crate::BitReader;
#[doc = "Field `RS2` writer - XDMAC Channel 2 Read Suspend Bit"]
pub type RS2_W<'a, const O: u8> = crate::BitWriter<'a, GRS_SPEC, O>;
#[doc = "Field `RS3` reader - XDMAC Channel 3 Read Suspend Bit"]
pub type RS3_R = crate::BitReader;
#[doc = "Field `RS3` writer - XDMAC Channel 3 Read Suspend Bit"]
pub type RS3_W<'a, const O: u8> = crate::BitWriter<'a, GRS_SPEC, O>;
#[doc = "Field `RS4` reader - XDMAC Channel 4 Read Suspend Bit"]
pub type RS4_R = crate::BitReader;
#[doc = "Field `RS4` writer - XDMAC Channel 4 Read Suspend Bit"]
pub type RS4_W<'a, const O: u8> = crate::BitWriter<'a, GRS_SPEC, O>;
#[doc = "Field `RS5` reader - XDMAC Channel 5 Read Suspend Bit"]
pub type RS5_R = crate::BitReader;
#[doc = "Field `RS5` writer - XDMAC Channel 5 Read Suspend Bit"]
pub type RS5_W<'a, const O: u8> = crate::BitWriter<'a, GRS_SPEC, O>;
#[doc = "Field `RS6` reader - XDMAC Channel 6 Read Suspend Bit"]
pub type RS6_R = crate::BitReader;
#[doc = "Field `RS6` writer - XDMAC Channel 6 Read Suspend Bit"]
pub type RS6_W<'a, const O: u8> = crate::BitWriter<'a, GRS_SPEC, O>;
#[doc = "Field `RS7` reader - XDMAC Channel 7 Read Suspend Bit"]
pub type RS7_R = crate::BitReader;
#[doc = "Field `RS7` writer - XDMAC Channel 7 Read Suspend Bit"]
pub type RS7_W<'a, const O: u8> = crate::BitWriter<'a, GRS_SPEC, O>;
#[doc = "Field `RS8` reader - XDMAC Channel 8 Read Suspend Bit"]
pub type RS8_R = crate::BitReader;
#[doc = "Field `RS8` writer - XDMAC Channel 8 Read Suspend Bit"]
pub type RS8_W<'a, const O: u8> = crate::BitWriter<'a, GRS_SPEC, O>;
#[doc = "Field `RS9` reader - XDMAC Channel 9 Read Suspend Bit"]
pub type RS9_R = crate::BitReader;
#[doc = "Field `RS9` writer - XDMAC Channel 9 Read Suspend Bit"]
pub type RS9_W<'a, const O: u8> = crate::BitWriter<'a, GRS_SPEC, O>;
#[doc = "Field `RS10` reader - XDMAC Channel 10 Read Suspend Bit"]
pub type RS10_R = crate::BitReader;
#[doc = "Field `RS10` writer - XDMAC Channel 10 Read Suspend Bit"]
pub type RS10_W<'a, const O: u8> = crate::BitWriter<'a, GRS_SPEC, O>;
#[doc = "Field `RS11` reader - XDMAC Channel 11 Read Suspend Bit"]
pub type RS11_R = crate::BitReader;
#[doc = "Field `RS11` writer - XDMAC Channel 11 Read Suspend Bit"]
pub type RS11_W<'a, const O: u8> = crate::BitWriter<'a, GRS_SPEC, O>;
#[doc = "Field `RS12` reader - XDMAC Channel 12 Read Suspend Bit"]
pub type RS12_R = crate::BitReader;
#[doc = "Field `RS12` writer - XDMAC Channel 12 Read Suspend Bit"]
pub type RS12_W<'a, const O: u8> = crate::BitWriter<'a, GRS_SPEC, O>;
#[doc = "Field `RS13` reader - XDMAC Channel 13 Read Suspend Bit"]
pub type RS13_R = crate::BitReader;
#[doc = "Field `RS13` writer - XDMAC Channel 13 Read Suspend Bit"]
pub type RS13_W<'a, const O: u8> = crate::BitWriter<'a, GRS_SPEC, O>;
#[doc = "Field `RS14` reader - XDMAC Channel 14 Read Suspend Bit"]
pub type RS14_R = crate::BitReader;
#[doc = "Field `RS14` writer - XDMAC Channel 14 Read Suspend Bit"]
pub type RS14_W<'a, const O: u8> = crate::BitWriter<'a, GRS_SPEC, O>;
#[doc = "Field `RS15` reader - XDMAC Channel 15 Read Suspend Bit"]
pub type RS15_R = crate::BitReader;
#[doc = "Field `RS15` writer - XDMAC Channel 15 Read Suspend Bit"]
pub type RS15_W<'a, const O: u8> = crate::BitWriter<'a, GRS_SPEC, O>;
#[doc = "Field `RS16` reader - XDMAC Channel 16 Read Suspend Bit"]
pub type RS16_R = crate::BitReader;
#[doc = "Field `RS16` writer - XDMAC Channel 16 Read Suspend Bit"]
pub type RS16_W<'a, const O: u8> = crate::BitWriter<'a, GRS_SPEC, O>;
#[doc = "Field `RS17` reader - XDMAC Channel 17 Read Suspend Bit"]
pub type RS17_R = crate::BitReader;
#[doc = "Field `RS17` writer - XDMAC Channel 17 Read Suspend Bit"]
pub type RS17_W<'a, const O: u8> = crate::BitWriter<'a, GRS_SPEC, O>;
#[doc = "Field `RS18` reader - XDMAC Channel 18 Read Suspend Bit"]
pub type RS18_R = crate::BitReader;
#[doc = "Field `RS18` writer - XDMAC Channel 18 Read Suspend Bit"]
pub type RS18_W<'a, const O: u8> = crate::BitWriter<'a, GRS_SPEC, O>;
#[doc = "Field `RS19` reader - XDMAC Channel 19 Read Suspend Bit"]
pub type RS19_R = crate::BitReader;
#[doc = "Field `RS19` writer - XDMAC Channel 19 Read Suspend Bit"]
pub type RS19_W<'a, const O: u8> = crate::BitWriter<'a, GRS_SPEC, O>;
#[doc = "Field `RS20` reader - XDMAC Channel 20 Read Suspend Bit"]
pub type RS20_R = crate::BitReader;
#[doc = "Field `RS20` writer - XDMAC Channel 20 Read Suspend Bit"]
pub type RS20_W<'a, const O: u8> = crate::BitWriter<'a, GRS_SPEC, O>;
#[doc = "Field `RS21` reader - XDMAC Channel 21 Read Suspend Bit"]
pub type RS21_R = crate::BitReader;
#[doc = "Field `RS21` writer - XDMAC Channel 21 Read Suspend Bit"]
pub type RS21_W<'a, const O: u8> = crate::BitWriter<'a, GRS_SPEC, O>;
#[doc = "Field `RS22` reader - XDMAC Channel 22 Read Suspend Bit"]
pub type RS22_R = crate::BitReader;
#[doc = "Field `RS22` writer - XDMAC Channel 22 Read Suspend Bit"]
pub type RS22_W<'a, const O: u8> = crate::BitWriter<'a, GRS_SPEC, O>;
#[doc = "Field `RS23` reader - XDMAC Channel 23 Read Suspend Bit"]
pub type RS23_R = crate::BitReader;
#[doc = "Field `RS23` writer - XDMAC Channel 23 Read Suspend Bit"]
pub type RS23_W<'a, const O: u8> = crate::BitWriter<'a, GRS_SPEC, O>;
impl R {
    #[doc = "Bit 0 - XDMAC Channel 0 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs0(&self) -> RS0_R {
        RS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs1(&self) -> RS1_R {
        RS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs2(&self) -> RS2_R {
        RS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs3(&self) -> RS3_R {
        RS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs4(&self) -> RS4_R {
        RS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs5(&self) -> RS5_R {
        RS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs6(&self) -> RS6_R {
        RS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs7(&self) -> RS7_R {
        RS7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs8(&self) -> RS8_R {
        RS8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs9(&self) -> RS9_R {
        RS9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs10(&self) -> RS10_R {
        RS10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs11(&self) -> RS11_R {
        RS11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs12(&self) -> RS12_R {
        RS12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs13(&self) -> RS13_R {
        RS13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs14(&self) -> RS14_R {
        RS14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs15(&self) -> RS15_R {
        RS15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs16(&self) -> RS16_R {
        RS16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs17(&self) -> RS17_R {
        RS17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs18(&self) -> RS18_R {
        RS18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs19(&self) -> RS19_R {
        RS19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs20(&self) -> RS20_R {
        RS20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs21(&self) -> RS21_R {
        RS21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs22(&self) -> RS22_R {
        RS22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs23(&self) -> RS23_R {
        RS23_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XDMAC Channel 0 Read Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rs0(&mut self) -> RS0_W<0> {
        RS0_W::new(self)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Read Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rs1(&mut self) -> RS1_W<1> {
        RS1_W::new(self)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Read Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rs2(&mut self) -> RS2_W<2> {
        RS2_W::new(self)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Read Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rs3(&mut self) -> RS3_W<3> {
        RS3_W::new(self)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Read Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rs4(&mut self) -> RS4_W<4> {
        RS4_W::new(self)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Read Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rs5(&mut self) -> RS5_W<5> {
        RS5_W::new(self)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Read Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rs6(&mut self) -> RS6_W<6> {
        RS6_W::new(self)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Read Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rs7(&mut self) -> RS7_W<7> {
        RS7_W::new(self)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Read Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rs8(&mut self) -> RS8_W<8> {
        RS8_W::new(self)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Read Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rs9(&mut self) -> RS9_W<9> {
        RS9_W::new(self)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Read Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rs10(&mut self) -> RS10_W<10> {
        RS10_W::new(self)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Read Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rs11(&mut self) -> RS11_W<11> {
        RS11_W::new(self)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Read Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rs12(&mut self) -> RS12_W<12> {
        RS12_W::new(self)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Read Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rs13(&mut self) -> RS13_W<13> {
        RS13_W::new(self)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Read Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rs14(&mut self) -> RS14_W<14> {
        RS14_W::new(self)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Read Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rs15(&mut self) -> RS15_W<15> {
        RS15_W::new(self)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Read Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rs16(&mut self) -> RS16_W<16> {
        RS16_W::new(self)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Read Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rs17(&mut self) -> RS17_W<17> {
        RS17_W::new(self)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Read Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rs18(&mut self) -> RS18_W<18> {
        RS18_W::new(self)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Read Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rs19(&mut self) -> RS19_W<19> {
        RS19_W::new(self)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Read Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rs20(&mut self) -> RS20_W<20> {
        RS20_W::new(self)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Read Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rs21(&mut self) -> RS21_W<21> {
        RS21_W::new(self)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Read Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rs22(&mut self) -> RS22_W<22> {
        RS22_W::new(self)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Read Suspend Bit"]
    #[inline(always)]
    #[must_use]
    pub fn rs23(&mut self) -> RS23_W<23> {
        RS23_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Channel Read Suspend Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grs](index.html) module"]
pub struct GRS_SPEC;
impl crate::RegisterSpec for GRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [grs::R](R) reader structure"]
impl crate::Readable for GRS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [grs::W](W) writer structure"]
impl crate::Writable for GRS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GRS to value 0"]
impl crate::Resettable for GRS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
