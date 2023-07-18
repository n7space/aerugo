#[doc = "Register `MSS` reader"]
pub struct R(crate::R<MSS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSS` writer"]
pub struct W(crate::W<MSS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSS_SPEC>;
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
impl From<crate::W<MSS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSTSYSCMD` reader - Reset System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub type RSTSYSCMD_R = crate::BitReader;
#[doc = "Field `RSTSYSCMD` writer - Reset System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub type RSTSYSCMD_W<'a, const O: u8> = crate::BitWriter<'a, MSS_SPEC, O>;
#[doc = "Field `LKSYSCMD` reader - Network Lock System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub type LKSYSCMD_R = crate::BitReader;
#[doc = "Field `LKSYSCMD` writer - Network Lock System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub type LKSYSCMD_W<'a, const O: u8> = crate::BitWriter<'a, MSS_SPEC, O>;
#[doc = "Field `ULKSYSCMD` reader - Network Unlock System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub type ULKSYSCMD_R = crate::BitReader;
#[doc = "Field `ULKSYSCMD` writer - Network Unlock System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub type ULKSYSCMD_W<'a, const O: u8> = crate::BitWriter<'a, MSS_SPEC, O>;
#[doc = "Field `CSSYSCMD` reader - Channel Scan System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub type CSSYSCMD_R = crate::BitReader;
#[doc = "Field `CSSYSCMD` writer - Channel Scan System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub type CSSYSCMD_W<'a, const O: u8> = crate::BitWriter<'a, MSS_SPEC, O>;
#[doc = "Field `SWSYSCMD` reader - Software System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub type SWSYSCMD_R = crate::BitReader;
#[doc = "Field `SWSYSCMD` writer - Software System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub type SWSYSCMD_W<'a, const O: u8> = crate::BitWriter<'a, MSS_SPEC, O>;
#[doc = "Field `SERVREQ` reader - Service Request Enabled"]
pub type SERVREQ_R = crate::BitReader;
#[doc = "Field `SERVREQ` writer - Service Request Enabled"]
pub type SERVREQ_W<'a, const O: u8> = crate::BitWriter<'a, MSS_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Reset System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn rstsyscmd(&self) -> RSTSYSCMD_R {
        RSTSYSCMD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Network Lock System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn lksyscmd(&self) -> LKSYSCMD_R {
        LKSYSCMD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Network Unlock System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn ulksyscmd(&self) -> ULKSYSCMD_R {
        ULKSYSCMD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel Scan System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn cssyscmd(&self) -> CSSYSCMD_R {
        CSSYSCMD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn swsyscmd(&self) -> SWSYSCMD_R {
        SWSYSCMD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Service Request Enabled"]
    #[inline(always)]
    pub fn servreq(&self) -> SERVREQ_R {
        SERVREQ_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    #[must_use]
    pub fn rstsyscmd(&mut self) -> RSTSYSCMD_W<0> {
        RSTSYSCMD_W::new(self)
    }
    #[doc = "Bit 1 - Network Lock System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    #[must_use]
    pub fn lksyscmd(&mut self) -> LKSYSCMD_W<1> {
        LKSYSCMD_W::new(self)
    }
    #[doc = "Bit 2 - Network Unlock System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    #[must_use]
    pub fn ulksyscmd(&mut self) -> ULKSYSCMD_W<2> {
        ULKSYSCMD_W::new(self)
    }
    #[doc = "Bit 3 - Channel Scan System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    #[must_use]
    pub fn cssyscmd(&mut self) -> CSSYSCMD_W<3> {
        CSSYSCMD_W::new(self)
    }
    #[doc = "Bit 4 - Software System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    #[must_use]
    pub fn swsyscmd(&mut self) -> SWSYSCMD_W<4> {
        SWSYSCMD_W::new(self)
    }
    #[doc = "Bit 5 - Service Request Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn servreq(&mut self) -> SERVREQ_W<5> {
        SERVREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MediaLB System Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mss](index.html) module"]
pub struct MSS_SPEC;
impl crate::RegisterSpec for MSS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mss::R](R) reader structure"]
impl crate::Readable for MSS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mss::W](W) writer structure"]
impl crate::Writable for MSS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSS to value 0"]
impl crate::Resettable for MSS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
