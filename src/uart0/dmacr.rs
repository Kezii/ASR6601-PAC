#[doc = "Register `DMACR` reader"]
pub type R = crate::R<DmacrSpec>;
#[doc = "Register `DMACR` writer"]
pub type W = crate::W<DmacrSpec>;
#[doc = "Field `RX_EN` reader - Rx en"]
pub type RxEnR = crate::BitReader;
#[doc = "Field `RX_EN` writer - Rx en"]
pub type RxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_EN` reader - Tx en"]
pub type TxEnR = crate::BitReader;
#[doc = "Field `TX_EN` writer - Tx en"]
pub type TxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONERR_EN` reader - Onerr en"]
pub type OnerrEnR = crate::BitReader;
#[doc = "Field `ONERR_EN` writer - Onerr en"]
pub type OnerrEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rx en"]
    #[inline(always)]
    pub fn rx_en(&self) -> RxEnR {
        RxEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx en"]
    #[inline(always)]
    pub fn tx_en(&self) -> TxEnR {
        TxEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Onerr en"]
    #[inline(always)]
    pub fn onerr_en(&self) -> OnerrEnR {
        OnerrEnR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx en"]
    #[inline(always)]
    pub fn rx_en(&mut self) -> RxEnW<'_, DmacrSpec> {
        RxEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Tx en"]
    #[inline(always)]
    pub fn tx_en(&mut self) -> TxEnW<'_, DmacrSpec> {
        TxEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Onerr en"]
    #[inline(always)]
    pub fn onerr_en(&mut self) -> OnerrEnW<'_, DmacrSpec> {
        OnerrEnW::new(self, 2)
    }
}
#[doc = "DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacrSpec;
impl crate::RegisterSpec for DmacrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacr::R`](R) reader structure"]
impl crate::Readable for DmacrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacr::W`](W) writer structure"]
impl crate::Writable for DmacrSpec {
    type Safety = crate::Unsafe;
}
