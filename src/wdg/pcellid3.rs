#[doc = "Register `PCELLID3` reader"]
pub type R = crate::R<Pcellid3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "component id register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`pcellid3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcellid3Spec;
impl crate::RegisterSpec for Pcellid3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcellid3::R`](R) reader structure"]
impl crate::Readable for Pcellid3Spec {}
