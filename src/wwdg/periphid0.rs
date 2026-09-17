#[doc = "Register `PERIPHID0` reader"]
pub type R = crate::R<Periphid0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "peripheral id register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Periphid0Spec;
impl crate::RegisterSpec for Periphid0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periphid0::R`](R) reader structure"]
impl crate::Readable for Periphid0Spec {}
