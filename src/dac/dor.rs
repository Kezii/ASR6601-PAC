#[doc = "Register `DOR` reader"]
pub type R = crate::R<DorSpec>;
#[doc = "Field `DATA` reader - Converted output data, 12-bit"]
pub type DataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Converted output data, 12-bit"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "data output register\n\nYou can [`read`](crate::Reg::read) this register and get [`dor::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DorSpec;
impl crate::RegisterSpec for DorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dor::R`](R) reader structure"]
impl crate::Readable for DorSpec {}
