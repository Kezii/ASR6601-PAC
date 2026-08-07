#[doc = "Register `IP_TRIM_H` reader"]
pub type R = crate::R<IpTrimHSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "analog ip trimming high register\n\nYou can [`read`](crate::Reg::read) this register and get [`ip_trim_h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpTrimHSpec;
impl crate::RegisterSpec for IpTrimHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ip_trim_h::R`](R) reader structure"]
impl crate::Readable for IpTrimHSpec {}
