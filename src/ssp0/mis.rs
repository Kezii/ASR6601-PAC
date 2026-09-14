#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Field `RORMIS` reader - receive overrun interrupt masked status"]
pub type RormisR = crate::BitReader;
#[doc = "Field `RTMIS` reader - receive timeout interrupt masked status"]
pub type RtmisR = crate::BitReader;
#[doc = "Field `RXMIS` reader - receive interrupt masked status"]
pub type RxmisR = crate::BitReader;
#[doc = "Field `TXMIS` reader - transmit interrupt masked status"]
pub type TxmisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - receive overrun interrupt masked status"]
    #[inline(always)]
    pub fn rormis(&self) -> RormisR {
        RormisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - receive timeout interrupt masked status"]
    #[inline(always)]
    pub fn rtmis(&self) -> RtmisR {
        RtmisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - receive interrupt masked status"]
    #[inline(always)]
    pub fn rxmis(&self) -> RxmisR {
        RxmisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - transmit interrupt masked status"]
    #[inline(always)]
    pub fn txmis(&self) -> TxmisR {
        TxmisR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
