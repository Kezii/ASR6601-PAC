#[doc = "Register `I2S_COMP_TYPE` reader"]
pub type R = crate::R<I2sCompTypeSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "component type register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_comp_type::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sCompTypeSpec;
impl crate::RegisterSpec for I2sCompTypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_comp_type::R`](R) reader structure"]
impl crate::Readable for I2sCompTypeSpec {}
