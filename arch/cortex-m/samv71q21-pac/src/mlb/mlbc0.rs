#[doc = "Register `MLBC0` reader"]
pub type R = crate::R<MLBC0_SPEC>;
#[doc = "Register `MLBC0` writer"]
pub type W = crate::W<MLBC0_SPEC>;
#[doc = "Field `MLBEN` reader - MediaLB Enable"]
pub type MLBEN_R = crate::BitReader;
#[doc = "Field `MLBEN` writer - MediaLB Enable"]
pub type MLBEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MLBCLK` reader - MLBCLK (MediaLB clock) Speed Select"]
pub type MLBCLK_R = crate::FieldReader<MLBCLKSELECT_A>;
#[doc = "MLBCLK (MediaLB clock) Speed Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MLBCLKSELECT_A {
    #[doc = "0: 256xFs (for MLBPEN = 0)"]
    _256_FS = 0,
    #[doc = "1: 512xFs (for MLBPEN = 0)"]
    _512_FS = 1,
    #[doc = "2: 1024xFs (for MLBPEN = 0)"]
    _1024_FS = 2,
}
impl From<MLBCLKSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: MLBCLKSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MLBCLKSELECT_A {
    type Ux = u8;
}
impl MLBCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MLBCLKSELECT_A> {
        match self.bits {
            0 => Some(MLBCLKSELECT_A::_256_FS),
            1 => Some(MLBCLKSELECT_A::_512_FS),
            2 => Some(MLBCLKSELECT_A::_1024_FS),
            _ => None,
        }
    }
    #[doc = "256xFs (for MLBPEN = 0)"]
    #[inline(always)]
    pub fn is_256_fs(&self) -> bool {
        *self == MLBCLKSELECT_A::_256_FS
    }
    #[doc = "512xFs (for MLBPEN = 0)"]
    #[inline(always)]
    pub fn is_512_fs(&self) -> bool {
        *self == MLBCLKSELECT_A::_512_FS
    }
    #[doc = "1024xFs (for MLBPEN = 0)"]
    #[inline(always)]
    pub fn is_1024_fs(&self) -> bool {
        *self == MLBCLKSELECT_A::_1024_FS
    }
}
#[doc = "Field `MLBCLK` writer - MLBCLK (MediaLB clock) Speed Select"]
pub type MLBCLK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, MLBCLKSELECT_A>;
impl<'a, REG, const O: u8> MLBCLK_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "256xFs (for MLBPEN = 0)"]
    #[inline(always)]
    pub fn _256_fs(self) -> &'a mut crate::W<REG> {
        self.variant(MLBCLKSELECT_A::_256_FS)
    }
    #[doc = "512xFs (for MLBPEN = 0)"]
    #[inline(always)]
    pub fn _512_fs(self) -> &'a mut crate::W<REG> {
        self.variant(MLBCLKSELECT_A::_512_FS)
    }
    #[doc = "1024xFs (for MLBPEN = 0)"]
    #[inline(always)]
    pub fn _1024_fs(self) -> &'a mut crate::W<REG> {
        self.variant(MLBCLKSELECT_A::_1024_FS)
    }
}
#[doc = "Field `ZERO` reader - Must be Written to 0"]
pub type ZERO_R = crate::BitReader;
#[doc = "Field `ZERO` writer - Must be Written to 0"]
pub type ZERO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MLBLK` reader - MediaLB Lock Status (read-only)"]
pub type MLBLK_R = crate::BitReader;
#[doc = "Field `MLBLK` writer - MediaLB Lock Status (read-only)"]
pub type MLBLK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ASYRETRY` reader - Asynchronous Tx Packet Retry"]
pub type ASYRETRY_R = crate::BitReader;
#[doc = "Field `ASYRETRY` writer - Asynchronous Tx Packet Retry"]
pub type ASYRETRY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTLRETRY` reader - Control Tx Packet Retry"]
pub type CTLRETRY_R = crate::BitReader;
#[doc = "Field `CTLRETRY` writer - Control Tx Packet Retry"]
pub type CTLRETRY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FCNT` reader - The number of frames per sub-buffer for synchronous channels"]
pub type FCNT_R = crate::FieldReader<FCNTSELECT_A>;
#[doc = "The number of frames per sub-buffer for synchronous channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FCNTSELECT_A {
    #[doc = "0: 1 frame per sub-buffer (Operation is the same as Standard mode.)"]
    _1_FRAME = 0,
    #[doc = "1: 2 frames per sub-buffer"]
    _2_FRAMES = 1,
    #[doc = "2: 4 frames per sub-buffer"]
    _4_FRAMES = 2,
    #[doc = "3: 8 frames per sub-buffer"]
    _8_FRAMES = 3,
    #[doc = "4: 16 frames per sub-buffer"]
    _16_FRAMES = 4,
    #[doc = "5: 32 frames per sub-buffer"]
    _32_FRAMES = 5,
    #[doc = "6: 64 frames per sub-buffer"]
    _64_FRAMES = 6,
}
impl From<FCNTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: FCNTSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FCNTSELECT_A {
    type Ux = u8;
}
impl FCNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FCNTSELECT_A> {
        match self.bits {
            0 => Some(FCNTSELECT_A::_1_FRAME),
            1 => Some(FCNTSELECT_A::_2_FRAMES),
            2 => Some(FCNTSELECT_A::_4_FRAMES),
            3 => Some(FCNTSELECT_A::_8_FRAMES),
            4 => Some(FCNTSELECT_A::_16_FRAMES),
            5 => Some(FCNTSELECT_A::_32_FRAMES),
            6 => Some(FCNTSELECT_A::_64_FRAMES),
            _ => None,
        }
    }
    #[doc = "1 frame per sub-buffer (Operation is the same as Standard mode.)"]
    #[inline(always)]
    pub fn is_1_frame(&self) -> bool {
        *self == FCNTSELECT_A::_1_FRAME
    }
    #[doc = "2 frames per sub-buffer"]
    #[inline(always)]
    pub fn is_2_frames(&self) -> bool {
        *self == FCNTSELECT_A::_2_FRAMES
    }
    #[doc = "4 frames per sub-buffer"]
    #[inline(always)]
    pub fn is_4_frames(&self) -> bool {
        *self == FCNTSELECT_A::_4_FRAMES
    }
    #[doc = "8 frames per sub-buffer"]
    #[inline(always)]
    pub fn is_8_frames(&self) -> bool {
        *self == FCNTSELECT_A::_8_FRAMES
    }
    #[doc = "16 frames per sub-buffer"]
    #[inline(always)]
    pub fn is_16_frames(&self) -> bool {
        *self == FCNTSELECT_A::_16_FRAMES
    }
    #[doc = "32 frames per sub-buffer"]
    #[inline(always)]
    pub fn is_32_frames(&self) -> bool {
        *self == FCNTSELECT_A::_32_FRAMES
    }
    #[doc = "64 frames per sub-buffer"]
    #[inline(always)]
    pub fn is_64_frames(&self) -> bool {
        *self == FCNTSELECT_A::_64_FRAMES
    }
}
#[doc = "Field `FCNT` writer - The number of frames per sub-buffer for synchronous channels"]
pub type FCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, FCNTSELECT_A>;
impl<'a, REG, const O: u8> FCNT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 frame per sub-buffer (Operation is the same as Standard mode.)"]
    #[inline(always)]
    pub fn _1_frame(self) -> &'a mut crate::W<REG> {
        self.variant(FCNTSELECT_A::_1_FRAME)
    }
    #[doc = "2 frames per sub-buffer"]
    #[inline(always)]
    pub fn _2_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FCNTSELECT_A::_2_FRAMES)
    }
    #[doc = "4 frames per sub-buffer"]
    #[inline(always)]
    pub fn _4_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FCNTSELECT_A::_4_FRAMES)
    }
    #[doc = "8 frames per sub-buffer"]
    #[inline(always)]
    pub fn _8_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FCNTSELECT_A::_8_FRAMES)
    }
    #[doc = "16 frames per sub-buffer"]
    #[inline(always)]
    pub fn _16_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FCNTSELECT_A::_16_FRAMES)
    }
    #[doc = "32 frames per sub-buffer"]
    #[inline(always)]
    pub fn _32_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FCNTSELECT_A::_32_FRAMES)
    }
    #[doc = "64 frames per sub-buffer"]
    #[inline(always)]
    pub fn _64_frames(self) -> &'a mut crate::W<REG> {
        self.variant(FCNTSELECT_A::_64_FRAMES)
    }
}
impl R {
    #[doc = "Bit 0 - MediaLB Enable"]
    #[inline(always)]
    pub fn mlben(&self) -> MLBEN_R {
        MLBEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:4 - MLBCLK (MediaLB clock) Speed Select"]
    #[inline(always)]
    pub fn mlbclk(&self) -> MLBCLK_R {
        MLBCLK_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - Must be Written to 0"]
    #[inline(always)]
    pub fn zero(&self) -> ZERO_R {
        ZERO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - MediaLB Lock Status (read-only)"]
    #[inline(always)]
    pub fn mlblk(&self) -> MLBLK_R {
        MLBLK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - Asynchronous Tx Packet Retry"]
    #[inline(always)]
    pub fn asyretry(&self) -> ASYRETRY_R {
        ASYRETRY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Control Tx Packet Retry"]
    #[inline(always)]
    pub fn ctlretry(&self) -> CTLRETRY_R {
        CTLRETRY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:17 - The number of frames per sub-buffer for synchronous channels"]
    #[inline(always)]
    pub fn fcnt(&self) -> FCNT_R {
        FCNT_R::new(((self.bits >> 15) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MediaLB Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mlben(&mut self) -> MLBEN_W<MLBC0_SPEC, 0> {
        MLBEN_W::new(self)
    }
    #[doc = "Bits 2:4 - MLBCLK (MediaLB clock) Speed Select"]
    #[inline(always)]
    #[must_use]
    pub fn mlbclk(&mut self) -> MLBCLK_W<MLBC0_SPEC, 2> {
        MLBCLK_W::new(self)
    }
    #[doc = "Bit 5 - Must be Written to 0"]
    #[inline(always)]
    #[must_use]
    pub fn zero(&mut self) -> ZERO_W<MLBC0_SPEC, 5> {
        ZERO_W::new(self)
    }
    #[doc = "Bit 7 - MediaLB Lock Status (read-only)"]
    #[inline(always)]
    #[must_use]
    pub fn mlblk(&mut self) -> MLBLK_W<MLBC0_SPEC, 7> {
        MLBLK_W::new(self)
    }
    #[doc = "Bit 12 - Asynchronous Tx Packet Retry"]
    #[inline(always)]
    #[must_use]
    pub fn asyretry(&mut self) -> ASYRETRY_W<MLBC0_SPEC, 12> {
        ASYRETRY_W::new(self)
    }
    #[doc = "Bit 14 - Control Tx Packet Retry"]
    #[inline(always)]
    #[must_use]
    pub fn ctlretry(&mut self) -> CTLRETRY_W<MLBC0_SPEC, 14> {
        CTLRETRY_W::new(self)
    }
    #[doc = "Bits 15:17 - The number of frames per sub-buffer for synchronous channels"]
    #[inline(always)]
    #[must_use]
    pub fn fcnt(&mut self) -> FCNT_W<MLBC0_SPEC, 15> {
        FCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MediaLB Control 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mlbc0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mlbc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MLBC0_SPEC;
impl crate::RegisterSpec for MLBC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mlbc0::R`](R) reader structure"]
impl crate::Readable for MLBC0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mlbc0::W`](W) writer structure"]
impl crate::Writable for MLBC0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MLBC0 to value 0"]
impl crate::Resettable for MLBC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
