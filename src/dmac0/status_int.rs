#[doc = "Register `STATUS_INT` reader"]
pub type R = crate::R<StatusIntSpec>;
#[doc = "Field `TFR` reader - Transfer-complete interrupt status"]
pub type TfrR = crate::BitReader;
#[doc = "Field `BLOCK` reader - Block-complete interrupt status"]
pub type BlockR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transfer-complete interrupt status"]
    #[inline(always)]
    pub fn tfr(&self) -> TfrR {
        TfrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Block-complete interrupt status"]
    #[inline(always)]
    pub fn block(&self) -> BlockR {
        BlockR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "combined DMA interrupt status; bit 0 transfer, bit 1 block\n\nYou can [`read`](crate::Reg::read) this register and get [`status_int::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusIntSpec;
impl crate::RegisterSpec for StatusIntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_int::R`](R) reader structure"]
impl crate::Readable for StatusIntSpec {}
