#[doc = "Register `DEVEPTIMR_INTRPT_MODE[%s]` reader"]
pub struct R(crate::R<DEVEPTIMR_INTRPT_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVEPTIMR_INTRPT_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVEPTIMR_INTRPT_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVEPTIMR_INTRPT_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXINE` reader - Transmitted IN Data Interrupt"]
pub type TXINE_R = crate::BitReader;
#[doc = "Field `RXOUTE` reader - Received OUT Data Interrupt"]
pub type RXOUTE_R = crate::BitReader;
#[doc = "Field `RXSTPE` reader - Received SETUP Interrupt"]
pub type RXSTPE_R = crate::BitReader;
#[doc = "Field `NAKOUTE` reader - NAKed OUT Interrupt"]
pub type NAKOUTE_R = crate::BitReader;
#[doc = "Field `NAKINE` reader - NAKed IN Interrupt"]
pub type NAKINE_R = crate::BitReader;
#[doc = "Field `OVERFE` reader - Overflow Interrupt"]
pub type OVERFE_R = crate::BitReader;
#[doc = "Field `STALLEDE` reader - STALLed Interrupt"]
pub type STALLEDE_R = crate::BitReader;
#[doc = "Field `SHORTPACKETE` reader - Short Packet Interrupt"]
pub type SHORTPACKETE_R = crate::BitReader;
#[doc = "Field `NBUSYBKE` reader - Number of Busy Banks Interrupt"]
pub type NBUSYBKE_R = crate::BitReader;
#[doc = "Field `KILLBK` reader - Kill IN Bank"]
pub type KILLBK_R = crate::BitReader;
#[doc = "Field `FIFOCON` reader - FIFO Control"]
pub type FIFOCON_R = crate::BitReader;
#[doc = "Field `EPDISHDMA` reader - Endpoint Interrupts Disable HDMA Request"]
pub type EPDISHDMA_R = crate::BitReader;
#[doc = "Field `NYETDIS` reader - NYET Token Disable"]
pub type NYETDIS_R = crate::BitReader;
#[doc = "Field `RSTDT` reader - Reset Data Toggle"]
pub type RSTDT_R = crate::BitReader;
#[doc = "Field `STALLRQ` reader - STALL Request"]
pub type STALLRQ_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt"]
    #[inline(always)]
    pub fn txine(&self) -> TXINE_R {
        TXINE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt"]
    #[inline(always)]
    pub fn rxoute(&self) -> RXOUTE_R {
        RXOUTE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Received SETUP Interrupt"]
    #[inline(always)]
    pub fn rxstpe(&self) -> RXSTPE_R {
        RXSTPE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt"]
    #[inline(always)]
    pub fn nakoute(&self) -> NAKOUTE_R {
        NAKOUTE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAKed IN Interrupt"]
    #[inline(always)]
    pub fn nakine(&self) -> NAKINE_R {
        NAKINE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overflow Interrupt"]
    #[inline(always)]
    pub fn overfe(&self) -> OVERFE_R {
        OVERFE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - STALLed Interrupt"]
    #[inline(always)]
    pub fn stallede(&self) -> STALLEDE_R {
        STALLEDE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Short Packet Interrupt"]
    #[inline(always)]
    pub fn shortpackete(&self) -> SHORTPACKETE_R {
        SHORTPACKETE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt"]
    #[inline(always)]
    pub fn nbusybke(&self) -> NBUSYBKE_R {
        NBUSYBKE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Kill IN Bank"]
    #[inline(always)]
    pub fn killbk(&self) -> KILLBK_R {
        KILLBK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - FIFO Control"]
    #[inline(always)]
    pub fn fifocon(&self) -> FIFOCON_R {
        FIFOCON_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Endpoint Interrupts Disable HDMA Request"]
    #[inline(always)]
    pub fn epdishdma(&self) -> EPDISHDMA_R {
        EPDISHDMA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NYET Token Disable"]
    #[inline(always)]
    pub fn nyetdis(&self) -> NYETDIS_R {
        NYETDIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reset Data Toggle"]
    #[inline(always)]
    pub fn rstdt(&self) -> RSTDT_R {
        RSTDT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - STALL Request"]
    #[inline(always)]
    pub fn stallrq(&self) -> STALLRQ_R {
        STALLRQ_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "Device Endpoint Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptimr_intrpt_mode](index.html) module"]
pub struct DEVEPTIMR_INTRPT_MODE_SPEC;
impl crate::RegisterSpec for DEVEPTIMR_INTRPT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [deveptimr_intrpt_mode::R](R) reader structure"]
impl crate::Readable for DEVEPTIMR_INTRPT_MODE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVEPTIMR_INTRPT_MODE[%s]
to value 0"]
impl crate::Resettable for DEVEPTIMR_INTRPT_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
