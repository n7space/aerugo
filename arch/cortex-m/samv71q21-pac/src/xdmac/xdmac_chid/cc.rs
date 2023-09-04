#[doc = "Register `CC` reader"]
pub type R = crate::R<CC_SPEC>;
#[doc = "Register `CC` writer"]
pub type W = crate::W<CC_SPEC>;
#[doc = "Field `TYPE` reader - Channel x Transfer Type"]
pub type TYPE_R = crate::BitReader<TYPESELECT_A>;
#[doc = "Channel x Transfer Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TYPESELECT_A {
    #[doc = "0: Self-triggered mode (memory-to-memory transfer)."]
    MEM_TRAN = 0,
    #[doc = "1: Synchronized mode (peripheral-to-memory or memory-to-peripheral transfer)."]
    PER_TRAN = 1,
}
impl From<TYPESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TYPESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TYPESELECT_A {
        match self.bits {
            false => TYPESELECT_A::MEM_TRAN,
            true => TYPESELECT_A::PER_TRAN,
        }
    }
    #[doc = "Self-triggered mode (memory-to-memory transfer)."]
    #[inline(always)]
    pub fn is_mem_tran(&self) -> bool {
        *self == TYPESELECT_A::MEM_TRAN
    }
    #[doc = "Synchronized mode (peripheral-to-memory or memory-to-peripheral transfer)."]
    #[inline(always)]
    pub fn is_per_tran(&self) -> bool {
        *self == TYPESELECT_A::PER_TRAN
    }
}
#[doc = "Field `TYPE` writer - Channel x Transfer Type"]
pub type TYPE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TYPESELECT_A>;
impl<'a, REG, const O: u8> TYPE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Self-triggered mode (memory-to-memory transfer)."]
    #[inline(always)]
    pub fn mem_tran(self) -> &'a mut crate::W<REG> {
        self.variant(TYPESELECT_A::MEM_TRAN)
    }
    #[doc = "Synchronized mode (peripheral-to-memory or memory-to-peripheral transfer)."]
    #[inline(always)]
    pub fn per_tran(self) -> &'a mut crate::W<REG> {
        self.variant(TYPESELECT_A::PER_TRAN)
    }
}
#[doc = "Field `MBSIZE` reader - Channel x Memory Burst Size"]
pub type MBSIZE_R = crate::FieldReader<MBSIZESELECT_A>;
#[doc = "Channel x Memory Burst Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MBSIZESELECT_A {
    #[doc = "0: The memory burst size is set to one."]
    SINGLE = 0,
    #[doc = "1: The memory burst size is set to four."]
    FOUR = 1,
    #[doc = "2: The memory burst size is set to eight."]
    EIGHT = 2,
    #[doc = "3: The memory burst size is set to sixteen."]
    SIXTEEN = 3,
}
impl From<MBSIZESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: MBSIZESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MBSIZESELECT_A {
    type Ux = u8;
}
impl MBSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MBSIZESELECT_A {
        match self.bits {
            0 => MBSIZESELECT_A::SINGLE,
            1 => MBSIZESELECT_A::FOUR,
            2 => MBSIZESELECT_A::EIGHT,
            3 => MBSIZESELECT_A::SIXTEEN,
            _ => unreachable!(),
        }
    }
    #[doc = "The memory burst size is set to one."]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == MBSIZESELECT_A::SINGLE
    }
    #[doc = "The memory burst size is set to four."]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == MBSIZESELECT_A::FOUR
    }
    #[doc = "The memory burst size is set to eight."]
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        *self == MBSIZESELECT_A::EIGHT
    }
    #[doc = "The memory burst size is set to sixteen."]
    #[inline(always)]
    pub fn is_sixteen(&self) -> bool {
        *self == MBSIZESELECT_A::SIXTEEN
    }
}
#[doc = "Field `MBSIZE` writer - Channel x Memory Burst Size"]
pub type MBSIZE_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, MBSIZESELECT_A>;
impl<'a, REG, const O: u8> MBSIZE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The memory burst size is set to one."]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(MBSIZESELECT_A::SINGLE)
    }
    #[doc = "The memory burst size is set to four."]
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(MBSIZESELECT_A::FOUR)
    }
    #[doc = "The memory burst size is set to eight."]
    #[inline(always)]
    pub fn eight(self) -> &'a mut crate::W<REG> {
        self.variant(MBSIZESELECT_A::EIGHT)
    }
    #[doc = "The memory burst size is set to sixteen."]
    #[inline(always)]
    pub fn sixteen(self) -> &'a mut crate::W<REG> {
        self.variant(MBSIZESELECT_A::SIXTEEN)
    }
}
#[doc = "Field `DSYNC` reader - Channel x Synchronization"]
pub type DSYNC_R = crate::BitReader<DSYNCSELECT_A>;
#[doc = "Channel x Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSYNCSELECT_A {
    #[doc = "0: Peripheral-to-memory transfer."]
    PER2MEM = 0,
    #[doc = "1: Memory-to-peripheral transfer."]
    MEM2PER = 1,
}
impl From<DSYNCSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DSYNCSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSYNCSELECT_A {
        match self.bits {
            false => DSYNCSELECT_A::PER2MEM,
            true => DSYNCSELECT_A::MEM2PER,
        }
    }
    #[doc = "Peripheral-to-memory transfer."]
    #[inline(always)]
    pub fn is_per2mem(&self) -> bool {
        *self == DSYNCSELECT_A::PER2MEM
    }
    #[doc = "Memory-to-peripheral transfer."]
    #[inline(always)]
    pub fn is_mem2per(&self) -> bool {
        *self == DSYNCSELECT_A::MEM2PER
    }
}
#[doc = "Field `DSYNC` writer - Channel x Synchronization"]
pub type DSYNC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DSYNCSELECT_A>;
impl<'a, REG, const O: u8> DSYNC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral-to-memory transfer."]
    #[inline(always)]
    pub fn per2mem(self) -> &'a mut crate::W<REG> {
        self.variant(DSYNCSELECT_A::PER2MEM)
    }
    #[doc = "Memory-to-peripheral transfer."]
    #[inline(always)]
    pub fn mem2per(self) -> &'a mut crate::W<REG> {
        self.variant(DSYNCSELECT_A::MEM2PER)
    }
}
#[doc = "Field `SWREQ` reader - Channel x Software Request Trigger"]
pub type SWREQ_R = crate::BitReader<SWREQSELECT_A>;
#[doc = "Channel x Software Request Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWREQSELECT_A {
    #[doc = "0: Hardware request line is connected to the peripheral request line."]
    HWR_CONNECTED = 0,
    #[doc = "1: Software request is connected to the peripheral request line."]
    SWR_CONNECTED = 1,
}
impl From<SWREQSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SWREQSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SWREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWREQSELECT_A {
        match self.bits {
            false => SWREQSELECT_A::HWR_CONNECTED,
            true => SWREQSELECT_A::SWR_CONNECTED,
        }
    }
    #[doc = "Hardware request line is connected to the peripheral request line."]
    #[inline(always)]
    pub fn is_hwr_connected(&self) -> bool {
        *self == SWREQSELECT_A::HWR_CONNECTED
    }
    #[doc = "Software request is connected to the peripheral request line."]
    #[inline(always)]
    pub fn is_swr_connected(&self) -> bool {
        *self == SWREQSELECT_A::SWR_CONNECTED
    }
}
#[doc = "Field `SWREQ` writer - Channel x Software Request Trigger"]
pub type SWREQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SWREQSELECT_A>;
impl<'a, REG, const O: u8> SWREQ_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware request line is connected to the peripheral request line."]
    #[inline(always)]
    pub fn hwr_connected(self) -> &'a mut crate::W<REG> {
        self.variant(SWREQSELECT_A::HWR_CONNECTED)
    }
    #[doc = "Software request is connected to the peripheral request line."]
    #[inline(always)]
    pub fn swr_connected(self) -> &'a mut crate::W<REG> {
        self.variant(SWREQSELECT_A::SWR_CONNECTED)
    }
}
#[doc = "Field `MEMSET` reader - Channel x Fill Block of memory"]
pub type MEMSET_R = crate::BitReader<MEMSETSELECT_A>;
#[doc = "Channel x Fill Block of memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEMSETSELECT_A {
    #[doc = "0: Memset is not activated."]
    NORMAL_MODE = 0,
    #[doc = "1: Sets the block of memory pointed by DA field to the specified value. This operation is performed on 8-, 16- or 32-bit basis."]
    HW_MODE = 1,
}
impl From<MEMSETSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: MEMSETSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl MEMSET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEMSETSELECT_A {
        match self.bits {
            false => MEMSETSELECT_A::NORMAL_MODE,
            true => MEMSETSELECT_A::HW_MODE,
        }
    }
    #[doc = "Memset is not activated."]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == MEMSETSELECT_A::NORMAL_MODE
    }
    #[doc = "Sets the block of memory pointed by DA field to the specified value. This operation is performed on 8-, 16- or 32-bit basis."]
    #[inline(always)]
    pub fn is_hw_mode(&self) -> bool {
        *self == MEMSETSELECT_A::HW_MODE
    }
}
#[doc = "Field `MEMSET` writer - Channel x Fill Block of memory"]
pub type MEMSET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MEMSETSELECT_A>;
impl<'a, REG, const O: u8> MEMSET_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Memset is not activated."]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut crate::W<REG> {
        self.variant(MEMSETSELECT_A::NORMAL_MODE)
    }
    #[doc = "Sets the block of memory pointed by DA field to the specified value. This operation is performed on 8-, 16- or 32-bit basis."]
    #[inline(always)]
    pub fn hw_mode(self) -> &'a mut crate::W<REG> {
        self.variant(MEMSETSELECT_A::HW_MODE)
    }
}
#[doc = "Field `CSIZE` reader - Channel x Chunk Size"]
pub type CSIZE_R = crate::FieldReader<CSIZESELECT_A>;
#[doc = "Channel x Chunk Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSIZESELECT_A {
    #[doc = "0: 1 data transferred"]
    CHK_1 = 0,
    #[doc = "1: 2 data transferred"]
    CHK_2 = 1,
    #[doc = "2: 4 data transferred"]
    CHK_4 = 2,
    #[doc = "3: 8 data transferred"]
    CHK_8 = 3,
    #[doc = "4: 16 data transferred"]
    CHK_16 = 4,
}
impl From<CSIZESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CSIZESELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CSIZESELECT_A {
    type Ux = u8;
}
impl CSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CSIZESELECT_A> {
        match self.bits {
            0 => Some(CSIZESELECT_A::CHK_1),
            1 => Some(CSIZESELECT_A::CHK_2),
            2 => Some(CSIZESELECT_A::CHK_4),
            3 => Some(CSIZESELECT_A::CHK_8),
            4 => Some(CSIZESELECT_A::CHK_16),
            _ => None,
        }
    }
    #[doc = "1 data transferred"]
    #[inline(always)]
    pub fn is_chk_1(&self) -> bool {
        *self == CSIZESELECT_A::CHK_1
    }
    #[doc = "2 data transferred"]
    #[inline(always)]
    pub fn is_chk_2(&self) -> bool {
        *self == CSIZESELECT_A::CHK_2
    }
    #[doc = "4 data transferred"]
    #[inline(always)]
    pub fn is_chk_4(&self) -> bool {
        *self == CSIZESELECT_A::CHK_4
    }
    #[doc = "8 data transferred"]
    #[inline(always)]
    pub fn is_chk_8(&self) -> bool {
        *self == CSIZESELECT_A::CHK_8
    }
    #[doc = "16 data transferred"]
    #[inline(always)]
    pub fn is_chk_16(&self) -> bool {
        *self == CSIZESELECT_A::CHK_16
    }
}
#[doc = "Field `CSIZE` writer - Channel x Chunk Size"]
pub type CSIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, CSIZESELECT_A>;
impl<'a, REG, const O: u8> CSIZE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 data transferred"]
    #[inline(always)]
    pub fn chk_1(self) -> &'a mut crate::W<REG> {
        self.variant(CSIZESELECT_A::CHK_1)
    }
    #[doc = "2 data transferred"]
    #[inline(always)]
    pub fn chk_2(self) -> &'a mut crate::W<REG> {
        self.variant(CSIZESELECT_A::CHK_2)
    }
    #[doc = "4 data transferred"]
    #[inline(always)]
    pub fn chk_4(self) -> &'a mut crate::W<REG> {
        self.variant(CSIZESELECT_A::CHK_4)
    }
    #[doc = "8 data transferred"]
    #[inline(always)]
    pub fn chk_8(self) -> &'a mut crate::W<REG> {
        self.variant(CSIZESELECT_A::CHK_8)
    }
    #[doc = "16 data transferred"]
    #[inline(always)]
    pub fn chk_16(self) -> &'a mut crate::W<REG> {
        self.variant(CSIZESELECT_A::CHK_16)
    }
}
#[doc = "Field `DWIDTH` reader - Channel x Data Width"]
pub type DWIDTH_R = crate::FieldReader<DWIDTHSELECT_A>;
#[doc = "Channel x Data Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DWIDTHSELECT_A {
    #[doc = "0: The data size is set to 8 bits"]
    BYTE = 0,
    #[doc = "1: The data size is set to 16 bits"]
    HALFWORD = 1,
    #[doc = "2: The data size is set to 32 bits"]
    WORD = 2,
}
impl From<DWIDTHSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DWIDTHSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DWIDTHSELECT_A {
    type Ux = u8;
}
impl DWIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DWIDTHSELECT_A> {
        match self.bits {
            0 => Some(DWIDTHSELECT_A::BYTE),
            1 => Some(DWIDTHSELECT_A::HALFWORD),
            2 => Some(DWIDTHSELECT_A::WORD),
            _ => None,
        }
    }
    #[doc = "The data size is set to 8 bits"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == DWIDTHSELECT_A::BYTE
    }
    #[doc = "The data size is set to 16 bits"]
    #[inline(always)]
    pub fn is_halfword(&self) -> bool {
        *self == DWIDTHSELECT_A::HALFWORD
    }
    #[doc = "The data size is set to 32 bits"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == DWIDTHSELECT_A::WORD
    }
}
#[doc = "Field `DWIDTH` writer - Channel x Data Width"]
pub type DWIDTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, DWIDTHSELECT_A>;
impl<'a, REG, const O: u8> DWIDTH_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The data size is set to 8 bits"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(DWIDTHSELECT_A::BYTE)
    }
    #[doc = "The data size is set to 16 bits"]
    #[inline(always)]
    pub fn halfword(self) -> &'a mut crate::W<REG> {
        self.variant(DWIDTHSELECT_A::HALFWORD)
    }
    #[doc = "The data size is set to 32 bits"]
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(DWIDTHSELECT_A::WORD)
    }
}
#[doc = "Field `SIF` reader - Channel x Source Interface Identifier"]
pub type SIF_R = crate::BitReader<SIFSELECT_A>;
#[doc = "Channel x Source Interface Identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIFSELECT_A {
    #[doc = "0: The data is read through the system bus interface 0."]
    AHB_IF0 = 0,
    #[doc = "1: The data is read through the system bus interface 1."]
    AHB_IF1 = 1,
}
impl From<SIFSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SIFSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIFSELECT_A {
        match self.bits {
            false => SIFSELECT_A::AHB_IF0,
            true => SIFSELECT_A::AHB_IF1,
        }
    }
    #[doc = "The data is read through the system bus interface 0."]
    #[inline(always)]
    pub fn is_ahb_if0(&self) -> bool {
        *self == SIFSELECT_A::AHB_IF0
    }
    #[doc = "The data is read through the system bus interface 1."]
    #[inline(always)]
    pub fn is_ahb_if1(&self) -> bool {
        *self == SIFSELECT_A::AHB_IF1
    }
}
#[doc = "Field `SIF` writer - Channel x Source Interface Identifier"]
pub type SIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SIFSELECT_A>;
impl<'a, REG, const O: u8> SIF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The data is read through the system bus interface 0."]
    #[inline(always)]
    pub fn ahb_if0(self) -> &'a mut crate::W<REG> {
        self.variant(SIFSELECT_A::AHB_IF0)
    }
    #[doc = "The data is read through the system bus interface 1."]
    #[inline(always)]
    pub fn ahb_if1(self) -> &'a mut crate::W<REG> {
        self.variant(SIFSELECT_A::AHB_IF1)
    }
}
#[doc = "Field `DIF` reader - Channel x Destination Interface Identifier"]
pub type DIF_R = crate::BitReader<DIFSELECT_A>;
#[doc = "Channel x Destination Interface Identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIFSELECT_A {
    #[doc = "0: The data is written through the system bus interface 0."]
    AHB_IF0 = 0,
    #[doc = "1: The data is written though the system bus interface 1."]
    AHB_IF1 = 1,
}
impl From<DIFSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DIFSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIFSELECT_A {
        match self.bits {
            false => DIFSELECT_A::AHB_IF0,
            true => DIFSELECT_A::AHB_IF1,
        }
    }
    #[doc = "The data is written through the system bus interface 0."]
    #[inline(always)]
    pub fn is_ahb_if0(&self) -> bool {
        *self == DIFSELECT_A::AHB_IF0
    }
    #[doc = "The data is written though the system bus interface 1."]
    #[inline(always)]
    pub fn is_ahb_if1(&self) -> bool {
        *self == DIFSELECT_A::AHB_IF1
    }
}
#[doc = "Field `DIF` writer - Channel x Destination Interface Identifier"]
pub type DIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DIFSELECT_A>;
impl<'a, REG, const O: u8> DIF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The data is written through the system bus interface 0."]
    #[inline(always)]
    pub fn ahb_if0(self) -> &'a mut crate::W<REG> {
        self.variant(DIFSELECT_A::AHB_IF0)
    }
    #[doc = "The data is written though the system bus interface 1."]
    #[inline(always)]
    pub fn ahb_if1(self) -> &'a mut crate::W<REG> {
        self.variant(DIFSELECT_A::AHB_IF1)
    }
}
#[doc = "Field `SAM` reader - Channel x Source Addressing Mode"]
pub type SAM_R = crate::FieldReader<SAMSELECT_A>;
#[doc = "Channel x Source Addressing Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAMSELECT_A {
    #[doc = "0: The address remains unchanged."]
    FIXED_AM = 0,
    #[doc = "1: The addressing mode is incremented (the increment size is set to the data size)."]
    INCREMENTED_AM = 1,
    #[doc = "2: The microblock stride is added at the microblock boundary."]
    UBS_AM = 2,
    #[doc = "3: The microblock stride is added at the microblock boundary, the data stride is added at the data boundary."]
    UBS_DS_AM = 3,
}
impl From<SAMSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SAMSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SAMSELECT_A {
    type Ux = u8;
}
impl SAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMSELECT_A {
        match self.bits {
            0 => SAMSELECT_A::FIXED_AM,
            1 => SAMSELECT_A::INCREMENTED_AM,
            2 => SAMSELECT_A::UBS_AM,
            3 => SAMSELECT_A::UBS_DS_AM,
            _ => unreachable!(),
        }
    }
    #[doc = "The address remains unchanged."]
    #[inline(always)]
    pub fn is_fixed_am(&self) -> bool {
        *self == SAMSELECT_A::FIXED_AM
    }
    #[doc = "The addressing mode is incremented (the increment size is set to the data size)."]
    #[inline(always)]
    pub fn is_incremented_am(&self) -> bool {
        *self == SAMSELECT_A::INCREMENTED_AM
    }
    #[doc = "The microblock stride is added at the microblock boundary."]
    #[inline(always)]
    pub fn is_ubs_am(&self) -> bool {
        *self == SAMSELECT_A::UBS_AM
    }
    #[doc = "The microblock stride is added at the microblock boundary, the data stride is added at the data boundary."]
    #[inline(always)]
    pub fn is_ubs_ds_am(&self) -> bool {
        *self == SAMSELECT_A::UBS_DS_AM
    }
}
#[doc = "Field `SAM` writer - Channel x Source Addressing Mode"]
pub type SAM_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, SAMSELECT_A>;
impl<'a, REG, const O: u8> SAM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The address remains unchanged."]
    #[inline(always)]
    pub fn fixed_am(self) -> &'a mut crate::W<REG> {
        self.variant(SAMSELECT_A::FIXED_AM)
    }
    #[doc = "The addressing mode is incremented (the increment size is set to the data size)."]
    #[inline(always)]
    pub fn incremented_am(self) -> &'a mut crate::W<REG> {
        self.variant(SAMSELECT_A::INCREMENTED_AM)
    }
    #[doc = "The microblock stride is added at the microblock boundary."]
    #[inline(always)]
    pub fn ubs_am(self) -> &'a mut crate::W<REG> {
        self.variant(SAMSELECT_A::UBS_AM)
    }
    #[doc = "The microblock stride is added at the microblock boundary, the data stride is added at the data boundary."]
    #[inline(always)]
    pub fn ubs_ds_am(self) -> &'a mut crate::W<REG> {
        self.variant(SAMSELECT_A::UBS_DS_AM)
    }
}
#[doc = "Field `DAM` reader - Channel x Destination Addressing Mode"]
pub type DAM_R = crate::FieldReader<DAMSELECT_A>;
#[doc = "Channel x Destination Addressing Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAMSELECT_A {
    #[doc = "0: The address remains unchanged."]
    FIXED_AM = 0,
    #[doc = "1: The addressing mode is incremented (the increment size is set to the data size)."]
    INCREMENTED_AM = 1,
    #[doc = "2: The microblock stride is added at the microblock boundary."]
    UBS_AM = 2,
    #[doc = "3: The microblock stride is added at the microblock boundary; the data stride is added at the data boundary."]
    UBS_DS_AM = 3,
}
impl From<DAMSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DAMSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DAMSELECT_A {
    type Ux = u8;
}
impl DAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAMSELECT_A {
        match self.bits {
            0 => DAMSELECT_A::FIXED_AM,
            1 => DAMSELECT_A::INCREMENTED_AM,
            2 => DAMSELECT_A::UBS_AM,
            3 => DAMSELECT_A::UBS_DS_AM,
            _ => unreachable!(),
        }
    }
    #[doc = "The address remains unchanged."]
    #[inline(always)]
    pub fn is_fixed_am(&self) -> bool {
        *self == DAMSELECT_A::FIXED_AM
    }
    #[doc = "The addressing mode is incremented (the increment size is set to the data size)."]
    #[inline(always)]
    pub fn is_incremented_am(&self) -> bool {
        *self == DAMSELECT_A::INCREMENTED_AM
    }
    #[doc = "The microblock stride is added at the microblock boundary."]
    #[inline(always)]
    pub fn is_ubs_am(&self) -> bool {
        *self == DAMSELECT_A::UBS_AM
    }
    #[doc = "The microblock stride is added at the microblock boundary; the data stride is added at the data boundary."]
    #[inline(always)]
    pub fn is_ubs_ds_am(&self) -> bool {
        *self == DAMSELECT_A::UBS_DS_AM
    }
}
#[doc = "Field `DAM` writer - Channel x Destination Addressing Mode"]
pub type DAM_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, DAMSELECT_A>;
impl<'a, REG, const O: u8> DAM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The address remains unchanged."]
    #[inline(always)]
    pub fn fixed_am(self) -> &'a mut crate::W<REG> {
        self.variant(DAMSELECT_A::FIXED_AM)
    }
    #[doc = "The addressing mode is incremented (the increment size is set to the data size)."]
    #[inline(always)]
    pub fn incremented_am(self) -> &'a mut crate::W<REG> {
        self.variant(DAMSELECT_A::INCREMENTED_AM)
    }
    #[doc = "The microblock stride is added at the microblock boundary."]
    #[inline(always)]
    pub fn ubs_am(self) -> &'a mut crate::W<REG> {
        self.variant(DAMSELECT_A::UBS_AM)
    }
    #[doc = "The microblock stride is added at the microblock boundary; the data stride is added at the data boundary."]
    #[inline(always)]
    pub fn ubs_ds_am(self) -> &'a mut crate::W<REG> {
        self.variant(DAMSELECT_A::UBS_DS_AM)
    }
}
#[doc = "Field `INITD` reader - Channel Initialization Terminated (this bit is read-only)"]
pub type INITD_R = crate::BitReader<INITDSELECT_A>;
#[doc = "Channel Initialization Terminated (this bit is read-only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INITDSELECT_A {
    #[doc = "0: Channel initialization is in progress."]
    IN_PROGRESS = 0,
    #[doc = "1: Channel initialization is completed."]
    TERMINATED = 1,
}
impl From<INITDSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: INITDSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl INITD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INITDSELECT_A {
        match self.bits {
            false => INITDSELECT_A::IN_PROGRESS,
            true => INITDSELECT_A::TERMINATED,
        }
    }
    #[doc = "Channel initialization is in progress."]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == INITDSELECT_A::IN_PROGRESS
    }
    #[doc = "Channel initialization is completed."]
    #[inline(always)]
    pub fn is_terminated(&self) -> bool {
        *self == INITDSELECT_A::TERMINATED
    }
}
#[doc = "Field `INITD` writer - Channel Initialization Terminated (this bit is read-only)"]
pub type INITD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, INITDSELECT_A>;
impl<'a, REG, const O: u8> INITD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel initialization is in progress."]
    #[inline(always)]
    pub fn in_progress(self) -> &'a mut crate::W<REG> {
        self.variant(INITDSELECT_A::IN_PROGRESS)
    }
    #[doc = "Channel initialization is completed."]
    #[inline(always)]
    pub fn terminated(self) -> &'a mut crate::W<REG> {
        self.variant(INITDSELECT_A::TERMINATED)
    }
}
#[doc = "Field `RDIP` reader - Read in Progress (this bit is read-only)"]
pub type RDIP_R = crate::BitReader<RDIPSELECT_A>;
#[doc = "Read in Progress (this bit is read-only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDIPSELECT_A {
    #[doc = "0: No active read transaction on the bus."]
    DONE = 0,
    #[doc = "1: A read transaction is in progress."]
    IN_PROGRESS = 1,
}
impl From<RDIPSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RDIPSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RDIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDIPSELECT_A {
        match self.bits {
            false => RDIPSELECT_A::DONE,
            true => RDIPSELECT_A::IN_PROGRESS,
        }
    }
    #[doc = "No active read transaction on the bus."]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == RDIPSELECT_A::DONE
    }
    #[doc = "A read transaction is in progress."]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == RDIPSELECT_A::IN_PROGRESS
    }
}
#[doc = "Field `RDIP` writer - Read in Progress (this bit is read-only)"]
pub type RDIP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, RDIPSELECT_A>;
impl<'a, REG, const O: u8> RDIP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No active read transaction on the bus."]
    #[inline(always)]
    pub fn done(self) -> &'a mut crate::W<REG> {
        self.variant(RDIPSELECT_A::DONE)
    }
    #[doc = "A read transaction is in progress."]
    #[inline(always)]
    pub fn in_progress(self) -> &'a mut crate::W<REG> {
        self.variant(RDIPSELECT_A::IN_PROGRESS)
    }
}
#[doc = "Field `WRIP` reader - Write in Progress (this bit is read-only)"]
pub type WRIP_R = crate::BitReader<WRIPSELECT_A>;
#[doc = "Write in Progress (this bit is read-only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRIPSELECT_A {
    #[doc = "0: No active write transaction on the bus."]
    DONE = 0,
    #[doc = "1: A write transaction is in progress."]
    IN_PROGRESS = 1,
}
impl From<WRIPSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WRIPSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WRIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRIPSELECT_A {
        match self.bits {
            false => WRIPSELECT_A::DONE,
            true => WRIPSELECT_A::IN_PROGRESS,
        }
    }
    #[doc = "No active write transaction on the bus."]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == WRIPSELECT_A::DONE
    }
    #[doc = "A write transaction is in progress."]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == WRIPSELECT_A::IN_PROGRESS
    }
}
#[doc = "Field `WRIP` writer - Write in Progress (this bit is read-only)"]
pub type WRIP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, WRIPSELECT_A>;
impl<'a, REG, const O: u8> WRIP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No active write transaction on the bus."]
    #[inline(always)]
    pub fn done(self) -> &'a mut crate::W<REG> {
        self.variant(WRIPSELECT_A::DONE)
    }
    #[doc = "A write transaction is in progress."]
    #[inline(always)]
    pub fn in_progress(self) -> &'a mut crate::W<REG> {
        self.variant(WRIPSELECT_A::IN_PROGRESS)
    }
}
#[doc = "Field `PERID` reader - Channel x Peripheral Hardware Request Line Identifier"]
pub type PERID_R = crate::FieldReader<PERIDSELECT_A>;
#[doc = "Channel x Peripheral Hardware Request Line Identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PERIDSELECT_A {
    #[doc = "0: HSMCI"]
    HSMCI = 0,
    #[doc = "1: SPI0_TX"]
    SPI0_TX = 1,
    #[doc = "2: SPI0_RX"]
    SPI0_RX = 2,
    #[doc = "3: SPI1_TX"]
    SPI1_TX = 3,
    #[doc = "4: SPI1_RX"]
    SPI1_RX = 4,
    #[doc = "5: QSPI_TX"]
    QSPI_TX = 5,
    #[doc = "6: QSPI_RX"]
    QSPI_RX = 6,
    #[doc = "7: USART0_TX"]
    USART0_TX = 7,
    #[doc = "8: USART0_RX"]
    USART0_RX = 8,
    #[doc = "9: USART1_TX"]
    USART1_TX = 9,
    #[doc = "10: USART1_RX"]
    USART1_RX = 10,
    #[doc = "11: USART2_TX"]
    USART2_TX = 11,
    #[doc = "12: USART2_RX"]
    USART2_RX = 12,
    #[doc = "13: PWM0"]
    PWM0 = 13,
    #[doc = "14: TWIHS0_TX"]
    TWIHS0_TX = 14,
    #[doc = "15: TWIHS0_RX"]
    TWIHS0_RX = 15,
    #[doc = "16: TWIHS1_TX"]
    TWIHS1_TX = 16,
    #[doc = "17: TWIHS1_RX"]
    TWIHS1_RX = 17,
    #[doc = "18: TWIHS2_TX"]
    TWIHS2_TX = 18,
    #[doc = "19: TWIHS2_RX"]
    TWIHS2_RX = 19,
    #[doc = "20: UART0_TX"]
    UART0_TX = 20,
    #[doc = "21: UART0_RX"]
    UART0_RX = 21,
    #[doc = "22: UART1_TX"]
    UART1_TX = 22,
    #[doc = "23: UART1_RX"]
    UART1_RX = 23,
    #[doc = "24: UART2_TX"]
    UART2_TX = 24,
    #[doc = "25: UART2_RX"]
    UART2_RX = 25,
    #[doc = "26: UART3_TX"]
    UART3_TX = 26,
    #[doc = "27: UART3_RX"]
    UART3_RX = 27,
    #[doc = "28: UART4_TX"]
    UART4_TX = 28,
    #[doc = "29: UART4_RX"]
    UART4_RX = 29,
    #[doc = "30: DACC0"]
    DACC0 = 30,
    #[doc = "31: DACC1"]
    DACC1 = 31,
    #[doc = "32: SSC_TX"]
    SSC_TX = 32,
    #[doc = "33: SSC_RX"]
    SSC_RX = 33,
    #[doc = "34: PIOA"]
    PIOA = 34,
    #[doc = "35: AFEC0"]
    AFEC0 = 35,
    #[doc = "36: AFEC1"]
    AFEC1 = 36,
    #[doc = "37: AES_TX"]
    AES_TX = 37,
    #[doc = "38: AES_RX"]
    AES_RX = 38,
    #[doc = "39: PWM1"]
    PWM1 = 39,
    #[doc = "40: TC0"]
    TC0 = 40,
    #[doc = "41: TC3"]
    TC3 = 41,
    #[doc = "42: TC6"]
    TC6 = 42,
    #[doc = "43: TC9"]
    TC9 = 43,
    #[doc = "44: I2SC0_TX_LEFT"]
    I2SC0_TX_LEFT = 44,
    #[doc = "45: I2SC0_RX_LEFT"]
    I2SC0_RX_LEFT = 45,
    #[doc = "46: I2SC1_TX_LEFT"]
    I2SC1_TX_LEFT = 46,
    #[doc = "47: I2SC1_RX_LEFT"]
    I2SC1_RX_LEFT = 47,
    #[doc = "48: I2SC0_TX_RIGHT"]
    I2SC0_TX_RIGHT = 48,
    #[doc = "49: I2SC0_RX_RIGHT"]
    I2SC0_RX_RIGHT = 49,
    #[doc = "50: I2SC1_TX_RIGHT"]
    I2SC1_TX_RIGHT = 50,
    #[doc = "51: I2SC1_RX_RIGHT"]
    I2SC1_RX_RIGHT = 51,
}
impl From<PERIDSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PERIDSELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PERIDSELECT_A {
    type Ux = u8;
}
impl PERID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PERIDSELECT_A> {
        match self.bits {
            0 => Some(PERIDSELECT_A::HSMCI),
            1 => Some(PERIDSELECT_A::SPI0_TX),
            2 => Some(PERIDSELECT_A::SPI0_RX),
            3 => Some(PERIDSELECT_A::SPI1_TX),
            4 => Some(PERIDSELECT_A::SPI1_RX),
            5 => Some(PERIDSELECT_A::QSPI_TX),
            6 => Some(PERIDSELECT_A::QSPI_RX),
            7 => Some(PERIDSELECT_A::USART0_TX),
            8 => Some(PERIDSELECT_A::USART0_RX),
            9 => Some(PERIDSELECT_A::USART1_TX),
            10 => Some(PERIDSELECT_A::USART1_RX),
            11 => Some(PERIDSELECT_A::USART2_TX),
            12 => Some(PERIDSELECT_A::USART2_RX),
            13 => Some(PERIDSELECT_A::PWM0),
            14 => Some(PERIDSELECT_A::TWIHS0_TX),
            15 => Some(PERIDSELECT_A::TWIHS0_RX),
            16 => Some(PERIDSELECT_A::TWIHS1_TX),
            17 => Some(PERIDSELECT_A::TWIHS1_RX),
            18 => Some(PERIDSELECT_A::TWIHS2_TX),
            19 => Some(PERIDSELECT_A::TWIHS2_RX),
            20 => Some(PERIDSELECT_A::UART0_TX),
            21 => Some(PERIDSELECT_A::UART0_RX),
            22 => Some(PERIDSELECT_A::UART1_TX),
            23 => Some(PERIDSELECT_A::UART1_RX),
            24 => Some(PERIDSELECT_A::UART2_TX),
            25 => Some(PERIDSELECT_A::UART2_RX),
            26 => Some(PERIDSELECT_A::UART3_TX),
            27 => Some(PERIDSELECT_A::UART3_RX),
            28 => Some(PERIDSELECT_A::UART4_TX),
            29 => Some(PERIDSELECT_A::UART4_RX),
            30 => Some(PERIDSELECT_A::DACC0),
            31 => Some(PERIDSELECT_A::DACC1),
            32 => Some(PERIDSELECT_A::SSC_TX),
            33 => Some(PERIDSELECT_A::SSC_RX),
            34 => Some(PERIDSELECT_A::PIOA),
            35 => Some(PERIDSELECT_A::AFEC0),
            36 => Some(PERIDSELECT_A::AFEC1),
            37 => Some(PERIDSELECT_A::AES_TX),
            38 => Some(PERIDSELECT_A::AES_RX),
            39 => Some(PERIDSELECT_A::PWM1),
            40 => Some(PERIDSELECT_A::TC0),
            41 => Some(PERIDSELECT_A::TC3),
            42 => Some(PERIDSELECT_A::TC6),
            43 => Some(PERIDSELECT_A::TC9),
            44 => Some(PERIDSELECT_A::I2SC0_TX_LEFT),
            45 => Some(PERIDSELECT_A::I2SC0_RX_LEFT),
            46 => Some(PERIDSELECT_A::I2SC1_TX_LEFT),
            47 => Some(PERIDSELECT_A::I2SC1_RX_LEFT),
            48 => Some(PERIDSELECT_A::I2SC0_TX_RIGHT),
            49 => Some(PERIDSELECT_A::I2SC0_RX_RIGHT),
            50 => Some(PERIDSELECT_A::I2SC1_TX_RIGHT),
            51 => Some(PERIDSELECT_A::I2SC1_RX_RIGHT),
            _ => None,
        }
    }
    #[doc = "HSMCI"]
    #[inline(always)]
    pub fn is_hsmci(&self) -> bool {
        *self == PERIDSELECT_A::HSMCI
    }
    #[doc = "SPI0_TX"]
    #[inline(always)]
    pub fn is_spi0_tx(&self) -> bool {
        *self == PERIDSELECT_A::SPI0_TX
    }
    #[doc = "SPI0_RX"]
    #[inline(always)]
    pub fn is_spi0_rx(&self) -> bool {
        *self == PERIDSELECT_A::SPI0_RX
    }
    #[doc = "SPI1_TX"]
    #[inline(always)]
    pub fn is_spi1_tx(&self) -> bool {
        *self == PERIDSELECT_A::SPI1_TX
    }
    #[doc = "SPI1_RX"]
    #[inline(always)]
    pub fn is_spi1_rx(&self) -> bool {
        *self == PERIDSELECT_A::SPI1_RX
    }
    #[doc = "QSPI_TX"]
    #[inline(always)]
    pub fn is_qspi_tx(&self) -> bool {
        *self == PERIDSELECT_A::QSPI_TX
    }
    #[doc = "QSPI_RX"]
    #[inline(always)]
    pub fn is_qspi_rx(&self) -> bool {
        *self == PERIDSELECT_A::QSPI_RX
    }
    #[doc = "USART0_TX"]
    #[inline(always)]
    pub fn is_usart0_tx(&self) -> bool {
        *self == PERIDSELECT_A::USART0_TX
    }
    #[doc = "USART0_RX"]
    #[inline(always)]
    pub fn is_usart0_rx(&self) -> bool {
        *self == PERIDSELECT_A::USART0_RX
    }
    #[doc = "USART1_TX"]
    #[inline(always)]
    pub fn is_usart1_tx(&self) -> bool {
        *self == PERIDSELECT_A::USART1_TX
    }
    #[doc = "USART1_RX"]
    #[inline(always)]
    pub fn is_usart1_rx(&self) -> bool {
        *self == PERIDSELECT_A::USART1_RX
    }
    #[doc = "USART2_TX"]
    #[inline(always)]
    pub fn is_usart2_tx(&self) -> bool {
        *self == PERIDSELECT_A::USART2_TX
    }
    #[doc = "USART2_RX"]
    #[inline(always)]
    pub fn is_usart2_rx(&self) -> bool {
        *self == PERIDSELECT_A::USART2_RX
    }
    #[doc = "PWM0"]
    #[inline(always)]
    pub fn is_pwm0(&self) -> bool {
        *self == PERIDSELECT_A::PWM0
    }
    #[doc = "TWIHS0_TX"]
    #[inline(always)]
    pub fn is_twihs0_tx(&self) -> bool {
        *self == PERIDSELECT_A::TWIHS0_TX
    }
    #[doc = "TWIHS0_RX"]
    #[inline(always)]
    pub fn is_twihs0_rx(&self) -> bool {
        *self == PERIDSELECT_A::TWIHS0_RX
    }
    #[doc = "TWIHS1_TX"]
    #[inline(always)]
    pub fn is_twihs1_tx(&self) -> bool {
        *self == PERIDSELECT_A::TWIHS1_TX
    }
    #[doc = "TWIHS1_RX"]
    #[inline(always)]
    pub fn is_twihs1_rx(&self) -> bool {
        *self == PERIDSELECT_A::TWIHS1_RX
    }
    #[doc = "TWIHS2_TX"]
    #[inline(always)]
    pub fn is_twihs2_tx(&self) -> bool {
        *self == PERIDSELECT_A::TWIHS2_TX
    }
    #[doc = "TWIHS2_RX"]
    #[inline(always)]
    pub fn is_twihs2_rx(&self) -> bool {
        *self == PERIDSELECT_A::TWIHS2_RX
    }
    #[doc = "UART0_TX"]
    #[inline(always)]
    pub fn is_uart0_tx(&self) -> bool {
        *self == PERIDSELECT_A::UART0_TX
    }
    #[doc = "UART0_RX"]
    #[inline(always)]
    pub fn is_uart0_rx(&self) -> bool {
        *self == PERIDSELECT_A::UART0_RX
    }
    #[doc = "UART1_TX"]
    #[inline(always)]
    pub fn is_uart1_tx(&self) -> bool {
        *self == PERIDSELECT_A::UART1_TX
    }
    #[doc = "UART1_RX"]
    #[inline(always)]
    pub fn is_uart1_rx(&self) -> bool {
        *self == PERIDSELECT_A::UART1_RX
    }
    #[doc = "UART2_TX"]
    #[inline(always)]
    pub fn is_uart2_tx(&self) -> bool {
        *self == PERIDSELECT_A::UART2_TX
    }
    #[doc = "UART2_RX"]
    #[inline(always)]
    pub fn is_uart2_rx(&self) -> bool {
        *self == PERIDSELECT_A::UART2_RX
    }
    #[doc = "UART3_TX"]
    #[inline(always)]
    pub fn is_uart3_tx(&self) -> bool {
        *self == PERIDSELECT_A::UART3_TX
    }
    #[doc = "UART3_RX"]
    #[inline(always)]
    pub fn is_uart3_rx(&self) -> bool {
        *self == PERIDSELECT_A::UART3_RX
    }
    #[doc = "UART4_TX"]
    #[inline(always)]
    pub fn is_uart4_tx(&self) -> bool {
        *self == PERIDSELECT_A::UART4_TX
    }
    #[doc = "UART4_RX"]
    #[inline(always)]
    pub fn is_uart4_rx(&self) -> bool {
        *self == PERIDSELECT_A::UART4_RX
    }
    #[doc = "DACC0"]
    #[inline(always)]
    pub fn is_dacc0(&self) -> bool {
        *self == PERIDSELECT_A::DACC0
    }
    #[doc = "DACC1"]
    #[inline(always)]
    pub fn is_dacc1(&self) -> bool {
        *self == PERIDSELECT_A::DACC1
    }
    #[doc = "SSC_TX"]
    #[inline(always)]
    pub fn is_ssc_tx(&self) -> bool {
        *self == PERIDSELECT_A::SSC_TX
    }
    #[doc = "SSC_RX"]
    #[inline(always)]
    pub fn is_ssc_rx(&self) -> bool {
        *self == PERIDSELECT_A::SSC_RX
    }
    #[doc = "PIOA"]
    #[inline(always)]
    pub fn is_pioa(&self) -> bool {
        *self == PERIDSELECT_A::PIOA
    }
    #[doc = "AFEC0"]
    #[inline(always)]
    pub fn is_afec0(&self) -> bool {
        *self == PERIDSELECT_A::AFEC0
    }
    #[doc = "AFEC1"]
    #[inline(always)]
    pub fn is_afec1(&self) -> bool {
        *self == PERIDSELECT_A::AFEC1
    }
    #[doc = "AES_TX"]
    #[inline(always)]
    pub fn is_aes_tx(&self) -> bool {
        *self == PERIDSELECT_A::AES_TX
    }
    #[doc = "AES_RX"]
    #[inline(always)]
    pub fn is_aes_rx(&self) -> bool {
        *self == PERIDSELECT_A::AES_RX
    }
    #[doc = "PWM1"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == PERIDSELECT_A::PWM1
    }
    #[doc = "TC0"]
    #[inline(always)]
    pub fn is_tc0(&self) -> bool {
        *self == PERIDSELECT_A::TC0
    }
    #[doc = "TC3"]
    #[inline(always)]
    pub fn is_tc3(&self) -> bool {
        *self == PERIDSELECT_A::TC3
    }
    #[doc = "TC6"]
    #[inline(always)]
    pub fn is_tc6(&self) -> bool {
        *self == PERIDSELECT_A::TC6
    }
    #[doc = "TC9"]
    #[inline(always)]
    pub fn is_tc9(&self) -> bool {
        *self == PERIDSELECT_A::TC9
    }
    #[doc = "I2SC0_TX_LEFT"]
    #[inline(always)]
    pub fn is_i2sc0_tx_left(&self) -> bool {
        *self == PERIDSELECT_A::I2SC0_TX_LEFT
    }
    #[doc = "I2SC0_RX_LEFT"]
    #[inline(always)]
    pub fn is_i2sc0_rx_left(&self) -> bool {
        *self == PERIDSELECT_A::I2SC0_RX_LEFT
    }
    #[doc = "I2SC1_TX_LEFT"]
    #[inline(always)]
    pub fn is_i2sc1_tx_left(&self) -> bool {
        *self == PERIDSELECT_A::I2SC1_TX_LEFT
    }
    #[doc = "I2SC1_RX_LEFT"]
    #[inline(always)]
    pub fn is_i2sc1_rx_left(&self) -> bool {
        *self == PERIDSELECT_A::I2SC1_RX_LEFT
    }
    #[doc = "I2SC0_TX_RIGHT"]
    #[inline(always)]
    pub fn is_i2sc0_tx_right(&self) -> bool {
        *self == PERIDSELECT_A::I2SC0_TX_RIGHT
    }
    #[doc = "I2SC0_RX_RIGHT"]
    #[inline(always)]
    pub fn is_i2sc0_rx_right(&self) -> bool {
        *self == PERIDSELECT_A::I2SC0_RX_RIGHT
    }
    #[doc = "I2SC1_TX_RIGHT"]
    #[inline(always)]
    pub fn is_i2sc1_tx_right(&self) -> bool {
        *self == PERIDSELECT_A::I2SC1_TX_RIGHT
    }
    #[doc = "I2SC1_RX_RIGHT"]
    #[inline(always)]
    pub fn is_i2sc1_rx_right(&self) -> bool {
        *self == PERIDSELECT_A::I2SC1_RX_RIGHT
    }
}
#[doc = "Field `PERID` writer - Channel x Peripheral Hardware Request Line Identifier"]
pub type PERID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O, PERIDSELECT_A>;
impl<'a, REG, const O: u8> PERID_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HSMCI"]
    #[inline(always)]
    pub fn hsmci(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::HSMCI)
    }
    #[doc = "SPI0_TX"]
    #[inline(always)]
    pub fn spi0_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::SPI0_TX)
    }
    #[doc = "SPI0_RX"]
    #[inline(always)]
    pub fn spi0_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::SPI0_RX)
    }
    #[doc = "SPI1_TX"]
    #[inline(always)]
    pub fn spi1_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::SPI1_TX)
    }
    #[doc = "SPI1_RX"]
    #[inline(always)]
    pub fn spi1_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::SPI1_RX)
    }
    #[doc = "QSPI_TX"]
    #[inline(always)]
    pub fn qspi_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::QSPI_TX)
    }
    #[doc = "QSPI_RX"]
    #[inline(always)]
    pub fn qspi_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::QSPI_RX)
    }
    #[doc = "USART0_TX"]
    #[inline(always)]
    pub fn usart0_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::USART0_TX)
    }
    #[doc = "USART0_RX"]
    #[inline(always)]
    pub fn usart0_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::USART0_RX)
    }
    #[doc = "USART1_TX"]
    #[inline(always)]
    pub fn usart1_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::USART1_TX)
    }
    #[doc = "USART1_RX"]
    #[inline(always)]
    pub fn usart1_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::USART1_RX)
    }
    #[doc = "USART2_TX"]
    #[inline(always)]
    pub fn usart2_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::USART2_TX)
    }
    #[doc = "USART2_RX"]
    #[inline(always)]
    pub fn usart2_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::USART2_RX)
    }
    #[doc = "PWM0"]
    #[inline(always)]
    pub fn pwm0(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::PWM0)
    }
    #[doc = "TWIHS0_TX"]
    #[inline(always)]
    pub fn twihs0_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::TWIHS0_TX)
    }
    #[doc = "TWIHS0_RX"]
    #[inline(always)]
    pub fn twihs0_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::TWIHS0_RX)
    }
    #[doc = "TWIHS1_TX"]
    #[inline(always)]
    pub fn twihs1_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::TWIHS1_TX)
    }
    #[doc = "TWIHS1_RX"]
    #[inline(always)]
    pub fn twihs1_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::TWIHS1_RX)
    }
    #[doc = "TWIHS2_TX"]
    #[inline(always)]
    pub fn twihs2_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::TWIHS2_TX)
    }
    #[doc = "TWIHS2_RX"]
    #[inline(always)]
    pub fn twihs2_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::TWIHS2_RX)
    }
    #[doc = "UART0_TX"]
    #[inline(always)]
    pub fn uart0_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::UART0_TX)
    }
    #[doc = "UART0_RX"]
    #[inline(always)]
    pub fn uart0_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::UART0_RX)
    }
    #[doc = "UART1_TX"]
    #[inline(always)]
    pub fn uart1_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::UART1_TX)
    }
    #[doc = "UART1_RX"]
    #[inline(always)]
    pub fn uart1_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::UART1_RX)
    }
    #[doc = "UART2_TX"]
    #[inline(always)]
    pub fn uart2_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::UART2_TX)
    }
    #[doc = "UART2_RX"]
    #[inline(always)]
    pub fn uart2_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::UART2_RX)
    }
    #[doc = "UART3_TX"]
    #[inline(always)]
    pub fn uart3_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::UART3_TX)
    }
    #[doc = "UART3_RX"]
    #[inline(always)]
    pub fn uart3_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::UART3_RX)
    }
    #[doc = "UART4_TX"]
    #[inline(always)]
    pub fn uart4_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::UART4_TX)
    }
    #[doc = "UART4_RX"]
    #[inline(always)]
    pub fn uart4_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::UART4_RX)
    }
    #[doc = "DACC0"]
    #[inline(always)]
    pub fn dacc0(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::DACC0)
    }
    #[doc = "DACC1"]
    #[inline(always)]
    pub fn dacc1(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::DACC1)
    }
    #[doc = "SSC_TX"]
    #[inline(always)]
    pub fn ssc_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::SSC_TX)
    }
    #[doc = "SSC_RX"]
    #[inline(always)]
    pub fn ssc_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::SSC_RX)
    }
    #[doc = "PIOA"]
    #[inline(always)]
    pub fn pioa(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::PIOA)
    }
    #[doc = "AFEC0"]
    #[inline(always)]
    pub fn afec0(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::AFEC0)
    }
    #[doc = "AFEC1"]
    #[inline(always)]
    pub fn afec1(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::AFEC1)
    }
    #[doc = "AES_TX"]
    #[inline(always)]
    pub fn aes_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::AES_TX)
    }
    #[doc = "AES_RX"]
    #[inline(always)]
    pub fn aes_rx(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::AES_RX)
    }
    #[doc = "PWM1"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::PWM1)
    }
    #[doc = "TC0"]
    #[inline(always)]
    pub fn tc0(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::TC0)
    }
    #[doc = "TC3"]
    #[inline(always)]
    pub fn tc3(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::TC3)
    }
    #[doc = "TC6"]
    #[inline(always)]
    pub fn tc6(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::TC6)
    }
    #[doc = "TC9"]
    #[inline(always)]
    pub fn tc9(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::TC9)
    }
    #[doc = "I2SC0_TX_LEFT"]
    #[inline(always)]
    pub fn i2sc0_tx_left(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::I2SC0_TX_LEFT)
    }
    #[doc = "I2SC0_RX_LEFT"]
    #[inline(always)]
    pub fn i2sc0_rx_left(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::I2SC0_RX_LEFT)
    }
    #[doc = "I2SC1_TX_LEFT"]
    #[inline(always)]
    pub fn i2sc1_tx_left(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::I2SC1_TX_LEFT)
    }
    #[doc = "I2SC1_RX_LEFT"]
    #[inline(always)]
    pub fn i2sc1_rx_left(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::I2SC1_RX_LEFT)
    }
    #[doc = "I2SC0_TX_RIGHT"]
    #[inline(always)]
    pub fn i2sc0_tx_right(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::I2SC0_TX_RIGHT)
    }
    #[doc = "I2SC0_RX_RIGHT"]
    #[inline(always)]
    pub fn i2sc0_rx_right(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::I2SC0_RX_RIGHT)
    }
    #[doc = "I2SC1_TX_RIGHT"]
    #[inline(always)]
    pub fn i2sc1_tx_right(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::I2SC1_TX_RIGHT)
    }
    #[doc = "I2SC1_RX_RIGHT"]
    #[inline(always)]
    pub fn i2sc1_rx_right(self) -> &'a mut crate::W<REG> {
        self.variant(PERIDSELECT_A::I2SC1_RX_RIGHT)
    }
}
impl R {
    #[doc = "Bit 0 - Channel x Transfer Type"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Channel x Memory Burst Size"]
    #[inline(always)]
    pub fn mbsize(&self) -> MBSIZE_R {
        MBSIZE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - Channel x Synchronization"]
    #[inline(always)]
    pub fn dsync(&self) -> DSYNC_R {
        DSYNC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel x Software Request Trigger"]
    #[inline(always)]
    pub fn swreq(&self) -> SWREQ_R {
        SWREQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel x Fill Block of memory"]
    #[inline(always)]
    pub fn memset(&self) -> MEMSET_R {
        MEMSET_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Channel x Chunk Size"]
    #[inline(always)]
    pub fn csize(&self) -> CSIZE_R {
        CSIZE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - Channel x Data Width"]
    #[inline(always)]
    pub fn dwidth(&self) -> DWIDTH_R {
        DWIDTH_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - Channel x Source Interface Identifier"]
    #[inline(always)]
    pub fn sif(&self) -> SIF_R {
        SIF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel x Destination Interface Identifier"]
    #[inline(always)]
    pub fn dif(&self) -> DIF_R {
        DIF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Channel x Source Addressing Mode"]
    #[inline(always)]
    pub fn sam(&self) -> SAM_R {
        SAM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Channel x Destination Addressing Mode"]
    #[inline(always)]
    pub fn dam(&self) -> DAM_R {
        DAM_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21 - Channel Initialization Terminated (this bit is read-only)"]
    #[inline(always)]
    pub fn initd(&self) -> INITD_R {
        INITD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Read in Progress (this bit is read-only)"]
    #[inline(always)]
    pub fn rdip(&self) -> RDIP_R {
        RDIP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Write in Progress (this bit is read-only)"]
    #[inline(always)]
    pub fn wrip(&self) -> WRIP_R {
        WRIP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - Channel x Peripheral Hardware Request Line Identifier"]
    #[inline(always)]
    pub fn perid(&self) -> PERID_R {
        PERID_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Channel x Transfer Type"]
    #[inline(always)]
    #[must_use]
    pub fn type_(&mut self) -> TYPE_W<CC_SPEC, 0> {
        TYPE_W::new(self)
    }
    #[doc = "Bits 1:2 - Channel x Memory Burst Size"]
    #[inline(always)]
    #[must_use]
    pub fn mbsize(&mut self) -> MBSIZE_W<CC_SPEC, 1> {
        MBSIZE_W::new(self)
    }
    #[doc = "Bit 4 - Channel x Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn dsync(&mut self) -> DSYNC_W<CC_SPEC, 4> {
        DSYNC_W::new(self)
    }
    #[doc = "Bit 6 - Channel x Software Request Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swreq(&mut self) -> SWREQ_W<CC_SPEC, 6> {
        SWREQ_W::new(self)
    }
    #[doc = "Bit 7 - Channel x Fill Block of memory"]
    #[inline(always)]
    #[must_use]
    pub fn memset(&mut self) -> MEMSET_W<CC_SPEC, 7> {
        MEMSET_W::new(self)
    }
    #[doc = "Bits 8:10 - Channel x Chunk Size"]
    #[inline(always)]
    #[must_use]
    pub fn csize(&mut self) -> CSIZE_W<CC_SPEC, 8> {
        CSIZE_W::new(self)
    }
    #[doc = "Bits 11:12 - Channel x Data Width"]
    #[inline(always)]
    #[must_use]
    pub fn dwidth(&mut self) -> DWIDTH_W<CC_SPEC, 11> {
        DWIDTH_W::new(self)
    }
    #[doc = "Bit 13 - Channel x Source Interface Identifier"]
    #[inline(always)]
    #[must_use]
    pub fn sif(&mut self) -> SIF_W<CC_SPEC, 13> {
        SIF_W::new(self)
    }
    #[doc = "Bit 14 - Channel x Destination Interface Identifier"]
    #[inline(always)]
    #[must_use]
    pub fn dif(&mut self) -> DIF_W<CC_SPEC, 14> {
        DIF_W::new(self)
    }
    #[doc = "Bits 16:17 - Channel x Source Addressing Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sam(&mut self) -> SAM_W<CC_SPEC, 16> {
        SAM_W::new(self)
    }
    #[doc = "Bits 18:19 - Channel x Destination Addressing Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dam(&mut self) -> DAM_W<CC_SPEC, 18> {
        DAM_W::new(self)
    }
    #[doc = "Bit 21 - Channel Initialization Terminated (this bit is read-only)"]
    #[inline(always)]
    #[must_use]
    pub fn initd(&mut self) -> INITD_W<CC_SPEC, 21> {
        INITD_W::new(self)
    }
    #[doc = "Bit 22 - Read in Progress (this bit is read-only)"]
    #[inline(always)]
    #[must_use]
    pub fn rdip(&mut self) -> RDIP_W<CC_SPEC, 22> {
        RDIP_W::new(self)
    }
    #[doc = "Bit 23 - Write in Progress (this bit is read-only)"]
    #[inline(always)]
    #[must_use]
    pub fn wrip(&mut self) -> WRIP_W<CC_SPEC, 23> {
        WRIP_W::new(self)
    }
    #[doc = "Bits 24:30 - Channel x Peripheral Hardware Request Line Identifier"]
    #[inline(always)]
    #[must_use]
    pub fn perid(&mut self) -> PERID_W<CC_SPEC, 24> {
        PERID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CC_SPEC;
impl crate::RegisterSpec for CC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc::R`](R) reader structure"]
impl crate::Readable for CC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cc::W`](W) writer structure"]
impl crate::Writable for CC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CC to value 0"]
impl crate::Resettable for CC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
