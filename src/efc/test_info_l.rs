#[doc = "Register `TEST_INFO_L` reader"]
pub type R = crate::R<TestInfoLSpec>;
#[doc = "Field `INFO` reader - Test info low value"]
pub type InfoR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Test info low value"]
    #[inline(always)]
    pub fn info(&self) -> InfoR {
        InfoR::new(self.bits)
    }
}
#[doc = "test info low register\n\nYou can [`read`](crate::Reg::read) this register and get [`test_info_l::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestInfoLSpec;
impl crate::RegisterSpec for TestInfoLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test_info_l::R`](R) reader structure"]
impl crate::Readable for TestInfoLSpec {}
