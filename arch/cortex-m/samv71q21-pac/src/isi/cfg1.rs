#[doc = "Register `CFG1` reader"]
pub struct R(crate::R<CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG1` writer"]
pub struct W(crate::W<CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG1_SPEC>;
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
impl From<crate::W<CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSYNC_POL` reader - Horizontal Synchronization Polarity"]
pub type HSYNC_POL_R = crate::BitReader;
#[doc = "Field `HSYNC_POL` writer - Horizontal Synchronization Polarity"]
pub type HSYNC_POL_W<'a, const O: u8> = crate::BitWriter<'a, CFG1_SPEC, O>;
#[doc = "Field `VSYNC_POL` reader - Vertical Synchronization Polarity"]
pub type VSYNC_POL_R = crate::BitReader;
#[doc = "Field `VSYNC_POL` writer - Vertical Synchronization Polarity"]
pub type VSYNC_POL_W<'a, const O: u8> = crate::BitWriter<'a, CFG1_SPEC, O>;
#[doc = "Field `PIXCLK_POL` reader - Pixel Clock Polarity"]
pub type PIXCLK_POL_R = crate::BitReader;
#[doc = "Field `PIXCLK_POL` writer - Pixel Clock Polarity"]
pub type PIXCLK_POL_W<'a, const O: u8> = crate::BitWriter<'a, CFG1_SPEC, O>;
#[doc = "Field `GRAYLE` reader - Grayscale Little Endian"]
pub type GRAYLE_R = crate::BitReader;
#[doc = "Field `GRAYLE` writer - Grayscale Little Endian"]
pub type GRAYLE_W<'a, const O: u8> = crate::BitWriter<'a, CFG1_SPEC, O>;
#[doc = "Field `EMB_SYNC` reader - Embedded Synchronization"]
pub type EMB_SYNC_R = crate::BitReader;
#[doc = "Field `EMB_SYNC` writer - Embedded Synchronization"]
pub type EMB_SYNC_W<'a, const O: u8> = crate::BitWriter<'a, CFG1_SPEC, O>;
#[doc = "Field `CRC_SYNC` reader - Embedded Synchronization Correction"]
pub type CRC_SYNC_R = crate::BitReader;
#[doc = "Field `CRC_SYNC` writer - Embedded Synchronization Correction"]
pub type CRC_SYNC_W<'a, const O: u8> = crate::BitWriter<'a, CFG1_SPEC, O>;
#[doc = "Field `FRATE` reader - Frame Rate \\[0..7\\]"]
pub type FRATE_R = crate::FieldReader;
#[doc = "Field `FRATE` writer - Frame Rate \\[0..7\\]"]
pub type FRATE_W<'a, const O: u8> = crate::FieldWriter<'a, CFG1_SPEC, 3, O>;
#[doc = "Field `DISCR` reader - Disable Codec Request"]
pub type DISCR_R = crate::BitReader;
#[doc = "Field `DISCR` writer - Disable Codec Request"]
pub type DISCR_W<'a, const O: u8> = crate::BitWriter<'a, CFG1_SPEC, O>;
#[doc = "Field `FULL` reader - Full Mode is Allowed"]
pub type FULL_R = crate::BitReader;
#[doc = "Field `FULL` writer - Full Mode is Allowed"]
pub type FULL_W<'a, const O: u8> = crate::BitWriter<'a, CFG1_SPEC, O>;
#[doc = "Field `THMASK` reader - Threshold Mask"]
pub type THMASK_R = crate::FieldReader<THMASKSELECT_A>;
#[doc = "Threshold Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum THMASKSELECT_A {
    #[doc = "0: Only 4 beats AHB burst allowed"]
    BEATS_4 = 0,
    #[doc = "1: Only 4 and 8 beats AHB burst allowed"]
    BEATS_8 = 1,
    #[doc = "2: 4, 8 and 16 beats AHB burst allowed"]
    BEATS_16 = 2,
}
impl From<THMASKSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: THMASKSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for THMASKSELECT_A {
    type Ux = u8;
}
impl THMASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<THMASKSELECT_A> {
        match self.bits {
            0 => Some(THMASKSELECT_A::BEATS_4),
            1 => Some(THMASKSELECT_A::BEATS_8),
            2 => Some(THMASKSELECT_A::BEATS_16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BEATS_4`"]
    #[inline(always)]
    pub fn is_beats_4(&self) -> bool {
        *self == THMASKSELECT_A::BEATS_4
    }
    #[doc = "Checks if the value of the field is `BEATS_8`"]
    #[inline(always)]
    pub fn is_beats_8(&self) -> bool {
        *self == THMASKSELECT_A::BEATS_8
    }
    #[doc = "Checks if the value of the field is `BEATS_16`"]
    #[inline(always)]
    pub fn is_beats_16(&self) -> bool {
        *self == THMASKSELECT_A::BEATS_16
    }
}
#[doc = "Field `THMASK` writer - Threshold Mask"]
pub type THMASK_W<'a, const O: u8> = crate::FieldWriter<'a, CFG1_SPEC, 2, O, THMASKSELECT_A>;
impl<'a, const O: u8> THMASK_W<'a, O> {
    #[doc = "Only 4 beats AHB burst allowed"]
    #[inline(always)]
    pub fn beats_4(self) -> &'a mut W {
        self.variant(THMASKSELECT_A::BEATS_4)
    }
    #[doc = "Only 4 and 8 beats AHB burst allowed"]
    #[inline(always)]
    pub fn beats_8(self) -> &'a mut W {
        self.variant(THMASKSELECT_A::BEATS_8)
    }
    #[doc = "4, 8 and 16 beats AHB burst allowed"]
    #[inline(always)]
    pub fn beats_16(self) -> &'a mut W {
        self.variant(THMASKSELECT_A::BEATS_16)
    }
}
#[doc = "Field `SLD` reader - Start of Line Delay"]
pub type SLD_R = crate::FieldReader;
#[doc = "Field `SLD` writer - Start of Line Delay"]
pub type SLD_W<'a, const O: u8> = crate::FieldWriter<'a, CFG1_SPEC, 8, O>;
#[doc = "Field `SFD` reader - Start of Frame Delay"]
pub type SFD_R = crate::FieldReader;
#[doc = "Field `SFD` writer - Start of Frame Delay"]
pub type SFD_W<'a, const O: u8> = crate::FieldWriter<'a, CFG1_SPEC, 8, O>;
impl R {
    #[doc = "Bit 2 - Horizontal Synchronization Polarity"]
    #[inline(always)]
    pub fn hsync_pol(&self) -> HSYNC_POL_R {
        HSYNC_POL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Vertical Synchronization Polarity"]
    #[inline(always)]
    pub fn vsync_pol(&self) -> VSYNC_POL_R {
        VSYNC_POL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pixel Clock Polarity"]
    #[inline(always)]
    pub fn pixclk_pol(&self) -> PIXCLK_POL_R {
        PIXCLK_POL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Grayscale Little Endian"]
    #[inline(always)]
    pub fn grayle(&self) -> GRAYLE_R {
        GRAYLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Embedded Synchronization"]
    #[inline(always)]
    pub fn emb_sync(&self) -> EMB_SYNC_R {
        EMB_SYNC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Embedded Synchronization Correction"]
    #[inline(always)]
    pub fn crc_sync(&self) -> CRC_SYNC_R {
        CRC_SYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Frame Rate \\[0..7\\]"]
    #[inline(always)]
    pub fn frate(&self) -> FRATE_R {
        FRATE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Disable Codec Request"]
    #[inline(always)]
    pub fn discr(&self) -> DISCR_R {
        DISCR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Full Mode is Allowed"]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Threshold Mask"]
    #[inline(always)]
    pub fn thmask(&self) -> THMASK_R {
        THMASK_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Start of Line Delay"]
    #[inline(always)]
    pub fn sld(&self) -> SLD_R {
        SLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Start of Frame Delay"]
    #[inline(always)]
    pub fn sfd(&self) -> SFD_R {
        SFD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - Horizontal Synchronization Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn hsync_pol(&mut self) -> HSYNC_POL_W<2> {
        HSYNC_POL_W::new(self)
    }
    #[doc = "Bit 3 - Vertical Synchronization Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn vsync_pol(&mut self) -> VSYNC_POL_W<3> {
        VSYNC_POL_W::new(self)
    }
    #[doc = "Bit 4 - Pixel Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pixclk_pol(&mut self) -> PIXCLK_POL_W<4> {
        PIXCLK_POL_W::new(self)
    }
    #[doc = "Bit 5 - Grayscale Little Endian"]
    #[inline(always)]
    #[must_use]
    pub fn grayle(&mut self) -> GRAYLE_W<5> {
        GRAYLE_W::new(self)
    }
    #[doc = "Bit 6 - Embedded Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn emb_sync(&mut self) -> EMB_SYNC_W<6> {
        EMB_SYNC_W::new(self)
    }
    #[doc = "Bit 7 - Embedded Synchronization Correction"]
    #[inline(always)]
    #[must_use]
    pub fn crc_sync(&mut self) -> CRC_SYNC_W<7> {
        CRC_SYNC_W::new(self)
    }
    #[doc = "Bits 8:10 - Frame Rate \\[0..7\\]"]
    #[inline(always)]
    #[must_use]
    pub fn frate(&mut self) -> FRATE_W<8> {
        FRATE_W::new(self)
    }
    #[doc = "Bit 11 - Disable Codec Request"]
    #[inline(always)]
    #[must_use]
    pub fn discr(&mut self) -> DISCR_W<11> {
        DISCR_W::new(self)
    }
    #[doc = "Bit 12 - Full Mode is Allowed"]
    #[inline(always)]
    #[must_use]
    pub fn full(&mut self) -> FULL_W<12> {
        FULL_W::new(self)
    }
    #[doc = "Bits 13:14 - Threshold Mask"]
    #[inline(always)]
    #[must_use]
    pub fn thmask(&mut self) -> THMASK_W<13> {
        THMASK_W::new(self)
    }
    #[doc = "Bits 16:23 - Start of Line Delay"]
    #[inline(always)]
    #[must_use]
    pub fn sld(&mut self) -> SLD_W<16> {
        SLD_W::new(self)
    }
    #[doc = "Bits 24:31 - Start of Frame Delay"]
    #[inline(always)]
    #[must_use]
    pub fn sfd(&mut self) -> SFD_W<24> {
        SFD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISI Configuration 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg1](index.html) module"]
pub struct CFG1_SPEC;
impl crate::RegisterSpec for CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg1::R](R) reader structure"]
impl crate::Readable for CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg1::W](W) writer structure"]
impl crate::Writable for CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
