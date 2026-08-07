#[doc = "Register `SN_H` reader"]
pub type R = crate::R<SnHSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "serial number high register\n\nYou can [`read`](crate::Reg::read) this register and get [`sn_h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SnHSpec;
impl crate::RegisterSpec for SnHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sn_h::R`](R) reader structure"]
impl crate::Readable for SnHSpec {}
