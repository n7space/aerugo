#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISR_SPEC>;
#[doc = "Field `DATRDY` reader - Data Ready (cleared by setting bit START or bit SWRST in AES_CR or by reading AES_ODATARx)"]
pub type DATRDY_R = crate::BitReader;
#[doc = "Field `URAD` reader - Unspecified Register Access Detection Status (cleared by writing SWRST in AES_CR)"]
pub type URAD_R = crate::BitReader;
#[doc = "Field `URAT` reader - Unspecified Register Access (cleared by writing SWRST in AES_CR)"]
pub type URAT_R = crate::FieldReader<URATSELECT_A>;
#[doc = "Unspecified Register Access (cleared by writing SWRST in AES_CR)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum URATSELECT_A {
    #[doc = "0: Input Data Register written during the data processing when SMOD = 0x2 mode."]
    IDR_WR_PROCESSING = 0,
    #[doc = "1: Output Data Register read during the data processing."]
    ODR_RD_PROCESSING = 1,
    #[doc = "2: Mode Register written during the data processing."]
    MR_WR_PROCESSING = 2,
    #[doc = "3: Output Data Register read during the sub-keys generation."]
    ODR_RD_SUBKGEN = 3,
    #[doc = "4: Mode Register written during the sub-keys generation."]
    MR_WR_SUBKGEN = 4,
    #[doc = "5: Write-only register read access."]
    WOR_RD_ACCESS = 5,
}
impl From<URATSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: URATSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for URATSELECT_A {
    type Ux = u8;
}
impl URAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<URATSELECT_A> {
        match self.bits {
            0 => Some(URATSELECT_A::IDR_WR_PROCESSING),
            1 => Some(URATSELECT_A::ODR_RD_PROCESSING),
            2 => Some(URATSELECT_A::MR_WR_PROCESSING),
            3 => Some(URATSELECT_A::ODR_RD_SUBKGEN),
            4 => Some(URATSELECT_A::MR_WR_SUBKGEN),
            5 => Some(URATSELECT_A::WOR_RD_ACCESS),
            _ => None,
        }
    }
    #[doc = "Input Data Register written during the data processing when SMOD = 0x2 mode."]
    #[inline(always)]
    pub fn is_idr_wr_processing(&self) -> bool {
        *self == URATSELECT_A::IDR_WR_PROCESSING
    }
    #[doc = "Output Data Register read during the data processing."]
    #[inline(always)]
    pub fn is_odr_rd_processing(&self) -> bool {
        *self == URATSELECT_A::ODR_RD_PROCESSING
    }
    #[doc = "Mode Register written during the data processing."]
    #[inline(always)]
    pub fn is_mr_wr_processing(&self) -> bool {
        *self == URATSELECT_A::MR_WR_PROCESSING
    }
    #[doc = "Output Data Register read during the sub-keys generation."]
    #[inline(always)]
    pub fn is_odr_rd_subkgen(&self) -> bool {
        *self == URATSELECT_A::ODR_RD_SUBKGEN
    }
    #[doc = "Mode Register written during the sub-keys generation."]
    #[inline(always)]
    pub fn is_mr_wr_subkgen(&self) -> bool {
        *self == URATSELECT_A::MR_WR_SUBKGEN
    }
    #[doc = "Write-only register read access."]
    #[inline(always)]
    pub fn is_wor_rd_access(&self) -> bool {
        *self == URATSELECT_A::WOR_RD_ACCESS
    }
}
#[doc = "Field `TAGRDY` reader - GCM Tag Ready"]
pub type TAGRDY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Data Ready (cleared by setting bit START or bit SWRST in AES_CR or by reading AES_ODATARx)"]
    #[inline(always)]
    pub fn datrdy(&self) -> DATRDY_R {
        DATRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Unspecified Register Access Detection Status (cleared by writing SWRST in AES_CR)"]
    #[inline(always)]
    pub fn urad(&self) -> URAD_R {
        URAD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Unspecified Register Access (cleared by writing SWRST in AES_CR)"]
    #[inline(always)]
    pub fn urat(&self) -> URAT_R {
        URAT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - GCM Tag Ready"]
    #[inline(always)]
    pub fn tagrdy(&self) -> TAGRDY_R {
        TAGRDY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISR_SPEC {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
