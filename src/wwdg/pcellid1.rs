#[doc = "Register `PCELLID1` reader"]
pub type R = crate::R<Pcellid1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "component id register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pcellid1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcellid1Spec;
impl crate::RegisterSpec for Pcellid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcellid1::R`](R) reader structure"]
impl crate::Readable for Pcellid1Spec {}
