#[doc = "Register `CMDR` writer"]
pub type W = crate::W<CMDR_SPEC>;
#[doc = "Field `CMDNB` writer - Command Number"]
pub type CMDNB_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Response Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RSPTYPSELECT_AW {
    #[doc = "0: No response"]
    NORESP = 0,
    #[doc = "1: 48-bit response"]
    _48_BIT = 1,
    #[doc = "2: 136-bit response"]
    _136_BIT = 2,
    #[doc = "3: R1b response type"]
    R1B = 3,
}
impl From<RSPTYPSELECT_AW> for u8 {
    #[inline(always)]
    fn from(variant: RSPTYPSELECT_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RSPTYPSELECT_AW {
    type Ux = u8;
}
#[doc = "Field `RSPTYP` writer - Response Type"]
pub type RSPTYP_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, RSPTYPSELECT_AW>;
impl<'a, REG, const O: u8> RSPTYP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No response"]
    #[inline(always)]
    pub fn noresp(self) -> &'a mut crate::W<REG> {
        self.variant(RSPTYPSELECT_AW::NORESP)
    }
    #[doc = "48-bit response"]
    #[inline(always)]
    pub fn _48_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RSPTYPSELECT_AW::_48_BIT)
    }
    #[doc = "136-bit response"]
    #[inline(always)]
    pub fn _136_bit(self) -> &'a mut crate::W<REG> {
        self.variant(RSPTYPSELECT_AW::_136_BIT)
    }
    #[doc = "R1b response type"]
    #[inline(always)]
    pub fn r1b(self) -> &'a mut crate::W<REG> {
        self.variant(RSPTYPSELECT_AW::R1B)
    }
}
#[doc = "Special Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPCMDSELECT_AW {
    #[doc = "0: Not a special CMD."]
    STD = 0,
    #[doc = "1: Initialization CMD: 74 clock cycles for initialization sequence."]
    INIT = 1,
    #[doc = "2: Synchronized CMD: Wait for the end of the current data block transfer before sending the pending command."]
    SYNC = 2,
    #[doc = "3: CE-ATA Completion Signal disable Command. The host cancels the ability for the device to return a command completion signal on the command line."]
    CE_ATA = 3,
    #[doc = "4: Interrupt command: Corresponds to the Interrupt Mode (CMD40)."]
    IT_CMD = 4,
    #[doc = "5: Interrupt response: Corresponds to the Interrupt Mode (CMD40)."]
    IT_RESP = 5,
    #[doc = "6: Boot Operation Request. Start a boot operation mode, the host processor can read boot data from the MMC device directly."]
    BOR = 6,
    #[doc = "7: End Boot Operation. This command allows the host processor to terminate the boot operation mode."]
    EBO = 7,
}
impl From<SPCMDSELECT_AW> for u8 {
    #[inline(always)]
    fn from(variant: SPCMDSELECT_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPCMDSELECT_AW {
    type Ux = u8;
}
#[doc = "Field `SPCMD` writer - Special Command"]
pub type SPCMD_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, SPCMDSELECT_AW>;
impl<'a, REG, const O: u8> SPCMD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Not a special CMD."]
    #[inline(always)]
    pub fn std(self) -> &'a mut crate::W<REG> {
        self.variant(SPCMDSELECT_AW::STD)
    }
    #[doc = "Initialization CMD: 74 clock cycles for initialization sequence."]
    #[inline(always)]
    pub fn init(self) -> &'a mut crate::W<REG> {
        self.variant(SPCMDSELECT_AW::INIT)
    }
    #[doc = "Synchronized CMD: Wait for the end of the current data block transfer before sending the pending command."]
    #[inline(always)]
    pub fn sync(self) -> &'a mut crate::W<REG> {
        self.variant(SPCMDSELECT_AW::SYNC)
    }
    #[doc = "CE-ATA Completion Signal disable Command. The host cancels the ability for the device to return a command completion signal on the command line."]
    #[inline(always)]
    pub fn ce_ata(self) -> &'a mut crate::W<REG> {
        self.variant(SPCMDSELECT_AW::CE_ATA)
    }
    #[doc = "Interrupt command: Corresponds to the Interrupt Mode (CMD40)."]
    #[inline(always)]
    pub fn it_cmd(self) -> &'a mut crate::W<REG> {
        self.variant(SPCMDSELECT_AW::IT_CMD)
    }
    #[doc = "Interrupt response: Corresponds to the Interrupt Mode (CMD40)."]
    #[inline(always)]
    pub fn it_resp(self) -> &'a mut crate::W<REG> {
        self.variant(SPCMDSELECT_AW::IT_RESP)
    }
    #[doc = "Boot Operation Request. Start a boot operation mode, the host processor can read boot data from the MMC device directly."]
    #[inline(always)]
    pub fn bor(self) -> &'a mut crate::W<REG> {
        self.variant(SPCMDSELECT_AW::BOR)
    }
    #[doc = "End Boot Operation. This command allows the host processor to terminate the boot operation mode."]
    #[inline(always)]
    pub fn ebo(self) -> &'a mut crate::W<REG> {
        self.variant(SPCMDSELECT_AW::EBO)
    }
}
#[doc = "Open Drain Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPDCMDSELECT_AW {
    #[doc = "0: Push pull command."]
    PUSHPULL = 0,
    #[doc = "1: Open drain command."]
    OPENDRAIN = 1,
}
impl From<OPDCMDSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: OPDCMDSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPDCMD` writer - Open Drain Command"]
pub type OPDCMD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OPDCMDSELECT_AW>;
impl<'a, REG, const O: u8> OPDCMD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Push pull command."]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(OPDCMDSELECT_AW::PUSHPULL)
    }
    #[doc = "Open drain command."]
    #[inline(always)]
    pub fn opendrain(self) -> &'a mut crate::W<REG> {
        self.variant(OPDCMDSELECT_AW::OPENDRAIN)
    }
}
#[doc = "Max Latency for Command to Response\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MAXLATSELECT_AW {
    #[doc = "0: 5-cycle max latency."]
    _5 = 0,
    #[doc = "1: 64-cycle max latency."]
    _64 = 1,
}
impl From<MAXLATSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: MAXLATSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAXLAT` writer - Max Latency for Command to Response"]
pub type MAXLAT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MAXLATSELECT_AW>;
impl<'a, REG, const O: u8> MAXLAT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "5-cycle max latency."]
    #[inline(always)]
    pub fn _5(self) -> &'a mut crate::W<REG> {
        self.variant(MAXLATSELECT_AW::_5)
    }
    #[doc = "64-cycle max latency."]
    #[inline(always)]
    pub fn _64(self) -> &'a mut crate::W<REG> {
        self.variant(MAXLATSELECT_AW::_64)
    }
}
#[doc = "Transfer Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRCMDSELECT_AW {
    #[doc = "0: No data transfer"]
    NO_DATA = 0,
    #[doc = "1: Start data transfer"]
    START_DATA = 1,
    #[doc = "2: Stop data transfer"]
    STOP_DATA = 2,
}
impl From<TRCMDSELECT_AW> for u8 {
    #[inline(always)]
    fn from(variant: TRCMDSELECT_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRCMDSELECT_AW {
    type Ux = u8;
}
#[doc = "Field `TRCMD` writer - Transfer Command"]
pub type TRCMD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, TRCMDSELECT_AW>;
impl<'a, REG, const O: u8> TRCMD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No data transfer"]
    #[inline(always)]
    pub fn no_data(self) -> &'a mut crate::W<REG> {
        self.variant(TRCMDSELECT_AW::NO_DATA)
    }
    #[doc = "Start data transfer"]
    #[inline(always)]
    pub fn start_data(self) -> &'a mut crate::W<REG> {
        self.variant(TRCMDSELECT_AW::START_DATA)
    }
    #[doc = "Stop data transfer"]
    #[inline(always)]
    pub fn stop_data(self) -> &'a mut crate::W<REG> {
        self.variant(TRCMDSELECT_AW::STOP_DATA)
    }
}
#[doc = "Transfer Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRDIRSELECT_AW {
    #[doc = "0: Write."]
    WRITE = 0,
    #[doc = "1: Read."]
    READ = 1,
}
impl From<TRDIRSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: TRDIRSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRDIR` writer - Transfer Direction"]
pub type TRDIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TRDIRSELECT_AW>;
impl<'a, REG, const O: u8> TRDIR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write."]
    #[inline(always)]
    pub fn write(self) -> &'a mut crate::W<REG> {
        self.variant(TRDIRSELECT_AW::WRITE)
    }
    #[doc = "Read."]
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(TRDIRSELECT_AW::READ)
    }
}
#[doc = "Transfer Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRTYPSELECT_AW {
    #[doc = "0: MMC/SD Card Single Block"]
    SINGLE = 0,
    #[doc = "1: MMC/SD Card Multiple Block"]
    MULTIPLE = 1,
    #[doc = "2: MMC Stream"]
    STREAM = 2,
    #[doc = "4: SDIO Byte"]
    BYTE = 4,
    #[doc = "5: SDIO Block"]
    BLOCK = 5,
}
impl From<TRTYPSELECT_AW> for u8 {
    #[inline(always)]
    fn from(variant: TRTYPSELECT_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRTYPSELECT_AW {
    type Ux = u8;
}
#[doc = "Field `TRTYP` writer - Transfer Type"]
pub type TRTYP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, TRTYPSELECT_AW>;
impl<'a, REG, const O: u8> TRTYP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MMC/SD Card Single Block"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(TRTYPSELECT_AW::SINGLE)
    }
    #[doc = "MMC/SD Card Multiple Block"]
    #[inline(always)]
    pub fn multiple(self) -> &'a mut crate::W<REG> {
        self.variant(TRTYPSELECT_AW::MULTIPLE)
    }
    #[doc = "MMC Stream"]
    #[inline(always)]
    pub fn stream(self) -> &'a mut crate::W<REG> {
        self.variant(TRTYPSELECT_AW::STREAM)
    }
    #[doc = "SDIO Byte"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(TRTYPSELECT_AW::BYTE)
    }
    #[doc = "SDIO Block"]
    #[inline(always)]
    pub fn block(self) -> &'a mut crate::W<REG> {
        self.variant(TRTYPSELECT_AW::BLOCK)
    }
}
#[doc = "SDIO Special Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IOSPCMDSELECT_AW {
    #[doc = "0: Not an SDIO Special Command"]
    STD = 0,
    #[doc = "1: SDIO Suspend Command"]
    SUSPEND = 1,
    #[doc = "2: SDIO Resume Command"]
    RESUME = 2,
}
impl From<IOSPCMDSELECT_AW> for u8 {
    #[inline(always)]
    fn from(variant: IOSPCMDSELECT_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IOSPCMDSELECT_AW {
    type Ux = u8;
}
#[doc = "Field `IOSPCMD` writer - SDIO Special Command"]
pub type IOSPCMD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, IOSPCMDSELECT_AW>;
impl<'a, REG, const O: u8> IOSPCMD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Not an SDIO Special Command"]
    #[inline(always)]
    pub fn std(self) -> &'a mut crate::W<REG> {
        self.variant(IOSPCMDSELECT_AW::STD)
    }
    #[doc = "SDIO Suspend Command"]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut crate::W<REG> {
        self.variant(IOSPCMDSELECT_AW::SUSPEND)
    }
    #[doc = "SDIO Resume Command"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut crate::W<REG> {
        self.variant(IOSPCMDSELECT_AW::RESUME)
    }
}
#[doc = "ATA with Command Completion Signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ATACSSELECT_AW {
    #[doc = "0: Normal operation mode."]
    NORMAL = 0,
    #[doc = "1: This bit indicates that a completion signal is expected within a programmed amount of time (HSMCI_CSTOR)."]
    COMPLETION = 1,
}
impl From<ATACSSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: ATACSSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATACS` writer - ATA with Command Completion Signal"]
pub type ATACS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ATACSSELECT_AW>;
impl<'a, REG, const O: u8> ATACS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation mode."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(ATACSSELECT_AW::NORMAL)
    }
    #[doc = "This bit indicates that a completion signal is expected within a programmed amount of time (HSMCI_CSTOR)."]
    #[inline(always)]
    pub fn completion(self) -> &'a mut crate::W<REG> {
        self.variant(ATACSSELECT_AW::COMPLETION)
    }
}
#[doc = "Field `BOOT_ACK` writer - Boot Operation Acknowledge"]
pub type BOOT_ACK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bits 0:5 - Command Number"]
    #[inline(always)]
    #[must_use]
    pub fn cmdnb(&mut self) -> CMDNB_W<CMDR_SPEC, 0> {
        CMDNB_W::new(self)
    }
    #[doc = "Bits 6:7 - Response Type"]
    #[inline(always)]
    #[must_use]
    pub fn rsptyp(&mut self) -> RSPTYP_W<CMDR_SPEC, 6> {
        RSPTYP_W::new(self)
    }
    #[doc = "Bits 8:10 - Special Command"]
    #[inline(always)]
    #[must_use]
    pub fn spcmd(&mut self) -> SPCMD_W<CMDR_SPEC, 8> {
        SPCMD_W::new(self)
    }
    #[doc = "Bit 11 - Open Drain Command"]
    #[inline(always)]
    #[must_use]
    pub fn opdcmd(&mut self) -> OPDCMD_W<CMDR_SPEC, 11> {
        OPDCMD_W::new(self)
    }
    #[doc = "Bit 12 - Max Latency for Command to Response"]
    #[inline(always)]
    #[must_use]
    pub fn maxlat(&mut self) -> MAXLAT_W<CMDR_SPEC, 12> {
        MAXLAT_W::new(self)
    }
    #[doc = "Bits 16:17 - Transfer Command"]
    #[inline(always)]
    #[must_use]
    pub fn trcmd(&mut self) -> TRCMD_W<CMDR_SPEC, 16> {
        TRCMD_W::new(self)
    }
    #[doc = "Bit 18 - Transfer Direction"]
    #[inline(always)]
    #[must_use]
    pub fn trdir(&mut self) -> TRDIR_W<CMDR_SPEC, 18> {
        TRDIR_W::new(self)
    }
    #[doc = "Bits 19:21 - Transfer Type"]
    #[inline(always)]
    #[must_use]
    pub fn trtyp(&mut self) -> TRTYP_W<CMDR_SPEC, 19> {
        TRTYP_W::new(self)
    }
    #[doc = "Bits 24:25 - SDIO Special Command"]
    #[inline(always)]
    #[must_use]
    pub fn iospcmd(&mut self) -> IOSPCMD_W<CMDR_SPEC, 24> {
        IOSPCMD_W::new(self)
    }
    #[doc = "Bit 26 - ATA with Command Completion Signal"]
    #[inline(always)]
    #[must_use]
    pub fn atacs(&mut self) -> ATACS_W<CMDR_SPEC, 26> {
        ATACS_W::new(self)
    }
    #[doc = "Bit 27 - Boot Operation Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn boot_ack(&mut self) -> BOOT_ACK_W<CMDR_SPEC, 27> {
        BOOT_ACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMDR_SPEC;
impl crate::RegisterSpec for CMDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmdr::W`](W) writer structure"]
impl crate::Writable for CMDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMDR to value 0"]
impl crate::Resettable for CMDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
