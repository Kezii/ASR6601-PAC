#[doc = "Register `SN_H` reader"]
pub type R = crate::R<SnHSpec>;
#[doc = "Field `SERIAL_NUM_HIGH` reader - More significant 32 bits of the chip serial number"]
pub type SerialNumHighR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - More significant 32 bits of the chip serial number"]
    #[inline(always)]
    pub fn serial_num_high(&self) -> SerialNumHighR {
        SerialNumHighR::new(self.bits)
    }
}
#[doc = "serial number high register\n\nYou can [`read`](crate::Reg::read) this register and get [`sn_h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SnHSpec;
impl crate::RegisterSpec for SnHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sn_h::R`](R) reader structure"]
impl crate::Readable for SnHSpec {}
