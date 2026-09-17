#[doc = "Register `PERIPHID7` reader"]
pub type R = crate::R<Periphid7Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "peripheral id register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Periphid7Spec;
impl crate::RegisterSpec for Periphid7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periphid7::R`](R) reader structure"]
impl crate::Readable for Periphid7Spec {}
