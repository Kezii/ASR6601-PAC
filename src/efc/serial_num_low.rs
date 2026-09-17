#[doc = "Register `SERIAL_NUM_LOW` reader"]
pub type R = crate::R<SerialNumLowSpec>;
#[doc = "Field `SERIAL_NUM_LOW` reader - Less significant 32 bits of the chip serial number"]
pub type SerialNumLowR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Less significant 32 bits of the chip serial number"]
    #[inline(always)]
    pub fn serial_num_low(&self) -> SerialNumLowR {
        SerialNumLowR::new(self.bits)
    }
}
#[doc = "serial number low register\n\nYou can [`read`](crate::Reg::read) this register and get [`serial_num_low::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SerialNumLowSpec;
impl crate::RegisterSpec for SerialNumLowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`serial_num_low::R`](R) reader structure"]
impl crate::Readable for SerialNumLowSpec {}
