#[doc = "Register `RFIFO_WPTR` reader"]
pub type R = crate::R<RfifoWptrSpec>;
#[doc = "Field `DATA` reader - position in the receive fifo where hardware will write the next entry"]
pub type DataR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - position in the receive fifo where hardware will write the next entry"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "read fifo write pointer register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfifo_wptr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfifoWptrSpec;
impl crate::RegisterSpec for RfifoWptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfifo_wptr::R`](R) reader structure"]
impl crate::Readable for RfifoWptrSpec {}
