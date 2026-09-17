#[doc = "Register `IP_TRIM_H` reader"]
pub type R = crate::R<IpTrimHSpec>;
#[doc = "Field `TRIM` reader - Analog ip trimming high value"]
pub type TrimR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Analog ip trimming high value"]
    #[inline(always)]
    pub fn trim(&self) -> TrimR {
        TrimR::new(self.bits)
    }
}
#[doc = "analog ip trimming high register\n\nYou can [`read`](crate::Reg::read) this register and get [`ip_trim_h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpTrimHSpec;
impl crate::RegisterSpec for IpTrimHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ip_trim_h::R`](R) reader structure"]
impl crate::Readable for IpTrimHSpec {}
