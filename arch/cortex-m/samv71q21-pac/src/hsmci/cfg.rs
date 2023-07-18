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
#[doc = "Field `FIFOMODE` reader - HSMCI Internal FIFO control mode"]
pub type FIFOMODE_R = crate::BitReader;
#[doc = "Field `FIFOMODE` writer - HSMCI Internal FIFO control mode"]
pub type FIFOMODE_W<'a, const O: u8> = crate::BitWriter<'a, CFG_SPEC, O>;
#[doc = "Field `FERRCTRL` reader - Flow Error flag reset control mode"]
pub type FERRCTRL_R = crate::BitReader;
#[doc = "Field `FERRCTRL` writer - Flow Error flag reset control mode"]
pub type FERRCTRL_W<'a, const O: u8> = crate::BitWriter<'a, CFG_SPEC, O>;
#[doc = "Field `HSMODE` reader - High Speed Mode"]
pub type HSMODE_R = crate::BitReader;
#[doc = "Field `HSMODE` writer - High Speed Mode"]
pub type HSMODE_W<'a, const O: u8> = crate::BitWriter<'a, CFG_SPEC, O>;
#[doc = "Field `LSYNC` reader - Synchronize on the last block"]
pub type LSYNC_R = crate::BitReader;
#[doc = "Field `LSYNC` writer - Synchronize on the last block"]
pub type LSYNC_W<'a, const O: u8> = crate::BitWriter<'a, CFG_SPEC, O>;
impl R {
    #[doc = "Bit 0 - HSMCI Internal FIFO control mode"]
    #[inline(always)]
    pub fn fifomode(&self) -> FIFOMODE_R {
        FIFOMODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Flow Error flag reset control mode"]
    #[inline(always)]
    pub fn ferrctrl(&self) -> FERRCTRL_R {
        FERRCTRL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - High Speed Mode"]
    #[inline(always)]
    pub fn hsmode(&self) -> HSMODE_R {
        HSMODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Synchronize on the last block"]
    #[inline(always)]
    pub fn lsync(&self) -> LSYNC_R {
        LSYNC_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSMCI Internal FIFO control mode"]
    #[inline(always)]
    #[must_use]
    pub fn fifomode(&mut self) -> FIFOMODE_W<0> {
        FIFOMODE_W::new(self)
    }
    #[doc = "Bit 4 - Flow Error flag reset control mode"]
    #[inline(always)]
    #[must_use]
    pub fn ferrctrl(&mut self) -> FERRCTRL_W<4> {
        FERRCTRL_W::new(self)
    }
    #[doc = "Bit 8 - High Speed Mode"]
    #[inline(always)]
    #[must_use]
    pub fn hsmode(&mut self) -> HSMODE_W<8> {
        HSMODE_W::new(self)
    }
    #[doc = "Bit 12 - Synchronize on the last block"]
    #[inline(always)]
    #[must_use]
    pub fn lsync(&mut self) -> LSYNC_W<12> {
        LSYNC_W::new(self)
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
