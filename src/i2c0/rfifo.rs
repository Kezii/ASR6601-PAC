#[doc = "Register `RFIFO` reader"]
pub type R = crate::R<RfifoSpec>;
#[doc = "Field `DATA` reader - i2c bus receive data for read transactions"]
pub type DataR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - i2c bus receive data for read transactions"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "read fifo register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfifo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfifoSpec;
impl crate::RegisterSpec for RfifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfifo::R`](R) reader structure"]
impl crate::Readable for RfifoSpec {}
