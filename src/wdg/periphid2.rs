#[doc = "Register `PERIPHID2` reader"]
pub type R = crate::R<Periphid2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "peripheral id register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Periphid2Spec;
impl crate::RegisterSpec for Periphid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periphid2::R`](R) reader structure"]
impl crate::Readable for Periphid2Spec {}
