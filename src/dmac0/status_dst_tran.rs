#[doc = "Register `STATUS_DST_TRAN` reader"]
pub type R = crate::R<StatusDstTranSpec>;
#[doc = "Field `CHAN0_STATUS` reader - Destination processing completed status of DMA channel 0"]
pub type Chan0StatusR = crate::BitReader;
#[doc = "Field `CHAN1_STATUS` reader - Destination processing completed status of DMA channel 1"]
pub type Chan1StatusR = crate::BitReader;
#[doc = "Field `CHAN2_STATUS` reader - Destination processing completed status of DMA channel 2"]
pub type Chan2StatusR = crate::BitReader;
#[doc = "Field `CHAN3_STATUS` reader - Destination processing completed status of DMA channel 3"]
pub type Chan3StatusR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Destination processing completed status of DMA channel 0"]
    #[inline(always)]
    pub fn chan0_status(&self) -> Chan0StatusR {
        Chan0StatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Destination processing completed status of DMA channel 1"]
    #[inline(always)]
    pub fn chan1_status(&self) -> Chan1StatusR {
        Chan1StatusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Destination processing completed status of DMA channel 2"]
    #[inline(always)]
    pub fn chan2_status(&self) -> Chan2StatusR {
        Chan2StatusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Destination processing completed status of DMA channel 3"]
    #[inline(always)]
    pub fn chan3_status(&self) -> Chan3StatusR {
        Chan3StatusR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "DMA destination processing completed interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status_dst_tran::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusDstTranSpec;
impl crate::RegisterSpec for StatusDstTranSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`status_dst_tran::R`](R) reader structure"]
impl crate::Readable for StatusDstTranSpec {}
