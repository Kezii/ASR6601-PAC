#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `RW_MODE` reader - Rw mode"]
pub type RwModeR = crate::BitReader;
#[doc = "Field `RW_MODE` writer - Rw mode"]
pub type RwModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK_STATUS` reader - Ack status"]
pub type AckStatusR = crate::BitReader;
#[doc = "Field `ACK_STATUS` writer - Ack status"]
pub type AckStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNIT_BUSY` reader - Unit busy"]
pub type UnitBusyR = crate::BitReader;
#[doc = "Field `UNIT_BUSY` writer - Unit busy"]
pub type UnitBusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS_BUSY` reader - Bus busy"]
pub type BusBusyR = crate::BitReader;
#[doc = "Field `BUS_BUSY` writer - Bus busy"]
pub type BusBusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARB_LOSS_DET` reader - Arb loss det"]
pub type ArbLossDetR = crate::BitReader;
#[doc = "Field `ARB_LOSS_DET` writer - Arb loss det"]
pub type ArbLossDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDBR_EMPTY` reader - Idbr empty"]
pub type IdbrEmptyR = crate::BitReader;
#[doc = "Field `IDBR_EMPTY` writer - Idbr empty"]
pub type IdbrEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBR_FULL` reader - Dbr full"]
pub type DbrFullR = crate::BitReader;
#[doc = "Field `DBR_FULL` writer - Dbr full"]
pub type DbrFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GENERAL_CALL` reader - General call"]
pub type GeneralCallR = crate::BitReader;
#[doc = "Field `GENERAL_CALL` writer - General call"]
pub type GeneralCallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS_ERROR` reader - Bus error"]
pub type BusErrorR = crate::BitReader;
#[doc = "Field `BUS_ERROR` writer - Bus error"]
pub type BusErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAVE_ADDR_DET` reader - Slave addr det"]
pub type SlaveAddrDetR = crate::BitReader;
#[doc = "Field `SLAVE_ADDR_DET` writer - Slave addr det"]
pub type SlaveAddrDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAVE_STOP_DET` reader - Slave stop det"]
pub type SlaveStopDetR = crate::BitReader;
#[doc = "Field `SLAVE_STOP_DET` writer - Slave stop det"]
pub type SlaveStopDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASTER_STOP_DET` reader - Master stop det"]
pub type MasterStopDetR = crate::BitReader;
#[doc = "Field `MASTER_STOP_DET` writer - Master stop det"]
pub type MasterStopDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_DONE` reader - Trans done"]
pub type TransDoneR = crate::BitReader;
#[doc = "Field `TRANS_DONE` writer - Trans done"]
pub type TransDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFIFO_EMPTY` reader - Tfifo empty"]
pub type TfifoEmptyR = crate::BitReader;
#[doc = "Field `TFIFO_EMPTY` writer - Tfifo empty"]
pub type TfifoEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFIFO_HALFFULL` reader - Rfifo halffull"]
pub type RfifoHalffullR = crate::BitReader;
#[doc = "Field `RFIFO_HALFFULL` writer - Rfifo halffull"]
pub type RfifoHalffullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFIFO_FULL` reader - Rfifo full"]
pub type RfifoFullR = crate::BitReader;
#[doc = "Field `RFIFO_FULL` writer - Rfifo full"]
pub type RfifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFIFO_OVERRUN` reader - Rfifo overrun"]
pub type RfifoOverrunR = crate::BitReader;
#[doc = "Field `RFIFO_OVERRUN` writer - Rfifo overrun"]
pub type RfifoOverrunW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 13 - Rw mode"]
    #[inline(always)]
    pub fn rw_mode(&self) -> RwModeR {
        RwModeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Ack status"]
    #[inline(always)]
    pub fn ack_status(&self) -> AckStatusR {
        AckStatusR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Unit busy"]
    #[inline(always)]
    pub fn unit_busy(&self) -> UnitBusyR {
        UnitBusyR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Bus busy"]
    #[inline(always)]
    pub fn bus_busy(&self) -> BusBusyR {
        BusBusyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Arb loss det"]
    #[inline(always)]
    pub fn arb_loss_det(&self) -> ArbLossDetR {
        ArbLossDetR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Idbr empty"]
    #[inline(always)]
    pub fn idbr_empty(&self) -> IdbrEmptyR {
        IdbrEmptyR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Dbr full"]
    #[inline(always)]
    pub fn dbr_full(&self) -> DbrFullR {
        DbrFullR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - General call"]
    #[inline(always)]
    pub fn general_call(&self) -> GeneralCallR {
        GeneralCallR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Bus error"]
    #[inline(always)]
    pub fn bus_error(&self) -> BusErrorR {
        BusErrorR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Slave addr det"]
    #[inline(always)]
    pub fn slave_addr_det(&self) -> SlaveAddrDetR {
        SlaveAddrDetR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Slave stop det"]
    #[inline(always)]
    pub fn slave_stop_det(&self) -> SlaveStopDetR {
        SlaveStopDetR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Master stop det"]
    #[inline(always)]
    pub fn master_stop_det(&self) -> MasterStopDetR {
        MasterStopDetR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Trans done"]
    #[inline(always)]
    pub fn trans_done(&self) -> TransDoneR {
        TransDoneR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Tfifo empty"]
    #[inline(always)]
    pub fn tfifo_empty(&self) -> TfifoEmptyR {
        TfifoEmptyR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Rfifo halffull"]
    #[inline(always)]
    pub fn rfifo_halffull(&self) -> RfifoHalffullR {
        RfifoHalffullR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Rfifo full"]
    #[inline(always)]
    pub fn rfifo_full(&self) -> RfifoFullR {
        RfifoFullR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Rfifo overrun"]
    #[inline(always)]
    pub fn rfifo_overrun(&self) -> RfifoOverrunR {
        RfifoOverrunR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - Rw mode"]
    #[inline(always)]
    pub fn rw_mode(&mut self) -> RwModeW<'_, SrSpec> {
        RwModeW::new(self, 13)
    }
    #[doc = "Bit 14 - Ack status"]
    #[inline(always)]
    pub fn ack_status(&mut self) -> AckStatusW<'_, SrSpec> {
        AckStatusW::new(self, 14)
    }
    #[doc = "Bit 15 - Unit busy"]
    #[inline(always)]
    pub fn unit_busy(&mut self) -> UnitBusyW<'_, SrSpec> {
        UnitBusyW::new(self, 15)
    }
    #[doc = "Bit 16 - Bus busy"]
    #[inline(always)]
    pub fn bus_busy(&mut self) -> BusBusyW<'_, SrSpec> {
        BusBusyW::new(self, 16)
    }
    #[doc = "Bit 18 - Arb loss det"]
    #[inline(always)]
    pub fn arb_loss_det(&mut self) -> ArbLossDetW<'_, SrSpec> {
        ArbLossDetW::new(self, 18)
    }
    #[doc = "Bit 19 - Idbr empty"]
    #[inline(always)]
    pub fn idbr_empty(&mut self) -> IdbrEmptyW<'_, SrSpec> {
        IdbrEmptyW::new(self, 19)
    }
    #[doc = "Bit 20 - Dbr full"]
    #[inline(always)]
    pub fn dbr_full(&mut self) -> DbrFullW<'_, SrSpec> {
        DbrFullW::new(self, 20)
    }
    #[doc = "Bit 21 - General call"]
    #[inline(always)]
    pub fn general_call(&mut self) -> GeneralCallW<'_, SrSpec> {
        GeneralCallW::new(self, 21)
    }
    #[doc = "Bit 22 - Bus error"]
    #[inline(always)]
    pub fn bus_error(&mut self) -> BusErrorW<'_, SrSpec> {
        BusErrorW::new(self, 22)
    }
    #[doc = "Bit 23 - Slave addr det"]
    #[inline(always)]
    pub fn slave_addr_det(&mut self) -> SlaveAddrDetW<'_, SrSpec> {
        SlaveAddrDetW::new(self, 23)
    }
    #[doc = "Bit 24 - Slave stop det"]
    #[inline(always)]
    pub fn slave_stop_det(&mut self) -> SlaveStopDetW<'_, SrSpec> {
        SlaveStopDetW::new(self, 24)
    }
    #[doc = "Bit 25 - Master stop det"]
    #[inline(always)]
    pub fn master_stop_det(&mut self) -> MasterStopDetW<'_, SrSpec> {
        MasterStopDetW::new(self, 25)
    }
    #[doc = "Bit 27 - Trans done"]
    #[inline(always)]
    pub fn trans_done(&mut self) -> TransDoneW<'_, SrSpec> {
        TransDoneW::new(self, 27)
    }
    #[doc = "Bit 28 - Tfifo empty"]
    #[inline(always)]
    pub fn tfifo_empty(&mut self) -> TfifoEmptyW<'_, SrSpec> {
        TfifoEmptyW::new(self, 28)
    }
    #[doc = "Bit 29 - Rfifo halffull"]
    #[inline(always)]
    pub fn rfifo_halffull(&mut self) -> RfifoHalffullW<'_, SrSpec> {
        RfifoHalffullW::new(self, 29)
    }
    #[doc = "Bit 30 - Rfifo full"]
    #[inline(always)]
    pub fn rfifo_full(&mut self) -> RfifoFullW<'_, SrSpec> {
        RfifoFullW::new(self, 30)
    }
    #[doc = "Bit 31 - Rfifo overrun"]
    #[inline(always)]
    pub fn rfifo_overrun(&mut self) -> RfifoOverrunW<'_, SrSpec> {
        RfifoOverrunW::new(self, 31)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
