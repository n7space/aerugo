#[doc = "Register `MIEN` reader"]
pub struct R(crate::R<MIEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIEN` writer"]
pub struct W(crate::W<MIEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIEN_SPEC>;
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
impl From<crate::W<MIEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISOC_PE` reader - Isochronous Rx Protocol Error Enable"]
pub type ISOC_PE_R = crate::BitReader;
#[doc = "Field `ISOC_PE` writer - Isochronous Rx Protocol Error Enable"]
pub type ISOC_PE_W<'a, const O: u8> = crate::BitWriter<'a, MIEN_SPEC, O>;
#[doc = "Field `ISOC_BUFO` reader - Isochronous Rx Buffer Overflow Enable"]
pub type ISOC_BUFO_R = crate::BitReader;
#[doc = "Field `ISOC_BUFO` writer - Isochronous Rx Buffer Overflow Enable"]
pub type ISOC_BUFO_W<'a, const O: u8> = crate::BitWriter<'a, MIEN_SPEC, O>;
#[doc = "Field `SYNC_PE` reader - Synchronous Protocol Error Enable"]
pub type SYNC_PE_R = crate::BitReader;
#[doc = "Field `SYNC_PE` writer - Synchronous Protocol Error Enable"]
pub type SYNC_PE_W<'a, const O: u8> = crate::BitWriter<'a, MIEN_SPEC, O>;
#[doc = "Field `ARX_DONE` reader - Asynchronous Rx Done Enable"]
pub type ARX_DONE_R = crate::BitReader;
#[doc = "Field `ARX_DONE` writer - Asynchronous Rx Done Enable"]
pub type ARX_DONE_W<'a, const O: u8> = crate::BitWriter<'a, MIEN_SPEC, O>;
#[doc = "Field `ARX_PE` reader - Asynchronous Rx Protocol Error Enable"]
pub type ARX_PE_R = crate::BitReader;
#[doc = "Field `ARX_PE` writer - Asynchronous Rx Protocol Error Enable"]
pub type ARX_PE_W<'a, const O: u8> = crate::BitWriter<'a, MIEN_SPEC, O>;
#[doc = "Field `ARX_BREAK` reader - Asynchronous Rx Break Enable"]
pub type ARX_BREAK_R = crate::BitReader;
#[doc = "Field `ARX_BREAK` writer - Asynchronous Rx Break Enable"]
pub type ARX_BREAK_W<'a, const O: u8> = crate::BitWriter<'a, MIEN_SPEC, O>;
#[doc = "Field `ATX_DONE` reader - Asynchronous Tx Packet Done Enable"]
pub type ATX_DONE_R = crate::BitReader;
#[doc = "Field `ATX_DONE` writer - Asynchronous Tx Packet Done Enable"]
pub type ATX_DONE_W<'a, const O: u8> = crate::BitWriter<'a, MIEN_SPEC, O>;
#[doc = "Field `ATX_PE` reader - Asynchronous Tx Protocol Error Enable"]
pub type ATX_PE_R = crate::BitReader;
#[doc = "Field `ATX_PE` writer - Asynchronous Tx Protocol Error Enable"]
pub type ATX_PE_W<'a, const O: u8> = crate::BitWriter<'a, MIEN_SPEC, O>;
#[doc = "Field `ATX_BREAK` reader - Asynchronous Tx Break Enable"]
pub type ATX_BREAK_R = crate::BitReader;
#[doc = "Field `ATX_BREAK` writer - Asynchronous Tx Break Enable"]
pub type ATX_BREAK_W<'a, const O: u8> = crate::BitWriter<'a, MIEN_SPEC, O>;
#[doc = "Field `CRX_DONE` reader - Control Rx Packet Done Enable"]
pub type CRX_DONE_R = crate::BitReader;
#[doc = "Field `CRX_DONE` writer - Control Rx Packet Done Enable"]
pub type CRX_DONE_W<'a, const O: u8> = crate::BitWriter<'a, MIEN_SPEC, O>;
#[doc = "Field `CRX_PE` reader - Control Rx Protocol Error Enable"]
pub type CRX_PE_R = crate::BitReader;
#[doc = "Field `CRX_PE` writer - Control Rx Protocol Error Enable"]
pub type CRX_PE_W<'a, const O: u8> = crate::BitWriter<'a, MIEN_SPEC, O>;
#[doc = "Field `CRX_BREAK` reader - Control Rx Break Enable"]
pub type CRX_BREAK_R = crate::BitReader;
#[doc = "Field `CRX_BREAK` writer - Control Rx Break Enable"]
pub type CRX_BREAK_W<'a, const O: u8> = crate::BitWriter<'a, MIEN_SPEC, O>;
#[doc = "Field `CTX_DONE` reader - Control Tx Packet Done Enable"]
pub type CTX_DONE_R = crate::BitReader;
#[doc = "Field `CTX_DONE` writer - Control Tx Packet Done Enable"]
pub type CTX_DONE_W<'a, const O: u8> = crate::BitWriter<'a, MIEN_SPEC, O>;
#[doc = "Field `CTX_PE` reader - Control Tx Protocol Error Enable"]
pub type CTX_PE_R = crate::BitReader;
#[doc = "Field `CTX_PE` writer - Control Tx Protocol Error Enable"]
pub type CTX_PE_W<'a, const O: u8> = crate::BitWriter<'a, MIEN_SPEC, O>;
#[doc = "Field `CTX_BREAK` reader - Control Tx Break Enable"]
pub type CTX_BREAK_R = crate::BitReader;
#[doc = "Field `CTX_BREAK` writer - Control Tx Break Enable"]
pub type CTX_BREAK_W<'a, const O: u8> = crate::BitWriter<'a, MIEN_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Isochronous Rx Protocol Error Enable"]
    #[inline(always)]
    pub fn isoc_pe(&self) -> ISOC_PE_R {
        ISOC_PE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Isochronous Rx Buffer Overflow Enable"]
    #[inline(always)]
    pub fn isoc_bufo(&self) -> ISOC_BUFO_R {
        ISOC_BUFO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Synchronous Protocol Error Enable"]
    #[inline(always)]
    pub fn sync_pe(&self) -> SYNC_PE_R {
        SYNC_PE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Asynchronous Rx Done Enable"]
    #[inline(always)]
    pub fn arx_done(&self) -> ARX_DONE_R {
        ARX_DONE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Asynchronous Rx Protocol Error Enable"]
    #[inline(always)]
    pub fn arx_pe(&self) -> ARX_PE_R {
        ARX_PE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Asynchronous Rx Break Enable"]
    #[inline(always)]
    pub fn arx_break(&self) -> ARX_BREAK_R {
        ARX_BREAK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Asynchronous Tx Packet Done Enable"]
    #[inline(always)]
    pub fn atx_done(&self) -> ATX_DONE_R {
        ATX_DONE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Asynchronous Tx Protocol Error Enable"]
    #[inline(always)]
    pub fn atx_pe(&self) -> ATX_PE_R {
        ATX_PE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Asynchronous Tx Break Enable"]
    #[inline(always)]
    pub fn atx_break(&self) -> ATX_BREAK_R {
        ATX_BREAK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Control Rx Packet Done Enable"]
    #[inline(always)]
    pub fn crx_done(&self) -> CRX_DONE_R {
        CRX_DONE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Control Rx Protocol Error Enable"]
    #[inline(always)]
    pub fn crx_pe(&self) -> CRX_PE_R {
        CRX_PE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Control Rx Break Enable"]
    #[inline(always)]
    pub fn crx_break(&self) -> CRX_BREAK_R {
        CRX_BREAK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Control Tx Packet Done Enable"]
    #[inline(always)]
    pub fn ctx_done(&self) -> CTX_DONE_R {
        CTX_DONE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Control Tx Protocol Error Enable"]
    #[inline(always)]
    pub fn ctx_pe(&self) -> CTX_PE_R {
        CTX_PE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Control Tx Break Enable"]
    #[inline(always)]
    pub fn ctx_break(&self) -> CTX_BREAK_R {
        CTX_BREAK_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Isochronous Rx Protocol Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn isoc_pe(&mut self) -> ISOC_PE_W<0> {
        ISOC_PE_W::new(self)
    }
    #[doc = "Bit 1 - Isochronous Rx Buffer Overflow Enable"]
    #[inline(always)]
    #[must_use]
    pub fn isoc_bufo(&mut self) -> ISOC_BUFO_W<1> {
        ISOC_BUFO_W::new(self)
    }
    #[doc = "Bit 16 - Synchronous Protocol Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sync_pe(&mut self) -> SYNC_PE_W<16> {
        SYNC_PE_W::new(self)
    }
    #[doc = "Bit 17 - Asynchronous Rx Done Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arx_done(&mut self) -> ARX_DONE_W<17> {
        ARX_DONE_W::new(self)
    }
    #[doc = "Bit 18 - Asynchronous Rx Protocol Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arx_pe(&mut self) -> ARX_PE_W<18> {
        ARX_PE_W::new(self)
    }
    #[doc = "Bit 19 - Asynchronous Rx Break Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arx_break(&mut self) -> ARX_BREAK_W<19> {
        ARX_BREAK_W::new(self)
    }
    #[doc = "Bit 20 - Asynchronous Tx Packet Done Enable"]
    #[inline(always)]
    #[must_use]
    pub fn atx_done(&mut self) -> ATX_DONE_W<20> {
        ATX_DONE_W::new(self)
    }
    #[doc = "Bit 21 - Asynchronous Tx Protocol Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn atx_pe(&mut self) -> ATX_PE_W<21> {
        ATX_PE_W::new(self)
    }
    #[doc = "Bit 22 - Asynchronous Tx Break Enable"]
    #[inline(always)]
    #[must_use]
    pub fn atx_break(&mut self) -> ATX_BREAK_W<22> {
        ATX_BREAK_W::new(self)
    }
    #[doc = "Bit 24 - Control Rx Packet Done Enable"]
    #[inline(always)]
    #[must_use]
    pub fn crx_done(&mut self) -> CRX_DONE_W<24> {
        CRX_DONE_W::new(self)
    }
    #[doc = "Bit 25 - Control Rx Protocol Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn crx_pe(&mut self) -> CRX_PE_W<25> {
        CRX_PE_W::new(self)
    }
    #[doc = "Bit 26 - Control Rx Break Enable"]
    #[inline(always)]
    #[must_use]
    pub fn crx_break(&mut self) -> CRX_BREAK_W<26> {
        CRX_BREAK_W::new(self)
    }
    #[doc = "Bit 27 - Control Tx Packet Done Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctx_done(&mut self) -> CTX_DONE_W<27> {
        CTX_DONE_W::new(self)
    }
    #[doc = "Bit 28 - Control Tx Protocol Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctx_pe(&mut self) -> CTX_PE_W<28> {
        CTX_PE_W::new(self)
    }
    #[doc = "Bit 29 - Control Tx Break Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctx_break(&mut self) -> CTX_BREAK_W<29> {
        CTX_BREAK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MediaLB Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mien](index.html) module"]
pub struct MIEN_SPEC;
impl crate::RegisterSpec for MIEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mien::R](R) reader structure"]
impl crate::Readable for MIEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mien::W](W) writer structure"]
impl crate::Writable for MIEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIEN to value 0"]
impl crate::Resettable for MIEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
