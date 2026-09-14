#[doc = "Register `FR` reader"]
pub type R = crate::R<FrSpec>;
#[doc = "Field `BUSY` reader - uart busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `RXFE` reader - receive fifo empty"]
pub type RxfeR = crate::BitReader;
#[doc = "Field `TXFF` reader - transmit fifo full"]
pub type TxffR = crate::BitReader;
#[doc = "Field `RXFF` reader - receive fifo full"]
pub type RxffR = crate::BitReader;
#[doc = "Field `TXFE` reader - transmit fifo empty"]
pub type TxfeR = crate::BitReader;
impl R {
    #[doc = "Bit 3 - uart busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - receive fifo empty"]
    #[inline(always)]
    pub fn rxfe(&self) -> RxfeR {
        RxfeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - transmit fifo full"]
    #[inline(always)]
    pub fn txff(&self) -> TxffR {
        TxffR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - receive fifo full"]
    #[inline(always)]
    pub fn rxff(&self) -> RxffR {
        RxffR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - transmit fifo empty"]
    #[inline(always)]
    pub fn txfe(&self) -> TxfeR {
        TxfeR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`fr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrSpec;
impl crate::RegisterSpec for FrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fr::R`](R) reader structure"]
impl crate::Readable for FrSpec {}
