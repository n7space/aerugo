#[doc = "Register `ST2CW1` reader"]
pub struct R(crate::R<ST2CW1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ST2CW1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ST2CW1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ST2CW1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ST2CW1` writer"]
pub struct W(crate::W<ST2CW1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ST2CW1_SPEC>;
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
impl From<crate::W<ST2CW1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ST2CW1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSVAL` reader - Offset Value in Bytes"]
pub type OFFSVAL_R = crate::FieldReader;
#[doc = "Field `OFFSVAL` writer - Offset Value in Bytes"]
pub type OFFSVAL_W<'a, const O: u8> = crate::FieldWriter<'a, ST2CW1_SPEC, 7, O>;
#[doc = "Field `OFFSSTRT` reader - Ethernet Frame Offset Start"]
pub type OFFSSTRT_R = crate::FieldReader<OFFSSTRTSELECT_A>;
#[doc = "Ethernet Frame Offset Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OFFSSTRTSELECT_A {
    #[doc = "0: Offset from the start of the frame"]
    FRAMESTART = 0,
    #[doc = "1: Offset from the byte after the EtherType field"]
    ETHERTYPE = 1,
    #[doc = "2: Offset from the byte after the IP header field"]
    IP = 2,
    #[doc = "3: Offset from the byte after the TCP/UDP header field"]
    TCP_UDP = 3,
}
impl From<OFFSSTRTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: OFFSSTRTSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OFFSSTRTSELECT_A {
    type Ux = u8;
}
impl OFFSSTRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFFSSTRTSELECT_A {
        match self.bits {
            0 => OFFSSTRTSELECT_A::FRAMESTART,
            1 => OFFSSTRTSELECT_A::ETHERTYPE,
            2 => OFFSSTRTSELECT_A::IP,
            3 => OFFSSTRTSELECT_A::TCP_UDP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FRAMESTART`"]
    #[inline(always)]
    pub fn is_framestart(&self) -> bool {
        *self == OFFSSTRTSELECT_A::FRAMESTART
    }
    #[doc = "Checks if the value of the field is `ETHERTYPE`"]
    #[inline(always)]
    pub fn is_ethertype(&self) -> bool {
        *self == OFFSSTRTSELECT_A::ETHERTYPE
    }
    #[doc = "Checks if the value of the field is `IP`"]
    #[inline(always)]
    pub fn is_ip(&self) -> bool {
        *self == OFFSSTRTSELECT_A::IP
    }
    #[doc = "Checks if the value of the field is `TCP_UDP`"]
    #[inline(always)]
    pub fn is_tcp_udp(&self) -> bool {
        *self == OFFSSTRTSELECT_A::TCP_UDP
    }
}
#[doc = "Field `OFFSSTRT` writer - Ethernet Frame Offset Start"]
pub type OFFSSTRT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, ST2CW1_SPEC, 2, O, OFFSSTRTSELECT_A>;
impl<'a, const O: u8> OFFSSTRT_W<'a, O> {
    #[doc = "Offset from the start of the frame"]
    #[inline(always)]
    pub fn framestart(self) -> &'a mut W {
        self.variant(OFFSSTRTSELECT_A::FRAMESTART)
    }
    #[doc = "Offset from the byte after the EtherType field"]
    #[inline(always)]
    pub fn ethertype(self) -> &'a mut W {
        self.variant(OFFSSTRTSELECT_A::ETHERTYPE)
    }
    #[doc = "Offset from the byte after the IP header field"]
    #[inline(always)]
    pub fn ip(self) -> &'a mut W {
        self.variant(OFFSSTRTSELECT_A::IP)
    }
    #[doc = "Offset from the byte after the TCP/UDP header field"]
    #[inline(always)]
    pub fn tcp_udp(self) -> &'a mut W {
        self.variant(OFFSSTRTSELECT_A::TCP_UDP)
    }
}
impl R {
    #[doc = "Bits 0:6 - Offset Value in Bytes"]
    #[inline(always)]
    pub fn offsval(&self) -> OFFSVAL_R {
        OFFSVAL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:8 - Ethernet Frame Offset Start"]
    #[inline(always)]
    pub fn offsstrt(&self) -> OFFSSTRT_R {
        OFFSSTRT_R::new(((self.bits >> 7) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Offset Value in Bytes"]
    #[inline(always)]
    #[must_use]
    pub fn offsval(&mut self) -> OFFSVAL_W<0> {
        OFFSVAL_W::new(self)
    }
    #[doc = "Bits 7:8 - Ethernet Frame Offset Start"]
    #[inline(always)]
    #[must_use]
    pub fn offsstrt(&mut self) -> OFFSSTRT_W<7> {
        OFFSSTRT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Screening Type 2 Compare Word 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw1](index.html) module"]
pub struct ST2CW1_SPEC;
impl crate::RegisterSpec for ST2CW1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [st2cw1::R](R) reader structure"]
impl crate::Readable for ST2CW1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [st2cw1::W](W) writer structure"]
impl crate::Writable for ST2CW1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ST2CW1 to value 0"]
impl crate::Resettable for ST2CW1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
