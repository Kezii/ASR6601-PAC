#[doc = "Register `PERIPHID3` reader"]
pub type R = crate::R<Periphid3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "peripheral id register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Periphid3Spec;
impl crate::RegisterSpec for Periphid3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periphid3::R`](R) reader structure"]
impl crate::Readable for Periphid3Spec {}
