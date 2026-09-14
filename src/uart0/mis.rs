#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Field `RXMIS` reader - receive interrupt masked status"]
pub type RxmisR = crate::BitReader;
#[doc = "Field `TXMIS` reader - transmit interrupt masked status"]
pub type TxmisR = crate::BitReader;
#[doc = "Field `RTMIS` reader - receive timeout interrupt masked status"]
pub type RtmisR = crate::BitReader;
#[doc = "Field `FEMIS` reader - framing error interrupt masked status"]
pub type FemisR = crate::BitReader;
#[doc = "Field `PEMIS` reader - parity error interrupt masked status"]
pub type PemisR = crate::BitReader;
#[doc = "Field `BEMIS` reader - break error interrupt masked status"]
pub type BemisR = crate::BitReader;
#[doc = "Field `OEMIS` reader - overrun error interrupt masked status"]
pub type OemisR = crate::BitReader;
impl R {
    #[doc = "Bit 4 - receive interrupt masked status"]
    #[inline(always)]
    pub fn rxmis(&self) -> RxmisR {
        RxmisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - transmit interrupt masked status"]
    #[inline(always)]
    pub fn txmis(&self) -> TxmisR {
        TxmisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - receive timeout interrupt masked status"]
    #[inline(always)]
    pub fn rtmis(&self) -> RtmisR {
        RtmisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - framing error interrupt masked status"]
    #[inline(always)]
    pub fn femis(&self) -> FemisR {
        FemisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - parity error interrupt masked status"]
    #[inline(always)]
    pub fn pemis(&self) -> PemisR {
        PemisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - break error interrupt masked status"]
    #[inline(always)]
    pub fn bemis(&self) -> BemisR {
        BemisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - overrun error interrupt masked status"]
    #[inline(always)]
    pub fn oemis(&self) -> OemisR {
        OemisR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
