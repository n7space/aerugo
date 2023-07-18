#[doc = "Register `HSTDMASTATUS` reader"]
pub struct R(crate::R<HSTDMASTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSTDMASTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSTDMASTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSTDMASTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSTDMASTATUS` writer"]
pub struct W(crate::W<HSTDMASTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSTDMASTATUS_SPEC>;
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
impl From<crate::W<HSTDMASTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSTDMASTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHANN_ENB` reader - Channel Enable Status"]
pub type CHANN_ENB_R = crate::BitReader;
#[doc = "Field `CHANN_ENB` writer - Channel Enable Status"]
pub type CHANN_ENB_W<'a, const O: u8> = crate::BitWriter<'a, HSTDMASTATUS_SPEC, O>;
#[doc = "Field `CHANN_ACT` reader - Channel Active Status"]
pub type CHANN_ACT_R = crate::BitReader;
#[doc = "Field `CHANN_ACT` writer - Channel Active Status"]
pub type CHANN_ACT_W<'a, const O: u8> = crate::BitWriter<'a, HSTDMASTATUS_SPEC, O>;
#[doc = "Field `END_TR_ST` reader - End of Channel Transfer Status"]
pub type END_TR_ST_R = crate::BitReader;
#[doc = "Field `END_TR_ST` writer - End of Channel Transfer Status"]
pub type END_TR_ST_W<'a, const O: u8> = crate::BitWriter<'a, HSTDMASTATUS_SPEC, O>;
#[doc = "Field `END_BF_ST` reader - End of Channel Buffer Status"]
pub type END_BF_ST_R = crate::BitReader;
#[doc = "Field `END_BF_ST` writer - End of Channel Buffer Status"]
pub type END_BF_ST_W<'a, const O: u8> = crate::BitWriter<'a, HSTDMASTATUS_SPEC, O>;
#[doc = "Field `DESC_LDST` reader - Descriptor Loaded Status"]
pub type DESC_LDST_R = crate::BitReader;
#[doc = "Field `DESC_LDST` writer - Descriptor Loaded Status"]
pub type DESC_LDST_W<'a, const O: u8> = crate::BitWriter<'a, HSTDMASTATUS_SPEC, O>;
#[doc = "Field `BUFF_COUNT` reader - Buffer Byte Count"]
pub type BUFF_COUNT_R = crate::FieldReader<u16>;
#[doc = "Field `BUFF_COUNT` writer - Buffer Byte Count"]
pub type BUFF_COUNT_W<'a, const O: u8> = crate::FieldWriter<'a, HSTDMASTATUS_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bit 0 - Channel Enable Status"]
    #[inline(always)]
    pub fn chann_enb(&self) -> CHANN_ENB_R {
        CHANN_ENB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Active Status"]
    #[inline(always)]
    pub fn chann_act(&self) -> CHANN_ACT_R {
        CHANN_ACT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - End of Channel Transfer Status"]
    #[inline(always)]
    pub fn end_tr_st(&self) -> END_TR_ST_R {
        END_TR_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of Channel Buffer Status"]
    #[inline(always)]
    pub fn end_bf_st(&self) -> END_BF_ST_R {
        END_BF_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Descriptor Loaded Status"]
    #[inline(always)]
    pub fn desc_ldst(&self) -> DESC_LDST_R {
        DESC_LDST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Buffer Byte Count"]
    #[inline(always)]
    pub fn buff_count(&self) -> BUFF_COUNT_R {
        BUFF_COUNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Enable Status"]
    #[inline(always)]
    #[must_use]
    pub fn chann_enb(&mut self) -> CHANN_ENB_W<0> {
        CHANN_ENB_W::new(self)
    }
    #[doc = "Bit 1 - Channel Active Status"]
    #[inline(always)]
    #[must_use]
    pub fn chann_act(&mut self) -> CHANN_ACT_W<1> {
        CHANN_ACT_W::new(self)
    }
    #[doc = "Bit 4 - End of Channel Transfer Status"]
    #[inline(always)]
    #[must_use]
    pub fn end_tr_st(&mut self) -> END_TR_ST_W<4> {
        END_TR_ST_W::new(self)
    }
    #[doc = "Bit 5 - End of Channel Buffer Status"]
    #[inline(always)]
    #[must_use]
    pub fn end_bf_st(&mut self) -> END_BF_ST_W<5> {
        END_BF_ST_W::new(self)
    }
    #[doc = "Bit 6 - Descriptor Loaded Status"]
    #[inline(always)]
    #[must_use]
    pub fn desc_ldst(&mut self) -> DESC_LDST_W<6> {
        DESC_LDST_W::new(self)
    }
    #[doc = "Bits 16:31 - Buffer Byte Count"]
    #[inline(always)]
    #[must_use]
    pub fn buff_count(&mut self) -> BUFF_COUNT_W<16> {
        BUFF_COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host DMA Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstdmastatus](index.html) module"]
pub struct HSTDMASTATUS_SPEC;
impl crate::RegisterSpec for HSTDMASTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hstdmastatus::R](R) reader structure"]
impl crate::Readable for HSTDMASTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hstdmastatus::W](W) writer structure"]
impl crate::Writable for HSTDMASTATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSTDMASTATUS to value 0"]
impl crate::Resettable for HSTDMASTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
