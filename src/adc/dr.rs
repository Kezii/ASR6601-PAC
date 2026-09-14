#[doc = "Register `DR` reader"]
pub type R = crate::R<DrSpec>;
#[doc = "Field `DATA` reader - adc converted data, bit 11 is sign in differential mode"]
pub type DataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - adc converted data, bit 11 is sign in differential mode"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DrSpec;
impl crate::RegisterSpec for DrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DrSpec {}
