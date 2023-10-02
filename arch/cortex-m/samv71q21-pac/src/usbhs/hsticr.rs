#[doc = "Register `HSTICR` writer"]
pub type W = crate::W<HSTICR_SPEC>;
#[doc = "Field `DCONNIC` writer - Device Connection Interrupt Clear"]
pub type DCONNIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DDISCIC` writer - Device Disconnection Interrupt Clear"]
pub type DDISCIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSTIC` writer - USB Reset Sent Interrupt Clear"]
pub type RSTIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSMEDIC` writer - Downstream Resume Sent Interrupt Clear"]
pub type RSMEDIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXRSMIC` writer - Upstream Resume Received Interrupt Clear"]
pub type RXRSMIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSOFIC` writer - Host Start of Frame Interrupt Clear"]
pub type HSOFIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HWUPIC` writer - Host Wake-Up Interrupt Clear"]
pub type HWUPIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Device Connection Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dconnic(&mut self) -> DCONNIC_W<HSTICR_SPEC, 0> {
        DCONNIC_W::new(self)
    }
    #[doc = "Bit 1 - Device Disconnection Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ddiscic(&mut self) -> DDISCIC_W<HSTICR_SPEC, 1> {
        DDISCIC_W::new(self)
    }
    #[doc = "Bit 2 - USB Reset Sent Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rstic(&mut self) -> RSTIC_W<HSTICR_SPEC, 2> {
        RSTIC_W::new(self)
    }
    #[doc = "Bit 3 - Downstream Resume Sent Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rsmedic(&mut self) -> RSMEDIC_W<HSTICR_SPEC, 3> {
        RSMEDIC_W::new(self)
    }
    #[doc = "Bit 4 - Upstream Resume Received Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxrsmic(&mut self) -> RXRSMIC_W<HSTICR_SPEC, 4> {
        RXRSMIC_W::new(self)
    }
    #[doc = "Bit 5 - Host Start of Frame Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsofic(&mut self) -> HSOFIC_W<HSTICR_SPEC, 5> {
        HSOFIC_W::new(self)
    }
    #[doc = "Bit 6 - Host Wake-Up Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hwupic(&mut self) -> HWUPIC_W<HSTICR_SPEC, 6> {
        HWUPIC_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Host Global Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsticr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSTICR_SPEC;
impl crate::RegisterSpec for HSTICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hsticr::W`](W) writer structure"]
impl crate::Writable for HSTICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSTICR to value 0"]
impl crate::Resettable for HSTICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
