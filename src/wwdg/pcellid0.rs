#[doc = "Register `PCELLID0` reader"]
pub type R = crate::R<Pcellid0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "component id register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pcellid0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcellid0Spec;
impl crate::RegisterSpec for Pcellid0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcellid0::R`](R) reader structure"]
impl crate::Readable for Pcellid0Spec {}
