#[doc = "Register `I2S_COMP_VERSION` reader"]
pub type R = crate::R<I2sCompVersionSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "component version register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_comp_version::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sCompVersionSpec;
impl crate::RegisterSpec for I2sCompVersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_comp_version::R`](R) reader structure"]
impl crate::Readable for I2sCompVersionSpec {}
