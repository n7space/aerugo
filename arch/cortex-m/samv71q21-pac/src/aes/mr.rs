#[doc = "Register `MR` reader"]
pub type R = crate::R<MR_SPEC>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MR_SPEC>;
#[doc = "Field `CIPHER` reader - Processing Mode"]
pub type CIPHER_R = crate::BitReader;
#[doc = "Field `CIPHER` writer - Processing Mode"]
pub type CIPHER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GTAGEN` reader - GCM Automatic Tag Generation Enable"]
pub type GTAGEN_R = crate::BitReader;
#[doc = "Field `GTAGEN` writer - GCM Automatic Tag Generation Enable"]
pub type GTAGEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DUALBUFF` reader - Dual Input Buffer"]
pub type DUALBUFF_R = crate::BitReader<DUALBUFFSELECT_A>;
#[doc = "Dual Input Buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DUALBUFFSELECT_A {
    #[doc = "0: AES_IDATARx cannot be written during processing of previous block."]
    INACTIVE = 0,
    #[doc = "1: AES_IDATARx can be written during processing of previous block when SMOD = 2. It speeds up the overall runtime of large files."]
    ACTIVE = 1,
}
impl From<DUALBUFFSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DUALBUFFSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DUALBUFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DUALBUFFSELECT_A {
        match self.bits {
            false => DUALBUFFSELECT_A::INACTIVE,
            true => DUALBUFFSELECT_A::ACTIVE,
        }
    }
    #[doc = "AES_IDATARx cannot be written during processing of previous block."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DUALBUFFSELECT_A::INACTIVE
    }
    #[doc = "AES_IDATARx can be written during processing of previous block when SMOD = 2. It speeds up the overall runtime of large files."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == DUALBUFFSELECT_A::ACTIVE
    }
}
#[doc = "Field `DUALBUFF` writer - Dual Input Buffer"]
pub type DUALBUFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DUALBUFFSELECT_A>;
impl<'a, REG, const O: u8> DUALBUFF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AES_IDATARx cannot be written during processing of previous block."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(DUALBUFFSELECT_A::INACTIVE)
    }
    #[doc = "AES_IDATARx can be written during processing of previous block when SMOD = 2. It speeds up the overall runtime of large files."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(DUALBUFFSELECT_A::ACTIVE)
    }
}
#[doc = "Field `PROCDLY` reader - Processing Delay"]
pub type PROCDLY_R = crate::FieldReader;
#[doc = "Field `PROCDLY` writer - Processing Delay"]
pub type PROCDLY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SMOD` reader - Start Mode"]
pub type SMOD_R = crate::FieldReader<SMODSELECT_A>;
#[doc = "Start Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMODSELECT_A {
    #[doc = "0: Manual Mode"]
    MANUAL_START = 0,
    #[doc = "1: Auto Mode"]
    AUTO_START = 1,
    #[doc = "2: AES_IDATAR0 access only Auto Mode (DMA)"]
    IDATAR0_START = 2,
}
impl From<SMODSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SMODSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMODSELECT_A {
    type Ux = u8;
}
impl SMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SMODSELECT_A> {
        match self.bits {
            0 => Some(SMODSELECT_A::MANUAL_START),
            1 => Some(SMODSELECT_A::AUTO_START),
            2 => Some(SMODSELECT_A::IDATAR0_START),
            _ => None,
        }
    }
    #[doc = "Manual Mode"]
    #[inline(always)]
    pub fn is_manual_start(&self) -> bool {
        *self == SMODSELECT_A::MANUAL_START
    }
    #[doc = "Auto Mode"]
    #[inline(always)]
    pub fn is_auto_start(&self) -> bool {
        *self == SMODSELECT_A::AUTO_START
    }
    #[doc = "AES_IDATAR0 access only Auto Mode (DMA)"]
    #[inline(always)]
    pub fn is_idatar0_start(&self) -> bool {
        *self == SMODSELECT_A::IDATAR0_START
    }
}
#[doc = "Field `SMOD` writer - Start Mode"]
pub type SMOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, SMODSELECT_A>;
impl<'a, REG, const O: u8> SMOD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Manual Mode"]
    #[inline(always)]
    pub fn manual_start(self) -> &'a mut crate::W<REG> {
        self.variant(SMODSELECT_A::MANUAL_START)
    }
    #[doc = "Auto Mode"]
    #[inline(always)]
    pub fn auto_start(self) -> &'a mut crate::W<REG> {
        self.variant(SMODSELECT_A::AUTO_START)
    }
    #[doc = "AES_IDATAR0 access only Auto Mode (DMA)"]
    #[inline(always)]
    pub fn idatar0_start(self) -> &'a mut crate::W<REG> {
        self.variant(SMODSELECT_A::IDATAR0_START)
    }
}
#[doc = "Field `KEYSIZE` reader - Key Size"]
pub type KEYSIZE_R = crate::FieldReader<KEYSIZESELECT_A>;
#[doc = "Key Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEYSIZESELECT_A {
    #[doc = "0: AES Key Size is 128 bits"]
    AES128 = 0,
    #[doc = "1: AES Key Size is 192 bits"]
    AES192 = 1,
    #[doc = "2: AES Key Size is 256 bits"]
    AES256 = 2,
}
impl From<KEYSIZESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: KEYSIZESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KEYSIZESELECT_A {
    type Ux = u8;
}
impl KEYSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KEYSIZESELECT_A> {
        match self.bits {
            0 => Some(KEYSIZESELECT_A::AES128),
            1 => Some(KEYSIZESELECT_A::AES192),
            2 => Some(KEYSIZESELECT_A::AES256),
            _ => None,
        }
    }
    #[doc = "AES Key Size is 128 bits"]
    #[inline(always)]
    pub fn is_aes128(&self) -> bool {
        *self == KEYSIZESELECT_A::AES128
    }
    #[doc = "AES Key Size is 192 bits"]
    #[inline(always)]
    pub fn is_aes192(&self) -> bool {
        *self == KEYSIZESELECT_A::AES192
    }
    #[doc = "AES Key Size is 256 bits"]
    #[inline(always)]
    pub fn is_aes256(&self) -> bool {
        *self == KEYSIZESELECT_A::AES256
    }
}
#[doc = "Field `KEYSIZE` writer - Key Size"]
pub type KEYSIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, KEYSIZESELECT_A>;
impl<'a, REG, const O: u8> KEYSIZE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AES Key Size is 128 bits"]
    #[inline(always)]
    pub fn aes128(self) -> &'a mut crate::W<REG> {
        self.variant(KEYSIZESELECT_A::AES128)
    }
    #[doc = "AES Key Size is 192 bits"]
    #[inline(always)]
    pub fn aes192(self) -> &'a mut crate::W<REG> {
        self.variant(KEYSIZESELECT_A::AES192)
    }
    #[doc = "AES Key Size is 256 bits"]
    #[inline(always)]
    pub fn aes256(self) -> &'a mut crate::W<REG> {
        self.variant(KEYSIZESELECT_A::AES256)
    }
}
#[doc = "Field `OPMOD` reader - Operating Mode"]
pub type OPMOD_R = crate::FieldReader<OPMODSELECT_A>;
#[doc = "Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPMODSELECT_A {
    #[doc = "0: ECB: Electronic Code Book mode"]
    ECB = 0,
    #[doc = "1: CBC: Cipher Block Chaining mode"]
    CBC = 1,
    #[doc = "2: OFB: Output Feedback mode"]
    OFB = 2,
    #[doc = "3: CFB: Cipher Feedback mode"]
    CFB = 3,
    #[doc = "4: CTR: Counter mode (16-bit internal counter)"]
    CTR = 4,
    #[doc = "5: GCM: Galois/Counter mode"]
    GCM = 5,
}
impl From<OPMODSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: OPMODSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OPMODSELECT_A {
    type Ux = u8;
}
impl OPMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPMODSELECT_A> {
        match self.bits {
            0 => Some(OPMODSELECT_A::ECB),
            1 => Some(OPMODSELECT_A::CBC),
            2 => Some(OPMODSELECT_A::OFB),
            3 => Some(OPMODSELECT_A::CFB),
            4 => Some(OPMODSELECT_A::CTR),
            5 => Some(OPMODSELECT_A::GCM),
            _ => None,
        }
    }
    #[doc = "ECB: Electronic Code Book mode"]
    #[inline(always)]
    pub fn is_ecb(&self) -> bool {
        *self == OPMODSELECT_A::ECB
    }
    #[doc = "CBC: Cipher Block Chaining mode"]
    #[inline(always)]
    pub fn is_cbc(&self) -> bool {
        *self == OPMODSELECT_A::CBC
    }
    #[doc = "OFB: Output Feedback mode"]
    #[inline(always)]
    pub fn is_ofb(&self) -> bool {
        *self == OPMODSELECT_A::OFB
    }
    #[doc = "CFB: Cipher Feedback mode"]
    #[inline(always)]
    pub fn is_cfb(&self) -> bool {
        *self == OPMODSELECT_A::CFB
    }
    #[doc = "CTR: Counter mode (16-bit internal counter)"]
    #[inline(always)]
    pub fn is_ctr(&self) -> bool {
        *self == OPMODSELECT_A::CTR
    }
    #[doc = "GCM: Galois/Counter mode"]
    #[inline(always)]
    pub fn is_gcm(&self) -> bool {
        *self == OPMODSELECT_A::GCM
    }
}
#[doc = "Field `OPMOD` writer - Operating Mode"]
pub type OPMOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, OPMODSELECT_A>;
impl<'a, REG, const O: u8> OPMOD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ECB: Electronic Code Book mode"]
    #[inline(always)]
    pub fn ecb(self) -> &'a mut crate::W<REG> {
        self.variant(OPMODSELECT_A::ECB)
    }
    #[doc = "CBC: Cipher Block Chaining mode"]
    #[inline(always)]
    pub fn cbc(self) -> &'a mut crate::W<REG> {
        self.variant(OPMODSELECT_A::CBC)
    }
    #[doc = "OFB: Output Feedback mode"]
    #[inline(always)]
    pub fn ofb(self) -> &'a mut crate::W<REG> {
        self.variant(OPMODSELECT_A::OFB)
    }
    #[doc = "CFB: Cipher Feedback mode"]
    #[inline(always)]
    pub fn cfb(self) -> &'a mut crate::W<REG> {
        self.variant(OPMODSELECT_A::CFB)
    }
    #[doc = "CTR: Counter mode (16-bit internal counter)"]
    #[inline(always)]
    pub fn ctr(self) -> &'a mut crate::W<REG> {
        self.variant(OPMODSELECT_A::CTR)
    }
    #[doc = "GCM: Galois/Counter mode"]
    #[inline(always)]
    pub fn gcm(self) -> &'a mut crate::W<REG> {
        self.variant(OPMODSELECT_A::GCM)
    }
}
#[doc = "Field `LOD` reader - Last Output Data Mode"]
pub type LOD_R = crate::BitReader;
#[doc = "Field `LOD` writer - Last Output Data Mode"]
pub type LOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CFBS` reader - Cipher Feedback Data Size"]
pub type CFBS_R = crate::FieldReader<CFBSSELECT_A>;
#[doc = "Cipher Feedback Data Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CFBSSELECT_A {
    #[doc = "0: 128-bit"]
    SIZE_128BIT = 0,
    #[doc = "1: 64-bit"]
    SIZE_64BIT = 1,
    #[doc = "2: 32-bit"]
    SIZE_32BIT = 2,
    #[doc = "3: 16-bit"]
    SIZE_16BIT = 3,
    #[doc = "4: 8-bit"]
    SIZE_8BIT = 4,
}
impl From<CFBSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CFBSSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CFBSSELECT_A {
    type Ux = u8;
}
impl CFBS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFBSSELECT_A> {
        match self.bits {
            0 => Some(CFBSSELECT_A::SIZE_128BIT),
            1 => Some(CFBSSELECT_A::SIZE_64BIT),
            2 => Some(CFBSSELECT_A::SIZE_32BIT),
            3 => Some(CFBSSELECT_A::SIZE_16BIT),
            4 => Some(CFBSSELECT_A::SIZE_8BIT),
            _ => None,
        }
    }
    #[doc = "128-bit"]
    #[inline(always)]
    pub fn is_size_128bit(&self) -> bool {
        *self == CFBSSELECT_A::SIZE_128BIT
    }
    #[doc = "64-bit"]
    #[inline(always)]
    pub fn is_size_64bit(&self) -> bool {
        *self == CFBSSELECT_A::SIZE_64BIT
    }
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn is_size_32bit(&self) -> bool {
        *self == CFBSSELECT_A::SIZE_32BIT
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn is_size_16bit(&self) -> bool {
        *self == CFBSSELECT_A::SIZE_16BIT
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn is_size_8bit(&self) -> bool {
        *self == CFBSSELECT_A::SIZE_8BIT
    }
}
#[doc = "Field `CFBS` writer - Cipher Feedback Data Size"]
pub type CFBS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, CFBSSELECT_A>;
impl<'a, REG, const O: u8> CFBS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "128-bit"]
    #[inline(always)]
    pub fn size_128bit(self) -> &'a mut crate::W<REG> {
        self.variant(CFBSSELECT_A::SIZE_128BIT)
    }
    #[doc = "64-bit"]
    #[inline(always)]
    pub fn size_64bit(self) -> &'a mut crate::W<REG> {
        self.variant(CFBSSELECT_A::SIZE_64BIT)
    }
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn size_32bit(self) -> &'a mut crate::W<REG> {
        self.variant(CFBSSELECT_A::SIZE_32BIT)
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn size_16bit(self) -> &'a mut crate::W<REG> {
        self.variant(CFBSSELECT_A::SIZE_16BIT)
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn size_8bit(self) -> &'a mut crate::W<REG> {
        self.variant(CFBSSELECT_A::SIZE_8BIT)
    }
}
#[doc = "Field `CKEY` reader - Countermeasure Key"]
pub type CKEY_R = crate::FieldReader<CKEYSELECT_A>;
#[doc = "Countermeasure Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKEYSELECT_A {
    #[doc = "14: This field must be written with 0xE to allow CMTYPx bit configuration changes. Any other values will abort the write operation in CMTYPx bits.Always reads as 0."]
    PASSWD = 14,
}
impl From<CKEYSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CKEYSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKEYSELECT_A {
    type Ux = u8;
}
impl CKEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKEYSELECT_A> {
        match self.bits {
            14 => Some(CKEYSELECT_A::PASSWD),
            _ => None,
        }
    }
    #[doc = "This field must be written with 0xE to allow CMTYPx bit configuration changes. Any other values will abort the write operation in CMTYPx bits.Always reads as 0."]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        *self == CKEYSELECT_A::PASSWD
    }
}
#[doc = "Field `CKEY` writer - Countermeasure Key"]
pub type CKEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, CKEYSELECT_A>;
impl<'a, REG, const O: u8> CKEY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "This field must be written with 0xE to allow CMTYPx bit configuration changes. Any other values will abort the write operation in CMTYPx bits.Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(CKEYSELECT_A::PASSWD)
    }
}
impl R {
    #[doc = "Bit 0 - Processing Mode"]
    #[inline(always)]
    pub fn cipher(&self) -> CIPHER_R {
        CIPHER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GCM Automatic Tag Generation Enable"]
    #[inline(always)]
    pub fn gtagen(&self) -> GTAGEN_R {
        GTAGEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Dual Input Buffer"]
    #[inline(always)]
    pub fn dualbuff(&self) -> DUALBUFF_R {
        DUALBUFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Processing Delay"]
    #[inline(always)]
    pub fn procdly(&self) -> PROCDLY_R {
        PROCDLY_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Start Mode"]
    #[inline(always)]
    pub fn smod(&self) -> SMOD_R {
        SMOD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Key Size"]
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - Operating Mode"]
    #[inline(always)]
    pub fn opmod(&self) -> OPMOD_R {
        OPMOD_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Last Output Data Mode"]
    #[inline(always)]
    pub fn lod(&self) -> LOD_R {
        LOD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Cipher Feedback Data Size"]
    #[inline(always)]
    pub fn cfbs(&self) -> CFBS_R {
        CFBS_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:23 - Countermeasure Key"]
    #[inline(always)]
    pub fn ckey(&self) -> CKEY_R {
        CKEY_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Processing Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cipher(&mut self) -> CIPHER_W<MR_SPEC, 0> {
        CIPHER_W::new(self)
    }
    #[doc = "Bit 1 - GCM Automatic Tag Generation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gtagen(&mut self) -> GTAGEN_W<MR_SPEC, 1> {
        GTAGEN_W::new(self)
    }
    #[doc = "Bit 3 - Dual Input Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn dualbuff(&mut self) -> DUALBUFF_W<MR_SPEC, 3> {
        DUALBUFF_W::new(self)
    }
    #[doc = "Bits 4:7 - Processing Delay"]
    #[inline(always)]
    #[must_use]
    pub fn procdly(&mut self) -> PROCDLY_W<MR_SPEC, 4> {
        PROCDLY_W::new(self)
    }
    #[doc = "Bits 8:9 - Start Mode"]
    #[inline(always)]
    #[must_use]
    pub fn smod(&mut self) -> SMOD_W<MR_SPEC, 8> {
        SMOD_W::new(self)
    }
    #[doc = "Bits 10:11 - Key Size"]
    #[inline(always)]
    #[must_use]
    pub fn keysize(&mut self) -> KEYSIZE_W<MR_SPEC, 10> {
        KEYSIZE_W::new(self)
    }
    #[doc = "Bits 12:14 - Operating Mode"]
    #[inline(always)]
    #[must_use]
    pub fn opmod(&mut self) -> OPMOD_W<MR_SPEC, 12> {
        OPMOD_W::new(self)
    }
    #[doc = "Bit 15 - Last Output Data Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lod(&mut self) -> LOD_W<MR_SPEC, 15> {
        LOD_W::new(self)
    }
    #[doc = "Bits 16:18 - Cipher Feedback Data Size"]
    #[inline(always)]
    #[must_use]
    pub fn cfbs(&mut self) -> CFBS_W<MR_SPEC, 16> {
        CFBS_W::new(self)
    }
    #[doc = "Bits 20:23 - Countermeasure Key"]
    #[inline(always)]
    #[must_use]
    pub fn ckey(&mut self) -> CKEY_W<MR_SPEC, 20> {
        CKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
