#[doc = "Register `TEST_INFO_L` reader"]
pub type R = crate::R<TestInfoLSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "test info low register\n\nYou can [`read`](crate::Reg::read) this register and get [`test_info_l::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestInfoLSpec;
impl crate::RegisterSpec for TestInfoLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test_info_l::R`](R) reader structure"]
impl crate::Readable for TestInfoLSpec {}
