#[doc = "Register `PERIPHID5` reader"]
pub type R = crate::R<Periphid5Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "peripheral id register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Periphid5Spec;
impl crate::RegisterSpec for Periphid5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periphid5::R`](R) reader structure"]
impl crate::Readable for Periphid5Spec {}
