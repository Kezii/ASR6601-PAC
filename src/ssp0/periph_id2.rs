#[doc = "Register `PERIPH_ID2` reader"]
pub type R = crate::R<PeriphId2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "peripheral identification register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`periph_id2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriphId2Spec;
impl crate::RegisterSpec for PeriphId2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periph_id2::R`](R) reader structure"]
impl crate::Readable for PeriphId2Spec {}
