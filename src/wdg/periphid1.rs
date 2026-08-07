#[doc = "Register `PERIPHID1` reader"]
pub type R = crate::R<Periphid1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "peripheral id register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Periphid1Spec;
impl crate::RegisterSpec for Periphid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periphid1::R`](R) reader structure"]
impl crate::Readable for Periphid1Spec {}
