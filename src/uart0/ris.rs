#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Field `RXRIS` reader - receive interrupt raw status"]
pub type RxrisR = crate::BitReader;
#[doc = "Field `TXRIS` reader - transmit interrupt raw status"]
pub type TxrisR = crate::BitReader;
#[doc = "Field `RTRIS` reader - receive timeout interrupt raw status"]
pub type RtrisR = crate::BitReader;
#[doc = "Field `FERIS` reader - framing error interrupt raw status"]
pub type FerisR = crate::BitReader;
#[doc = "Field `PERIS` reader - parity error interrupt raw status"]
pub type PerisR = crate::BitReader;
#[doc = "Field `BERIS` reader - break error interrupt raw status"]
pub type BerisR = crate::BitReader;
#[doc = "Field `OERIS` reader - overrun error interrupt raw status"]
pub type OerisR = crate::BitReader;
impl R {
    #[doc = "Bit 4 - receive interrupt raw status"]
    #[inline(always)]
    pub fn rxris(&self) -> RxrisR {
        RxrisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - transmit interrupt raw status"]
    #[inline(always)]
    pub fn txris(&self) -> TxrisR {
        TxrisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - receive timeout interrupt raw status"]
    #[inline(always)]
    pub fn rtris(&self) -> RtrisR {
        RtrisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - framing error interrupt raw status"]
    #[inline(always)]
    pub fn feris(&self) -> FerisR {
        FerisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - parity error interrupt raw status"]
    #[inline(always)]
    pub fn peris(&self) -> PerisR {
        PerisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - break error interrupt raw status"]
    #[inline(always)]
    pub fn beris(&self) -> BerisR {
        BerisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - overrun error interrupt raw status"]
    #[inline(always)]
    pub fn oeris(&self) -> OerisR {
        OerisR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "raw interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
