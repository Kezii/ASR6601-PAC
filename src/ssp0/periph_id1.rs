#[doc = "Register `PERIPH_ID1` reader"]
pub type R = crate::R<PeriphId1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "peripheral identification register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`periph_id1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriphId1Spec;
impl crate::RegisterSpec for PeriphId1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periph_id1::R`](R) reader structure"]
impl crate::Readable for PeriphId1Spec {}
