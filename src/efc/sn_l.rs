#[doc = "Register `SN_L` reader"]
pub type R = crate::R<SnLSpec>;
#[doc = "Field `SERIAL_NUM_LOW` reader - Less significant 32 bits of the chip serial number"]
pub type SerialNumLowR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Less significant 32 bits of the chip serial number"]
    #[inline(always)]
    pub fn serial_num_low(&self) -> SerialNumLowR {
        SerialNumLowR::new(self.bits)
    }
}
#[doc = "serial number low register\n\nYou can [`read`](crate::Reg::read) this register and get [`sn_l::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SnLSpec;
impl crate::RegisterSpec for SnLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sn_l::R`](R) reader structure"]
impl crate::Readable for SnLSpec {}
