#[doc = "Register `CFG2` reader"]
pub type R = crate::R<CFG2_SPEC>;
#[doc = "Register `CFG2` writer"]
pub type W = crate::W<CFG2_SPEC>;
#[doc = "Field `IM_VSIZE` reader - Vertical Size of the Image Sensor \\[0..2047\\]"]
pub type IM_VSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `IM_VSIZE` writer - Vertical Size of the Image Sensor \\[0..2047\\]"]
pub type IM_VSIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
#[doc = "Field `GS_MODE` reader - Grayscale Pixel Format Mode"]
pub type GS_MODE_R = crate::BitReader;
#[doc = "Field `GS_MODE` writer - Grayscale Pixel Format Mode"]
pub type GS_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RGB_MODE` reader - RGB Input Mode"]
pub type RGB_MODE_R = crate::BitReader;
#[doc = "Field `RGB_MODE` writer - RGB Input Mode"]
pub type RGB_MODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GRAYSCALE` reader - Grayscale Mode Format Enable"]
pub type GRAYSCALE_R = crate::BitReader;
#[doc = "Field `GRAYSCALE` writer - Grayscale Mode Format Enable"]
pub type GRAYSCALE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RGB_SWAP` reader - RGB Format Swap Mode"]
pub type RGB_SWAP_R = crate::BitReader;
#[doc = "Field `RGB_SWAP` writer - RGB Format Swap Mode"]
pub type RGB_SWAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `COL_SPACE` reader - Color Space for the Image Data"]
pub type COL_SPACE_R = crate::BitReader;
#[doc = "Field `COL_SPACE` writer - Color Space for the Image Data"]
pub type COL_SPACE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IM_HSIZE` reader - Horizontal Size of the Image Sensor \\[0..2047\\]"]
pub type IM_HSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `IM_HSIZE` writer - Horizontal Size of the Image Sensor \\[0..2047\\]"]
pub type IM_HSIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
#[doc = "Field `YCC_SWAP` reader - YCrCb Format Swap Mode"]
pub type YCC_SWAP_R = crate::FieldReader<YCC_SWAPSELECT_A>;
#[doc = "YCrCb Format Swap Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum YCC_SWAPSELECT_A {
    #[doc = "0: Byte 0 Cb(i)Byte 1 Y(i)Byte 2 Cr(i)Byte 3 Y(i+1)"]
    DEFAULT = 0,
    #[doc = "1: Byte 0 Cr(i)Byte 1 Y(i)Byte 2 Cb(i)Byte 3 Y(i+1)"]
    MODE1 = 1,
    #[doc = "2: Byte 0 Y(i)Byte 1 Cb(i)Byte 2 Y(i+1)Byte 3 Cr(i)"]
    MODE2 = 2,
    #[doc = "3: Byte 0 Y(i)Byte 1 Cr(i)Byte 2 Y(i+1)Byte 3 Cb(i)"]
    MODE3 = 3,
}
impl From<YCC_SWAPSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: YCC_SWAPSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for YCC_SWAPSELECT_A {
    type Ux = u8;
}
impl YCC_SWAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> YCC_SWAPSELECT_A {
        match self.bits {
            0 => YCC_SWAPSELECT_A::DEFAULT,
            1 => YCC_SWAPSELECT_A::MODE1,
            2 => YCC_SWAPSELECT_A::MODE2,
            3 => YCC_SWAPSELECT_A::MODE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Byte 0 Cb(i)Byte 1 Y(i)Byte 2 Cr(i)Byte 3 Y(i+1)"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == YCC_SWAPSELECT_A::DEFAULT
    }
    #[doc = "Byte 0 Cr(i)Byte 1 Y(i)Byte 2 Cb(i)Byte 3 Y(i+1)"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == YCC_SWAPSELECT_A::MODE1
    }
    #[doc = "Byte 0 Y(i)Byte 1 Cb(i)Byte 2 Y(i+1)Byte 3 Cr(i)"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == YCC_SWAPSELECT_A::MODE2
    }
    #[doc = "Byte 0 Y(i)Byte 1 Cr(i)Byte 2 Y(i+1)Byte 3 Cb(i)"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == YCC_SWAPSELECT_A::MODE3
    }
}
#[doc = "Field `YCC_SWAP` writer - YCrCb Format Swap Mode"]
pub type YCC_SWAP_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, YCC_SWAPSELECT_A>;
impl<'a, REG, const O: u8> YCC_SWAP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Byte 0 Cb(i)Byte 1 Y(i)Byte 2 Cr(i)Byte 3 Y(i+1)"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(YCC_SWAPSELECT_A::DEFAULT)
    }
    #[doc = "Byte 0 Cr(i)Byte 1 Y(i)Byte 2 Cb(i)Byte 3 Y(i+1)"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(YCC_SWAPSELECT_A::MODE1)
    }
    #[doc = "Byte 0 Y(i)Byte 1 Cb(i)Byte 2 Y(i+1)Byte 3 Cr(i)"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut crate::W<REG> {
        self.variant(YCC_SWAPSELECT_A::MODE2)
    }
    #[doc = "Byte 0 Y(i)Byte 1 Cr(i)Byte 2 Y(i+1)Byte 3 Cb(i)"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut crate::W<REG> {
        self.variant(YCC_SWAPSELECT_A::MODE3)
    }
}
#[doc = "Field `RGB_CFG` reader - RGB Pixel Mapping Configuration"]
pub type RGB_CFG_R = crate::FieldReader<RGB_CFGSELECT_A>;
#[doc = "RGB Pixel Mapping Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RGB_CFGSELECT_A {
    #[doc = "0: Byte 0 R/G(MSB)Byte 1 G(LSB)/BByte 2 R/G(MSB)Byte 3 G(LSB)/B"]
    DEFAULT = 0,
    #[doc = "1: Byte 0 B/G(MSB)Byte 1 G(LSB)/RByte 2 B/G(MSB)Byte 3 G(LSB)/R"]
    MODE1 = 1,
    #[doc = "2: Byte 0 G(LSB)/RByte 1 B/G(MSB)Byte 2 G(LSB)/RByte 3 B/G(MSB)"]
    MODE2 = 2,
    #[doc = "3: Byte 0 G(LSB)/BByte 1 R/G(MSB)Byte 2 G(LSB)/BByte 3 R/G(MSB)"]
    MODE3 = 3,
}
impl From<RGB_CFGSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: RGB_CFGSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RGB_CFGSELECT_A {
    type Ux = u8;
}
impl RGB_CFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGB_CFGSELECT_A {
        match self.bits {
            0 => RGB_CFGSELECT_A::DEFAULT,
            1 => RGB_CFGSELECT_A::MODE1,
            2 => RGB_CFGSELECT_A::MODE2,
            3 => RGB_CFGSELECT_A::MODE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Byte 0 R/G(MSB)Byte 1 G(LSB)/BByte 2 R/G(MSB)Byte 3 G(LSB)/B"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == RGB_CFGSELECT_A::DEFAULT
    }
    #[doc = "Byte 0 B/G(MSB)Byte 1 G(LSB)/RByte 2 B/G(MSB)Byte 3 G(LSB)/R"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == RGB_CFGSELECT_A::MODE1
    }
    #[doc = "Byte 0 G(LSB)/RByte 1 B/G(MSB)Byte 2 G(LSB)/RByte 3 B/G(MSB)"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == RGB_CFGSELECT_A::MODE2
    }
    #[doc = "Byte 0 G(LSB)/BByte 1 R/G(MSB)Byte 2 G(LSB)/BByte 3 R/G(MSB)"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == RGB_CFGSELECT_A::MODE3
    }
}
#[doc = "Field `RGB_CFG` writer - RGB Pixel Mapping Configuration"]
pub type RGB_CFG_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, RGB_CFGSELECT_A>;
impl<'a, REG, const O: u8> RGB_CFG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Byte 0 R/G(MSB)Byte 1 G(LSB)/BByte 2 R/G(MSB)Byte 3 G(LSB)/B"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(RGB_CFGSELECT_A::DEFAULT)
    }
    #[doc = "Byte 0 B/G(MSB)Byte 1 G(LSB)/RByte 2 B/G(MSB)Byte 3 G(LSB)/R"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(RGB_CFGSELECT_A::MODE1)
    }
    #[doc = "Byte 0 G(LSB)/RByte 1 B/G(MSB)Byte 2 G(LSB)/RByte 3 B/G(MSB)"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut crate::W<REG> {
        self.variant(RGB_CFGSELECT_A::MODE2)
    }
    #[doc = "Byte 0 G(LSB)/BByte 1 R/G(MSB)Byte 2 G(LSB)/BByte 3 R/G(MSB)"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut crate::W<REG> {
        self.variant(RGB_CFGSELECT_A::MODE3)
    }
}
impl R {
    #[doc = "Bits 0:10 - Vertical Size of the Image Sensor \\[0..2047\\]"]
    #[inline(always)]
    pub fn im_vsize(&self) -> IM_VSIZE_R {
        IM_VSIZE_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Grayscale Pixel Format Mode"]
    #[inline(always)]
    pub fn gs_mode(&self) -> GS_MODE_R {
        GS_MODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RGB Input Mode"]
    #[inline(always)]
    pub fn rgb_mode(&self) -> RGB_MODE_R {
        RGB_MODE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Grayscale Mode Format Enable"]
    #[inline(always)]
    pub fn grayscale(&self) -> GRAYSCALE_R {
        GRAYSCALE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RGB Format Swap Mode"]
    #[inline(always)]
    pub fn rgb_swap(&self) -> RGB_SWAP_R {
        RGB_SWAP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Color Space for the Image Data"]
    #[inline(always)]
    pub fn col_space(&self) -> COL_SPACE_R {
        COL_SPACE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:26 - Horizontal Size of the Image Sensor \\[0..2047\\]"]
    #[inline(always)]
    pub fn im_hsize(&self) -> IM_HSIZE_R {
        IM_HSIZE_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 28:29 - YCrCb Format Swap Mode"]
    #[inline(always)]
    pub fn ycc_swap(&self) -> YCC_SWAP_R {
        YCC_SWAP_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - RGB Pixel Mapping Configuration"]
    #[inline(always)]
    pub fn rgb_cfg(&self) -> RGB_CFG_R {
        RGB_CFG_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - Vertical Size of the Image Sensor \\[0..2047\\]"]
    #[inline(always)]
    #[must_use]
    pub fn im_vsize(&mut self) -> IM_VSIZE_W<CFG2_SPEC, 0> {
        IM_VSIZE_W::new(self)
    }
    #[doc = "Bit 11 - Grayscale Pixel Format Mode"]
    #[inline(always)]
    #[must_use]
    pub fn gs_mode(&mut self) -> GS_MODE_W<CFG2_SPEC, 11> {
        GS_MODE_W::new(self)
    }
    #[doc = "Bit 12 - RGB Input Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rgb_mode(&mut self) -> RGB_MODE_W<CFG2_SPEC, 12> {
        RGB_MODE_W::new(self)
    }
    #[doc = "Bit 13 - Grayscale Mode Format Enable"]
    #[inline(always)]
    #[must_use]
    pub fn grayscale(&mut self) -> GRAYSCALE_W<CFG2_SPEC, 13> {
        GRAYSCALE_W::new(self)
    }
    #[doc = "Bit 14 - RGB Format Swap Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rgb_swap(&mut self) -> RGB_SWAP_W<CFG2_SPEC, 14> {
        RGB_SWAP_W::new(self)
    }
    #[doc = "Bit 15 - Color Space for the Image Data"]
    #[inline(always)]
    #[must_use]
    pub fn col_space(&mut self) -> COL_SPACE_W<CFG2_SPEC, 15> {
        COL_SPACE_W::new(self)
    }
    #[doc = "Bits 16:26 - Horizontal Size of the Image Sensor \\[0..2047\\]"]
    #[inline(always)]
    #[must_use]
    pub fn im_hsize(&mut self) -> IM_HSIZE_W<CFG2_SPEC, 16> {
        IM_HSIZE_W::new(self)
    }
    #[doc = "Bits 28:29 - YCrCb Format Swap Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ycc_swap(&mut self) -> YCC_SWAP_W<CFG2_SPEC, 28> {
        YCC_SWAP_W::new(self)
    }
    #[doc = "Bits 30:31 - RGB Pixel Mapping Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn rgb_cfg(&mut self) -> RGB_CFG_W<CFG2_SPEC, 30> {
        RGB_CFG_W::new(self)
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
#[doc = "ISI Configuration 2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG2_SPEC;
impl crate::RegisterSpec for CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg2::R`](R) reader structure"]
impl crate::Readable for CFG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg2::W`](W) writer structure"]
impl crate::Writable for CFG2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG2 to value 0"]
impl crate::Resettable for CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
