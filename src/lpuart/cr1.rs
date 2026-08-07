#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `START_VALID_INT` reader - Start valid int"]
pub type StartValidIntR = crate::BitReader;
#[doc = "Field `START_VALID_INT` writer - Start valid int"]
pub type StartValidIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DONE_INT` reader - Rx done int"]
pub type RxDoneIntR = crate::BitReader;
#[doc = "Field `RX_DONE_INT` writer - Rx done int"]
pub type RxDoneIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START_INVALID_INT` reader - Start invalid int"]
pub type StartInvalidIntR = crate::BitReader;
#[doc = "Field `START_INVALID_INT` writer - Start invalid int"]
pub type StartInvalidIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITY_ERR_INT` reader - Parity err int"]
pub type ParityErrIntR = crate::BitReader;
#[doc = "Field `PARITY_ERR_INT` writer - Parity err int"]
pub type ParityErrIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP_ERR_INT` reader - Stop err int"]
pub type StopErrIntR = crate::BitReader;
#[doc = "Field `STOP_ERR_INT` writer - Stop err int"]
pub type StopErrIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_OVERFLOW_INT` reader - Rx overflow int"]
pub type RxOverflowIntR = crate::BitReader;
#[doc = "Field `RX_OVERFLOW_INT` writer - Rx overflow int"]
pub type RxOverflowIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_NOT_EMPTY_INT` reader - Rx not empty int"]
pub type RxNotEmptyIntR = crate::BitReader;
#[doc = "Field `RX_NOT_EMPTY_INT` writer - Rx not empty int"]
pub type RxNotEmptyIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_EMPTY_INT` reader - Tx empty int"]
pub type TxEmptyIntR = crate::BitReader;
#[doc = "Field `TX_EMPTY_INT` writer - Tx empty int"]
pub type TxEmptyIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DONE_INT` reader - Tx done int"]
pub type TxDoneIntR = crate::BitReader;
#[doc = "Field `TX_DONE_INT` writer - Tx done int"]
pub type TxDoneIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_ENABLE` reader - Tx enable"]
pub type TxEnableR = crate::BitReader;
#[doc = "Field `TX_ENABLE` writer - Tx enable"]
pub type TxEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DMA` reader - Tx dma"]
pub type TxDmaR = crate::BitReader;
#[doc = "Field `TX_DMA` writer - Tx dma"]
pub type TxDmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DMA` reader - Rx dma"]
pub type RxDmaR = crate::BitReader;
#[doc = "Field `RX_DMA` writer - Rx dma"]
pub type RxDmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTS_ENABLE` reader - Cts enable"]
pub type CtsEnableR = crate::BitReader;
#[doc = "Field `CTS_ENABLE` writer - Cts enable"]
pub type CtsEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start valid int"]
    #[inline(always)]
    pub fn start_valid_int(&self) -> StartValidIntR {
        StartValidIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx done int"]
    #[inline(always)]
    pub fn rx_done_int(&self) -> RxDoneIntR {
        RxDoneIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Start invalid int"]
    #[inline(always)]
    pub fn start_invalid_int(&self) -> StartInvalidIntR {
        StartInvalidIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Parity err int"]
    #[inline(always)]
    pub fn parity_err_int(&self) -> ParityErrIntR {
        ParityErrIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stop err int"]
    #[inline(always)]
    pub fn stop_err_int(&self) -> StopErrIntR {
        StopErrIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx overflow int"]
    #[inline(always)]
    pub fn rx_overflow_int(&self) -> RxOverflowIntR {
        RxOverflowIntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx not empty int"]
    #[inline(always)]
    pub fn rx_not_empty_int(&self) -> RxNotEmptyIntR {
        RxNotEmptyIntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tx empty int"]
    #[inline(always)]
    pub fn tx_empty_int(&self) -> TxEmptyIntR {
        TxEmptyIntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Tx done int"]
    #[inline(always)]
    pub fn tx_done_int(&self) -> TxDoneIntR {
        TxDoneIntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Tx enable"]
    #[inline(always)]
    pub fn tx_enable(&self) -> TxEnableR {
        TxEnableR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Tx dma"]
    #[inline(always)]
    pub fn tx_dma(&self) -> TxDmaR {
        TxDmaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rx dma"]
    #[inline(always)]
    pub fn rx_dma(&self) -> RxDmaR {
        RxDmaR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Cts enable"]
    #[inline(always)]
    pub fn cts_enable(&self) -> CtsEnableR {
        CtsEnableR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start valid int"]
    #[inline(always)]
    pub fn start_valid_int(&mut self) -> StartValidIntW<'_, Cr1Spec> {
        StartValidIntW::new(self, 0)
    }
    #[doc = "Bit 1 - Rx done int"]
    #[inline(always)]
    pub fn rx_done_int(&mut self) -> RxDoneIntW<'_, Cr1Spec> {
        RxDoneIntW::new(self, 1)
    }
    #[doc = "Bit 2 - Start invalid int"]
    #[inline(always)]
    pub fn start_invalid_int(&mut self) -> StartInvalidIntW<'_, Cr1Spec> {
        StartInvalidIntW::new(self, 2)
    }
    #[doc = "Bit 3 - Parity err int"]
    #[inline(always)]
    pub fn parity_err_int(&mut self) -> ParityErrIntW<'_, Cr1Spec> {
        ParityErrIntW::new(self, 3)
    }
    #[doc = "Bit 4 - Stop err int"]
    #[inline(always)]
    pub fn stop_err_int(&mut self) -> StopErrIntW<'_, Cr1Spec> {
        StopErrIntW::new(self, 4)
    }
    #[doc = "Bit 5 - Rx overflow int"]
    #[inline(always)]
    pub fn rx_overflow_int(&mut self) -> RxOverflowIntW<'_, Cr1Spec> {
        RxOverflowIntW::new(self, 5)
    }
    #[doc = "Bit 6 - Rx not empty int"]
    #[inline(always)]
    pub fn rx_not_empty_int(&mut self) -> RxNotEmptyIntW<'_, Cr1Spec> {
        RxNotEmptyIntW::new(self, 6)
    }
    #[doc = "Bit 7 - Tx empty int"]
    #[inline(always)]
    pub fn tx_empty_int(&mut self) -> TxEmptyIntW<'_, Cr1Spec> {
        TxEmptyIntW::new(self, 7)
    }
    #[doc = "Bit 8 - Tx done int"]
    #[inline(always)]
    pub fn tx_done_int(&mut self) -> TxDoneIntW<'_, Cr1Spec> {
        TxDoneIntW::new(self, 8)
    }
    #[doc = "Bit 9 - Tx enable"]
    #[inline(always)]
    pub fn tx_enable(&mut self) -> TxEnableW<'_, Cr1Spec> {
        TxEnableW::new(self, 9)
    }
    #[doc = "Bit 10 - Tx dma"]
    #[inline(always)]
    pub fn tx_dma(&mut self) -> TxDmaW<'_, Cr1Spec> {
        TxDmaW::new(self, 10)
    }
    #[doc = "Bit 11 - Rx dma"]
    #[inline(always)]
    pub fn rx_dma(&mut self) -> RxDmaW<'_, Cr1Spec> {
        RxDmaW::new(self, 11)
    }
    #[doc = "Bit 12 - Cts enable"]
    #[inline(always)]
    pub fn cts_enable(&mut self) -> CtsEnableW<'_, Cr1Spec> {
        CtsEnableW::new(self, 12)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
