#[doc = "Register `HSTDMACONTROL` reader"]
pub struct R(crate::R<HSTDMACONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSTDMACONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSTDMACONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSTDMACONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSTDMACONTROL` writer"]
pub struct W(crate::W<HSTDMACONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSTDMACONTROL_SPEC>;
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
impl From<crate::W<HSTDMACONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSTDMACONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHANN_ENB` reader - Channel Enable Command"]
pub type CHANN_ENB_R = crate::BitReader;
#[doc = "Field `CHANN_ENB` writer - Channel Enable Command"]
pub type CHANN_ENB_W<'a, const O: u8> = crate::BitWriter<'a, HSTDMACONTROL_SPEC, O>;
#[doc = "Field `LDNXT_DSC` reader - Load Next Channel Transfer Descriptor Enable Command"]
pub type LDNXT_DSC_R = crate::BitReader;
#[doc = "Field `LDNXT_DSC` writer - Load Next Channel Transfer Descriptor Enable Command"]
pub type LDNXT_DSC_W<'a, const O: u8> = crate::BitWriter<'a, HSTDMACONTROL_SPEC, O>;
#[doc = "Field `END_TR_EN` reader - End of Transfer Enable Control (OUT transfers only)"]
pub type END_TR_EN_R = crate::BitReader;
#[doc = "Field `END_TR_EN` writer - End of Transfer Enable Control (OUT transfers only)"]
pub type END_TR_EN_W<'a, const O: u8> = crate::BitWriter<'a, HSTDMACONTROL_SPEC, O>;
#[doc = "Field `END_B_EN` reader - End of Buffer Enable Control"]
pub type END_B_EN_R = crate::BitReader;
#[doc = "Field `END_B_EN` writer - End of Buffer Enable Control"]
pub type END_B_EN_W<'a, const O: u8> = crate::BitWriter<'a, HSTDMACONTROL_SPEC, O>;
#[doc = "Field `END_TR_IT` reader - End of Transfer Interrupt Enable"]
pub type END_TR_IT_R = crate::BitReader;
#[doc = "Field `END_TR_IT` writer - End of Transfer Interrupt Enable"]
pub type END_TR_IT_W<'a, const O: u8> = crate::BitWriter<'a, HSTDMACONTROL_SPEC, O>;
#[doc = "Field `END_BUFFIT` reader - End of Buffer Interrupt Enable"]
pub type END_BUFFIT_R = crate::BitReader;
#[doc = "Field `END_BUFFIT` writer - End of Buffer Interrupt Enable"]
pub type END_BUFFIT_W<'a, const O: u8> = crate::BitWriter<'a, HSTDMACONTROL_SPEC, O>;
#[doc = "Field `DESC_LD_IT` reader - Descriptor Loaded Interrupt Enable"]
pub type DESC_LD_IT_R = crate::BitReader;
#[doc = "Field `DESC_LD_IT` writer - Descriptor Loaded Interrupt Enable"]
pub type DESC_LD_IT_W<'a, const O: u8> = crate::BitWriter<'a, HSTDMACONTROL_SPEC, O>;
#[doc = "Field `BURST_LCK` reader - Burst Lock Enable"]
pub type BURST_LCK_R = crate::BitReader;
#[doc = "Field `BURST_LCK` writer - Burst Lock Enable"]
pub type BURST_LCK_W<'a, const O: u8> = crate::BitWriter<'a, HSTDMACONTROL_SPEC, O>;
#[doc = "Field `BUFF_LENGTH` reader - Buffer Byte Length (Write-only)"]
pub type BUFF_LENGTH_R = crate::FieldReader<u16>;
#[doc = "Field `BUFF_LENGTH` writer - Buffer Byte Length (Write-only)"]
pub type BUFF_LENGTH_W<'a, const O: u8> = crate::FieldWriter<'a, HSTDMACONTROL_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bit 0 - Channel Enable Command"]
    #[inline(always)]
    pub fn chann_enb(&self) -> CHANN_ENB_R {
        CHANN_ENB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Load Next Channel Transfer Descriptor Enable Command"]
    #[inline(always)]
    pub fn ldnxt_dsc(&self) -> LDNXT_DSC_R {
        LDNXT_DSC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of Transfer Enable Control (OUT transfers only)"]
    #[inline(always)]
    pub fn end_tr_en(&self) -> END_TR_EN_R {
        END_TR_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of Buffer Enable Control"]
    #[inline(always)]
    pub fn end_b_en(&self) -> END_B_EN_R {
        END_B_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End of Transfer Interrupt Enable"]
    #[inline(always)]
    pub fn end_tr_it(&self) -> END_TR_IT_R {
        END_TR_IT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn end_buffit(&self) -> END_BUFFIT_R {
        END_BUFFIT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Descriptor Loaded Interrupt Enable"]
    #[inline(always)]
    pub fn desc_ld_it(&self) -> DESC_LD_IT_R {
        DESC_LD_IT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Burst Lock Enable"]
    #[inline(always)]
    pub fn burst_lck(&self) -> BURST_LCK_R {
        BURST_LCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Buffer Byte Length (Write-only)"]
    #[inline(always)]
    pub fn buff_length(&self) -> BUFF_LENGTH_R {
        BUFF_LENGTH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Enable Command"]
    #[inline(always)]
    #[must_use]
    pub fn chann_enb(&mut self) -> CHANN_ENB_W<0> {
        CHANN_ENB_W::new(self)
    }
    #[doc = "Bit 1 - Load Next Channel Transfer Descriptor Enable Command"]
    #[inline(always)]
    #[must_use]
    pub fn ldnxt_dsc(&mut self) -> LDNXT_DSC_W<1> {
        LDNXT_DSC_W::new(self)
    }
    #[doc = "Bit 2 - End of Transfer Enable Control (OUT transfers only)"]
    #[inline(always)]
    #[must_use]
    pub fn end_tr_en(&mut self) -> END_TR_EN_W<2> {
        END_TR_EN_W::new(self)
    }
    #[doc = "Bit 3 - End of Buffer Enable Control"]
    #[inline(always)]
    #[must_use]
    pub fn end_b_en(&mut self) -> END_B_EN_W<3> {
        END_B_EN_W::new(self)
    }
    #[doc = "Bit 4 - End of Transfer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn end_tr_it(&mut self) -> END_TR_IT_W<4> {
        END_TR_IT_W::new(self)
    }
    #[doc = "Bit 5 - End of Buffer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn end_buffit(&mut self) -> END_BUFFIT_W<5> {
        END_BUFFIT_W::new(self)
    }
    #[doc = "Bit 6 - Descriptor Loaded Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn desc_ld_it(&mut self) -> DESC_LD_IT_W<6> {
        DESC_LD_IT_W::new(self)
    }
    #[doc = "Bit 7 - Burst Lock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn burst_lck(&mut self) -> BURST_LCK_W<7> {
        BURST_LCK_W::new(self)
    }
    #[doc = "Bits 16:31 - Buffer Byte Length (Write-only)"]
    #[inline(always)]
    #[must_use]
    pub fn buff_length(&mut self) -> BUFF_LENGTH_W<16> {
        BUFF_LENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host DMA Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstdmacontrol](index.html) module"]
pub struct HSTDMACONTROL_SPEC;
impl crate::RegisterSpec for HSTDMACONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hstdmacontrol::R](R) reader structure"]
impl crate::Readable for HSTDMACONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hstdmacontrol::W](W) writer structure"]
impl crate::Writable for HSTDMACONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSTDMACONTROL to value 0"]
impl crate::Resettable for HSTDMACONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
