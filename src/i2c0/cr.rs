#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `START` reader - Start"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - Start"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - Stop"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - Stop"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKNAK` reader - Acknak"]
pub type AcknakR = crate::BitReader;
#[doc = "Field `ACKNAK` writer - Acknak"]
pub type AcknakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_BYTE` reader - Trans byte"]
pub type TransByteR = crate::BitReader;
#[doc = "Field `TRANS_BYTE` writer - Trans byte"]
pub type TransByteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_BEGIN` reader - Trans begin"]
pub type TransBeginR = crate::BitReader;
#[doc = "Field `TRANS_BEGIN` writer - Trans begin"]
pub type TransBeginW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_EN` reader - Fifo en"]
pub type FifoEnR = crate::BitReader;
#[doc = "Field `FIFO_EN` writer - Fifo en"]
pub type FifoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_EN` reader - Dma en"]
pub type DmaEnR = crate::BitReader;
#[doc = "Field `DMA_EN` writer - Dma en"]
pub type DmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Bus mode"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BusMode {
    #[doc = "1: Fast"]
    Fast = 1,
    #[doc = "2: High"]
    High = 2,
    #[doc = "0: Standard"]
    Standard = 0,
}
impl From<BusMode> for u8 {
    #[inline(always)]
    fn from(variant: BusMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BusMode {
    type Ux = u8;
}
impl crate::IsEnum for BusMode {}
#[doc = "Field `BUS_MODE` reader - Bus mode"]
pub type BusModeR = crate::FieldReader<BusMode>;
impl BusModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BusMode> {
        match self.bits {
            1 => Some(BusMode::Fast),
            2 => Some(BusMode::High),
            0 => Some(BusMode::Standard),
            _ => None,
        }
    }
    #[doc = "Fast"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == BusMode::Fast
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == BusMode::High
    }
    #[doc = "Standard"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == BusMode::Standard
    }
}
#[doc = "Field `BUS_MODE` writer - Bus mode"]
pub type BusModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, BusMode>;
impl<'a, REG> BusModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Fast"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut crate::W<REG> {
        self.variant(BusMode::Fast)
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(BusMode::High)
    }
    #[doc = "Standard"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(BusMode::Standard)
    }
}
#[doc = "Field `UNIT_RESET` reader - Unit reset"]
pub type UnitResetR = crate::BitReader;
#[doc = "Field `UNIT_RESET` writer - Unit reset"]
pub type UnitResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS_RESET_REQUEST` reader - Bus reset request"]
pub type BusResetRequestR = crate::BitReader;
#[doc = "Field `BUS_RESET_REQUEST` writer - Bus reset request"]
pub type BusResetRequestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASTER_ABORT` reader - Master abort"]
pub type MasterAbortR = crate::BitReader;
#[doc = "Field `MASTER_ABORT` writer - Master abort"]
pub type MasterAbortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCL_EN` reader - Scl en"]
pub type SclEnR = crate::BitReader;
#[doc = "Field `SCL_EN` writer - Scl en"]
pub type SclEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWSI_UNIT_EN` reader - Twsi unit en"]
pub type TwsiUnitEnR = crate::BitReader;
#[doc = "Field `TWSI_UNIT_EN` writer - Twsi unit en"]
pub type TwsiUnitEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARB_LOSS_DET_INTR_EN` reader - Arb loss det intr en"]
pub type ArbLossDetIntrEnR = crate::BitReader;
#[doc = "Field `ARB_LOSS_DET_INTR_EN` writer - Arb loss det intr en"]
pub type ArbLossDetIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDBR_EMPTY_INTR_EN` reader - Idbr empty intr en"]
pub type IdbrEmptyIntrEnR = crate::BitReader;
#[doc = "Field `IDBR_EMPTY_INTR_EN` writer - Idbr empty intr en"]
pub type IdbrEmptyIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBR_FULL_INTR_EN` reader - Dbr full intr en"]
pub type DbrFullIntrEnR = crate::BitReader;
#[doc = "Field `DBR_FULL_INTR_EN` writer - Dbr full intr en"]
pub type DbrFullIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GENERAL_CALL_DIS` reader - General call dis"]
pub type GeneralCallDisR = crate::BitReader;
#[doc = "Field `GENERAL_CALL_DIS` writer - General call dis"]
pub type GeneralCallDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS_ERROR_INTR_EN` reader - Bus error intr en"]
pub type BusErrorIntrEnR = crate::BitReader;
#[doc = "Field `BUS_ERROR_INTR_EN` writer - Bus error intr en"]
pub type BusErrorIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAVE_ADDR_DET_INTR_EN` reader - Slave addr det intr en"]
pub type SlaveAddrDetIntrEnR = crate::BitReader;
#[doc = "Field `SLAVE_ADDR_DET_INTR_EN` writer - Slave addr det intr en"]
pub type SlaveAddrDetIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAVE_STOP_DET_INTR_EN` reader - Slave stop det intr en"]
pub type SlaveStopDetIntrEnR = crate::BitReader;
#[doc = "Field `SLAVE_STOP_DET_INTR_EN` writer - Slave stop det intr en"]
pub type SlaveStopDetIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASTER_STOP_DET_INTR_EN` reader - Master stop det intr en"]
pub type MasterStopDetIntrEnR = crate::BitReader;
#[doc = "Field `MASTER_STOP_DET_INTR_EN` writer - Master stop det intr en"]
pub type MasterStopDetIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASTER_STOP_DET_EN` reader - Master stop det en"]
pub type MasterStopDetEnR = crate::BitReader;
#[doc = "Field `MASTER_STOP_DET_EN` writer - Master stop det en"]
pub type MasterStopDetEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_DONE_INTR_EN` reader - Trans done intr en"]
pub type TransDoneIntrEnR = crate::BitReader;
#[doc = "Field `TRANS_DONE_INTR_EN` writer - Trans done intr en"]
pub type TransDoneIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFIFO_EMPTY_INTR_EN` reader - Tfifo empty intr en"]
pub type TfifoEmptyIntrEnR = crate::BitReader;
#[doc = "Field `TFIFO_EMPTY_INTR_EN` writer - Tfifo empty intr en"]
pub type TfifoEmptyIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFIFO_HALFFULL_INTR_EN` reader - Rfifo halffull intr en"]
pub type RfifoHalffullIntrEnR = crate::BitReader;
#[doc = "Field `RFIFO_HALFFULL_INTR_EN` writer - Rfifo halffull intr en"]
pub type RfifoHalffullIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFIFO_FULL_INTR_EN` reader - Rfifo full intr en"]
pub type RfifoFullIntrEnR = crate::BitReader;
#[doc = "Field `RFIFO_FULL_INTR_EN` writer - Rfifo full intr en"]
pub type RfifoFullIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFIFO_OVERRUN_INTR_EN` reader - Rfifo overrun intr en"]
pub type RfifoOverrunIntrEnR = crate::BitReader;
#[doc = "Field `RFIFO_OVERRUN_INTR_EN` writer - Rfifo overrun intr en"]
pub type RfifoOverrunIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stop"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Acknak"]
    #[inline(always)]
    pub fn acknak(&self) -> AcknakR {
        AcknakR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Trans byte"]
    #[inline(always)]
    pub fn trans_byte(&self) -> TransByteR {
        TransByteR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Trans begin"]
    #[inline(always)]
    pub fn trans_begin(&self) -> TransBeginR {
        TransBeginR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Fifo en"]
    #[inline(always)]
    pub fn fifo_en(&self) -> FifoEnR {
        FifoEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Dma en"]
    #[inline(always)]
    pub fn dma_en(&self) -> DmaEnR {
        DmaEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Bus mode"]
    #[inline(always)]
    pub fn bus_mode(&self) -> BusModeR {
        BusModeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Unit reset"]
    #[inline(always)]
    pub fn unit_reset(&self) -> UnitResetR {
        UnitResetR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bus reset request"]
    #[inline(always)]
    pub fn bus_reset_request(&self) -> BusResetRequestR {
        BusResetRequestR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Master abort"]
    #[inline(always)]
    pub fn master_abort(&self) -> MasterAbortR {
        MasterAbortR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Scl en"]
    #[inline(always)]
    pub fn scl_en(&self) -> SclEnR {
        SclEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Twsi unit en"]
    #[inline(always)]
    pub fn twsi_unit_en(&self) -> TwsiUnitEnR {
        TwsiUnitEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - Arb loss det intr en"]
    #[inline(always)]
    pub fn arb_loss_det_intr_en(&self) -> ArbLossDetIntrEnR {
        ArbLossDetIntrEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Idbr empty intr en"]
    #[inline(always)]
    pub fn idbr_empty_intr_en(&self) -> IdbrEmptyIntrEnR {
        IdbrEmptyIntrEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Dbr full intr en"]
    #[inline(always)]
    pub fn dbr_full_intr_en(&self) -> DbrFullIntrEnR {
        DbrFullIntrEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - General call dis"]
    #[inline(always)]
    pub fn general_call_dis(&self) -> GeneralCallDisR {
        GeneralCallDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Bus error intr en"]
    #[inline(always)]
    pub fn bus_error_intr_en(&self) -> BusErrorIntrEnR {
        BusErrorIntrEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Slave addr det intr en"]
    #[inline(always)]
    pub fn slave_addr_det_intr_en(&self) -> SlaveAddrDetIntrEnR {
        SlaveAddrDetIntrEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Slave stop det intr en"]
    #[inline(always)]
    pub fn slave_stop_det_intr_en(&self) -> SlaveStopDetIntrEnR {
        SlaveStopDetIntrEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Master stop det intr en"]
    #[inline(always)]
    pub fn master_stop_det_intr_en(&self) -> MasterStopDetIntrEnR {
        MasterStopDetIntrEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Master stop det en"]
    #[inline(always)]
    pub fn master_stop_det_en(&self) -> MasterStopDetEnR {
        MasterStopDetEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Trans done intr en"]
    #[inline(always)]
    pub fn trans_done_intr_en(&self) -> TransDoneIntrEnR {
        TransDoneIntrEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Tfifo empty intr en"]
    #[inline(always)]
    pub fn tfifo_empty_intr_en(&self) -> TfifoEmptyIntrEnR {
        TfifoEmptyIntrEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Rfifo halffull intr en"]
    #[inline(always)]
    pub fn rfifo_halffull_intr_en(&self) -> RfifoHalffullIntrEnR {
        RfifoHalffullIntrEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Rfifo full intr en"]
    #[inline(always)]
    pub fn rfifo_full_intr_en(&self) -> RfifoFullIntrEnR {
        RfifoFullIntrEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Rfifo overrun intr en"]
    #[inline(always)]
    pub fn rfifo_overrun_intr_en(&self) -> RfifoOverrunIntrEnR {
        RfifoOverrunIntrEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, CrSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Stop"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, CrSpec> {
        StopW::new(self, 1)
    }
    #[doc = "Bit 2 - Acknak"]
    #[inline(always)]
    pub fn acknak(&mut self) -> AcknakW<'_, CrSpec> {
        AcknakW::new(self, 2)
    }
    #[doc = "Bit 3 - Trans byte"]
    #[inline(always)]
    pub fn trans_byte(&mut self) -> TransByteW<'_, CrSpec> {
        TransByteW::new(self, 3)
    }
    #[doc = "Bit 4 - Trans begin"]
    #[inline(always)]
    pub fn trans_begin(&mut self) -> TransBeginW<'_, CrSpec> {
        TransBeginW::new(self, 4)
    }
    #[doc = "Bit 5 - Fifo en"]
    #[inline(always)]
    pub fn fifo_en(&mut self) -> FifoEnW<'_, CrSpec> {
        FifoEnW::new(self, 5)
    }
    #[doc = "Bit 7 - Dma en"]
    #[inline(always)]
    pub fn dma_en(&mut self) -> DmaEnW<'_, CrSpec> {
        DmaEnW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Bus mode"]
    #[inline(always)]
    pub fn bus_mode(&mut self) -> BusModeW<'_, CrSpec> {
        BusModeW::new(self, 8)
    }
    #[doc = "Bit 10 - Unit reset"]
    #[inline(always)]
    pub fn unit_reset(&mut self) -> UnitResetW<'_, CrSpec> {
        UnitResetW::new(self, 10)
    }
    #[doc = "Bit 11 - Bus reset request"]
    #[inline(always)]
    pub fn bus_reset_request(&mut self) -> BusResetRequestW<'_, CrSpec> {
        BusResetRequestW::new(self, 11)
    }
    #[doc = "Bit 12 - Master abort"]
    #[inline(always)]
    pub fn master_abort(&mut self) -> MasterAbortW<'_, CrSpec> {
        MasterAbortW::new(self, 12)
    }
    #[doc = "Bit 13 - Scl en"]
    #[inline(always)]
    pub fn scl_en(&mut self) -> SclEnW<'_, CrSpec> {
        SclEnW::new(self, 13)
    }
    #[doc = "Bit 14 - Twsi unit en"]
    #[inline(always)]
    pub fn twsi_unit_en(&mut self) -> TwsiUnitEnW<'_, CrSpec> {
        TwsiUnitEnW::new(self, 14)
    }
    #[doc = "Bit 18 - Arb loss det intr en"]
    #[inline(always)]
    pub fn arb_loss_det_intr_en(&mut self) -> ArbLossDetIntrEnW<'_, CrSpec> {
        ArbLossDetIntrEnW::new(self, 18)
    }
    #[doc = "Bit 19 - Idbr empty intr en"]
    #[inline(always)]
    pub fn idbr_empty_intr_en(&mut self) -> IdbrEmptyIntrEnW<'_, CrSpec> {
        IdbrEmptyIntrEnW::new(self, 19)
    }
    #[doc = "Bit 20 - Dbr full intr en"]
    #[inline(always)]
    pub fn dbr_full_intr_en(&mut self) -> DbrFullIntrEnW<'_, CrSpec> {
        DbrFullIntrEnW::new(self, 20)
    }
    #[doc = "Bit 21 - General call dis"]
    #[inline(always)]
    pub fn general_call_dis(&mut self) -> GeneralCallDisW<'_, CrSpec> {
        GeneralCallDisW::new(self, 21)
    }
    #[doc = "Bit 22 - Bus error intr en"]
    #[inline(always)]
    pub fn bus_error_intr_en(&mut self) -> BusErrorIntrEnW<'_, CrSpec> {
        BusErrorIntrEnW::new(self, 22)
    }
    #[doc = "Bit 23 - Slave addr det intr en"]
    #[inline(always)]
    pub fn slave_addr_det_intr_en(&mut self) -> SlaveAddrDetIntrEnW<'_, CrSpec> {
        SlaveAddrDetIntrEnW::new(self, 23)
    }
    #[doc = "Bit 24 - Slave stop det intr en"]
    #[inline(always)]
    pub fn slave_stop_det_intr_en(&mut self) -> SlaveStopDetIntrEnW<'_, CrSpec> {
        SlaveStopDetIntrEnW::new(self, 24)
    }
    #[doc = "Bit 25 - Master stop det intr en"]
    #[inline(always)]
    pub fn master_stop_det_intr_en(&mut self) -> MasterStopDetIntrEnW<'_, CrSpec> {
        MasterStopDetIntrEnW::new(self, 25)
    }
    #[doc = "Bit 26 - Master stop det en"]
    #[inline(always)]
    pub fn master_stop_det_en(&mut self) -> MasterStopDetEnW<'_, CrSpec> {
        MasterStopDetEnW::new(self, 26)
    }
    #[doc = "Bit 27 - Trans done intr en"]
    #[inline(always)]
    pub fn trans_done_intr_en(&mut self) -> TransDoneIntrEnW<'_, CrSpec> {
        TransDoneIntrEnW::new(self, 27)
    }
    #[doc = "Bit 28 - Tfifo empty intr en"]
    #[inline(always)]
    pub fn tfifo_empty_intr_en(&mut self) -> TfifoEmptyIntrEnW<'_, CrSpec> {
        TfifoEmptyIntrEnW::new(self, 28)
    }
    #[doc = "Bit 29 - Rfifo halffull intr en"]
    #[inline(always)]
    pub fn rfifo_halffull_intr_en(&mut self) -> RfifoHalffullIntrEnW<'_, CrSpec> {
        RfifoHalffullIntrEnW::new(self, 29)
    }
    #[doc = "Bit 30 - Rfifo full intr en"]
    #[inline(always)]
    pub fn rfifo_full_intr_en(&mut self) -> RfifoFullIntrEnW<'_, CrSpec> {
        RfifoFullIntrEnW::new(self, 30)
    }
    #[doc = "Bit 31 - Rfifo overrun intr en"]
    #[inline(always)]
    pub fn rfifo_overrun_intr_en(&mut self) -> RfifoOverrunIntrEnW<'_, CrSpec> {
        RfifoOverrunIntrEnW::new(self, 31)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
