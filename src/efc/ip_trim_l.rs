#[doc = "Register `IP_TRIM_L` reader"]
pub type R = crate::R<IpTrimLSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "analog ip trimming low register\n\nYou can [`read`](crate::Reg::read) this register and get [`ip_trim_l::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpTrimLSpec;
impl crate::RegisterSpec for IpTrimLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ip_trim_l::R`](R) reader structure"]
impl crate::Readable for IpTrimLSpec {}
