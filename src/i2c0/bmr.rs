#[doc = "Register `BMR` reader"]
pub type R = crate::R<BmrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "bus monitor register\n\nYou can [`read`](crate::Reg::read) this register and get [`bmr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BmrSpec;
impl crate::RegisterSpec for BmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmr::R`](R) reader structure"]
impl crate::Readable for BmrSpec {}
