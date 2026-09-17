#[doc = "Register `IP_TRIM_L` reader"]
pub type R = crate::R<IpTrimLSpec>;
#[doc = "Field `TRIM` reader - Analog ip trimming low value"]
pub type TrimR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Analog ip trimming low value"]
    #[inline(always)]
    pub fn trim(&self) -> TrimR {
        TrimR::new(self.bits)
    }
}
#[doc = "analog ip trimming low register\n\nYou can [`read`](crate::Reg::read) this register and get [`ip_trim_l::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpTrimLSpec;
impl crate::RegisterSpec for IpTrimLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ip_trim_l::R`](R) reader structure"]
impl crate::Readable for IpTrimLSpec {}
