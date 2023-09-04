#[doc = "Register `MSS` reader"]
pub type R = crate::R<MSS_SPEC>;
#[doc = "Register `MSS` writer"]
pub type W = crate::W<MSS_SPEC>;
#[doc = "Field `RSTSYSCMD` reader - Reset System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub type RSTSYSCMD_R = crate::BitReader;
#[doc = "Field `RSTSYSCMD` writer - Reset System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub type RSTSYSCMD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LKSYSCMD` reader - Network Lock System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub type LKSYSCMD_R = crate::BitReader;
#[doc = "Field `LKSYSCMD` writer - Network Lock System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub type LKSYSCMD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ULKSYSCMD` reader - Network Unlock System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub type ULKSYSCMD_R = crate::BitReader;
#[doc = "Field `ULKSYSCMD` writer - Network Unlock System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub type ULKSYSCMD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CSSYSCMD` reader - Channel Scan System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub type CSSYSCMD_R = crate::BitReader;
#[doc = "Field `CSSYSCMD` writer - Channel Scan System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub type CSSYSCMD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWSYSCMD` reader - Software System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub type SWSYSCMD_R = crate::BitReader;
#[doc = "Field `SWSYSCMD` writer - Software System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub type SWSYSCMD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SERVREQ` reader - Service Request Enabled"]
pub type SERVREQ_R = crate::BitReader;
#[doc = "Field `SERVREQ` writer - Service Request Enabled"]
pub type SERVREQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn rstsyscmd(&mut self) -> RSTSYSCMD_W<MSS_SPEC, 0> {
        RSTSYSCMD_W::new(self)
    }
    #[doc = "Bit 1 - Network Lock System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    #[must_use]
    pub fn lksyscmd(&mut self) -> LKSYSCMD_W<MSS_SPEC, 1> {
        LKSYSCMD_W::new(self)
    }
    #[doc = "Bit 2 - Network Unlock System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    #[must_use]
    pub fn ulksyscmd(&mut self) -> ULKSYSCMD_W<MSS_SPEC, 2> {
        ULKSYSCMD_W::new(self)
    }
    #[doc = "Bit 3 - Channel Scan System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    #[must_use]
    pub fn cssyscmd(&mut self) -> CSSYSCMD_W<MSS_SPEC, 3> {
        CSSYSCMD_W::new(self)
    }
    #[doc = "Bit 4 - Software System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    #[must_use]
    pub fn swsyscmd(&mut self) -> SWSYSCMD_W<MSS_SPEC, 4> {
        SWSYSCMD_W::new(self)
    }
    #[doc = "Bit 5 - Service Request Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn servreq(&mut self) -> SERVREQ_W<MSS_SPEC, 5> {
        SERVREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MediaLB System Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mss::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mss::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSS_SPEC;
impl crate::RegisterSpec for MSS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mss::R`](R) reader structure"]
impl crate::Readable for MSS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mss::W`](W) writer structure"]
impl crate::Writable for MSS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSS to value 0"]
impl crate::Resettable for MSS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
