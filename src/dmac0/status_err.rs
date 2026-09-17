#[doc = "Register `STATUS_ERR` reader"]
pub type R = crate::R<StatusErrSpec>;
#[doc = "Field `CHAN0_STATUS` reader - Transmission error status of DMA channel 0"]
pub type Chan0StatusR = crate::BitReader;
#[doc = "Field `CHAN1_STATUS` reader - Transmission error status of DMA channel 1"]
pub type Chan1StatusR = crate::BitReader;
#[doc = "Field `CHAN2_STATUS` reader - Transmission error status of DMA channel 2"]
pub type Chan2StatusR = crate::BitReader;
#[doc = "Field `CHAN3_STATUS` reader - Transmission error status of DMA channel 3"]
pub type Chan3StatusR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmission error status of DMA channel 0"]
    #[inline(always)]
    pub fn chan0_status(&self) -> Chan0StatusR {
        Chan0StatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmission error status of DMA channel 1"]
    #[inline(always)]
    pub fn chan1_status(&self) -> Chan1StatusR {
        Chan1StatusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmission error status of DMA channel 2"]
    #[inline(always)]
    pub fn chan2_status(&self) -> Chan2StatusR {
        Chan2StatusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmission error status of DMA channel 3"]
    #[inline(always)]
    pub fn chan3_status(&self) -> Chan3StatusR {
        Chan3StatusR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "DMA transmission error status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status_err::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusErrSpec;
impl crate::RegisterSpec for StatusErrSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`status_err::R`](R) reader structure"]
impl crate::Readable for StatusErrSpec {}
