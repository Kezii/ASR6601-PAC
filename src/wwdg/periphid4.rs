#[doc = "Register `PERIPHID4` reader"]
pub type R = crate::R<Periphid4Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "peripheral id register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Periphid4Spec;
impl crate::RegisterSpec for Periphid4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periphid4::R`](R) reader structure"]
impl crate::Readable for Periphid4Spec {}
