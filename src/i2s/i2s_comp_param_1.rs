#[doc = "Register `I2S_COMP_PARAM_1` reader"]
pub type R = crate::R<I2sCompParam1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "component parameter register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_comp_param_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sCompParam1Spec;
impl crate::RegisterSpec for I2sCompParam1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_comp_param_1::R`](R) reader structure"]
impl crate::Readable for I2sCompParam1Spec {}
