#[doc = "Register `IDR` writer"]
pub struct W(crate::W<IDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDR_SPEC>;
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
impl From<crate::W<IDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXCOMP` writer - Transmission Completed Interrupt Disable"]
pub type TXCOMP_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `RXRDY` writer - Receive Holding Register Ready Interrupt Disable"]
pub type RXRDY_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `TXRDY` writer - Transmit Holding Register Ready Interrupt Disable"]
pub type TXRDY_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `SVACC` writer - Slave Access Interrupt Disable"]
pub type SVACC_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `GACC` writer - General Call Access Interrupt Disable"]
pub type GACC_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Disable"]
pub type OVRE_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `UNRE` writer - Underrun Error Interrupt Disable"]
pub type UNRE_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `NACK` writer - Not Acknowledge Interrupt Disable"]
pub type NACK_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `ARBLST` writer - Arbitration Lost Interrupt Disable"]
pub type ARBLST_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `SCL_WS` writer - Clock Wait State Interrupt Disable"]
pub type SCL_WS_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `EOSACC` writer - End Of Slave Access Interrupt Disable"]
pub type EOSACC_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `MCACK` writer - Master Code Acknowledge Interrupt Disable"]
pub type MCACK_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `TOUT` writer - Timeout Error Interrupt Disable"]
pub type TOUT_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `PECERR` writer - PEC Error Interrupt Disable"]
pub type PECERR_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `SMBDAM` writer - SMBus Default Address Match Interrupt Disable"]
pub type SMBDAM_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
#[doc = "Field `SMBHHM` writer - SMBus Host Header Address Match Interrupt Disable"]
pub type SMBHHM_W<'a, const O: u8> = crate::BitWriter<'a, IDR_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Transmission Completed Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txcomp(&mut self) -> TXCOMP_W<0> {
        TXCOMP_W::new(self)
    }
    #[doc = "Bit 1 - Receive Holding Register Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RXRDY_W<1> {
        RXRDY_W::new(self)
    }
    #[doc = "Bit 2 - Transmit Holding Register Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TXRDY_W<2> {
        TXRDY_W::new(self)
    }
    #[doc = "Bit 4 - Slave Access Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn svacc(&mut self) -> SVACC_W<4> {
        SVACC_W::new(self)
    }
    #[doc = "Bit 5 - General Call Access Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn gacc(&mut self) -> GACC_W<5> {
        GACC_W::new(self)
    }
    #[doc = "Bit 6 - Overrun Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ovre(&mut self) -> OVRE_W<6> {
        OVRE_W::new(self)
    }
    #[doc = "Bit 7 - Underrun Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn unre(&mut self) -> UNRE_W<7> {
        UNRE_W::new(self)
    }
    #[doc = "Bit 8 - Not Acknowledge Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<8> {
        NACK_W::new(self)
    }
    #[doc = "Bit 9 - Arbitration Lost Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn arblst(&mut self) -> ARBLST_W<9> {
        ARBLST_W::new(self)
    }
    #[doc = "Bit 10 - Clock Wait State Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn scl_ws(&mut self) -> SCL_WS_W<10> {
        SCL_WS_W::new(self)
    }
    #[doc = "Bit 11 - End Of Slave Access Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn eosacc(&mut self) -> EOSACC_W<11> {
        EOSACC_W::new(self)
    }
    #[doc = "Bit 16 - Master Code Acknowledge Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mcack(&mut self) -> MCACK_W<16> {
        MCACK_W::new(self)
    }
    #[doc = "Bit 18 - Timeout Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn tout(&mut self) -> TOUT_W<18> {
        TOUT_W::new(self)
    }
    #[doc = "Bit 19 - PEC Error Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pecerr(&mut self) -> PECERR_W<19> {
        PECERR_W::new(self)
    }
    #[doc = "Bit 20 - SMBus Default Address Match Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn smbdam(&mut self) -> SMBDAM_W<20> {
        SMBDAM_W::new(self)
    }
    #[doc = "Bit 21 - SMBus Host Header Address Match Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn smbhhm(&mut self) -> SMBHHM_W<21> {
        SMBHHM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](index.html) module"]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [idr::W](W) writer structure"]
impl crate::Writable for IDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
