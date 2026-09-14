#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Field `RORRIS` reader - receive overrun interrupt raw status"]
pub type RorrisR = crate::BitReader;
#[doc = "Field `RTRIS` reader - receive timeout interrupt raw status"]
pub type RtrisR = crate::BitReader;
#[doc = "Field `RXRIS` reader - receive interrupt raw status"]
pub type RxrisR = crate::BitReader;
#[doc = "Field `TXRIS` reader - transmit interrupt raw status"]
pub type TxrisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - receive overrun interrupt raw status"]
    #[inline(always)]
    pub fn rorris(&self) -> RorrisR {
        RorrisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - receive timeout interrupt raw status"]
    #[inline(always)]
    pub fn rtris(&self) -> RtrisR {
        RtrisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - receive interrupt raw status"]
    #[inline(always)]
    pub fn rxris(&self) -> RxrisR {
        RxrisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - transmit interrupt raw status"]
    #[inline(always)]
    pub fn txris(&self) -> TxrisR {
        TxrisR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "raw interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
