#[doc = "Register `SR1` reader"]
pub type R = crate::R<Sr1Spec>;
#[doc = "Register `SR1` writer"]
pub type W = crate::W<Sr1Spec>;
#[doc = "Field `WRITE_SR0_STATE` reader - Write sr0 state"]
pub type WriteSr0StateR = crate::BitReader;
#[doc = "Field `WRITE_SR0_STATE` writer - Write sr0 state"]
pub type WriteSr0StateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_CR0_STATE` reader - Write cr0 state"]
pub type WriteCr0StateR = crate::BitReader;
#[doc = "Field `WRITE_CR0_STATE` writer - Write cr0 state"]
pub type WriteCr0StateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_NOT_EMPTY_STATE` reader - Rx not empty state"]
pub type RxNotEmptyStateR = crate::BitReader;
#[doc = "Field `RX_NOT_EMPTY_STATE` writer - Rx not empty state"]
pub type RxNotEmptyStateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_EMPTY_STATE` reader - Tx empty state"]
pub type TxEmptyStateR = crate::BitReader;
#[doc = "Field `TX_EMPTY_STATE` writer - Tx empty state"]
pub type TxEmptyStateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DONE_STATE` reader - Tx done state"]
pub type TxDoneStateR = crate::BitReader;
#[doc = "Field `TX_DONE_STATE` writer - Tx done state"]
pub type TxDoneStateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Write sr0 state"]
    #[inline(always)]
    pub fn write_sr0_state(&self) -> WriteSr0StateR {
        WriteSr0StateR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write cr0 state"]
    #[inline(always)]
    pub fn write_cr0_state(&self) -> WriteCr0StateR {
        WriteCr0StateR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx not empty state"]
    #[inline(always)]
    pub fn rx_not_empty_state(&self) -> RxNotEmptyStateR {
        RxNotEmptyStateR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tx empty state"]
    #[inline(always)]
    pub fn tx_empty_state(&self) -> TxEmptyStateR {
        TxEmptyStateR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Tx done state"]
    #[inline(always)]
    pub fn tx_done_state(&self) -> TxDoneStateR {
        TxDoneStateR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Write sr0 state"]
    #[inline(always)]
    pub fn write_sr0_state(&mut self) -> WriteSr0StateW<'_, Sr1Spec> {
        WriteSr0StateW::new(self, 1)
    }
    #[doc = "Bit 2 - Write cr0 state"]
    #[inline(always)]
    pub fn write_cr0_state(&mut self) -> WriteCr0StateW<'_, Sr1Spec> {
        WriteCr0StateW::new(self, 2)
    }
    #[doc = "Bit 3 - Rx not empty state"]
    #[inline(always)]
    pub fn rx_not_empty_state(&mut self) -> RxNotEmptyStateW<'_, Sr1Spec> {
        RxNotEmptyStateW::new(self, 3)
    }
    #[doc = "Bit 4 - Tx empty state"]
    #[inline(always)]
    pub fn tx_empty_state(&mut self) -> TxEmptyStateW<'_, Sr1Spec> {
        TxEmptyStateW::new(self, 4)
    }
    #[doc = "Bit 5 - Tx done state"]
    #[inline(always)]
    pub fn tx_done_state(&mut self) -> TxDoneStateW<'_, Sr1Spec> {
        TxDoneStateW::new(self, 5)
    }
}
#[doc = "status register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sr1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sr1Spec;
impl crate::RegisterSpec for Sr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr1::R`](R) reader structure"]
impl crate::Readable for Sr1Spec {}
#[doc = "`write(|w| ..)` method takes [`sr1::W`](W) writer structure"]
impl crate::Writable for Sr1Spec {
    type Safety = crate::Unsafe;
}
