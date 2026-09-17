#[doc = "Register `PERIPHID3` reader"]
pub type R = crate::R<Periphid3Spec>;
#[doc = "Field `CONFIGURATION` reader - configuration, fixed 0x00"]
pub type ConfigurationR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - configuration, fixed 0x00"]
    #[inline(always)]
    pub fn configuration(&self) -> ConfigurationR {
        ConfigurationR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "peripheral ID register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Periphid3Spec;
impl crate::RegisterSpec for Periphid3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periphid3::R`](R) reader structure"]
impl crate::Readable for Periphid3Spec {}
