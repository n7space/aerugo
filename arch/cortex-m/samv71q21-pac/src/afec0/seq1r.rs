#[doc = "Register `SEQ1R` reader"]
pub type R = crate::R<SEQ1R_SPEC>;
#[doc = "Register `SEQ1R` writer"]
pub type W = crate::W<SEQ1R_SPEC>;
#[doc = "Field `USCH0` reader - User Sequence Number 0"]
pub type USCH0_R = crate::FieldReader;
#[doc = "Field `USCH0` writer - User Sequence Number 0"]
pub type USCH0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `USCH1` reader - User Sequence Number 1"]
pub type USCH1_R = crate::FieldReader;
#[doc = "Field `USCH1` writer - User Sequence Number 1"]
pub type USCH1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `USCH2` reader - User Sequence Number 2"]
pub type USCH2_R = crate::FieldReader;
#[doc = "Field `USCH2` writer - User Sequence Number 2"]
pub type USCH2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `USCH3` reader - User Sequence Number 3"]
pub type USCH3_R = crate::FieldReader;
#[doc = "Field `USCH3` writer - User Sequence Number 3"]
pub type USCH3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `USCH4` reader - User Sequence Number 4"]
pub type USCH4_R = crate::FieldReader;
#[doc = "Field `USCH4` writer - User Sequence Number 4"]
pub type USCH4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `USCH5` reader - User Sequence Number 5"]
pub type USCH5_R = crate::FieldReader;
#[doc = "Field `USCH5` writer - User Sequence Number 5"]
pub type USCH5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `USCH6` reader - User Sequence Number 6"]
pub type USCH6_R = crate::FieldReader;
#[doc = "Field `USCH6` writer - User Sequence Number 6"]
pub type USCH6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `USCH7` reader - User Sequence Number 7"]
pub type USCH7_R = crate::FieldReader;
#[doc = "Field `USCH7` writer - User Sequence Number 7"]
pub type USCH7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - User Sequence Number 0"]
    #[inline(always)]
    pub fn usch0(&self) -> USCH0_R {
        USCH0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - User Sequence Number 1"]
    #[inline(always)]
    pub fn usch1(&self) -> USCH1_R {
        USCH1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - User Sequence Number 2"]
    #[inline(always)]
    pub fn usch2(&self) -> USCH2_R {
        USCH2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - User Sequence Number 3"]
    #[inline(always)]
    pub fn usch3(&self) -> USCH3_R {
        USCH3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - User Sequence Number 4"]
    #[inline(always)]
    pub fn usch4(&self) -> USCH4_R {
        USCH4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - User Sequence Number 5"]
    #[inline(always)]
    pub fn usch5(&self) -> USCH5_R {
        USCH5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - User Sequence Number 6"]
    #[inline(always)]
    pub fn usch6(&self) -> USCH6_R {
        USCH6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - User Sequence Number 7"]
    #[inline(always)]
    pub fn usch7(&self) -> USCH7_R {
        USCH7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - User Sequence Number 0"]
    #[inline(always)]
    #[must_use]
    pub fn usch0(&mut self) -> USCH0_W<SEQ1R_SPEC, 0> {
        USCH0_W::new(self)
    }
    #[doc = "Bits 4:7 - User Sequence Number 1"]
    #[inline(always)]
    #[must_use]
    pub fn usch1(&mut self) -> USCH1_W<SEQ1R_SPEC, 4> {
        USCH1_W::new(self)
    }
    #[doc = "Bits 8:11 - User Sequence Number 2"]
    #[inline(always)]
    #[must_use]
    pub fn usch2(&mut self) -> USCH2_W<SEQ1R_SPEC, 8> {
        USCH2_W::new(self)
    }
    #[doc = "Bits 12:15 - User Sequence Number 3"]
    #[inline(always)]
    #[must_use]
    pub fn usch3(&mut self) -> USCH3_W<SEQ1R_SPEC, 12> {
        USCH3_W::new(self)
    }
    #[doc = "Bits 16:19 - User Sequence Number 4"]
    #[inline(always)]
    #[must_use]
    pub fn usch4(&mut self) -> USCH4_W<SEQ1R_SPEC, 16> {
        USCH4_W::new(self)
    }
    #[doc = "Bits 20:23 - User Sequence Number 5"]
    #[inline(always)]
    #[must_use]
    pub fn usch5(&mut self) -> USCH5_W<SEQ1R_SPEC, 20> {
        USCH5_W::new(self)
    }
    #[doc = "Bits 24:27 - User Sequence Number 6"]
    #[inline(always)]
    #[must_use]
    pub fn usch6(&mut self) -> USCH6_W<SEQ1R_SPEC, 24> {
        USCH6_W::new(self)
    }
    #[doc = "Bits 28:31 - User Sequence Number 7"]
    #[inline(always)]
    #[must_use]
    pub fn usch7(&mut self) -> USCH7_W<SEQ1R_SPEC, 28> {
        USCH7_W::new(self)
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
#[doc = "AFEC Channel Sequence 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEQ1R_SPEC;
impl crate::RegisterSpec for SEQ1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq1r::R`](R) reader structure"]
impl crate::Readable for SEQ1R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`seq1r::W`](W) writer structure"]
impl crate::Writable for SEQ1R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEQ1R to value 0"]
impl crate::Resettable for SEQ1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
